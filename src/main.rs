fn main() -> std::process::ExitCode {
    speech_to_text::app::cli::start(speech_to_text::embedded, std::env::args_os().skip(1).collect())
}
