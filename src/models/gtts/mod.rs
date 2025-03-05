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
pub mod languages;

use pyo3::{prelude::*, types::PyModule};
use super::*;

#[derive(Clone, Debug)]
pub struct GttsModel {
    language : languages::Languages,
    module: Py<PyModule>,
}

impl GttsModel{
    pub fn new(language : Option<languages::Languages>) -> Result<Self, Box<dyn Error>>{
        let m = Python::with_gil(|py|{
            let activators = PyModule::from_code_bound(py, r#"
from gtts import gTTS
def say(message, path, lang):
    tts = gTTS(message, lang)
    tts.save(path)
            "#, "gtts.py", "Gtts"
            ).unwrap();

            let language = match language{
                Some(x) => x,
                None => languages::Languages::English,
            };

            return Self{
                language,
                module: activators.unbind(),
            };
        });

        return Ok(m);
    }
}

impl Default for GttsModel{
    fn default() -> Self {
        return Self::new(None).unwrap();
    }
}

impl NaturalModelTrait for GttsModel{
    type SynthesizeType = f32;
    fn save(&mut self, message: String, path : String) -> Result<(), Box<dyn Error>> {
        Python::with_gil(|py|{
            let _ =self.module.getattr(py, "say").unwrap().call1(py, (message, path.clone(), self.language.as_code(),));
        });
        did_save(path.as_str())
    }

    fn say(&mut self, message : String) -> Result<(), Box<dyn Error>>{
       speak_model(self, message) 
    }

    fn synthesize(&mut self, message : String) -> Result<SynthesizedAudio<Self::SynthesizeType>, Box<dyn Error>> {
        synthesize_model(self, message)
    }
}
