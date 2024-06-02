use msedge::MSEdgeModel;

use crate::{*, models::{gtts::GttsModel}};

#[test]
fn gtts_test(){
    let mut natural = NaturalTtsBuilder::default()
        .gtts_model(GttsModel::default())
        .default_model(Model::Gtts)
        .build().unwrap();
    let _ = natural.say_auto("Hello, World!".to_string());
}



#[test]
fn msedge_test(){
    let mut natural = NaturalTtsBuilder::default()
        .msedge_model(MSEdgeModel::default())
        .default_model(Model::Gtts)
        .build().unwrap();
    let _ = natural.say_auto("Hello, World!".to_string());
}
