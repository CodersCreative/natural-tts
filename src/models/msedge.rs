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
use msedge_tts::{tts::{client::connect, SpeechConfig as OtherConfig}, voice::{get_voices_list, Voice}};
use crate::utils::{play_audio, save_wav};
use super::{NaturalModelTrait, Spec, SynthesizedAudio};

#[derive(Clone, Debug)]
pub struct MSEdgeModel{
    config : SpeechConfig,
}

impl MSEdgeModel{
    pub fn new_from_voice(voice : Voice) -> Self {
        Self{ config : SpeechConfig::from(&voice) }
    }

    pub fn new(config : SpeechConfig) -> Self {
        Self{ config }
    }
}

impl Default for MSEdgeModel{
    fn default() -> Self {
        let voice = get_voices_list().unwrap();
        Self::new(SpeechConfig::from(voice.first().unwrap()))
    }
}

impl NaturalModelTrait for MSEdgeModel{
    type SynthesizeType = f32;
    fn save(&self, message : String, path : String) -> Result<(), Box<dyn std::error::Error>> {
        let synthesized = Self::synthesize(self, message)?;
        
        let rate = match self.config.rate{
            x if x <= 0 => 16000,
            x => x
        };

        let _ = save_wav(&synthesized.data, path.as_str(), rate as u32);
        Ok(())
    }

    fn say(&mut self, message : String) -> Result<(), Box<dyn std::error::Error>> {
        let synthesized = Self::synthesize(self, message)?;
        
        let rate = match self.config.rate{
            x if x <= 0 => 16000,
            x => x
        };

        match synthesized.spec{
            Spec::Wav(x) => play_audio(synthesized.data, x.sample_rate),
            Spec::Synthesized(_, _) => play_audio(synthesized.data, rate as u32)
        }
        Ok(())
    }

    fn synthesize(&self, message : String) -> Result<SynthesizedAudio<Self::SynthesizeType>, Box<dyn std::error::Error>> {
        let mut tts = connect().unwrap();
        let audio = tts.synthesize(message.as_str(), &self.config.as_msedge())?;
        Ok(SynthesizedAudio::new(audio.audio_bytes.iter().map(|x| x.clone() as f32).collect(), Spec::Synthesized(audio.audio_format, audio.audio_metadata), None))
    }
}

#[derive(Debug, Clone)]
pub struct SpeechConfig {
    pub voice_name: String,
    pub audio_format: String,
    pub pitch: i32,
    pub rate: i32,
    pub volume: i32,
}

impl SpeechConfig{
    pub fn as_msedge(&self) -> OtherConfig{
        OtherConfig{
            voice_name : self.voice_name.clone(),
            audio_format: self.audio_format.clone(),
            pitch: self.pitch,
            rate: self.rate,
            volume: self.volume,
        }
    }
}


impl From<&msedge_tts::tts::SpeechConfig> for SpeechConfig {
    fn from(config: &msedge_tts::tts::SpeechConfig) -> Self {
        Self{
            voice_name : config.voice_name.clone(),
            audio_format: config.audio_format.clone(),
            pitch: config.pitch,
            rate: config.rate,
            volume: config.volume,
        }
    }
}

impl From<&Voice> for SpeechConfig {
    fn from(voice: &Voice) -> Self {
        let mscfg = OtherConfig::from(voice);
        Self::from(&mscfg)
    }
}
