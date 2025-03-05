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
use std::error::Error;
use tts::Tts;
use crate::TtsError;
use super::{NaturalModelTrait, SynthesizedAudio};

#[derive(Clone)]
pub struct TtsModel(pub Tts);

impl TtsModel{
    pub fn new() -> Result<Self, Box<dyn Error>>{
        let def = Tts::default()?;
        return Ok(Self(def));
    }
}

impl Default for TtsModel{
    fn default() -> Self {
        return Self::new().unwrap();
    }
}

impl NaturalModelTrait for TtsModel{
    type SynthesizeType = f32;
    fn save(&self, _message : String, _path : String) -> Result<(), Box<dyn Error>> {
        Err(TtsError::NotSupported.into())
    }

    fn say(&mut self, message : String) -> Result<(), Box<dyn Error>> {
        let is_speaking = self.0.is_speaking();
        
        if let Ok(speaking) = is_speaking{
            if speaking{
                return Ok(());
            }
        }

        let _ = self.0.speak(message, false);
        Ok(())
    }

    fn synthesize(&self, _message : String) -> Result<SynthesizedAudio<Self::SynthesizeType>, Box<dyn Error>> {
        Err(TtsError::NotSupported.into())
    }
}

