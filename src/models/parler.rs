
use std::error::Error;

use pyo3::{prelude::*, types::PyModule};
use super::*;

#[derive(Clone, Debug)]
pub struct ParlerModel {
    pub description : String,
    module: Py<PyModule>,
    model: Py<pyo3::PyAny>,
    tokenizer: Py<pyo3::PyAny>,
    device: String,
}

impl ParlerModel{
    pub fn new(description : String, model : String, use_gpu : bool) -> Result<Self, Box<dyn Error>>{
        let m = Python::with_gil(|py|{
            let activators = PyModule::from_code_bound(py, r#"
import torch
from parler_tts import ParlerTTSForConditionalGeneration
from transformers import AutoTokenizer, convert_slow_tokenizer
import transformers
import soundfile as sf

transformers.logging.set_verbosity_error()

def get_device(gpu):
    device = "cuda:0" if torch.cuda.is_available() && gpu else "cpu"
    return device

def get_model(device, model):
    model = ParlerTTSForConditionalGeneration.from_pretrained(model).to(device)
    return model

def get_tokenizer(model):
    return AutoTokenizer.from_pretrained(model)

def say(model, tokenizer, device, description, message, path):
    input_ids = tokenizer(description, return_tensors="pt").input_ids.to(device)
    prompt_input_ids = tokenizer(message, return_tensors="pt").input_ids.to(device)
    generation = model.generate(input_ids=input_ids, prompt_input_ids=prompt_input_ids)
    audio_arr = generation.cpu().numpy().squeeze()
    print("generated")
    sf.write(path, audio_arr, model.config.sampling_rate)
            "#, "parler.py", "Parler"
            ).unwrap();

            let device : String= activators.getattr("get_device").unwrap().call1((use_gpu,)).unwrap().extract().unwrap();
            let model = activators.getattr("get_model").unwrap().call1((device.clone(), model.clone())).unwrap().unbind();
            let tokenizer = activators.getattr("get_tokenizer").unwrap().call1((model.clone(),)).unwrap().unbind();
            return Self{
                module: activators.unbind(),
                description,
                model,
                tokenizer,
                device,
            };
        });

        return Ok(m);
    }
}

impl Default for ParlerModel{
    fn default() -> Self {
        let desc = "A female speaker in fast calming voice in a quiet environment".to_string();
        let model = "parler-tts/parler-tts-mini-expresso".to_string();
        return Self::new(desc.to_string(), model, true).unwrap();
    }
}

impl NaturalModelTrait for ParlerModel{
    type SynthesizeType = f32;
    fn save(&mut self, message: String, path : String)-> Result<(), Box<dyn Error>>{
        Python::with_gil(|py|{
            let args = (self.model.clone(), self.tokenizer.clone().into_py(py), self.device.clone().into_py(py), self.description.clone(), message, path.clone());
            let _ =self.module.getattr(py, "say").unwrap().call1(py, args);
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
