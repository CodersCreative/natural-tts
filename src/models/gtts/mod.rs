pub mod languages;
pub mod url;
use super::*;
use minreq::get;
use std::io::Write;
use url::EncodedFragment;

#[derive(Clone, Debug)]
pub struct GttsModel {
    pub volume: f32,
    pub language: languages::Languages,
    pub tld: String,
}

pub enum Speed {
    Normal,
    Slow,
}

impl GttsModel {
    pub fn new(volume: f32, language: languages::Languages, tld: String) -> Self {
        Self {
            language,
            volume,
            tld,
        }
    }

    pub fn generate(&self, message: String, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let len = message.len();
        if len > 100 {
            return Err(format!("The text is too long. Max length is {}", 100).into());
        }
        let language = self.language.as_code();
        let text = EncodedFragment::fragmenter(&message)?;
        let rep = get(format!("https://translate.google.{}/translate_tts?ie=UTF-8&q={}&tl={}&total=1&idx=0&textlen={}&tl={}&client=tw-ob", self.tld, text.encoded, language, len, language))
          .send()
          .map_err(|e| format!("{}", e))?;
        let mut file = File::create(path)?;
        let bytes = rep.as_bytes();
        let _ = file.write_all(bytes)?;

        Ok(())
    }
}

impl Default for GttsModel {
    fn default() -> Self {
        return Self::new(1.0, languages::Languages::English, String::from("com"));
    }
}

impl NaturalModelTrait for GttsModel {
    type SynthesizeType = f32;
    fn save(&mut self, message: String, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let _ = self.generate(message, path)?;
        did_save(path)
    }
}
