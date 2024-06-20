use meta::MetaModel;

use crate::{*, models::{gtts::GttsModel, tts_rs::TtsModel, parler::ParlerModel, msedge::MSEdgeModel}};

#[test]
fn gtts_test(){
    let mut natural = NaturalTtsBuilder::default()
        .gtts_model(GttsModel::default())
        .default_model(Model::Gtts)
        .build().unwrap();
    let _ = natural.say_auto("Hello, World!".to_string());
}


#[test]
fn parler_test(){
    let desc = "A female speaker in fast calming voice in a quiet environment".to_string();
    let model = "parler-tts/parler-tts-mini-expresso".to_string();
    let parler = ParlerModel::new(desc, model, false);
    
    let mut natural = NaturalTtsBuilder::default()
        .parler_model(parler.unwrap())
        .default_model(Model::Parler)
        .build().unwrap();
    let _ = natural.say_auto("Hello, World!".to_string());
}

#[test]
fn msedge_test(){
    let mut natural = NaturalTtsBuilder::default()
        .msedge_model(MSEdgeModel::default())
        .default_model(Model::MSEdge)
        .build().unwrap();
    let _ = natural.say_auto("Hello, World!".to_string());
}

#[test]
fn tts_test(){
    let mut natural = NaturalTtsBuilder::default()
        .tts_model(TtsModel::default())
        .default_model(Model::TTS)
        .build().unwrap();
    let _ = natural.say_auto("Hello, World!".to_string());
}

#[test]
fn meta_test(){
    let mut natural = NaturalTtsBuilder::default()
        .meta_model(MetaModel::default())
        .default_model(Model::Meta)
        .build().unwrap();
    let _ = natural.say_auto("Hello, World!".to_string());
}
// #[test]
// fn coqui_test(){
//     let mut natural = NaturalTtsBuilder::default()
//         .coqui_model(CoquiModel::default())
//         .default_model(Model::Coqui)
//         .build().unwrap();
//     let _ = natural.say_auto("Hello, World!".to_string());
// }

#[test]
fn all_tts(){
    let desc = "A female speaker in fast calming voice in a quiet environment".to_string();
    let model = "parler-tts/parler-tts-mini-expresso".to_string();
    let parler = ParlerModel::new(desc, model, false);

    let mut natural = NaturalTtsBuilder::default()
        .default_model(Model::Gtts)
        .gtts_model(GttsModel::default())
        .parler_model(parler.unwrap())
        .tts_model(TtsModel::default())
        .build().unwrap();

    let _ = natural.say_auto("Hello, World!".to_string());
}
