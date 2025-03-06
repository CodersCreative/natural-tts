pub mod coqui;
pub mod parler;
pub mod gtts;
pub mod tts_rs;
pub mod msedge;
pub mod meta;

use hound::WavSpec;
use rodio::Sample;
use msedge_tts::tts::AudioMetadata;
use std::{error::Error, fs::File};
use crate::{utils::{get_path, play_wav_file, read_wav_file}, TtsError};

pub trait NaturalModelTrait {
    type SynthesizeType : Sample + Send;
    fn save(&mut self, message : String, path : String) -> Result<(), Box<dyn Error>>;
    fn say(&mut self, message : String) -> Result<(), Box<dyn Error>>;
    fn synthesize(&mut self, message : String) -> Result<SynthesizedAudio<Self::SynthesizeType>, Box<dyn Error>>;
}


pub fn speak_model<T : NaturalModelTrait>(model : &mut T, message : String) -> Result<(), Box<dyn Error>>{
    let path = "output.wav";
    let actual = get_path(path.to_string());
    let _ = std::fs::remove_file(actual.clone());
    let _ = model.save(message.clone(), actual.clone());
    let _ = play_wav_file(&actual);
    let _ = std::fs::remove_file(actual);
    Ok(())
}

pub fn synthesize_model<T : NaturalModelTrait>(model : &mut T, message : String) -> Result<SynthesizedAudio<f32>, Box<dyn Error>>{
    let path = "output.wav";
    let actual = get_path(path.to_string());
    let _ = std::fs::remove_file(actual.clone());
    let _ = model.save(message.clone(), actual.clone());
    let rwf = read_wav_file(&actual)?;
    let _ = std::fs::remove_file(actual);
    Ok(rwf)
}

pub enum Spec{
    Wav(WavSpec),
    Synthesized(String, Vec<AudioMetadata>),
    Unknown,
}

pub struct SynthesizedAudio<T : rodio::Sample>{
    pub spec : Spec,
    pub data : Vec<T>,
    pub duration : Option<i32>,
}

impl<T : rodio::Sample> SynthesizedAudio<T>{
    pub fn new(data : Vec<T>, spec : Spec, duration : Option<i32>) -> Self{
        return Self{
            data,
            spec,
            duration,
        };
    }
}

 pub fn did_save(path : &str) -> Result<(), Box<dyn Error>>{
     let file = File::open(path);
     match file{
        Ok(_) => Ok(()),
        Err(_) => Err(Box::new(TtsError::NotSaved)),
     }
 }
