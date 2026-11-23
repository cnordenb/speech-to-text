use crate::catalog::Model;
use crate::export::{ExportFormat, export};
use crate::whisper::{ModelFiles, Transcriber};
use crate::{Failure, audio};
use eframe::egui;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender, channel};

pub mod cli;

const AUDIO: [&str; 9] = ["ogg", "oga", "opus", "wav", "mp3", "flac", "m4a", "aac", "mp4"];

pub struct Job {
    pub audio: PathBuf,
    pub model: Model,
}

pub enum Event {
    Progress(f32),
    Finished(Result<String, Failure>),
}

pub type Provider = fn(Model) -> ModelFiles<'static>;

pub trait Dialogs {
    fn open_audio(&self) -> Option<PathBuf>;
    fn save_export(&self, format: ExportFormat, name: &str) -> Option<PathBuf>;
}

struct Native;

enum Status {
    Idle,
    Working(f32),
    Done,
    Failed(String),
    Exported(PathBuf),
}

pub struct App {
    dialogs: Box<dyn Dialogs>,
    jobs: Sender<Job>,
    events: Receiver<Event>,
    model: Model,
    title: String,
    transcript: String,
    status: Status,
}

impl Job {
    pub fn title(&self) -> String {
        self.audio.file_stem().unwrap_or_default().to_string_lossy().into_owned()
    }
}

pub fn serve(jobs: impl IntoIterator<Item = Job>, events: Sender<Event>, files: Provider, wake: impl Fn()) {
    let mut loaded = None;
    let report = |event: Event| {
        if events.send(event).is_ok() {
            wake();
        }
    };
    for job in jobs {
        let mut progress = |progress| report(Event::Progress(progress));
        let attempt = catch_unwind(AssertUnwindSafe(|| transcribe(&job, files, &mut loaded, &mut progress)));
        let broken = |_| Err(Failure("The audio could not be transcribed.".to_string()));
        report(Event::Finished(attempt.unwrap_or_else(broken)));
    }
}

fn transcribe(
    job: &Job,
    files: Provider,
    loaded: &mut Option<(Model, Transcriber)>,
    progress: &mut dyn FnMut(f32),
) -> Result<String, Failure> {
    let samples = audio::decode(&job.audio)?;
    let transcriber = match loaded.take() {
        Some((model, transcriber)) if model == job.model => transcriber,
        _ => Transcriber::load(&files(job.model))?,
    };
    loaded.insert((job.model, transcriber)).1.transcribe(&samples, progress)
}

fn visuals() -> egui::Visuals {
    let mut visuals = egui::Visuals::dark();
    visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_gray(235);
    visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_gray(245);
    visuals.selection.bg_fill = egui::Color32::from_rgb(0, 70, 110);
    visuals.error_fg_color = egui::Color32::from_rgb(255, 150, 150);
    visuals
}

pub fn run(files: Provider, first: Option<PathBuf>) -> eframe::Result {
    let viewport = egui::ViewportBuilder::default().with_inner_size([760.0, 560.0]).with_min_inner_size([480.0, 320.0]);
    let options = eframe::NativeOptions { viewport, ..Default::default() };
    eframe::run_native(
        "speech-to-text",
        options,
        Box::new(move |creation| {
            creation.egui_ctx.set_theme(egui::Theme::Dark);
            creation.egui_ctx.set_visuals_of(egui::Theme::Dark, visuals());
            let (jobs, queue) = channel();
            let (results, events) = channel();
            let window = creation.egui_ctx.clone();
            std::thread::spawn(move || serve(queue, results, files, move || window.request_repaint()));
            let mut app = App::new(Box::new(Native), jobs, events);
            if let Some(audio) = first {
                app.start(audio);
            }
            Ok(Box::new(app))
        }),
    )
}

impl Dialogs for Native {
    fn open_audio(&self) -> Option<PathBuf> {
        rfd::FileDialog::new().add_filter("Audio", &AUDIO).add_filter("All files", &["*"]).pick_file()
    }

    fn save_export(&self, format: ExportFormat, name: &str) -> Option<PathBuf> {
        rfd::FileDialog::new().add_filter(format.extension(), &[format.extension()]).set_file_name(name).save_file()
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.show(ui);
    }
}

impl App {
    pub fn new(dialogs: Box<dyn Dialogs>, jobs: Sender<Job>, events: Receiver<Event>) -> App {
        let (model, title, transcript) = (Model::default(), String::new(), String::new());
        App { dialogs, jobs, events, model, title, transcript, status: Status::Idle }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.receive();
        let dropped = ui.input(|input| input.raw.dropped_files.first().map(|file| file.path().to_path_buf()));
        egui::Panel::top("controls").show(ui, |ui| {
            ui.add_enabled_ui(!matches!(self.status, Status::Working(_)), |ui| {
                ui.horizontal(|ui| self.controls(ui, dropped));
            });
            self.report(ui);
        });
        egui::Panel::bottom("actions").show(ui, |ui| {
            ui.add_enabled_ui(!self.transcript.is_empty(), |ui| ui.horizontal(|ui| self.actions(ui)));
        });
        egui::CentralPanel::default().show(ui, |ui| {
            let label = ui.label("Transcript");
            egui::ScrollArea::vertical().show(ui, |ui| {
                let field = egui::TextEdit::multiline(&mut self.transcript).desired_width(f32::INFINITY);
                ui.add_sized(ui.available_size(), field).labelled_by(label.id);
            });
        });
    }

    fn receive(&mut self) {
        while let Ok(event) = self.events.try_recv() {
            match event {
                Event::Progress(progress) => self.status = Status::Working(progress),
                Event::Finished(Ok(transcript)) => {
                    self.transcript = transcript;
                    self.status = Status::Done;
                }
                Event::Finished(Err(Failure(reason))) => self.status = Status::Failed(reason),
            }
        }
    }

    fn controls(&mut self, ui: &mut egui::Ui, dropped: Option<PathBuf>) {
        let opened = if ui.button("Open audio file…").clicked() { self.dialogs.open_audio() } else { None };
        let label = ui.label("Model");
        let chooser = egui::ComboBox::from_id_salt("model").selected_text(self.model.name());
        let choices = |ui: &mut egui::Ui| {
            for model in Model::ALL {
                ui.selectable_value(&mut self.model, model, model.name());
            }
        };
        chooser.show_ui(ui, choices).response.labelled_by(label.id);
        if let Some(audio) = opened.or(dropped).filter(|_| ui.is_enabled()) {
            self.start(audio);
        }
    }

    fn start(&mut self, audio: PathBuf) {
        let job = Job { audio, model: self.model };
        self.title = job.title();
        self.transcript.clear();
        self.status = match self.jobs.send(job) {
            Ok(()) => Status::Working(0.0),
            Err(error) => Status::Failed(error.to_string()),
        };
    }

    fn report(&self, ui: &mut egui::Ui) {
        match &self.status {
            Status::Idle => ui.label("Open an audio file, or drop one on this window."),
            Status::Working(progress) => {
                ui.label(format!("Transcribing {} with the {} model…", self.title, self.model.name()));
                ui.add(egui::ProgressBar::new(*progress))
            }
            Status::Done => ui.label(format!("Transcribed {}.", self.title)),
            Status::Failed(reason) => ui.colored_label(ui.visuals().error_fg_color, reason),
            Status::Exported(path) => ui.label(format!("Saved {}.", path.display())),
        };
    }

    fn actions(&mut self, ui: &mut egui::Ui) {
        if ui.button("Copy").clicked() {
            ui.ctx().copy_text(self.transcript.clone());
        }
        for (format, label) in [(ExportFormat::Text, "Export as .txt"), (ExportFormat::Markdown, "Export as .md")] {
            let name = format!("{}.{}", self.title, format.extension());
            let chosen = if ui.button(label).clicked() { self.dialogs.save_export(format, &name) } else { None };
            if let Some(path) = chosen {
                self.status = match export(&path, format, &self.title, &self.transcript) {
                    Ok(()) => Status::Exported(path),
                    Err(Failure(reason)) => Status::Failed(reason),
                };
            }
        }
    }
}
