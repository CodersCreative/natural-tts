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

#[cfg(feature = "gtts")]
#[test]
fn gtts_test(){
    let mut natural = NaturalTtsBuilder::default()
        .gtts_model(GttsModel::default())
        .default_model(Model::Gtts)
        .build().unwrap();
    let _ = natural.say("Hello, World!".to_string());
}

#[cfg(feature = "parler")]
#[test]
fn parler_test(){
    let desc = "A female speaker in fast calming voice in a quiet environment".to_string();
    let model = "parler-tts/parler-tts-mini-expresso".to_string();
    let parler = ParlerModel::new(desc, model, false);
    
    let mut natural = NaturalTtsBuilder::default()
        .parler_model(parler.unwrap())
        .default_model(Model::Parler)
        .build().unwrap();
    let _ = natural.say("Hello, World!".to_string());
}

#[cfg(feature = "msedge")]
#[test]
fn msedge_test(){
    let mut natural = NaturalTtsBuilder::default()
        .msedge_model(MSEdgeModel::default())
        .default_model(Model::MSEdge)
        .build().unwrap();
    let _ = natural.say("Hello, World!".to_string());
}

#[cfg(feature = "tts-rs")]
#[test]
fn tts_test(){
    let mut natural = NaturalTtsBuilder::default()
        .tts_model(TtsModel::default())
        .default_model(Model::TTS)
        .build().unwrap();
    let _ = natural.say("Hello, World!".to_string());
}

#[cfg(feature = "meta")]
#[test]
fn meta_test(){
    let mut natural = NaturalTtsBuilder::default()
        .meta_model(MetaModel::default())
        .default_model(Model::Meta)
        .build().unwrap();
    let _ = natural.say("Hello, World!".to_string());
}

#[cfg(feature = "coqui")]
#[test]
fn coqui_test(){
    let mut natural = NaturalTtsBuilder::default()
        .coqui_model(CoquiModel::default())
        .default_model(Model::Coqui)
        .build().unwrap();
    let _ = natural.say("Hello, World!".to_string());
}

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

    let _ = natural.say("Hello, World!".to_string());
}
