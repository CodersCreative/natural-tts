# Natural TTS

#### Natural TTS (natural-tts) is a rust crate for easily implementing Text-To-Speech into your rust programs.

### Available TTS Engines / AIs:
[Coqui TTS](https://github.com/coqui-ai/TTS)\
[Parler TTS](https://github.com/huggingface/parler-tts)\
[Google Gtts](https://github.com/pndurette/gTTS)\
[TTS-RS](https://github.com/ndarilek/tts-rs)\
[MSEdge TTS](https://github.com/hs-CN/msedge-tts)

### Install Rust

[Install Rust](https://www.rust-lang.org/tools/install)

On Linux or MacOS:
```
curl --proto '=https' --tlsv1.2 -ssf https://sh.rustup.rs | sh
```

### Example of saying something using Gtts but initializing every model.

```Rust
use std::error::Error;
use crate::{*, models::{gtts::GttsModel, tts_rs::TtsModel, parler::ParlerModel, msedge::MSEdgeModel}};

fn main() -> Result<(), Box<dyn Error>>{
    // Create the ParlerModel
    let desc = "A female speaker in fast calming voice in a quiet environment".to_string();
    let model = "parler-tts/parler-tts-mini-expresso".to_string();
    let parler = ParlerModel::new(desc, model, false);
    
    // Create the NaturalTts using the Builder pattern
    let mut natural = NaturalTtsBuilder::default()
        .default_model(Model::Gtts)
        .gtts_model(GttsModel::default())
        .parler_model(parler.unwrap())
        .tts_model(TtsModel::default())
        .build()?;
        
    // Use the pre-included function to say a message using the default_model.
    let _ = natural.say_auto("Hello, World!".to_string())?;
}

```

### Example of saying something using Parler

```Rust
use std::error::Error;
use natural_tts::{*, models::parler::ParlerModel};

fn main() -> Result<(), Box<dyn Error>>{
    // Create the ParlerModel
    let desc = "A female speaker in fast calming voice in a quiet environment".to_string();
    let model = "parler-tts/parler-tts-mini-expresso".to_string();
    let parler = ParlerModel::new(desc, model, false);
    
    // Create the NaturalTts using the Builder pattern
    let mut natural = NaturalTtsBuilder::default()
        .parler_model(parler.unwrap())
        .default_model(Model::Parler)
        .build()?;
        
    // Use the pre-included function to say a message using the default_model.
    let _ = natural.say_auto("Hello, World!".to_string())?;
}

```

### Example of saying something using Gtts

```Rust
use std::error::Error;
use natural_tts::{*, models::gtts::GttsModel};

fn main() -> Result<(), Box<dyn Error>>{
    // Create the NaturalTts struct using the builder pattern.
    let mut natural = NaturalTtsBuilder::default()
        .gtts_model(GttsModel::default())
        .default_model(Model::Gtts)
        .build()?;

    // Use the pre-included function to say a message using the default_model.
    let _ = natural.say_auto("Hello, World!".to_string())?;
    Ok(())
}

```

### Example of saying something using MSEdge

```Rust
use std::error::Error;
use natural_tts::{*, models::msedge::MSEdgeModel};

fn main() -> Result<(), Box<dyn Error>>{

    // Create the NaturalTts struct using the builder pattern.
    let mut natural = NaturalTtsBuilder::default()
        .msedge_model(MSEdgeModel::default())
        .default_model(Model::MSEdge)
        .build()?;

    // Use the pre-included function to say a message using the default_model.
    let _ = natural.say_auto("Hello, World!".to_string())?;
    Ok(())
}

```

### Example of saying something using TTS

```Rust
use std::error::Error;
use natural_tts::{*, models::parler::TtsModel};

fn main() -> Result<(), Box<dyn Error>>{

    // Create the NaturalTts struct using the builder pattern.
    let mut natural = NaturalTtsBuilder::default()
        .tts_model(TtsModel::default())
        .default_model(Model::TTS)
        .build()?;

    // Use the pre-included function to say a message using the default_model.
    let _ = natural.say_auto("Hello, World!".to_string())?;
    Ok(())
}

```

### Example of saying something using Coqui Tts
#### Disclaimer : Currently not supported.

```Rust
use std::error::Error;
use natural_tts::{*, models::parler::CoquiModel};

fn main() -> Result<(), Box<dyn Error>>{

    // Create the NaturalTts struct using the builder pattern.
    let mut natural = NaturalTtsBuilder::default()
        .coqui_model(CoquiModel::default())
        .default_model(Model::Coqui)
        .build().unwrap();

    // Use the pre-included function to say a message using the default_model.
    let _ = natural.say_auto("Hello, World!".to_string())?;
    Ok(())
}

```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)
