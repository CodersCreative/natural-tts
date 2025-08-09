pub mod bs1770;
pub mod utils;

use super::{did_save, NaturalModelTrait, SynthesizedAudio};
use crate::TtsError;
use candle_core::{DType, Device, IndexOp, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::{
    generation::LogitsProcessor,
    models::{
        encodec,
        metavoice::{adapters, gpt, transformer},
    },
};
use derive_builder::Builder;
use hf_hub::api::sync::Api;
use hound::WavSpec;
use rand::{distributions::Distribution, SeedableRng};
use std::{error::Error, io::Write, path::PathBuf};
use utils::*;

const MODEL_NAME: &str = "lmz/candle-metavoice";

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct MetaModelOptions {
    #[builder(default = "false")]
    pub cpu: bool,
    #[builder(default = "MODEL_NAME.to_string()")]
    model_name: String,
    #[builder(default = "false")]
    pub tracing: bool,
    #[builder(default = "None")]
    pub spk_emb: Option<String>,
    #[builder(default = "1024")]
    pub encodec_ntokens: u32,
    #[builder(default = "299792458")]
    pub seed: u64,
    #[builder(default = "8")]
    pub max_tokens: u64,
    #[builder(default = "3.0")]
    pub guidance_scale: f64,
    #[builder(default = "1.0")]
    pub temperature: f64,
}

#[derive(Clone)]
pub struct MetaModel {
    pub first_stage_model: transformer::Model,
    pub device: Device,
    pub first_stage_meta: serde_json::Value,
    pub dtype: DType,
    pub encodec_weights: PathBuf,
    pub second_stage_weights: PathBuf,
    pub repo_path: String,
    pub seed: u64,
    pub guidance_scale: f64,
    pub temperature: f64,
    pub max_tokens: u64,
    pub spk_emb: Option<String>,
    pub encodec_ntokens: u32,
}

impl MetaModel {
    pub fn new(options: MetaModelOptions) -> Result<Self, Box<dyn Error>> {
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

        let repo = api.model(options.model_name.clone());

        let first_stage_meta = repo.get("first_stage.meta.json")?;
        let first_stage_meta: serde_json::Value =
            serde_json::from_reader(&std::fs::File::open(first_stage_meta)?)?;

        let dtype = DType::F32;
        let first_stage_config = transformer::Config::cfg1b_v0_1();

        let first_stage_weights = repo.get("first_stage.safetensors")?;
        let first_stage_vb =
            unsafe { VarBuilder::from_mmaped_safetensors(&[first_stage_weights], dtype, &device)? };
        let first_stage_model = transformer::Model::new(&first_stage_config, first_stage_vb)?;

        let second_stage_weights = repo.get("second_stage.safetensors")?;

        let encodec_weights = Api::new()?
            .model("sanchit-gandhi/encodec_24khz".to_string())
            .get("model.safetensors")?;

        return Ok(Self {
            first_stage_model,
            device,
            first_stage_meta,
            dtype,
            encodec_weights,
            second_stage_weights,
            repo_path: options.model_name,
            seed: options.seed,
            guidance_scale: options.guidance_scale,
            temperature: options.temperature,
            max_tokens: options.max_tokens,
            spk_emb: options.spk_emb,
            encodec_ntokens: options.encodec_ntokens,
        });
    }

    pub fn generate(&mut self, prompt: String) -> Result<SynthesizedAudio<f32>, Box<dyn Error>> {
        let second_stage_vb = unsafe {
            VarBuilder::from_mmaped_safetensors(
                &[self.second_stage_weights.clone()],
                self.dtype,
                &self.device,
            )?
        };
        let second_stage_config = gpt::Config::cfg1b_v0_1();
        let second_stage_model = gpt::Model::new(second_stage_config.clone(), second_stage_vb)?;

        let encodec_device = if self.device.is_metal() {
            candle_core::Device::Cpu
        } else {
            self.device.clone()
        };

        let encodec_vb = unsafe {
            VarBuilder::from_mmaped_safetensors(
                &[self.encodec_weights.clone()],
                self.dtype,
                &encodec_device,
            )?
        };
        let encodec_config = encodec::Config::default();
        let encodec_model = encodec::Model::new(&encodec_config, encodec_vb)?;

        let fs_tokenizer = get_fs_tokenizer(self.first_stage_meta.clone())?;
        let prompt_tokens = fs_tokenizer.encode(&prompt)?;
        let mut tokens = prompt_tokens.clone();

        let api = Api::new()?;
        let repo = api.model(self.repo_path.clone());

        let spk_emb_file = match &self.spk_emb {
            Some(w) => std::path::PathBuf::from(w),
            None => repo.get("spk_emb.safetensors")?,
        };

        let spk_emb = candle_core::safetensors::load(&spk_emb_file, &candle_core::Device::Cpu)?;

        let spk_emb = match spk_emb.get("spk_emb") {
            None => return Err(TtsError::Tensor.into()),
            Some(spk_emb) => spk_emb.to_dtype(self.dtype)?,
        };

        let spk_emb = spk_emb.to_device(&self.device)?;
        let mut logits_processor = LogitsProcessor::new(self.seed, Some(self.temperature), None);

        for index in 0..self.max_tokens {
            let context_size = if index > 0 { 1 } else { tokens.len() };
            let start_pos = tokens.len().saturating_sub(context_size);
            let ctxt = &tokens[start_pos..];
            let input = Tensor::new(ctxt, &self.device)?;
            let input = Tensor::stack(&[&input, &input], 0)?;
            let logits =
                self.first_stage_model
                    .forward(&input, &spk_emb, tokens.len() - context_size)?;
            //let logits0 = logits.i((0, 0))?;
            //let logits1 = logits.i((1, 0))?;
            let logits = logits.i((0, logits.dim(1)? - 1))?;
            //let logits = ((logits0 * self.guidance_scale)? + logits1 * (1. - self.guidance_scale))?;
            let logits = logits.to_dtype(self.dtype)?;
            let next_token = match logits_processor.sample(&logits) {
                Ok(x) => x,
                Err(e) => {
                    println!("{}", e.to_string());
                    continue;
                }
            };
            tokens.push(next_token);
            std::io::stdout().flush()?;
            if next_token == 2048 {
                break;
            }
        }

        let fie2c = adapters::FlattenedInterleavedEncodec2Codebook::new(self.encodec_ntokens);
        let (_, ids1, ids2) = fie2c.decode(&tokens);
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.seed + 1337);
        let encoded_text: Vec<_> = prompt_tokens.iter().map(|v| v - 1024).collect();
        let mut hierarchies_in1 = [
            encoded_text.as_slice(),
            ids1.as_slice(),
            &[self.encodec_ntokens],
        ]
        .concat();
        let mut hierarchies_in2 = [
            vec![self.encodec_ntokens; encoded_text.len()].as_slice(),
            ids2.as_slice(),
            &[self.encodec_ntokens],
        ]
        .concat();

        hierarchies_in1.resize(second_stage_config.block_size, self.encodec_ntokens);
        hierarchies_in2.resize(second_stage_config.block_size, self.encodec_ntokens);
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
        //let tilted_encodec = adapters::TiltedEncodec::new(512);
        let codes = codes.i(0)?.to_vec2::<u32>()?;
        let (_, audio_ids) = tilted_encodec.decode(&codes);
        let audio_ids = Tensor::new(audio_ids, &encodec_device)
            .unwrap()
            .unsqueeze(0)
            .unwrap();

        let pcm = encodec_model.decode(&audio_ids)?;
        let pcm = pcm.i(0)?.i(0)?.to_dtype(DType::F32)?;
        let pcm = normalize_loudness(&pcm, 24_000, true)?;

        let pcm = pcm.to_vec1::<f32>()?;
        return Ok(SynthesizedAudio::new(
            pcm,
            super::Spec::Wav(WavSpec {
                sample_rate: encodec_config.sampling_rate as u32,
                channels: encodec_config.audio_channels as u16,
                sample_format: hound::SampleFormat::Float,
                bits_per_sample: encodec_config.sampling_rate as u16,
            }),
            None,
        ));
    }
}

impl Default for MetaModel {
    fn default() -> Self {
        return Self::new(MetaModelOptionsBuilder::default().build().unwrap()).unwrap();
    }
}

impl NaturalModelTrait for MetaModel {
    type SynthesizeType = f32;

    fn save(&mut self, message: String, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let data = self.synthesize(message, path)?;
        let mut output = std::fs::File::create(&path)?;
        write_pcm_as_wav(&mut output, &data.data, 24_000 as u32)?;
        did_save(path)
    }

    fn synthesize(
        &mut self,
        message: String,
        _path: &PathBuf,
    ) -> Result<SynthesizedAudio<Self::SynthesizeType>, Box<dyn Error>> {
        self.generate(message)
    }
}
