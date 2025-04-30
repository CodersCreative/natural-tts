use super::*;
use pyo3::prelude::*;

#[derive(Debug)]
pub struct CoquiModel {
    model: Py<PyAny>,
    device: String,
}

impl Clone for CoquiModel {
    fn clone(&self) -> Self {
        return Python::with_gil(|py| -> Self {
            return Self {
                model: self.model.clone_ref(py),
                device: self.device.clone(),
            };
        });
    }
}

impl CoquiModel {
    pub fn new(model_name: String, use_gpu: bool) -> Result<Self, Box<dyn Error>> {
        let m = Python::with_gil(|py| -> Result<Self, Box<dyn Error>> {
            let torch = py.import("torch")?;
            let tts = py.import("TTS.api")?;

            let cuda: bool = torch
                .getattr("cuda")?
                .getattr("is_available")?
                .call0()?
                .extract()?;

            let device: String = if cuda && use_gpu {
                "cuda:0".to_string()
            } else {
                "cpu".to_string()
            };

            let model = tts
                .getattr("TTS")?
                .call1((("model_name", model_name), ("progress_bar", false)))?
                .getattr("to")?
                .call1((device.clone(), ("return_tensors", "pt")))?
                .unbind();

            return Ok(Self { model, device });
        });

        return m;
    }

    pub fn generate(&self, message: String, path: String) -> Result<(), Box<dyn Error>> {
        return Python::with_gil(|py| -> Result<(), Box<dyn Error>> {
            self.model
                .getattr(py, "tts_to_file")?
                .call1(py, (("text", message), ("file_path", path)))?;
            Ok(())
        });
    }
}

impl Default for CoquiModel {
    fn default() -> Self {
        return Self::new("tts_models/en/ljspeech/vits".to_string(), true).unwrap();
    }
}

impl NaturalModelTrait for CoquiModel {
    type SynthesizeType = f32;

    fn save(&mut self, message: String, path: String) -> Result<(), Box<dyn Error>> {
        let _ = self.generate(message, path.clone())?;
        did_save(path.as_str())
    }

    fn say(&mut self, message: String) -> Result<(), Box<dyn Error>> {
        speak_model(self, message)
    }

    fn synthesize(
        &mut self,
        message: String,
    ) -> Result<SynthesizedAudio<Self::SynthesizeType>, Box<dyn Error>> {
        synthesize_model(self, message)
    }
}
