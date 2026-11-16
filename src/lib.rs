pub mod app;
pub mod audio;
pub mod catalog;
pub mod export;
pub mod whisper;

include!(concat!(env!("OUT_DIR"), "/embedded.rs"));

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Failure(pub String);

impl<E: std::error::Error> From<E> for Failure {
    fn from(error: E) -> Self {
        Failure(error.to_string())
    }
}
