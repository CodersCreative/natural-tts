# Natural TTS [![Rust](https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white)]()

![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black) ![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)  ![macOS](https://img.shields.io/badge/mac%20os-000000?style=for-the-badge&logo=macos&logoColor=F0F0F0)

#### Natural TTS (natural-tts) is a rust crate for easily implementing Text-To-Speech into your rust programs.

### To Do:
* [ ] Add support for [Piper TTS](https://github.com/rhasspy/piper).
* [ ] Remove all pyo3 usage.

### Available TTS Engines / AIs:
[Parler TTS](https://github.com/huggingface/parler-tts)\
[Google Gtts](https://github.com/pndurette/gTTS)\
[TTS-RS](https://github.com/ndarilek/tts-rs)\
[MSEdge TTS](https://github.com/hs-CN/msedge-tts)\
[MetaVoice TTS](https://github.com/metavoiceio/metavoice-src)\
[Coqui TTS](https://github.com/coqui-ai/TTS)

### Example of saying something using Gtts but initializing every model.

```Rust
use std::error::Error;
use crate::{*, models::{gtts::GttsModel, tts_rs::TtsModel, parler::ParlerModel, msedge::MSEdgeModel, meta::MetaModel}};

fn main() -> Result<(), Box<dyn Error>>{
    // Create the NaturalTts using the Builder pattern
    let mut natural = NaturalTtsBuilder::default()
        .default_model(Model::Gtts)
        .gtts_model(GttsModel::default())
        .parler_model(ParlerModel::default())
        .msedge_model(MSEdgeModel::default())
        .meta_model(MetaModel::default())
        .tts_model(TtsModel::default())
        .build()?;

    // Start producing an output using the default_model.
    let _ = natural.start("Hello, World!".to_string(), &PathBuf::from("output.wav"));    

    // Get the rodio sink from natural tts
    let sink = natural.get_rodio_sink()?.0;

    // Play the audio until it finishes
    sink.sleep_until_end();

    Ok(())
}

```

### Example of saying something using Meta Voice.

```Rust
use std::error::Error;
use natural_tts::{*, models::meta::MetaModel};

fn main() -> Result<(), Box<dyn Error>>{
    // Create the NaturalTts struct using the builder pattern.
    let mut natural = NaturalTtsBuilder::default()
        .meta_model(MetaModel::default())
        .default_model(Model::Meta)
        .build()?;

    // Start producing an output using the default_model.
    let _ = natural.start("Hello, World!".to_string(), &PathBuf::from("output.wav"));

    // Play the audio until it finishes
    natural.sleep_until_end();
    Ok(())
}

```

### Example of saying something using Parler.

```Rust
use std::error::Error;
use natural_tts::{*, models::parler::ParlerModel};

fn main() -> Result<(), Box<dyn Error>>{
    // Create the NaturalTts using the Builder pattern
    let mut natural = NaturalTtsBuilder::default()
        .parler_model(ParlerModel::default())
        .default_model(Model::Parler)
        .build()?;

    // Start producing an output using the default_model.
    let _ = natural.start("Hello, World!".to_string(), &PathBuf::from("output.wav"));

    // Play the audio until it finishes
    natural.sleep_until_end();
    Ok(())    
}

```

### Example of saying something using Gtts.

```Rust
use std::error::Error;
use natural_tts::{*, models::gtts::GttsModel};

fn main() -> Result<(), Box<dyn Error>>{
    // Create the NaturalTts struct using the builder pattern.
    let mut natural = NaturalTtsBuilder::default()
        .gtts_model(GttsModel::default())
        .default_model(Model::Gtts)
        .build()?;

    // Start producing an output using the default_model.
    let _ = natural.start("Hello, World!".to_string(), &PathBuf::from("output.wav"));

    // Play the audio until it finishes
    natural.sleep_until_end();
    Ok(())
}

```

### Example of saying something using MSEdge.

```Rust
use std::error::Error;
use natural_tts::{*, models::msedge::MSEdgeModel};

fn main() -> Result<(), Box<dyn Error>>{
    // Create the NaturalTts struct using the builder pattern.
    let mut natural = NaturalTtsBuilder::default()
        .msedge_model(MSEdgeModel::default())
        .default_model(Model::MSEdge)
        .build()?;

    // Start producing an output using the default_model.
    let _ = natural.start("Hello, World!".to_string(), &PathBuf::from("output.wav"));

    // Play the audio until it finishes
    natural.sleep_until_end();
    Ok(())    
}

```

### Example of saying something using TTS.

```Rust
use std::error::Error;
use natural_tts::{*, models::parler::TtsModel};

fn main() -> Result<(), Box<dyn Error>>{

    // Create the NaturalTts struct using the builder pattern.
    let mut natural = NaturalTtsBuilder::default()
        .tts_model(TtsModel::default())
        .default_model(Model::TTS)
        .build()?;


    // Start producing an output using the default_model.
    let _ = natural.start("Hello, World!".to_string(), &PathBuf::from("output.wav"));

    // Play the audio until it finishes
    natural.sleep_until_end();
    Ok(())    
}

```

### Example of saying something using Coqui Tts.
#### Disclaimer : Currently only in test feature.

```Rust
use std::error::Error;
use natural_tts::{*, models::parler::CoquiModel};

fn main() -> Result<(), Box<dyn Error>>{

    // Create the NaturalTts struct using the builder pattern.
    let mut natural = NaturalTtsBuilder::default()
        .coqui_model(CoquiModel::default())
        .default_model(Model::Coqui)
        .build().unwrap();

    // Start producing an output using the default_model.
    let _ = natural.start("Hello, World!".to_string(), &PathBuf::from("output.wav"));

    // Play the audio until it finishes
    natural.sleep_until_end();
    Ok(())
}

```

## Contributing.

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.
