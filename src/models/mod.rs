#[cfg(feature = "coqui")]
pub mod coqui;
#[cfg(feature = "gtts")]
pub mod gtts;
#[cfg(feature = "meta")]
pub mod meta;
#[cfg(feature = "msedge")]
pub mod msedge;
#[cfg(feature = "parler")]
pub mod parler;
#[cfg(feature = "tts-rs")]
pub mod tts_rs;

use crate::{
    utils::{play_wav_file, read_wav_file},
    TtsError,
};
use hound::WavSpec;
#[cfg(feature = "msedge")]
use msedge_tts::tts::AudioMetadata;
use rodio::{cpal::Sample, OutputStream, Sink};
use std::{fs::File, path::PathBuf};

pub enum AudioHandler {
    Sink {
        sink: Sink,
        stream: OutputStream,
    },
    #[cfg(feature = "tts-rs")]
    Tts,
}

impl Clone for AudioHandler {
    fn clone(&self) -> Self {
        match self {
            Self::Sink { .. } => panic!("Sink cant be cloned"),
            #[cfg(feature = "tts-rs")]
            Self::Tts => Self::Tts,
        }
    }
}

impl From<(Sink, OutputStream)> for AudioHandler {
    fn from(value: (Sink, OutputStream)) -> Self {
        Self::Sink {
            sink: value.0,
            stream: value.1,
        }
    }
}

pub trait NaturalModelTrait {
    type SynthesizeType: Sample + Send + hound::Sample;
    fn save(&mut self, message: String, path: &PathBuf) -> Result<(), TtsError>;

    fn start(&mut self, message: String, path: &PathBuf) -> Result<AudioHandler, TtsError> {
        let _ = self.save(message.clone(), path);
        Ok(AudioHandler::from(play_wav_file(path)?))
    }

    fn synthesize(
        &mut self,
        message: String,
        path: &PathBuf,
    ) -> Result<SynthesizedAudio<Self::SynthesizeType>, TtsError> {
        let _ = self.save(message.clone(), path);
        let rwf = read_wav_file(path)?;
        Ok(rwf)
    }
}

pub enum Spec {
    Wav(WavSpec),
    #[cfg(feature = "msedge")]
    Synthesized(String, Vec<AudioMetadata>),
    Unknown,
}

pub struct SynthesizedAudio<T: Sample> {
    pub spec: Spec,
    pub data: Vec<T>,
    pub duration: Option<i32>,
}

impl<T: Sample> SynthesizedAudio<T> {
    pub fn new(data: Vec<T>, spec: Spec, duration: Option<i32>) -> Self {
        return Self {
            data,
            spec,
            duration,
        };
    }
}

pub fn did_save(path: &PathBuf) -> Result<(), TtsError> {
    let file = File::open(path);
    match file {
        Ok(_) => Ok(()),
        Err(_) => Err(TtsError::NotSaved),
    }
}
