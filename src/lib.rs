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
    /// Saves the given message to the given path as a WAV file.
    ///
    /// # Errors
    ///
    /// If the default model is not set, or if the model is not loaded, a `TtsError` is returned.
    ///
    fn save(&self, message : String, path : String) -> Result<(), Box<dyn Error>> {
        if let Some(model) = &self.default_model{
            return match model{
                #[cfg(feature = "coqui")]
                Model::Coqui => match &self.coqui_model{
                    Some(x) => x.save(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "parler")]
                Model::Parler => match &self.parler_model{
                    Some(x) => x.save(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "tts-rs")]
                Model::TTS => match &self.tts_model{
                    Some(x) => x.save(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "msedge")]
                Model::MSEdge => match &self.msedge_model{
                    Some(x) => x.save(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "meta")]
                Model::Meta => match &self.meta_model{
                    Some(x) => x.save(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "gtts")]
                _ => match &self.gtts_model{
                    Some(x) => x.save(message, path),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
            };
        }

        Err(Box::new(TtsError::NoDefaultModel))
    }
    /// Speaks the given message using the default model.
    ///
    /// # Errors
    ///
    /// If the default model is not set, or if the model is not loaded, a `TtsError` is returned.
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
    /// Synthesizes the given message using the default_model.
    ///
    /// # Errors
    ///
    /// If the default model is not set, or if the model is not loaded, a `TtsError` is returned.
    fn synthesize(&self, message : String) -> Result<models::SynthesizedAudio<Self::SynthesizeType>, Box<dyn Error>> {
        if let Some(model) = &self.default_model{
            return match model{
                #[cfg(feature = "coqui")]
                Model::Coqui => match &self.coqui_model{
                    Some(x) => x.synthesize(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "parler")]
                Model::Parler => match &self.parler_model{
                    Some(x) => x.synthesize(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "tts-rs")]
                Model::TTS => match &self.tts_model{
                    Some(x) => x.synthesize(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "msedge")]
                Model::MSEdge => match &self.msedge_model{
                    Some(x) => x.synthesize(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "meta")]
                Model::Meta => match &self.meta_model{
                    Some(x) => x.synthesize(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
                #[cfg(feature = "gtts")]
                _ => match &self.gtts_model{
                    Some(x) => x.synthesize(message),
                    None => Err(Box::new(TtsError::NotLoaded)),
                },
            };
        }

        Err(Box::new(TtsError::NoDefaultModel))
    }
}

impl NaturalTts{ 
    /// Automatically uses the default text-to-speech model to vocalize the given message.
    ///
    /// # Arguments
    ///
    /// * `message` - A `String` containing the message to be spoken.
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn Error>>` - Returns `Ok(())` if the message was successfully spoken,
    ///   or an error of type `Box<dyn Error>` if the operation failed.
    pub fn say_auto(&mut self, message : String) -> Result<(), Box<dyn Error>>{
        self.say(message)
    }
    
    /// Automatically uses the default text-to-speech model to save the given message as a wav file
    /// at the given path.
    ///
    /// # Arguments
    ///
    /// * `message` - A `String` containing the message to be saved.
    /// * `path` - A `String` containing the path to save the message as.  The file extension should
    ///   be `.wav`.
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn Error>>` - Returns `Ok(())` if the message was successfully saved,
    ///   or an error of type `Box<dyn Error>` if the operation failed.
    /// # Warnings
    /// **this function may fail as not every model supports this feature and will return** `"Box::new(TtsError::NotSupported)"`
    pub fn save_auto(&self, message : String, path : String) -> Result<(), Box<dyn Error>>{
        self.save(message, path)
    }
    
    /// Automatically uses the default text-to-speech model to synthesize the given message.
    ///
    /// # Arguments
    ///
    /// * `message` - A `String` containing the message to be synthesized.
    ///
    /// # Returns
    ///
    /// * `Result<models::SynthesizedAudio<f32>, Box<dyn Error>>` - Returns `Ok(models::SynthesizedAudio<f32>)`
    ///   if the message was successfully synthesized, or an error of type `Box<dyn Error>` if the
    ///   operation failed.
    /// # Warnings
    /// **this function may fail as not every model supports this feature and will return** `"Box::new(TtsError::NotSupported)"`
    pub fn synthesize_auto(&self, message : String) -> Result<models::SynthesizedAudio<f32>, Box<dyn Error>>{
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