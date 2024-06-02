use hound::{WavReader, WavSpec};
use rodio::cpal::FromSample;
use std::error::Error;
use rodio::{Decoder, Sink, OutputStream};
use crate::models::SynthesizedAudio;
use crate::models::Spec::Wav;
use rodio::buffer::SamplesBuffer;

pub fn read_wav_file(path: &str) -> Result<SynthesizedAudio<f32>, Box<dyn Error>> {
    let mut reader = WavReader::open(path)?;
    let mut f32_samples: Vec<f32> = Vec::new();

    reader.samples::<f32>().for_each(|s| {
        if let Ok(sample) = s{
            f32_samples.push(sample as f32);
        }
    });

    
    Ok(SynthesizedAudio::new(f32_samples, Wav(reader.spec()), Some(reader.duration() as i32)))
}

pub fn get_path(path : String) -> String{
    let mut new_path = env!("CARGO_MANIFEST_DIR").to_string();
    new_path.push_str(&format!("/src/{}", path));
    return new_path;
}

pub fn play_audio<T>(data: Vec<T>, rate : u32)
where
    T : rodio::Sample + Send + 'static, f32 : FromSample<T>    
{
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let source = SamplesBuffer::new(1, rate, data);
    let sink = rodio::Sink::try_new(&handle).unwrap();

    sink.append(source);

    sink.sleep_until_end();
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
