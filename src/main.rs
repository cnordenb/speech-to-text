#![windows_subsystem = "windows"]

fn main() -> eframe::Result {
    speech_to_text::app::run(speech_to_text::embedded, std::env::args_os().nth(1).map(Into::into))
}
