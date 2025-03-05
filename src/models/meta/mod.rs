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
pub mod bs1770;
pub mod utils;

use std::{path::PathBuf,io::Write,error::Error};
use candle_transformers::{models::{encodec, quantized_metavoice::transformer as qtransformer, metavoice::{adapters, gpt, transformer}}, generation::LogitsProcessor};
use derive_builder::Builder;
use candle_core::{DType, IndexOp, Tensor, Device};
use candle_nn::VarBuilder;
use hf_hub::api::sync::Api;
use rand::{distributions::Distribution, SeedableRng};
use utils::*;
use crate::{utils::{get_path, play_wav_file, read_wav_file}, TtsError};
use super::{did_save, NaturalModelTrait, SynthesizedAudio};

#[derive(Builder, Clone, Default)]
#[builder(setter(into))]
pub struct MetaModelOptions{
    #[builder(default = "false")]
    cpu : bool,
    #[builder(default = "false")]
    tracing : bool, 
    #[builder(default = "false")]
    quantized : bool, 
    #[builder(default = "None")]
    first_stage_meta : Option<String>, 
    #[builder(default = "None")]
    first_stage_weights: Option<String>, 
    #[builder(default = "None")]
    second_stage_weights : Option<String>,
    #[builder(default = "None")]
    encodec_weights: Option<String>,
    #[builder(default = "None")]
    spk_emb : Option<String>,
    #[builder(default = "1024")]
    encodec_ntokens: u32, 
    #[builder(default = "299792458")]
    seed : u64,
    #[builder(default = "2000")]
    max_tokens : u64,
    #[builder(default = "3.0")]
    guidance_scale : f64,
    #[builder(default = "1.0")]
    temperature : f64,
    #[builder(default = "None")]
    repo : Option<String>,
}

#[derive(Clone)]
pub struct MetaModel {
    pub first_stage_model : Transformer, 
    pub device : Device,
    pub first_stage_meta : serde_json::Value,
    pub dtype : DType, 
    pub second_stage_config : gpt::Config,
    pub encodec_config : encodec::Config,
    pub encodec_weights : PathBuf,
    pub second_stage_weights : PathBuf,
    pub encodec_device : Device, 
    pub repo_path : String,
    pub seed : u64,
    pub guidance_scale : f64,
    pub temperature : f64,
    pub max_tokens : u64,
    pub spk_emb : Option<String>,
    pub encodec_ntokens : u32,
}


impl MetaModel{
    pub fn new(options : MetaModelOptions) -> Result<Self, Box<dyn Error>> {
        use tracing_chrome::ChromeLayerBuilder;
        use tracing_subscriber::prelude::*;

        let _guard = if options.tracing {
            let (chrome_layer, guard) = ChromeLayerBuilder::new().build();
            tracing_subscriber::registry().with(chrome_layer).init();
            Some(guard)
        } else {
            None
        };

        let device = device(options.cpu)?;
        let api = Api::new()?;
        let repo_path = options.repo.unwrap_or_else(|| "lmz/candle-metavoice".to_string());

        let repo = api.model(repo_path.clone());
        
        let first_stage_meta = match &options.first_stage_meta {
            Some(w) => std::path::PathBuf::from(w),
            None => repo.get("first_stage.meta.json")?,
        };

        let first_stage_meta: serde_json::Value =
            serde_json::from_reader(&std::fs::File::open(first_stage_meta)?)?;

        let dtype = DType::F16;

        let first_stage_config = transformer::Config::cfg1b_v0_1();
        
        let first_stage_model = if options.quantized {
            let filename = match &options.first_stage_weights {
                Some(w) => std::path::PathBuf::from(w),
                None => repo.get("first_stage_q4k.gguf")?,
            };

            let vb = candle_transformers::quantized_var_builder::VarBuilder::from_gguf(filename, &device)?;
            
            let first_stage_model = qtransformer::Model::new(&first_stage_config, vb)?;
            
            Transformer::Quantized(first_stage_model)
        } else {
            let first_stage_weights = match &options.first_stage_weights {
                Some(w) => std::path::PathBuf::from(w),
                None => repo.get("first_stage.safetensors")?,
            };

            let first_stage_vb = unsafe { VarBuilder::from_mmaped_safetensors(&[first_stage_weights], dtype, &device)? };
            
            let first_stage_model = transformer::Model::new(&first_stage_config, first_stage_vb)?;
            
            Transformer::Normal(first_stage_model)
        };

        let encodec_device = if device.is_metal() {
            Device::Cpu
        } else {
            device.clone()
        };

        let second_stage_config = gpt::Config::cfg1b_v0_1();
        let encodec_config = encodec::Config::default();

        let second_stage_weights = match &options.second_stage_weights {
            Some(w) => std::path::PathBuf::from(w),
            None => repo.get("second_stage.safetensors")?,
        };

        let encodec_weights = match &options.encodec_weights {
            Some(w) => std::path::PathBuf::from(w.clone()),
            None => Api::new()?
                .model("facebook/encodec_24khz".to_string())
                .get("model.safetensors")?,
        };
        
        Ok(Self{
            first_stage_model,
            device,
            first_stage_meta,
            dtype,
            encodec_weights,
            second_stage_weights,
            encodec_config,
            second_stage_config,
            encodec_device,
            repo_path,
            seed : options.seed,
            guidance_scale : options.guidance_scale,
            temperature : options.temperature,
            max_tokens : options.max_tokens,
            spk_emb : options.spk_emb,
            encodec_ntokens : options.encodec_ntokens
        })
    }

    pub fn get_secondary_models(&self) -> Result<(gpt::Model, encodec::Model), Box<dyn Error>>{
        let second_stage_vb = unsafe { VarBuilder::from_mmaped_safetensors(&[self.second_stage_weights.clone()], self.dtype, &self.device)? };
        let second_stage_model = gpt::Model::new(self.second_stage_config.clone(), second_stage_vb)?;
        
        let encodec_vb = unsafe { VarBuilder::from_mmaped_safetensors(&[self.encodec_weights.clone()], self.dtype, &self.encodec_device)? };
        let encodec_model = encodec::Model::new(&self.encodec_config, encodec_vb)?;

        Ok((second_stage_model, encodec_model))
    }

    pub fn run (&self, prompt : String, filename : String,) -> Result<(), Box<dyn Error>>{
        let (second_stage_model, encodec_model) = self.get_secondary_models()?;

        let fs_tokenizer = get_fs_tokenizer(self.first_stage_meta.clone())?;

        let prompt_tokens = fs_tokenizer.encode(&prompt)?;
        let mut tokens = prompt_tokens.clone();
        
        let api = Api::new()?;
        let repo = api.model(self.repo_path.clone());
        
        let spk_emb_file = match &self.spk_emb {
            Some(w) => std::path::PathBuf::from(w),
            None => repo.get("spk_emb.safetensors")?,
        };

        let spk_emb = candle_core::safetensors::load(&spk_emb_file, &Device::Cpu)?;
        
        let spk_emb = match spk_emb.get("spk_emb") {
            None => return Err(TtsError::Tensor.into()),
            Some(spk_emb) => spk_emb.to_dtype(self.dtype)?,
        };
        
        let spk_emb = spk_emb.to_device(&self.device)?;
        let mut logits_processor = LogitsProcessor::new(self.seed, Some(self.temperature), Some(0.95));

        for index in 0..self.max_tokens {
            let context_size = if index > 0 { 1 } else { tokens.len() };
            let start_pos = tokens.len().saturating_sub(context_size);
            let ctxt = &tokens[start_pos..];
            let input = Tensor::new(ctxt, &self.device)?;
            let input = Tensor::stack(&[&input, &input], 0)?;
            let logits = match &self.first_stage_model {
                Transformer::Normal(m) => m.clone().forward(&input, &spk_emb, tokens.len() - context_size)?,
                Transformer::Quantized(m) => {
                    m.clone().forward(&input, &spk_emb, tokens.len() - context_size)?
                }
            };
            let logits0 = logits.i((0, 0))?;
            let logits1 = logits.i((1, 0))?;
            let logits = ((logits0 * self.guidance_scale)? + logits1 * (1. - self.guidance_scale))?;
            let logits = logits.to_dtype(DType::F32)?;
            let next_token = logits_processor.sample(&logits)?;
            tokens.push(next_token);
            print!(".");
            std::io::stdout().flush()?;
            if next_token == 2048 {
                break;
            }
        }
        let fie2c = adapters::FlattenedInterleavedEncodec2Codebook::new(self.encodec_ntokens);
        let (_, ids1, ids2) = fie2c.decode(&tokens);
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.seed + 1337);
        let encoded_text: Vec<_> = prompt_tokens.iter().map(|v| v - 1024).collect();
        let mut hierarchies_in1 =
            [encoded_text.as_slice(), ids1.as_slice(), &[self.encodec_ntokens]].concat();
        let mut hierarchies_in2 = [
            vec![self.encodec_ntokens; encoded_text.len()].as_slice(),
            ids2.as_slice(),
            &[self.encodec_ntokens],
        ]
        .concat();
        hierarchies_in1.resize(self.second_stage_config.block_size, self.encodec_ntokens);
        hierarchies_in2.resize(self.second_stage_config.block_size, self.encodec_ntokens);
        let in_x1 = Tensor::new(hierarchies_in1, &self.device)?;
        let in_x2 = Tensor::new(hierarchies_in2, &self.device)?;
        let in_x = Tensor::stack(&[in_x1, in_x2], 0)?.unsqueeze(0)?;
        let logits = second_stage_model.forward(&in_x)?;
        let mut codes = vec![];
        for logits in logits.iter() {
            let logits = logits.squeeze(0)?;
            let (seq_len, _) = logits.dims2()?;
            let mut codes_ = Vec::with_capacity(seq_len);
            for step in 0..seq_len {
                let logits = logits.i(step)?.to_dtype(DType::F32)?;
                let logits = &(&logits / 1.0)?;
                let prs = candle_nn::ops::softmax_last_dim(logits)?.to_vec1::<f32>()?;
                let distr = rand::distributions::WeightedIndex::new(prs.as_slice())?;
                let sample = distr.sample(&mut rng) as u32;
                codes_.push(sample)
            }
            codes.push(codes_)
        }

        let codes = Tensor::new(codes, &self.device)?.unsqueeze(0)?;
        let codes = Tensor::cat(&[in_x, codes], 1)?;
        let tilted_encodec = adapters::TiltedEncodec::new(self.encodec_ntokens);
        let codes = codes.i(0)?.to_vec2::<u32>()?;
        let (_, audio_ids) = tilted_encodec.decode(&codes);
        let audio_ids = Tensor::new(audio_ids, &self.encodec_device)?.unsqueeze(0)?;
        let pcm = encodec_model.decode(&audio_ids)?;
        let pcm = pcm.i(0)?.i(0)?.to_dtype(DType::F32)?;
        let pcm = normalize_loudness(&pcm, 24_000, true)?;
        let pcm = pcm.to_vec1::<f32>()?;
        let mut output = std::fs::File::create(&filename)?;
        write_pcm_as_wav(&mut output, &pcm, 24_000)?;
        Ok(())
    }
}

impl Default for MetaModel{
    fn default() -> Self {
        Self::new(MetaModelOptions::default()).unwrap()
    }
}

impl NaturalModelTrait for MetaModel{
    type SynthesizeType = f32;

    fn save(&self, message: String, path : String) -> Result<(), Box<dyn Error>>{
        let _ = self.run(message, path.clone())?;
        did_save(path.as_str())
    }

    fn say(&mut self, message : String)  -> Result<(), Box<dyn Error>>{
        let path = get_path("temp.wav".to_string());
        self.save(message, path.clone())?;
        play_wav_file(&path)?;
        std::fs::remove_file(path)?;
        Ok(())
    }

    fn synthesize(&self, message : String) -> Result<SynthesizedAudio<Self::SynthesizeType>, Box<dyn Error>> {
        let path = get_path("temp.wav".to_string());
        self.save(message, path.clone())?;
        let d = read_wav_file(&path)?;
        std::fs::remove_file(path)?;
        Ok(d)
    }
}


