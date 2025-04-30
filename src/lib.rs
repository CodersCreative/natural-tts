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
pub mod models;
mod test;
mod utils;

use crate::models::NaturalModelTrait;
use derive_builder::Builder;
use std::error::Error;
use thiserror::Error as TError;

#[cfg(feature = "gtts")]
use crate::models::gtts;
#[cfg(feature = "meta")]
use crate::models::meta;
#[cfg(feature = "msedge")]
use crate::models::msedge;
#[cfg(feature = "parler")]
use crate::models::parler;
#[cfg(feature = "tts-rs")]
use crate::models::tts_rs::TtsModel;

#[derive(Builder, Clone, Default)]
#[builder(setter(into))]
pub struct NaturalTts {
    pub default_model: Option<Model>,
    #[cfg(feature = "tts-rs")]
    #[builder(default = "None")]
    pub tts_model: Option<TtsModel>,

    #[cfg(feature = "parler")]
    #[builder(default = "None")]
    pub parler_model: Option<parler::ParlerModel>,

    #[cfg(feature = "coqui")]
    #[builder(default = "None")]
    pub coqui_model: Option<coqui::CoquiModel>,

    #[cfg(feature = "gtts")]
    #[builder(default = "None")]
    pub gtts_model: Option<gtts::GttsModel>,

    #[cfg(feature = "msedge")]
    #[builder(default = "None")]
    pub msedge_model: Option<msedge::MSEdgeModel>,

    #[cfg(feature = "meta")]
    #[builder(default = "None")]
    pub meta_model: Option<meta::MetaModel>,
}

impl NaturalModelTrait for NaturalTts {
    type SynthesizeType = f32;

    fn say(&mut self, message: String) -> Result<(), Box<dyn Error>> {
        if let Some(model) = &self.default_model {
            return match model {
                #[cfg(feature = "coqui")]
                Model::Coqui => match &mut self.coqui_model {
                    Some(x) => x.say(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "parler")]
                Model::Parler => match &mut self.parler_model {
                    Some(x) => x.say(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "tts-rs")]
                Model::TTS => match &mut self.tts_model {
                    Some(x) => x.say(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "msedge")]
                Model::MSEdge => match &mut self.msedge_model {
                    Some(x) => x.say(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "meta")]
                Model::Meta => match &mut self.meta_model {
                    Some(x) => x.say(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "gtts")]
                _ => match &mut self.gtts_model {
                    Some(x) => x.say(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
            };
        }

        Err(Box::new(TtsError::NoDefaultModel))
    }

    fn synthesize(
        &mut self,
        message: String,
    ) -> Result<models::SynthesizedAudio<Self::SynthesizeType>, Box<dyn Error>> {
        if let Some(model) = &self.default_model {
            return match model {
                #[cfg(feature = "coqui")]
                Model::Coqui => match &mut self.coqui_model {
                    Some(x) => x.synthesize(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "parler")]
                Model::Parler => match &mut self.parler_model {
                    Some(x) => x.synthesize(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "tts-rs")]
                Model::TTS => match &mut self.tts_model {
                    Some(x) => x.synthesize(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "msedge")]
                Model::MSEdge => match &mut self.msedge_model {
                    Some(x) => x.synthesize(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "meta")]
                Model::Meta => match &mut self.meta_model {
                    Some(x) => x.synthesize(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "gtts")]
                _ => match &mut self.gtts_model {
                    Some(x) => x.synthesize(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
            };
        }

        Err(Box::new(TtsError::NoDefaultModel))
    }

    fn save(&mut self, message: String, path: String) -> Result<(), Box<dyn Error>> {
        if let Some(model) = &self.default_model {
            return match model {
                #[cfg(feature = "coqui")]
                Model::Coqui => match &mut self.coqui_model {
                    Some(x) => x.save(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "parler")]
                Model::Parler => match &mut self.parler_model {
                    Some(x) => x.save(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "tts-rs")]
                Model::TTS => match &mut self.tts_model {
                    Some(x) => x.save(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "msedge")]
                Model::MSEdge => match &mut self.msedge_model {
                    Some(x) => x.save(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "meta")]
                Model::Meta => match &mut self.meta_model {
                    Some(x) => x.save(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "gtts")]
                _ => match &mut self.gtts_model {
                    Some(x) => x.save(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
            };
        }

        Err(Box::new(TtsError::NoDefaultModel))
    }
}

impl NaturalTts {
    pub fn say_auto(&mut self, message: String) -> Result<(), Box<dyn Error>> {
        self.say(message)
    }

    pub fn save_auto(&mut self, message: String, path: String) -> Result<(), Box<dyn Error>> {
        self.save(message, path)
    }

    pub fn synthesize_auto(
        &mut self,
        message: String,
    ) -> Result<models::SynthesizedAudio<f32>, Box<dyn Error>> {
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
    #[default]
    Gtts,
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
