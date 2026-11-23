use eframe::egui;
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use ra_ap_rustc_lexer::{FrontmatterAllowed, TokenKind, tokenize};
use speech_to_text::{Failure, embedded};
use speech_to_text::app::{App, Dialogs, Event, Job, Provider, cli, serve};
use speech_to_text::catalog::Model;
use speech_to_text::export::ExportFormat;
use speech_to_text::whisper::ModelFiles;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender, channel};

const TRANSCRIPT: &str = "The quick brown fox jumps over the lazy dog.";

const LONG: &str = "this recording is longer than 30 seconds. it exists to check that a long recording is \
    transcribed from its first word to its last. on monday the baker opened the shop at 7 in the morning. on tuesday \
    the weather was cold, and the streets were quiet. on wednesday a letter arrived from the north. on thursday the \
    garden was watered before breakfast. on friday everyone went home early. and the lights were turned off. the \
    last sentence comes now. the quick brown fox jumps over the lazy dog.";

static ASKED: Mutex<Vec<Model>> = Mutex::new(Vec::new());

fn project(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
}

fn recorded(model: Model) -> ModelFiles<'static> {
    ASKED.lock().unwrap().push(model);
    embedded(model)
}

fn outcomes(fixture: &str, models: &[Model], files: Provider) -> Vec<Result<String, Failure>> {
    let (jobs, queue) = channel();
    let (results, events) = channel();
    for model in models {
        jobs.send(Job { audio: project("tests/fixtures").join(fixture), model: *model }).unwrap();
    }
    drop(jobs);
    serve(queue, results, files, || ());
    let finished = |event| if let Event::Finished(outcome) = event { Some(outcome) } else { None };
    events.iter().filter_map(finished).collect()
}

fn transcribed(fixture: &str) -> String {
    outcomes(fixture, &[Model::Tiny], embedded).remove(0).unwrap().to_lowercase()
}

fn command_line(arguments: &[&str]) -> std::process::Output {
    let program = env!("CARGO_BIN_EXE_speech-to-text");
    std::process::Command::new(program).args(arguments).current_dir(project("tests/fixtures")).output().unwrap()
}

struct Chosen {
    audio: PathBuf,
    export: PathBuf,
}

impl Dialogs for Chosen {
    fn open_audio(&self) -> Option<PathBuf> {
        Some(self.audio.clone())
    }

    fn save_export(&self, _format: ExportFormat, _name: &str) -> Option<PathBuf> {
        Some(self.export.clone())
    }
}

#[derive(Debug)]
struct Dropped(PathBuf);

impl egui::DroppedFile for Dropped {
    fn path(&self) -> &Path {
        &self.0
    }

    fn bytes(&self) -> Result<Vec<u8>, String> {
        fs::read(&self.0).map_err(|error| error.to_string())
    }
}

struct Bench {
    harness: Harness<'static, App>,
    jobs: Receiver<Job>,
    events: Sender<Event>,
    export: PathBuf,
}

impl Bench {
    fn new(test: &str) -> Bench {
        let (jobs, queue) = channel();
        let (events, results) = channel();
        let export = std::env::temp_dir().join(format!("speech-to-text-{test}"));
        let chosen = Chosen { audio: PathBuf::from("voice note.ogg"), export: export.clone() };
        let app = App::new(Box::new(chosen), jobs, results);
        let harness = Harness::new_ui_state(|ui, app: &mut App| app.show(ui), app);
        Bench { harness, jobs: queue, events, export }
    }

    fn click(&mut self, label: &str) {
        self.harness.get_by_label(label).click();
        self.harness.run();
    }

    fn drop_file(&mut self, name: &str) {
        self.harness.input_mut().dropped_files.push(Arc::new(Dropped(PathBuf::from(name))));
        self.harness.run();
    }

    fn transcribe(&mut self) -> Job {
        self.click("Open audio file…");
        let job = self.jobs.try_recv().unwrap();
        self.events.send(Event::Finished(Ok(TRANSCRIPT.to_string()))).unwrap();
        self.harness.run();
        job
    }

    fn exported(&mut self, button: &str) -> String {
        self.transcribe();
        self.click(button);
        fs::read_to_string(&self.export).unwrap()
    }
}

fn sources() -> Vec<(PathBuf, String)> {
    ["src", "prebuild", "tests"]
        .iter()
        .flat_map(|folder| fs::read_dir(project(folder)).unwrap())
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "rs"))
        .map(|path| (path.clone(), fs::read_to_string(path).unwrap()))
        .collect()
}

fn tokens(source: &str) -> Vec<(TokenKind, &str)> {
    let mut start = 0;
    tokenize(source, FrontmatterAllowed::No)
        .map(|token| {
            let text = &source[start..start + token.len as usize];
            start += token.len as usize;
            (token.kind, text)
        })
        .collect()
}

fn longest_function(source: &str) -> usize {
    let code: Vec<(TokenKind, &str)> =
        tokens(source).into_iter().filter(|(kind, _)| *kind != TokenKind::Whitespace).collect();
    let named = |at: usize| code[at].1 == "fn" && code.get(at + 1).is_some_and(|next| next.0 == TokenKind::Ident);
    let offset = |at: usize| code[at].1.as_ptr() as usize - source.as_ptr() as usize;
    let mut longest = 0;
    for start in (0..code.len()).filter(|at| named(*at)) {
        let mut depth = 0;
        for end in start..code.len() {
            match code[end].0 {
                TokenKind::Semi if depth == 0 => break,
                TokenKind::OpenBrace => depth += 1,
                TokenKind::CloseBrace => depth -= 1,
                _ => continue,
            }
            if depth == 0 {
                let function = &source[offset(start)..=offset(end)];
                longest = longest.max(function.lines().filter(|line| !line.trim().is_empty()).count());
                break;
            }
        }
    }
    longest
}

#[test]
fn test_01_an_ogg_vorbis_file_is_transcribed() {
    assert!(transcribed("fox_vorbis.ogg").contains("quick brown fox"));
}

#[test]
fn test_02_an_ogg_opus_voice_note_is_transcribed() {
    assert!(transcribed("fox_opus_silk.ogg").contains("quick brown fox"));
}

#[test]
fn test_03_an_ogg_opus_file_in_celt_mode_is_transcribed() {
    assert!(transcribed("fox_opus_celt.ogg").contains("quick brown fox"));
}

#[test]
fn test_04_a_wav_file_is_transcribed() {
    assert!(transcribed("fox.wav").contains("quick brown fox"));
}

#[test]
fn test_05_a_stereo_recording_is_transcribed() {
    assert!(transcribed("fox_opus_stereo.ogg").contains("quick brown fox"));
}

#[test]
fn test_06_a_recording_over_thirty_seconds_is_transcribed_to_its_end() {
    assert_eq!(transcribed("long_vorbis.ogg"), LONG);
}

#[test]
fn test_07_a_recording_without_speech_gives_no_text() {
    let nothing = Failure("No speech was found in the audio file.".to_string());
    assert_eq!(outcomes("silence.ogg", &[Model::Tiny], embedded), [Err(nothing)]);
}

#[test]
fn test_08_the_transcript_is_shown_in_a_text_field() {
    let mut bench = Bench::new("test-08");
    bench.transcribe();
    let field = bench.harness.get_by_role(egui::accesskit::Role::MultilineTextInput);
    assert_eq!(field.value().as_deref(), Some(TRANSCRIPT));
}

#[test]
fn test_09_the_copy_button_copies_the_transcript() {
    let mut bench = Bench::new("test-09");
    bench.transcribe();
    bench.harness.get_by_label("Copy").click();
    bench.harness.step();
    let copied = egui::OutputCommand::CopyText(TRANSCRIPT.to_string());
    assert!(bench.harness.output().platform_output.commands.contains(&copied));
}

#[test]
fn test_10_the_transcript_is_exported_as_txt() {
    let mut bench = Bench::new("test-10.txt");
    assert_eq!(bench.exported("Export as .txt"), format!("{TRANSCRIPT}\n"));
}

#[test]
fn test_11_the_transcript_is_exported_as_md() {
    let mut bench = Bench::new("test-11.md");
    assert_eq!(bench.exported("Export as .md"), format!("# voice note\n\n{TRANSCRIPT}\n"));
}

#[test]
fn test_12_the_model_chosen_in_the_app_transcribes() {
    let mut bench = Bench::new("test-12");
    bench.click("Model");
    bench.click("base");
    assert_eq!(bench.transcribe().model, Model::Base);
}

#[test]
fn test_13_each_job_is_transcribed_by_its_own_model() {
    outcomes("fox.wav", &[Model::Tiny, Model::Base], recorded);
    assert_eq!(*ASKED.lock().unwrap(), [Model::Tiny, Model::Base]);
}

#[test]
fn test_14_a_file_dropped_on_the_window_is_transcribed() {
    let mut bench = Bench::new("test-14");
    bench.drop_file("dropped.ogg");
    assert_eq!(bench.jobs.try_recv().unwrap().audio, PathBuf::from("dropped.ogg"));
}

#[test]
fn test_15_a_file_dropped_during_a_transcription_starts_no_job() {
    let mut bench = Bench::new("test-15");
    bench.drop_file("first.ogg");
    bench.drop_file("second.ogg");
    assert_eq!(bench.jobs.try_iter().count(), 1);
}

#[test]
fn test_16_the_command_line_writes_the_transcript_to_standard_output() {
    let run = command_line(&["transcribe", "fox.wav", "--model", "tiny"]);
    assert!(String::from_utf8_lossy(&run.stdout).to_lowercase().contains("quick brown fox"));
}

#[test]
fn test_17_the_model_named_on_the_command_line_goes_with_the_job() {
    let arguments = ["transcribe", "voice note.ogg", "--model", "base"].map(std::ffi::OsString::from);
    let model = match cli::parse(&arguments) {
        Ok(cli::Command::Transcribe(request)) => Some(request.job.model),
        _ => None,
    };
    assert_eq!(model, Some(Model::Base));
}

#[test]
fn test_18_the_command_line_saves_the_transcript_to_the_file_named_by_output() {
    let saved = std::env::temp_dir().join("speech-to-text-test-18.md");
    let _ = fs::remove_file(&saved);
    command_line(&["transcribe", "fox.wav", "--model", "tiny", "--format", "md", "--output", saved.to_str().unwrap()]);
    assert!(fs::read_to_string(&saved).unwrap().to_lowercase().starts_with("# fox

the quick brown fox"));
}

#[test]
fn test_19_a_failure_ends_the_command_line_with_an_error_status() {
    assert_eq!(command_line(&["transcribe", "silence.ogg", "--model", "tiny"]).status.code(), Some(1));
}

#[test]
fn test_20_an_audio_file_named_as_the_only_argument_opens_the_window() {
    let first = match cli::parse(&[std::ffi::OsString::from("voice note.ogg")]) {
        Ok(cli::Command::Window(first)) => first,
        _ => None,
    };
    assert_eq!(first, Some(PathBuf::from("voice note.ogg")));
}

#[test]
fn test_21_the_code_has_no_comments() {
    let is_comment = |kind: TokenKind| matches!(kind, TokenKind::LineComment { .. } | TokenKind::BlockComment { .. });
    let commented: Vec<PathBuf> = sources()
        .into_iter()
        .filter(|(_, source)| tokens(source).iter().any(|(kind, _)| is_comment(*kind)))
        .map(|(path, _)| path)
        .collect();
    assert_eq!(commented, Vec::<PathBuf>::new());
}

#[test]
fn test_22_no_function_exceeds_60_lines() {
    let longest = sources().iter().map(|(_, source)| longest_function(source)).max();
    assert!(longest <= Some(60), "the longest function has {longest:?} lines");
}
