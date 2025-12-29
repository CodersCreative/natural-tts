use thiserror::Error;

#[derive(Debug, Error)]
pub enum TtsError {
    #[error("Not Supported")]
    NotSupported,
    #[error("Operation failed")]
    OperationFailed,
    #[error("Model Not Loaded")]
    NotLoaded,
    #[error("Didn't Save")]
    NotSaved,
    #[error("Default model not set")]
    NoDefaultModel,
    #[error("Tensor Error")]
    Tensor,
    #[error("No Tokenizer Key")]
    NoTokenizerKey,
    #[error("Hound Error : {0}")]
    Hound(#[from] hound::Error),
    #[error("Rodio Play Error : {0}")]
    RodioPlay(#[from] rodio::PlayError),
    #[error("Rodio Stream Error : {0}")]
    RodioStream(#[from] rodio::StreamError),
    #[error("Rodio Devices Error : {0}")]
    RodioDevices(#[from] rodio::DevicesError),
    #[error("IO Error : {0}")]
    IO(#[from] std::io::Error),
    #[error("TTS Error : {0}")]
    #[cfg(feature = "tts-rs")]
    Tts(#[from] tts::Error),
    #[error("MSEdge Error : {0}")]
    #[cfg(feature = "msedge")]
    MSEdge(#[from] msedge_tts::error::Error),
    #[error("Pyo3 Error : {0}")]
    #[cfg(feature = "coqui")]
    Py(#[from] pyo3::PyErr),
    #[error("Candle Error : {0}")]
    #[cfg(feature = "meta")]
    Candle(#[from] candle_core::Error),
    #[error("Rand Error : {0}")]
    #[cfg(feature = "meta")]
    RandWeights(#[from] rand::seq::WeightError),
    #[error("HF Hub Error : {0}")]
    #[cfg(feature = "meta")]
    HFHub(#[from] hf_hub::api::sync::ApiError),
    #[error("Serder Error : {0}")]
    #[cfg(feature = "meta")]
    Serde(#[from] serde_json::Error),
    #[error("Json Error")]
    Json,
    #[error("Error {0}")]
    Text(String),
}

impl From<String> for TtsError {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}
