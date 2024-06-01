use super::*;
use pyo3::{prelude::*, types::PyModule};

#[derive(Clone)]
pub struct CoquiModel {
    module: Py<PyModule>,
    model: Py<pyo3::PyAny>,
    device: String,
}

impl CoquiModel{
    pub fn new(model_name : String) -> Result<Self, String>{
        let m = Python::with_gil(|py|{
            let activators = PyModule::from_code_bound(py, r#"
import torch
#import TTS

def get_device():
    device = "cuda:0" if torch.cuda.is_available() else "cpu"
    #device = "cpu"
    return device

def get_model(name, device):
    #return TTS(model_name=name, progress_bar=False).to(device)
    return ""

def say(model, device, message, path):
    #model.tts_to_file(text=message, file_path=path)
    pass
            "#, "parler.py", "Parler"
            ).unwrap();

            let device : String= activators.getattr("get_device").unwrap().call0().unwrap().extract().unwrap();
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

impl NaturalModelTrait for CoquiModel{
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

    fn synthesize(&mut self, message : String) -> Result<Vec<f32>, Box<dyn Error>> {
        synthesize_model(self, message)
    }
}
