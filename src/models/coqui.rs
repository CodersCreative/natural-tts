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
use super::*;
use pyo3::{prelude::*, types::PyModule};

#[derive(Clone, Debug)]
pub struct CoquiModel {
    module: Py<PyModule>,
    model: Py<PyAny>,
    device: String,
}

impl CoquiModel{
    pub fn new(model_name : String, use_gpu : bool) -> Result<Self, Box<dyn Error>>{
        let m = Python::with_gil(|py|{
            let activators = PyModule::from_code_bound(py, r#"
import torch
from TTS.api import TTS

def get_device(gpu):
    if torch.cuda.is_available() and gpu:
        return "cuda:0"
    else:
        return "cpu"

def get_model(name, device):
    return TTS(model_name=name, progress_bar=False).to(device)

def say(model, device, message, path):
    model.tts_to_file(text=message, file_path=path)
            "#, "coqui.py", "Coqui"
            ).unwrap();

            let device : String= activators.getattr("get_device").unwrap().call1((use_gpu,)).unwrap().extract().unwrap();
            let model = activators.getattr("get_model").unwrap().call1((model_name, device.clone())).unwrap().unbind();

            return Self{
                module: activators.unbind(),
                model,
                device,
            };
        });

        Ok(m)
    }
}

impl Default for CoquiModel{
    fn default() -> Self {
        Self::new("tts_models/en/ljspeech/vits".to_string(), true).unwrap()
    }
}

impl NaturalModelTrait for CoquiModel{
    type SynthesizeType = f32;

    fn save(&self, message: String, path : String) -> Result<(), Box<dyn Error>>{
        Python::with_gil(|py|{
            let args = (self.model.clone(),  self.device.clone().into_py(py), message, path.clone());
            let _ =self.module.getattr(py, "say").unwrap().call1(py, args);
        });
        did_save(path.as_str())
    }

    fn say(&mut self, message : String)  -> Result<(), Box<dyn Error>>{
       speak_model(self, message) 
    }

    fn synthesize(&self, message : String) -> Result<SynthesizedAudio<Self::SynthesizeType>, Box<dyn Error>> {
        synthesize_model(self, message)
    }
}
