use crate::Failure;
use opus_decoder::OpusDecoder;
use rubato::audioadapter_buffers::owned::InterleavedOwned;
use rubato::{Fft, FixedSync, Resampler};
use std::fs::File;
use std::path::Path;
use symphonia::core::codecs::audio::well_known::CODEC_ID_OPUS;
use symphonia::core::codecs::audio::{AudioCodecParameters, AudioDecoder};
use symphonia::core::errors::Error;
use symphonia::core::formats::{FormatReader, TrackType};
use symphonia::core::formats::probe::Hint;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::packet::Packet;

pub const SAMPLE_RATE: usize = 16_000;

enum Codec {
    Opus(OpusDecoder),
    Other(Box<dyn AudioDecoder>),
}

struct Track {
    codec: Codec,
    channels: usize,
    rate: usize,
    frame: Vec<f32>,
    mono: Vec<f32>,
}

fn silent() -> Failure {
    Failure("The file holds no sound.".to_string())
}

fn open(path: &Path) -> Result<Box<dyn FormatReader>, Failure> {
    let stream = MediaSourceStream::new(Box::new(File::open(path)?), Default::default());
    let mut hint = Hint::new();
    if let Some(extension) = path.extension().and_then(|extension| extension.to_str()) {
        hint.with_extension(extension);
    }
    Ok(symphonia::default::get_probe().probe(&hint, stream, Default::default(), Default::default())?)
}

pub fn decode(path: &Path) -> Result<Vec<f32>, Failure> {
    let mut format = open(path)?;
    let found = format.default_track(TrackType::Audio).ok_or_else(silent)?;
    let id = found.id;
    let mut track = Track::new(found.codec_params.as_ref().and_then(|params| params.audio()))?;
    while let Some(packet) = format.next_packet()? {
        if packet.track_id == id {
            track.decode(&packet)?;
        }
    }
    resample(track.mono, track.rate)
}

impl Track {
    fn new(params: Option<&AudioCodecParameters>) -> Result<Track, Failure> {
        let params = params.ok_or_else(silent)?;
        let channels = params.channels.as_ref().map(|channels| channels.count()).unwrap_or_default();
        let (codec, rate) = if params.codec == CODEC_ID_OPUS {
            (Codec::Opus(OpusDecoder::new(SAMPLE_RATE as u32, channels)?), SAMPLE_RATE)
        } else {
            let decoder = symphonia::default::get_codecs().make_audio_decoder(params, &Default::default())?;
            (Codec::Other(decoder), params.sample_rate.unwrap_or_default() as usize)
        };
        Ok(Track { codec, channels, rate, frame: Vec::new(), mono: Vec::new() })
    }

    fn decode(&mut self, packet: &Packet) -> Result<(), Failure> {
        match &mut self.codec {
            Codec::Opus(decoder) => {
                self.frame.resize(decoder.max_frame_size_per_channel() * self.channels, 0.0);
                let frames = decoder.decode_float(&packet.data, &mut self.frame, false)?;
                self.frame.truncate(frames * self.channels);
            }
            Codec::Other(decoder) => match decoder.decode(packet) {
                Ok(buffer) => {
                    self.channels = buffer.spec().channels().count();
                    self.rate = buffer.spec().rate() as usize;
                    buffer.copy_to_vec_interleaved(&mut self.frame);
                }
                Err(Error::DecodeError(_)) => self.frame.clear(),
                Err(error) => return Err(error.into()),
            },
        }
        let mixed = self.frame.chunks_exact(self.channels).map(|frame| frame.iter().sum::<f32>() / frame.len() as f32);
        self.mono.extend(mixed);
        Ok(())
    }
}

fn resample(mono: Vec<f32>, rate: usize) -> Result<Vec<f32>, Failure> {
    if mono.is_empty() {
        return Err(silent());
    }
    if rate == SAMPLE_RATE {
        return Ok(mono);
    }
    let frames = mono.len();
    let mut resampler = Fft::<f32>::new(rate, SAMPLE_RATE, 1024, 1, FixedSync::Input)?;
    let input = InterleavedOwned::new_from(mono, 1, frames)?;
    Ok(resampler.process_all(&input, frames, None)?.take_data())
}
