// Copyright (c) 2024-2025 natural-tts
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
use hound::WavReader;
use std::{error::Error, io::Write};
use rodio::{cpal::FromSample, buffer::SamplesBuffer, Decoder, Sink, OutputStream};
use crate::models::{Spec::Wav, SynthesizedAudio};

/// Reads a .wav file and converts it into a SynthesizedAudio object.
///
/// # Arguments
///
/// * `path`: The path to the .wav file.
///
/// # Returns
///
/// * A SynthesizedAudio object containing the samples and metadata of the file.
///
/// # Errors
///
/// * If the file at `path` cannot be found or opened.
/// * If the file at `path` is not a valid .wav file.
/// * If there is an error reading the samples from the file.
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

/// Returns a path to a file within the `src/` directory.
///
/// # Arguments
///
/// * `path`: The path to the file, relative to the `src/` directory.
///
/// # Returns
///
/// * A string representing the absolute path to the file.
pub fn get_path(path : String) -> String{
    let mut new_path = env!("CARGO_MANIFEST_DIR").to_string();
    new_path.push_str(&format!("/src/{}", path));
    return new_path;
}

/// Plays a raw audio buffer using the default audio device.
///
/// # Arguments
///
/// * `data`: The raw audio samples to play.
/// * `rate`: The sample rate of the audio, in Hz.
///
/// # Errors
///
/// * If the default audio device cannot be opened.
/// * If there is an error playing the audio.
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

/// Plays a WAV file using the default audio device.
///
/// # Arguments
///
/// * `path`: The path to the WAV file to play.
///
/// # Errors
///
/// * If the file at `path` cannot be opened.
/// * If the WAV file is malformed.
/// * If there is an error playing the audio.
pub fn play_wav_file(path: &str) -> Result<(), Box<dyn Error>>{
    let file = std::fs::File::open(path)?;
    let decoder = Decoder::new(file)?;
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    sink.append(decoder);
    sink.sleep_until_end();
    
    Ok(())
}

/// Saves a slice of `f32` samples to a WAV file.
///
/// # Arguments
///
/// * `data`: The slice of `f32` samples to save.
/// * `filename`: The path to the WAV file to save.
/// * `sample_rate`: The sample rate of the audio, in Hz.
///
/// # Errors
///
/// * If there is an error creating or writing to the file at `filename`.
///
/// # Notes
///
/// * This function assumes that the `data` slice contains samples between -1.0 and 1.0.
/// * This function will panic if `data` is empty.
/// * This function does not support writing to non-WAV files, or writing non-PCM format WAV files.
pub fn save_wav(data: &[f32], filename: &str, sample_rate: u32) -> Result<(), std::io::Error> {
    let mut file = std::fs::File::create(filename)?;

    // Write WAV header
    let (chunk_size, bits_per_sample) = (44 + data.len() * 4, 32);
    let pcm = 1; // PCM format

    let header = [
        // RIFF chunk
        b'R', b'I', b'F', b'F',
        (chunk_size & 0xff) as u8, (chunk_size >> 8 & 0xff) as u8, (chunk_size >> 16 & 0xff) as u8, (chunk_size >> 24 & 0xff) as u8,
        // WAVE chunk
        b'W', b'A', b'V', b'E',
        // fmt subchunk
        b'f', b'm', b't', b' ',
        16, 0, 0, 0, // Subchunk size (16 for PCM format)
        pcm as u8, 0, // PCM format
        1, 0, // Mono channel
        (sample_rate & 0xff) as u8, (sample_rate >> 8 & 0xff) as u8, (sample_rate >> 16 & 0xff) as u8, (sample_rate >> 24 & 0xff) as u8,
        4, 0, // Average bytes per second (4 for 32-bit samples)
        4, 0, // Block align (4 for 32-bit samples, mono)
        bits_per_sample as u8, 0, // Bits per sample
        // data subchunk
        b'd', b'a', b't', b'a',
        (data.len() * 4) as u8, (data.len() * 4 >> 8 & 0xff) as u8, (data.len() * 4 >> 16 & 0xff) as u8, (data.len() * 4 >> 24 & 0xff) as u8,
    ];
    file.write_all(&header)?;

    // Write audio data (assuming f32 samples between -1.0 and 1.0)
    for sample in data {
        let i32_sample = (sample * 2147483647.0) as i32; // Convert to 32-bit signed integer
        let bytes = i32_sample.to_le_bytes();
        file.write_all(&bytes)?;
    }

    Ok(())
}
