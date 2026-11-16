use crate::Failure;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportFormat {
    Text,
    Markdown,
}

impl ExportFormat {
    pub fn extension(self) -> &'static str {
        match self {
            ExportFormat::Text => "txt",
            ExportFormat::Markdown => "md",
        }
    }

    pub fn render(self, title: &str, transcript: &str) -> String {
        match self {
            ExportFormat::Text => format!("{transcript}\n"),
            ExportFormat::Markdown => format!("# {title}\n\n{transcript}\n"),
        }
    }
}

pub fn export(path: &Path, format: ExportFormat, title: &str, transcript: &str) -> Result<(), Failure> {
    fs::write(path, format.render(title, transcript))?;
    Ok(())
}
