pub mod model;
use super::{did_save, NaturalModelTrait, SynthesizedAudio};
use candle_core::{DType, Device, IndexOp, Tensor};
use candle_nn::VarBuilder;
use derive_builder::Builder;
use hf_hub::api::sync::Api;
use hound::WavSpec;
use model::*;
use std::{error::Error, path::PathBuf};
use tokenizers::Tokenizer;

use super::meta::utils::*;

const MODEL_NAME: &str = "parler-tts/parler-tts-mini-v1";

#[derive(Builder, Clone, Default)]
#[builder(setter(into))]
pub struct ParlerModelOptions {
    #[builder(default = "false")]
    cpu: bool,
    description: String,
    #[builder(default = "false")]
    tracing: bool,
    #[builder(default = "1.0")]
    temperature: f64,
    #[builder(default = "None")]
    top_p: Option<f64>,
    #[builder(default = "299792458")]
    seed: u64,
    #[builder(default = "MODEL_NAME.to_string()")]
    model_name: String,
}

#[derive(Clone)]
pub struct ParlerModel {
    device: Device,
    config: Config,
    model: Model,
    description: String,
    temperature: f64,
    top_p: Option<f64>,
    seed: u64,
    tokenizer: Tokenizer,
}

impl ParlerModel {
    pub fn new(options: ParlerModelOptions) -> Result<Self, Box<dyn Error>> {
        use tracing_chrome::ChromeLayerBuilder;
        use tracing_subscriber::prelude::*;

        let _guard = if options.tracing {
            let (chrome_layer, guard) = ChromeLayerBuilder::new().build();
            tracing_subscriber::registry().with(chrome_layer).init();
            Some(guard)
        } else {
            None
        };

        let api = Api::new()?;
        let device = device(options.cpu)?;

        let revision = "main".to_string();

        let repo = api.repo(hf_hub::Repo::with_revision(
            options.model_name,
            hf_hub::RepoType::Model,
            revision.clone(),
        ));

        let config = repo.get("config.json")?;

        let model_files = match repo.get("model.safetensors") {
            Ok(x) => vec![x],
            Err(_) => hub_load_safetensors(&repo, "model.safetensors.index.json")?,
        };

        let vb = unsafe { VarBuilder::from_mmaped_safetensors(&model_files, DType::F32, &device)? };

        let config: Config = serde_json::from_reader(std::fs::File::open(config)?)?;
        let model = Model::new(&config, vb)?;

        let tokenizer = repo.get("tokenizer.json")?;

        let tokenizer = Tokenizer::from_file(tokenizer).unwrap();

        return Ok(Self {
            device,
            config,
            top_p: options.top_p,
            description: options.description,
            model,
            tokenizer,
            seed: options.seed,
            temperature: options.temperature,
        });
    }

    pub fn generate(&mut self, message: String) -> Result<SynthesizedAudio<f32>, Box<dyn Error>> {
        let description_tokens = self
            .tokenizer
            .encode(self.description.clone(), true)
            .unwrap()
            .get_ids()
            .to_vec();
        let description_tokens = Tensor::new(description_tokens, &self.device)?.unsqueeze(0)?;
        let prompt_tokens = self
            .tokenizer
            .encode(message, true)
            .unwrap()
            .get_ids()
            .to_vec();
        let prompt_tokens = Tensor::new(prompt_tokens, &self.device)?.unsqueeze(0)?;

        let lp = candle_transformers::generation::LogitsProcessor::new(
            self.seed,
            Some(self.temperature),
            self.top_p,
        );

        let codes = self
            .model
            .generate(&prompt_tokens, &description_tokens, lp, 512)?;
        let codes = codes.to_dtype(DType::I64)?;
        codes.save_safetensors("codes", "out.safetensors")?;
        let codes = codes.unsqueeze(0)?;

        let pcm = self
            .model
            .audio_encoder
            .decode_codes(&codes.to_device(&self.device)?)?;
        let pcm = pcm.i((0, 0))?;
        let pcm = normalize_loudness(&pcm, 24_000, true)?;
        let pcm = pcm.to_vec1::<f32>()?;

        return Ok(SynthesizedAudio::new(
            pcm,
            super::Spec::Wav(WavSpec {
                sample_rate: self.config.audio_encoder.sampling_rate,
                channels: 1,
                sample_format: hound::SampleFormat::Float,
                bits_per_sample: self.config.audio_encoder.model_bitrate as u16,
            }),
            None,
        ));
    }
}

impl Default for ParlerModel {
    fn default() -> Self {
        let desc = "A female speaker in fast calming voice in a quiet environment".to_string();
        let model = "parler-tts/parler-tts-mini-expresso".to_string();
        return Self::new(
            ParlerModelOptionsBuilder::default()
                .model_name(model)
                .description(desc)
                .build()
                .unwrap(),
        )
        .unwrap();
    }
}

impl NaturalModelTrait for ParlerModel {
    type SynthesizeType = f32;
    fn save(&mut self, message: String, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let data = self.synthesize(message, path)?;
        let mut output = std::fs::File::create(&path)?;
        write_pcm_as_wav(
            &mut output,
            &data.data,
            self.config.audio_encoder.sampling_rate,
        )?;
        did_save(path)
    }
}
