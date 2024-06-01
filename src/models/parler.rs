
use std::error::Error;

use pyo3::{prelude::*, types::PyModule};
use super::*;

#[derive(Clone, Debug)]
pub struct ParlerModel {
    module: Py<PyModule>,
    model: Py<pyo3::PyAny>,
    tokenizer: Py<pyo3::PyAny>,
    device: String,
}

impl ParlerModel{
    pub fn new() -> Result<Self, String>{
        let m = Python::with_gil(|py|{
            let activators = PyModule::from_code_bound(py, r#"
import torch
from parler_tts import ParlerTTSForConditionalGeneration
from transformers import AutoTokenizer, convert_slow_tokenizer
import transformers
import soundfile as sf

transformers.logging.set_verbosity_error()

def get_device():
    device = "cuda:0" if torch.cuda.is_available() else "cpu"
    device = "cpu"
    return device

def get_model(device):
    model = ParlerTTSForConditionalGeneration.from_pretrained("parler-tts/parler-tts-mini-expresso").to(device)
    return model

def get_tokenizer():
    return AutoTokenizer.from_pretrained("parler-tts/parler_tts_mini_v0.1")

def say(model, tokenizer, device, description, message, path):
    input_ids = tokenizer(description, return_tensors="pt").input_ids.to(device)
    prompt_input_ids = tokenizer(message, return_tensors="pt").input_ids.to(device)
    generation = model.generate(input_ids=input_ids, prompt_input_ids=prompt_input_ids)
    audio_arr = generation.cpu().numpy().squeeze()
    print("generated")
    sf.write(path, audio_arr, model.config.sampling_rate)
            "#, "parler.py", "Parler"
            ).unwrap();

            let device : String= activators.getattr("get_device").unwrap().call0().unwrap().extract().unwrap();
            let model = activators.getattr("get_model").unwrap().call1((device.clone(), )).unwrap().unbind();
            let tokenizer = activators.getattr("get_tokenizer").unwrap().call0().unwrap().unbind();
            return Self{
                module: activators.unbind(),
                model,
                tokenizer,
                device,
            };
        });

        return Ok(m);
    }
}

impl NaturalModelTrait for ParlerModel{
    fn save(&mut self, message: String, path : String)-> Result<(), Box<dyn Error>>{
        Python::with_gil(|py|{
            let args = (self.model.clone(), self.tokenizer.clone().into_py(py), self.device.clone().into_py(py), "A female speaker in fast calming voice in a quiet environment", message, path.clone());
            let _ =self.module.getattr(py, "say").unwrap().call1(py, args);
        });
        did_save(path.as_str())
    }

    fn say(&mut self, message : String) -> Result<(), Box<dyn Error>>{
       speak_model(self, message)
    }

    fn synthesize(&mut self, message : String) -> Result<Vec<f32>, Box<dyn Error>> {
        synthesize_model(self, message)
    }
}
