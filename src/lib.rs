pub mod models;
mod utils;
mod test;

use thiserror::Error as TError;
use std::error::Error;
use crate::models::{msedge, tts_rs::TtsModel, NaturalModelTrait, coqui, parler, gtts, meta};
use derive_builder::Builder;

#[derive(Builder, Clone, Default)]
#[builder(setter(into))]
pub struct NaturalTts{
    pub default_model : Option<Model>,
    #[cfg(feature = "tts-rs")]
    #[builder(default = "None")]
    pub tts_model : Option<TtsModel>,

    #[cfg(feature = "parler")]
    #[builder(default = "None")]
    pub parler_model : Option<parler::ParlerModel>,

    #[cfg(feature = "coqui")]
    #[builder(default = "None")]
    pub coqui_model : Option<coqui::CoquiModel>,

    #[cfg(feature = "gtts")]
    #[builder(default = "None")]
    pub gtts_model : Option<gtts::GttsModel>,

    #[cfg(feature = "msedge")]
    #[builder(default = "None")]
    pub msedge_model : Option<msedge::MSEdgeModel>,

    #[cfg(feature = "meta")]
    #[builder(default = "None")]
    pub meta_model : Option<meta::MetaModel>,
}

impl NaturalModelTrait for NaturalTts{
    type SynthesizeType = f32;
    
    fn say(&mut self, message : String) -> Result<(), Box<dyn Error>> {
        if let Some(model) = &self.default_model{
            return match model{
                #[cfg(feature = "coqui")]
                Model::Coqui => match &mut self.coqui_model{
                    Some(x) => x.say(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "parler")]
                Model::Parler => match &mut self.parler_model{
                    Some(x) => x.say(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "tts-rs")]
                Model::TTS => match &mut self.tts_model{
                    Some(x) => x.say(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "msedge")]
                Model::MSEdge => match &mut self.msedge_model{
                    Some(x) => x.say(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "meta")]
                Model::Meta => match &mut self.meta_model{
                    Some(x) => x.say(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "gtts")]
                _ => match &mut self.gtts_model{
                    Some(x) => x.say(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
            };
        }

        Err(Box::new(TtsError::NoDefaultModel))
    }

    fn synthesize(&mut self, message : String) -> Result<models::SynthesizedAudio<Self::SynthesizeType>, Box<dyn Error>> {
        if let Some(model) = &self.default_model{
            return match model{
                #[cfg(feature = "coqui")]
                Model::Coqui => match &mut self.coqui_model{
                    Some(x) => x.synthesize(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "parler")]
                Model::Parler => match &mut self.parler_model{
                    Some(x) => x.synthesize(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "tts-rs")]
                Model::TTS => match &mut self.tts_model{
                    Some(x) => x.synthesize(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "msedge")]
                Model::MSEdge => match &mut self.msedge_model{
                    Some(x) => x.synthesize(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "meta")]
                Model::Meta => match &mut self.meta_model{
                    Some(x) => x.synthesize(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "gtts")]
                _ => match &mut self.gtts_model{
                    Some(x) => x.synthesize(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
            };
        }

        Err(Box::new(TtsError::NoDefaultModel))
    }

    fn save(&mut self, message : String, path : String) -> Result<(), Box<dyn Error>> {
        if let Some(model) = &self.default_model{
            return match model{
                #[cfg(feature = "coqui")]
                Model::Coqui => match &mut self.coqui_model{
                    Some(x) => x.save(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "parler")]
                Model::Parler => match &mut self.parler_model{
                    Some(x) => x.save(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "tts-rs")]
                Model::TTS => match &mut self.tts_model{
                    Some(x) => x.save(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "msedge")]
                Model::MSEdge => match &mut self.msedge_model{
                    Some(x) => x.save(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "meta")]
                Model::Meta => match &mut self.meta_model{
                    Some(x) => x.save(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "gtts")]
                _ => match &mut self.gtts_model{
                    Some(x) => x.save(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
            };
        }

        Err(Box::new(TtsError::NoDefaultModel))
    }
}

impl NaturalTts{ 
    pub fn say_auto(&mut self, message : String) -> Result<(), Box<dyn Error>>{
        self.say(message)
    }
    
    pub fn save_auto(&mut self, message : String, path : String) -> Result<(), Box<dyn Error>>{
        self.save(message, path)
    }
    
    pub fn synthesize_auto(&mut self, message : String) -> Result<models::SynthesizedAudio<f32>, Box<dyn Error>>{
        self.synthesize(message)
    }
}

#[derive(Default, Clone)]
pub enum Model {
    #[cfg(feature = "coqui")]
    Coqui,

    #[cfg(feature = "parler")]
    Parler,

    #[cfg(feature = "tts-rs")]
    TTS,

    #[cfg(feature = "msedge")]
    MSEdge,
    
    #[cfg(feature = "meta")]
    Meta,
    
    #[cfg(feature = "gtts")]
    #[default] Gtts
}

#[derive(Debug, TError, Clone)]
pub enum TtsError {
    #[error("Not Supported")]
    NotSupported,
    #[error("Operation failed")]
    OperationFailed,
    #[error("Model Not Loaded")]
    NotLoaded,
    #[error("Didn't Save")]
    NotSaved,
    #[error("Default model not set")]
    NoDefaultModel,
    #[error("Tensor Error")]
    Tensor,
    #[error("No Tokenizer Key")]
    NoTokenizerKey,
    #[error("Json Error")]
    Json,
}
