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
mod utils;

#[cfg(test)]
mod test;

use crate::models::NaturalModelTrait;
use derive_builder::Builder;
use models::AudioHandler;
use rodio::Sink;
use std::{error::Error, path::PathBuf};
use thiserror::Error as TError;
use tts::Tts;

#[cfg(feature = "gtts")]
use crate::models::coqui;
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

#[derive(Builder, Default, Clone)]
#[builder(setter(into))]
pub struct NaturalTts {
    pub default_model: Option<Model>,
    #[builder(default = "None")]
    pub audio_handler: Option<AudioHandler>,

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

impl NaturalTts {
    pub fn get_tts_handler(&mut self) -> Result<&mut Tts, TtsError> {
        if let Some(tts) = &mut self.tts_model {
            Ok(&mut tts.0)
        } else {
            Err(TtsError::NotLoaded)
        }
    }

    pub fn get_rodio_sink(&mut self) -> Result<&mut Sink, TtsError> {
        match &mut self.audio_handler {
            Some(AudioHandler::Sink(x)) => Ok(x),
            Some(_) => return Err(TtsError::NotSupported),
            _ => return Err(TtsError::NotLoaded),
        }
    }

    pub fn start(&mut self, message: String, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        if let Some(model) = &self.default_model {
            self.audio_handler = Some(match model {
                #[cfg(feature = "coqui")]
                Model::Coqui => match &mut self.coqui_model {
                    Some(x) => Ok(x.start(message, path)?),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "parler")]
                Model::Parler => match &mut self.parler_model {
                    Some(x) => Ok(x.start(message, path)?),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "tts-rs")]
                Model::TTS => match &mut self.tts_model {
                    Some(x) => Ok(x.start(message, path)?),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "msedge")]
                Model::MSEdge => match &mut self.msedge_model {
                    Some(x) => Ok(x.start(message, path)?),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "meta")]
                Model::Meta => match &mut self.meta_model {
                    Some(x) => Ok(x.start(message, path)?),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "gtts")]
                _ => match &mut self.gtts_model {
                    Some(x) => Ok(x.start(message, path)?),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
            }?);

            return Ok(());
        }

        Err(Box::new(TtsError::NoDefaultModel))
    }

    pub fn synthesize(
        &mut self,
        message: String,
        path: &PathBuf,
    ) -> Result<models::SynthesizedAudio<f32>, Box<dyn Error>> {
        if let Some(model) = &self.default_model {
            return match model {
                #[cfg(feature = "coqui")]
                Model::Coqui => match &mut self.coqui_model {
                    Some(x) => x.synthesize(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "parler")]
                Model::Parler => match &mut self.parler_model {
                    Some(x) => x.synthesize(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "tts-rs")]
                Model::TTS => match &mut self.tts_model {
                    Some(x) => x.synthesize(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "msedge")]
                Model::MSEdge => match &mut self.msedge_model {
                    Some(x) => x.synthesize(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "meta")]
                Model::Meta => match &mut self.meta_model {
                    Some(x) => x.synthesize(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "gtts")]
                _ => match &mut self.gtts_model {
                    Some(x) => x.synthesize(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
            };
        }

        Err(Box::new(TtsError::NoDefaultModel))
    }

    pub fn save(&mut self, message: String, path: &PathBuf) -> Result<(), Box<dyn Error>> {
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

    pub fn resume(&mut self) -> Result<(), Box<dyn Error>> {
        match &mut self.audio_handler {
            Some(AudioHandler::Sink(x)) => x.play(),
            Some(_) => return Err(Box::new(TtsError::NotSupported)),
            _ => return Err(Box::new(TtsError::NotLoaded)),
        }

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        match &mut self.audio_handler {
            Some(AudioHandler::Sink(x)) => x.stop(),
            Some(AudioHandler::Tts) => match &mut self.tts_model {
                Some(x) => {
                    let _ = x.0.stop()?;
                }
                None => return Err(Box::new(TtsError::NotSupported)),
            },
            _ => return Err(Box::new(TtsError::NotLoaded)),
        }

        Ok(())
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
