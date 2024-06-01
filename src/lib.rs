pub mod models;
mod utils;
mod test;

use std::error::Error;

use models::NaturalModelTrait;
use tts::Tts;
use online::check;
use crate::models::{coqui, parler, gtts};
use thiserror::Error;
use derive_builder::Builder;

#[derive(Builder, Clone, Default)]
#[builder(setter(into))]
pub struct NaturalTts{
    pub default_model : Option<Model>,
    #[cfg(feature = "tts-rs")]
    #[builder(default = "None")]
    pub tts_model : Option<Tts>,

    #[cfg(feature = "parler")]
    #[builder(default = "None")]
    pub parler_model : Option<parler::ParlerModel>,

    #[cfg(feature = "coqui")]
    #[builder(default = "None")]
    pub coqui_model : Option<coqui::CoquiModel>,

    #[cfg(feature = "gtts")]
    #[builder(default = "None")]
    pub gtts_model : Option<gtts::GttsModel>,
}

impl NaturalTts{
    pub fn say_auto(&mut self, message : String) -> Result<(), Box<dyn Error>>{
        let is_online = check(Some(1)).is_ok();

        #[cfg(feature = "gtts")]
        let mut gtts_fn = || -> Result<(), Box<dyn Error>>{
            match is_online{
                true => match &mut self.gtts_model{
                    Some(x) => x.say(message.clone()),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                _ => match &mut self.parler_model{
                    Some(x) => x.say(message.clone()),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
            }
        };

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
                #[cfg(feature = "gtts")]
                _ => gtts_fn(), 
            };
        }

        Err(Box::new(TtsError::NoDefaultModel))
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

    #[cfg(feature = "gtts")]
    #[default] Gtts
}

#[derive(Debug, Error)]
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
}





