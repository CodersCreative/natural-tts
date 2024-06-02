use super::*;
use pyo3::{prelude::*, types::PyModule};
use rodio::Sample;

#[derive(Clone, Debug)]
pub struct CoquiModel {
    module: Py<PyModule>,
    model: Py<pyo3::PyAny>,
    device: String,
}

impl CoquiModel{
    pub fn new(model_name : String, use_gpu : bool) -> Result<Self, Box<dyn Error>>{
        return Err(Box::new(TtsError::NotSupported));
        let m = Python::with_gil(|py|{
            let activators = PyModule::from_code_bound(py, r#"
import torch
#import TTS

def get_device(gpu):
    if torch.cuda.is_available() && gpu:
        return "cuda:0"
    else:
        return "cpu"

def get_model(name, device):
    #return TTS(model_name=name, progress_bar=False).to(device)
    return ""

def say(model, device, message, path):
    #model.tts_to_file(text=message, file_path=path)
    pass
            "#, "parler.py", "Parler"
            ).unwrap();

            let device : String= activators.getattr("get_device").unwrap().call1((use_gpu,)).unwrap().extract().unwrap();
            let model = activators.getattr("get_model").unwrap().call1((model_name, device.clone())).unwrap().unbind();

            return Self{
                module: activators.unbind(),
                model,
                device,
            };
        });

        return Ok(m);
    }
}

impl Default for CoquiModel{
    fn default() -> Self {
        return Self::new("base".to_string(), true).unwrap();
    }
}

impl NaturalModelTrait for CoquiModel{
    type SynthesizeType = f32;

    fn save(&mut self, message: String, path : String) -> Result<(), Box<dyn Error>>{
        Python::with_gil(|py|{
            let args = (self.model.clone(),  self.device.clone().into_py(py), message, path.clone());
            let _ =self.module.getattr(py, "say").unwrap().call1(py, args);
        });
        did_save(path.as_str())
    }

    fn say(&mut self, message : String)  -> Result<(), Box<dyn Error>>{
       speak_model(self, message) 
    }

    fn synthesize(&mut self, message : String) -> Result<SynthesizedAudio<Self::SynthesizeType>, Box<dyn Error>> {
        synthesize_model(self, message)
    }
}
