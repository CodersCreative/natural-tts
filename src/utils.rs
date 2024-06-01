use hound::WavReader;
use std::error::Error;
use rodio::{Decoder, Sink, OutputStream};

pub fn read_wav_file(path: &str) -> Result<Vec<f32>, Box<dyn Error>> {
    let mut reader = WavReader::open(path)?;
    let mut f32_samples: Vec<f32> = Vec::new();

    reader.samples::<f32>().for_each(|s| {
        if let Ok(sample) = s{
            f32_samples.push(sample as f32);
        }
    });

    Ok(f32_samples)
}

pub fn get_path(path : String) -> String{
    let mut new_path = env!("CARGO_MANIFEST_DIR").to_string();
    new_path.push_str(&format!("/src/{}", path));
    return new_path;
}


pub fn play_wav_file(path: &str) -> Result<(), Box<dyn Error>>{
    let file = std::fs::File::open(path)?;
    let decoder = Decoder::new(file)?;
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    sink.append(decoder);
    sink.sleep_until_end();
    
    Ok(())
}
