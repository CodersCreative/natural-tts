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

### Example of saying something using Parler

```Rust
use std::error::Error;
use natural_tts::{*, models::parler::ParlerModel};

fn main() -> Result<(), Box<dyn Error>>{

    // Create the NaturalTts struct using the builder pattern.
    let mut natural = NaturalTtsBuilder::default()
        .parler_model(Some(ParlerModel::default()))
        .default_model(Model::Parler)
        .build()?;

    // Use the pre-included function to say a message using the default_model.
    let _ = natural.say_auto("Hello, World!".to_string());
    Ok(())
}

```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)
