use super::{Event, Job, Provider, run, serve};
use crate::Failure;
use crate::catalog::Model;
use crate::export::{ExportFormat, export};
use std::ffi::OsString;
use std::fmt::Display;
use std::io::{IsTerminal, stderr};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::slice::Iter;
use std::sync::mpsc::{Receiver, channel};
use std::thread;

const WRONG_USAGE: u8 = 2;

pub enum Command {
    Window(Option<PathBuf>),
    Transcribe(Request),
    Say(String),
}

pub struct Request {
    pub job: Job,
    pub format: ExportFormat,
    pub output: Option<PathBuf>,
}

pub fn start(files: Provider, args: Vec<OsString>) -> ExitCode {
    match parse(&args) {
        Ok(Command::Window(first)) => window(files, first),
        Ok(Command::Transcribe(request)) => transcribe(files, request),
        Ok(Command::Say(text)) => say(&text),
        Err(Failure(reason)) => wrong(&reason),
    }
}

pub fn parse(args: &[OsString]) -> Result<Command, Failure> {
    let option = |arg: &OsString| arg.to_string_lossy().starts_with('-');
    match args {
        [] => Ok(Command::Window(None)),
        [first, rest @ ..] if first == "transcribe" => transcription(rest).map(Command::Transcribe),
        [only] if only == "--help" || only == "-h" => Ok(Command::Say(usage())),
        [only] if only == "--version" => Ok(Command::Say(format!("speech-to-text {}\n", env!("CARGO_PKG_VERSION")))),
        [first, ..] if !option(first) => Ok(Command::Window(Some(PathBuf::from(first)))),
        _ => Err(Failure("The arguments were not understood.".to_string())),
    }
}

fn transcription(args: &[OsString]) -> Result<Request, Failure> {
    let job = Job { audio: PathBuf::new(), model: Model::default() };
    let mut request = Request { job, format: ExportFormat::default(), output: None };
    let mut files = Vec::new();
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.to_str().filter(|text| text.starts_with('-')) {
            Some(option) => set(&mut request, option, &mut args)?,
            None => files.push(PathBuf::from(arg)),
        }
    }
    request.job.audio = only(&files)?;
    Ok(request)
}

fn set(request: &mut Request, option: &str, rest: &mut Iter<OsString>) -> Result<(), Failure> {
    let mut given = || rest.next().ok_or_else(|| Failure(format!("{option} needs a value.")));
    match option {
        "--model" => request.job.model = choice(&Model::ALL, Model::name, given()?)?,
        "--format" => request.format = choice(&ExportFormat::ALL, ExportFormat::extension, given()?)?,
        "--output" => request.output = Some(PathBuf::from(given()?)),
        _ => return Err(Failure(format!("{option} is not an option."))),
    }
    Ok(())
}

fn only(files: &[PathBuf]) -> Result<PathBuf, Failure> {
    match files {
        [audio] => Ok(audio.clone()),
        [] => Err(Failure("Name the audio file to transcribe.".to_string())),
        _ => Err(Failure("Only one audio file can be transcribed at a time.".to_string())),
    }
}

fn choice<T: Copy>(choices: &[T], name: fn(T) -> &'static str, given: &OsString) -> Result<T, Failure> {
    let found = choices.iter().copied().find(|choice| given == name(*choice));
    let names: Vec<&str> = choices.iter().map(|choice| name(*choice)).collect();
    found.ok_or_else(|| Failure(format!("{} is not one of {}.", given.display(), names.join(", "))))
}

fn usage() -> String {
    let models: Vec<&str> = Model::ALL.iter().map(|model| model.name()).collect();
    let formats: Vec<&str> = ExportFormat::ALL.iter().map(|format| format.extension()).collect();
    format!(
        "speech-to-text transcribes the speech in an audio file to text.\n\n\
         Usage:\n  \
         speech-to-text [<audio file>]\n      \
         Opens the window, and transcribes the audio file if one is named.\n  \
         speech-to-text transcribe <audio file> [--model {}] [--format {}] [--output <file>]\n      \
         Writes the transcript to standard output, or saves it to the file named by --output.\n      \
         The model is {} and the format {} unless named.\n  \
         speech-to-text --help | --version\n\n\
         Exit status: 0 when the transcript was written, 1 on a failure, 2 on wrong usage.\n",
        models.join("|"),
        formats.join("|"),
        Model::default().name(),
        ExportFormat::default().extension(),
    )
}

fn transcribe(files: Provider, request: Request) -> ExitCode {
    let title = request.job.title();
    let (events, reports) = channel();
    thread::spawn(move || serve([request.job], events, files, || ()));
    let delivered = outcome(reports).and_then(|transcript| {
        deliver(&transcript, &title, request.format, request.output.as_deref())
    });
    delivered.map_or_else(|Failure(reason)| failed(reason), |()| ExitCode::SUCCESS)
}

fn outcome(reports: Receiver<Event>) -> Result<String, Failure> {
    let shown = stderr().is_terminal();
    for event in reports {
        match event {
            Event::Progress(done) if shown => eprint!("\rTranscribing… {:3.0}%", done * 100.0),
            Event::Progress(_) => {}
            Event::Finished(outcome) => {
                if shown {
                    eprint!("\r{:20}\r", "");
                }
                return outcome;
            }
        }
    }
    Err(Failure("The transcription stopped.".to_string()))
}

fn deliver(transcript: &str, title: &str, format: ExportFormat, output: Option<&Path>) -> Result<(), Failure> {
    match output {
        Some(path) => {
            export(path, format, title, transcript)?;
            eprintln!("Saved {}.", path.display());
        }
        None => print!("{}", format.render(title, transcript)),
    }
    Ok(())
}

fn window(files: Provider, first: Option<PathBuf>) -> ExitCode {
    leave_own_console();
    run(files, first).map_or_else(failed, |()| ExitCode::SUCCESS)
}

fn say(text: &str) -> ExitCode {
    print!("{text}");
    ExitCode::SUCCESS
}

fn wrong(reason: &str) -> ExitCode {
    eprint!("{reason}\n\n{}", usage());
    ExitCode::from(WRONG_USAGE)
}

fn failed(reason: impl Display) -> ExitCode {
    eprintln!("{reason}");
    ExitCode::FAILURE
}

#[cfg(windows)]
fn leave_own_console() {
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetConsoleProcessList(processes: *mut u32, room: u32) -> u32;
        safe fn FreeConsole() -> i32;
    }
    let mut first = 0;
    if unsafe { GetConsoleProcessList(&mut first, 1) } == 1 {
        FreeConsole();
    }
}

#[cfg(not(windows))]
fn leave_own_console() {}
