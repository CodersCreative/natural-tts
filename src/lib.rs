pub mod models;
mod utils;
mod test;

use std::error::Error;

use models::{msedge, tts_rs::TtsModel, NaturalModelTrait};
use tts::Tts;
use online::check;
use crate::models::{coqui, parler, gtts, meta};
use thiserror::Error;
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

impl NaturalTts{
    pub fn say_auto(&mut self, message : String) -> Result<(), Box<dyn Error>>{
        let is_online = check(Some(1)).is_ok();

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
                Model::Meta => match &mut self.msedge_model{
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

#[derive(Debug, Error, Clone)]
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





