use pyo3::{prelude::*, types::PyModule};
use super::*;
pub mod languages;
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
            "#, "parler.py", "Parler"
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
