use std::path::PathBuf;

use super::{AudioHandler, NaturalModelTrait, Spec, SynthesizedAudio};
use crate::utils::{play_audio, save_wav};
use msedge_tts::{
    tts::{client::connect, SpeechConfig as OtherConfig},
    voice::{get_voices_list, Voice},
};

#[derive(Clone, Debug)]
pub struct MSEdgeModel {
    config: SpeechConfig,
}

impl MSEdgeModel {
    pub fn new_from_voice(voice: Voice) -> Self {
        return Self {
            config: SpeechConfig::from(&voice),
        };
    }

    pub fn new(config: SpeechConfig) -> Self {
        return Self { config };
    }
}

impl Default for MSEdgeModel {
    fn default() -> Self {
        let voice = get_voices_list().unwrap();
        return Self::new(SpeechConfig::from(voice.first().unwrap()));
    }
}

impl NaturalModelTrait for MSEdgeModel {
    type SynthesizeType = f32;
    fn start(&mut self, message: String, path : &PathBuf) -> Result<AudioHandler, Box<dyn std::error::Error>> {
        let synthesized = Self::synthesize(self, message, path)?;

        let rate = match self.config.rate {
            x if x <= 0 => 16000,
            x => x,
        };

        Ok(AudioHandler::Sink(match synthesized.spec {
            Spec::Wav(x) => play_audio(&synthesized.data, x.sample_rate),
            _ => play_audio(&synthesized.data, rate as u32),
        }?))
    }

    fn save(&mut self, message: String, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let synthesized = Self::synthesize(self, message, path)?;

        let rate = match self.config.rate {
            x if x <= 0 => 16000,
            x => x,
        };

        let _ = save_wav(&synthesized.data, path, rate as u32);
        Ok(())
    }

    fn synthesize(
        &mut self,
        message: String,
        path : &PathBuf
    ) -> Result<super::SynthesizedAudio<Self::SynthesizeType>, Box<dyn std::error::Error>> {
        let mut tts = connect().unwrap();
        let audio = tts.synthesize(message.as_str(), &self.config.as_msedge())?;
        return Ok(SynthesizedAudio::new(
            audio.audio_bytes.iter().map(|x| x.clone() as f32).collect(),
            Spec::Synthesized(audio.audio_format, audio.audio_metadata),
            None,
        ));
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

impl SpeechConfig {
    pub fn as_msedge(&self) -> OtherConfig {
        return OtherConfig {
            voice_name: self.voice_name.clone(),
            audio_format: self.audio_format.clone(),
            pitch: self.pitch,
            rate: self.rate,
            volume: self.volume,
        };
    }
}

impl From<&msedge_tts::tts::SpeechConfig> for SpeechConfig {
    fn from(config: &msedge_tts::tts::SpeechConfig) -> Self {
        return Self {
            voice_name: config.voice_name.clone(),
            audio_format: config.audio_format.clone(),
            pitch: config.pitch,
            rate: config.rate,
            volume: config.volume,
        };
    }
}

impl From<&msedge_tts::voice::Voice> for SpeechConfig {
    fn from(voice: &msedge_tts::voice::Voice) -> Self {
        let mscfg = OtherConfig::from(voice);
        return Self::from(&mscfg);
    }
}
