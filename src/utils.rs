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
use crate::models::{Spec::Wav, SynthesizedAudio};
use hound::{Sample, WavReader};
use rodio::{buffer::SamplesBuffer, cpal::FromSample, Decoder, OutputStream, Sink};
use std::{error::Error, io::Write, path::PathBuf};

pub fn read_wav_file<T: Sample + Send + rodio::Sample>(
    path: &PathBuf,
) -> Result<SynthesizedAudio<T>, Box<dyn Error>> {
    let mut reader = WavReader::open(path)?;
    let mut samples: Vec<T> = Vec::new();

    reader.samples::<T>().for_each(|s| {
        if let Ok(sample) = s {
            samples.push(sample as T);
        }
    });

    Ok(SynthesizedAudio::new(
        samples,
        Wav(reader.spec()),
        Some(reader.duration() as i32),
    ))
}

#[allow(dead_code)]
pub fn get_path(path: String) -> String {
    let mut new_path = env!("CARGO_MANIFEST_DIR").to_string();
    new_path.push_str(&format!("/src/{}", path));
    return new_path;
}

pub fn play_audio<T>(data: &[T], rate: u32) -> Result<Sink, Box<dyn Error>>
where
    T: rodio::Sample + Send + 'static,
    f32: FromSample<T>,
{
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let source = SamplesBuffer::new(1, rate, data);
    let sink = rodio::Sink::try_new(&handle).unwrap();

    sink.append(source);

    Ok(sink)
}

pub fn play_wav_file(path: &PathBuf) -> Result<Sink, Box<dyn Error>> {
    let file = std::fs::File::open(path)?;
    let decoder = Decoder::new(file)?;
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    sink.append(decoder);
    Ok(sink)
}

pub fn save_wav(data: &[f32], filename: &PathBuf, sample_rate: u32) -> Result<(), std::io::Error> {
    let mut file = std::fs::File::create(filename)?;

    // Write WAV header
    let (chunk_size, bits_per_sample) = (44 + data.len() * 4, 32);
    let pcm = 1; // PCM format

    let header = [
        // RIFF chunk
        b'R',
        b'I',
        b'F',
        b'F',
        (chunk_size & 0xff) as u8,
        (chunk_size >> 8 & 0xff) as u8,
        (chunk_size >> 16 & 0xff) as u8,
        (chunk_size >> 24 & 0xff) as u8,
        // WAVE chunk
        b'W',
        b'A',
        b'V',
        b'E',
        // fmt subchunk
        b'f',
        b'm',
        b't',
        b' ',
        16,
        0,
        0,
        0, // Subchunk size (16 for PCM format)
        pcm as u8,
        0, // PCM format
        1,
        0, // Mono channel
        (sample_rate & 0xff) as u8,
        (sample_rate >> 8 & 0xff) as u8,
        (sample_rate >> 16 & 0xff) as u8,
        (sample_rate >> 24 & 0xff) as u8,
        4,
        0, // Average bytes per second (4 for 32-bit samples)
        4,
        0, // Block align (4 for 32-bit samples, mono)
        bits_per_sample as u8,
        0, // Bits per sample
        // data subchunk
        b'd',
        b'a',
        b't',
        b'a',
        (data.len() * 4) as u8,
        (data.len() * 4 >> 8 & 0xff) as u8,
        (data.len() * 4 >> 16 & 0xff) as u8,
        (data.len() * 4 >> 24 & 0xff) as u8,
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
