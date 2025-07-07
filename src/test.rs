#[cfg(test)]

#[cfg(feature = "gtts")]
use crate::models::coqui::CoquiModel;
#[cfg(feature = "gtts")]
use crate::models::gtts::GttsModel;
#[cfg(feature = "meta")]
use crate::models::meta::MetaModel;
#[cfg(feature = "msedge")]
use crate::models::msedge::MSEdgeModel;
#[cfg(feature = "parler")]
use crate::models::parler::ParlerModel;
#[cfg(feature = "tts-rs")]
use crate::models::tts_rs::TtsModel;

use std::{error::Error, path::PathBuf};
use crate::{Model, NaturalTtsBuilder};

#[cfg(feature = "gtts")]
#[test]
fn gtts_test() {
    let mut natural = NaturalTtsBuilder::default()
        .gtts_model(GttsModel::default())
        .default_model(Model::Gtts)
        .build()
        .unwrap();
    let _ = natural.start("Hello, World!".to_string(), &PathBuf::from("output.wav"));
}

#[cfg(feature = "parler")]
#[test]
fn parler_test() {
    let mut natural = NaturalTtsBuilder::default()
        .parler_model(ParlerModel::default())
        .default_model(Model::Parler)
        .build()
        .unwrap();
    let _ = natural.start("Hello, World!".to_string(), &PathBuf::from("output.wav"));
}

#[cfg(feature = "msedge")]
#[test]
fn msedge_test() {
    let mut natural = NaturalTtsBuilder::default()
        .msedge_model(MSEdgeModel::default())
        .default_model(Model::MSEdge)
        .build()
        .unwrap();
    let _ = natural.start("Hello, World!".to_string(), &PathBuf::from("output.wav"));
}

#[cfg(feature = "tts-rs")]
#[test]
fn tts_test() {
    let mut natural = NaturalTtsBuilder::default()
        .tts_model(TtsModel::default())
        .default_model(Model::TTS)
        .build()
        .unwrap();
    let _ = natural.start("Hello, World!".to_string(), &PathBuf::from("output.wav"));
}

#[cfg(feature = "meta")]
#[test]
fn meta_test() {
    let mut natural = NaturalTtsBuilder::default()
        .meta_model(MetaModel::default())
        .default_model(Model::Meta)
        .build()
        .unwrap();
    let _ = natural.start("Hello, World!".to_string(), &PathBuf::from("output.wav"));
}

#[cfg(feature = "coqui")]
#[test]
fn coqui_test() {
    let mut natural = NaturalTtsBuilder::default()
        .coqui_model(CoquiModel::default())
        .default_model(Model::Coqui)
        .build()
        .unwrap();
    let _ = natural.start("Hello, World!".to_string(), &PathBuf::from("output.wav"));
}
