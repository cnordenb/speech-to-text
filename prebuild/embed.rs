#[path = "../src/catalog.rs"]
mod catalog;

use catalog::Model;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const MODEL_FILES: [&str; 3] = ["config.json", "tokenizer.json", "model.safetensors"];
const MEL_FILTERS: &str = "melfilters.bytes";
const MEL_FILTERS_URL: &str =
    "https://raw.githubusercontent.com/huggingface/candle/0.11.0/candle-examples/examples/whisper/melfilters.bytes";

fn main() {
    println!("cargo::rerun-if-changed=prebuild/embed.rs");
    println!("cargo::rerun-if-changed=src/catalog.rs");
    println!("cargo::rerun-if-changed=models");
    let models = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("models");
    fetch(MEL_FILTERS_URL, &models.join(MEL_FILTERS));
    for model in Model::ALL {
        for file in MODEL_FILES {
            let url = format!("https://huggingface.co/openai/whisper-{}/resolve/main/{file}", model.name());
            fetch(&url, &models.join(model.name()).join(file));
        }
    }
    let generated = PathBuf::from(env::var("OUT_DIR").unwrap()).join("embedded.rs");
    let release = env::var("PROFILE").unwrap() == "release";
    fs::write(generated, embedded(&models, release)).unwrap();
}

fn fetch(url: &str, target: &Path) {
    if target.exists() {
        return;
    }
    fs::create_dir_all(target.parent().unwrap()).unwrap();
    let partial = target.with_extension("part");
    let status = Command::new("curl")
        .args(["--fail", "--location", "--silent", "--show-error", "--output"])
        .arg(&partial)
        .arg(url)
        .status()
        .expect("curl is needed to download the models");
    assert!(status.success(), "could not download {url}");
    fs::rename(&partial, target).unwrap();
}

fn embedded(models: &Path, release: bool) -> String {
    let arms: String = Model::ALL.iter().map(|model| arm(models, *model, release)).collect();
    format!(
        "pub fn embedded(model: catalog::Model) -> whisper::ModelFiles<'static> {{\n\
         match model {{\n{arms}}}\n}}\n"
    )
}

fn arm(models: &Path, model: Model, release: bool) -> String {
    let (read, kept) = if release { ("include_bytes!", "") } else { ("std::fs::read", ".unwrap().leak()") };
    let include = |path: PathBuf| format!("{read}({:?}){kept}", path.to_str().unwrap());
    let folder = models.join(model.name());
    format!(
        "catalog::Model::{model:?} => whisper::ModelFiles {{\n\
         config: {},\ntokenizer: {},\nweights: {},\nmel_filters: {},\n}},\n",
        include(folder.join(MODEL_FILES[0])),
        include(folder.join(MODEL_FILES[1])),
        include(folder.join(MODEL_FILES[2])),
        include(models.join(MEL_FILTERS)),
    )
}
