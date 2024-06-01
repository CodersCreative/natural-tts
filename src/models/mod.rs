pub mod coqui;
pub mod parler;
pub mod gtts;
use tts::Tts;
use std::error::Error;
use std::fs::File;
use crate::TtsError;

use crate::utils::{get_path, play_wav_file, read_wav_file};

pub trait NaturalModelTrait {
    fn save(&mut self, message : String, path : String) -> Result<(), Box<dyn Error>>;
    fn say(&mut self, message : String) -> Result<(), Box<dyn Error>>;
    fn synthesize(&mut self, message : String) -> Result<Vec<f32>, Box<dyn Error>>;
}


pub fn speak_model<T : NaturalModelTrait>(model : &mut T, message : String) -> Result<(), Box<dyn Error>>{
    let path = "text_to_speech/output.wav";
    let actual = get_path(path.to_string());
    std::fs::remove_file(actual.clone());
    model.save(message.clone(), actual.clone());
    play_wav_file(&actual);
    std::fs::remove_file(actual);
    Ok(())
}

pub fn synthesize_model<T : NaturalModelTrait>(model : &mut T, message : String) -> Result<Vec<f32>, Box<dyn Error>>{
    let path = "text_to_speech/output.wav";
    let actual = get_path(path.to_string());
    std::fs::remove_file(actual.clone());
    model.save(message.clone(), actual.clone());
    let rwf = read_wav_file(&actual)?;
    std::fs::remove_file(actual);
    Ok(rwf)
}
 pub fn did_save(path : &str) -> Result<(), Box<dyn Error>>{
     let file = File::open(path);
     match file{
        Ok(_) => Ok(()),
        Err(_) => Err(Box::new(TtsError::NotSaved)),
     }
 }
impl NaturalModelTrait for Tts{
    fn save(&mut self, message : String, path : String) -> Result<(), Box<dyn Error>> {
        Err(Box::new(TtsError::NotSupported))
    }

    fn say(&mut self, message : String) -> Result<(), Box<dyn Error>> {
        let is_speaking = self.is_speaking();
        
        if let Ok(speaking) = is_speaking{
            if speaking{
                return Ok(());
            }
        }

        let _ = self.speak(message, false);
        Ok(())
    }

    fn synthesize(&mut self, message : String) -> Result<Vec<f32>, Box<dyn Error>> {
        Err(Box::new(TtsError::NotSupported))
    }
}


