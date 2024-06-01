use crate::{*, models::{parler::ParlerModel, coqui::CoquiModel}};

#[test]
fn coqui_test(){
    let mut natural = NaturalTtsBuilder::default()
        .coqui_model(Some(CoquiModel::new("tts_models/en/ljspeech/vits".to_string()).unwrap()))
        .default_model(Model::Coqui)
        .build().unwrap();
    let _ = natural.say_auto("Hello, World!".to_string());
}
