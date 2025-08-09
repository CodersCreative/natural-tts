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
use rodio::{Sample, Sink};
use std::{error::Error, fs::File, path::PathBuf};

pub enum AudioHandler {
    Sink(Sink),
    Tts,
}

impl Clone for AudioHandler {
    fn clone(&self) -> Self {
        match self {
            Self::Sink(_) => panic!("Sink cant be cloned"),
            Self::Tts => Self::Tts,
        }
    }
}

impl From<Sink> for AudioHandler {
    fn from(value: Sink) -> Self {
        Self::Sink(value)
    }
}

pub trait NaturalModelTrait {
    type SynthesizeType: Sample + Send + hound::Sample;
    fn save(&mut self, message: String, path: &PathBuf) -> Result<(), Box<dyn Error>>;

    fn start(&mut self, message: String, path: &PathBuf) -> Result<AudioHandler, Box<dyn Error>> {
        let _ = self.save(message.clone(), path);
        Ok(AudioHandler::Sink(play_wav_file(&path)?))
    }

    fn synthesize(
        &mut self,
        message: String,
        path: &PathBuf,
    ) -> Result<SynthesizedAudio<Self::SynthesizeType>, Box<dyn Error>> {
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

pub struct SynthesizedAudio<T: rodio::Sample> {
    pub spec: Spec,
    pub data: Vec<T>,
    pub duration: Option<i32>,
}

impl<T: rodio::Sample> SynthesizedAudio<T> {
    pub fn new(data: Vec<T>, spec: Spec, duration: Option<i32>) -> Self {
        return Self {
            data,
            spec,
            duration,
        };
    }
}

pub fn did_save(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let file = File::open(path);
    match file {
        Ok(_) => Ok(()),
        Err(_) => Err(Box::new(TtsError::NotSaved)),
    }
}
