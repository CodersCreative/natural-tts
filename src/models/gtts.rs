use pyo3::{prelude::*, types::PyModule};
use super::*;

#[derive(Clone, Debug)]
pub struct GttsModel {
    module: Py<PyModule>,
}

impl GttsModel{
    pub fn new() -> Result<Self, String>{
        let m = Python::with_gil(|py|{
            let activators = PyModule::from_code_bound(py, r#"
from gtts import gTTS
def say(message, path):
    tts = gTTS(message)
    tts.save(path)
            "#, "parler.py", "Parler"
            ).unwrap();

            return Self{
                module: activators.unbind(),
            };
        });

        return Ok(m);
    }
}

impl NaturalModelTrait for GttsModel{
    fn save(&mut self, message: String, path : String) -> Result<(), Box<dyn Error>> {
        Python::with_gil(|py|{
            let _ =self.module.getattr(py, "say").unwrap().call1(py, (message, path.clone(), ));
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
