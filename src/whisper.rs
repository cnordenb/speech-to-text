use crate::Failure;
use crate::audio::SAMPLE_RATE;
use candle_core::{Device, IndexOp, Tensor};
use candle_nn::VarBuilder;
use candle_nn::ops::softmax;
use candle_transformers::models::whisper::audio::pcm_to_mel;
use candle_transformers::models::whisper::model::Whisper;
use candle_transformers::models::whisper::{self as constants, Config};
use tokenizers::Tokenizer;

const WINDOW: usize = constants::N_SAMPLES;
const SEARCH: usize = 5 * SAMPLE_RATE;
const PAUSE: usize = SAMPLE_RATE / 10;

pub struct ModelFiles<'a> {
    pub config: &'a [u8],
    pub tokenizer: &'a [u8],
    pub weights: &'a [u8],
    pub mel_filters: &'a [u8],
}

struct Tokens {
    start: u32,
    translate: u32,
    transcribe: u32,
    no_timestamps: u32,
    end: u32,
    no_speech: u32,
}

pub struct Transcriber {
    model: Whisper,
    tokenizer: Tokenizer,
    mel_filters: Vec<f32>,
    suppress: Tensor,
    tokens: Tokens,
}

fn unusable<E: ToString>(error: E) -> Failure {
    Failure(error.to_string())
}

fn chunks(samples: &[f32]) -> Vec<&[f32]> {
    let mut chunks = Vec::new();
    let mut rest = samples;
    while rest.len() > WINDOW {
        let (chunk, tail) = rest.split_at(WINDOW - SEARCH + quietest(&rest[WINDOW - SEARCH..WINDOW]));
        chunks.push(chunk);
        rest = tail;
    }
    chunks.push(rest);
    chunks
}

fn quietest(samples: &[f32]) -> usize {
    let energy = |window: &[f32]| window.iter().map(|sample| sample * sample).sum::<f32>();
    let pauses = samples.chunks(PAUSE).map(energy).enumerate();
    pauses.min_by(|quiet, loud| quiet.1.total_cmp(&loud.1)).map(|(index, _)| index * PAUSE).unwrap_or_default()
}

impl Tokens {
    fn new(tokenizer: &Tokenizer) -> Result<Tokens, Failure> {
        let id = |name: &str| tokenizer.token_to_id(name).ok_or_else(|| unusable(format!("The model lacks {name}.")));
        Ok(Tokens {
            start: id(constants::SOT_TOKEN)?,
            translate: id(constants::TRANSLATE_TOKEN)?,
            transcribe: id(constants::TRANSCRIBE_TOKEN)?,
            no_timestamps: id(constants::NO_TIMESTAMPS_TOKEN)?,
            end: id(constants::EOT_TOKEN)?,
            no_speech: id(constants::NO_SPEECH_TOKENS[0])?,
        })
    }
}

impl Transcriber {
    pub fn load(files: &ModelFiles) -> Result<Transcriber, Failure> {
        let config: Config = serde_json::from_slice(files.config)?;
        let tokenizer = Tokenizer::from_bytes(files.tokenizer).map_err(unusable)?;
        let suppressed = |token: u32| if config.suppress_tokens.contains(&token) { f32::NEG_INFINITY } else { 0.0 };
        let suppress: Vec<f32> = (0..config.vocab_size as u32).map(suppressed).collect();
        let weights = VarBuilder::from_slice_safetensors(files.weights, constants::DTYPE, &Device::Cpu)?;
        Ok(Transcriber {
            tokens: Tokens::new(&tokenizer)?,
            tokenizer,
            mel_filters: files.mel_filters.as_chunks().0.iter().map(|bytes| f32::from_le_bytes(*bytes)).collect(),
            suppress: Tensor::new(suppress.as_slice(), &Device::Cpu)?,
            model: Whisper::load(&weights, config)?,
        })
    }

    pub fn transcribe(&mut self, samples: &[f32], progress: &mut dyn FnMut(f32)) -> Result<String, Failure> {
        let chunks = chunks(samples);
        let mut language = None;
        let mut texts = Vec::new();
        for (done, chunk) in chunks.iter().enumerate() {
            progress(done as f32 / chunks.len() as f32);
            let features = self.encode(chunk)?;
            let odds = self.opening(&features)?;
            if odds[self.tokens.no_speech as usize] > constants::NO_SPEECH_THRESHOLD as f32 {
                continue;
            }
            let spoken = language.map_or_else(|| self.language(&odds), Ok)?;
            language = Some(spoken);
            texts.push(self.decode(&features, spoken)?);
        }
        texts.retain(|text| !text.is_empty());
        if texts.is_empty() {
            return Err(unusable("No speech was found in the audio file."));
        }
        Ok(texts.join(" "))
    }

    fn encode(&mut self, chunk: &[f32]) -> Result<Tensor, Failure> {
        let bins = self.model.config.num_mel_bins;
        let mel = pcm_to_mel(&self.model.config, chunk, &self.mel_filters);
        let frames = mel.len() / bins;
        let mel = Tensor::from_vec(mel, (1, bins, frames), &Device::Cpu)?;
        self.model.reset_kv_cache();
        Ok(self.model.encoder.forward(&mel.narrow(2, 0, frames.min(constants::N_FRAMES))?, false)?)
    }

    fn logits(&self, output: &Tensor, position: usize) -> Result<Tensor, Failure> {
        Ok(self.model.decoder.final_linear(&output.i((..1, position..=position))?)?.i(0)?.i(0)?)
    }

    fn forward(&mut self, tokens: &[u32], features: &Tensor) -> Result<Tensor, Failure> {
        let input = Tensor::new(tokens, &Device::Cpu)?.unsqueeze(0)?;
        Ok(self.model.decoder.forward(&input, features, false)?)
    }

    fn opening(&mut self, features: &Tensor) -> Result<Vec<f32>, Failure> {
        let output = self.forward(&[self.tokens.start], features)?;
        Ok(softmax(&self.logits(&output, 0)?, 0)?.to_vec1()?)
    }

    fn language(&self, odds: &[f32]) -> Result<u32, Failure> {
        let languages = self.tokens.start + 1..self.tokens.translate;
        let likeliest = languages.max_by(|one, other| odds[*one as usize].total_cmp(&odds[*other as usize]));
        likeliest.ok_or_else(|| unusable("The model knows no language."))
    }

    fn likeliest(&self, output: &Tensor, position: usize) -> Result<u32, Failure> {
        let allowed: Vec<f32> = self.logits(output, position)?.broadcast_add(&self.suppress)?.to_vec1()?;
        let likeliest = allowed.iter().enumerate().max_by(|one, other| one.1.total_cmp(other.1));
        Ok(likeliest.map_or(self.tokens.end, |(token, _)| token as u32))
    }

    fn decode(&mut self, features: &Tensor, language: u32) -> Result<String, Failure> {
        let prompt = [self.tokens.start, language, self.tokens.transcribe, self.tokens.no_timestamps];
        let limit = prompt.len() + self.model.config.max_target_positions / 2;
        let mut tokens = prompt.to_vec();
        while tokens.len() < limit {
            let output = self.forward(&tokens, features)?;
            let next = self.likeliest(&output, tokens.len() - 1)?;
            if next == self.tokens.end {
                break;
            }
            tokens.push(next);
        }
        Ok(self.tokenizer.decode(&tokens[prompt.len()..], true).map_err(unusable)?.trim().to_string())
    }
}
