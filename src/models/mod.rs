// Copyright (c) 2024-2025 natural-tts
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
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
    fn save(&self, message : String, path : String) -> Result<(), Box<dyn Error>>;
    fn say(&mut self, message : String) -> Result<(), Box<dyn Error>>;
    fn synthesize(&self, message : String) -> Result<SynthesizedAudio<Self::SynthesizeType>, Box<dyn Error>>;
}


pub fn speak_model<T : NaturalModelTrait>(model : &T, message : String) -> Result<(), Box<dyn Error>>{
    let path = "text_to_speech/output.wav";
    let actual = get_path(path.to_string());
    let _ = std::fs::remove_file(actual.clone());
    let _ = model.save(message.clone(), actual.clone());
    let _ = play_wav_file(&actual);
    let _ = std::fs::remove_file(actual);
    Ok(())
}

pub fn synthesize_model<T : NaturalModelTrait>(model : &T, message : String) -> Result<SynthesizedAudio<f32>, Box<dyn Error>>{
    let path = "text_to_speech/output.wav";
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
}

pub struct SynthesizedAudio<T : Sample>{
    pub spec : Spec,
    pub data : Vec<T>,
    pub duration : Option<i32>,
}

impl<T : Sample> SynthesizedAudio<T>{
    pub fn new(data : Vec<T>, spec : Spec, duration : Option<i32>) -> Self{
        Self{
            data,
            spec,
            duration,
        }
    }
}

 pub fn did_save(path : &str) -> Result<(), Box<dyn Error>>{
     let file = File::open(path);
     match file{
        Ok(_) => Ok(()),
        Err(_) => Err(Box::new(TtsError::NotSaved)),
     }
 }

