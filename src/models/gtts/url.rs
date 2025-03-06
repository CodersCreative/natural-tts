use percent_encoding::utf8_percent_encode;
use percent_encoding::AsciiSet;
use percent_encoding::CONTROLS;


const FRAGMENT: &AsciiSet =
  &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub struct EncodedFragment {
  pub encoded: String,
  pub decoded: String,
}

impl EncodedFragment {
  pub fn fragmenter(text: &str) -> Result<Self, String> {
    let raw_text = text;
    let text = utf8_percent_encode(raw_text, FRAGMENT).to_string();
    if text.is_empty() {
      return Err("Empty text".to_string());
    }
    Ok(Self {
      encoded: text,
      decoded: raw_text.to_string(),
    })
  }
}
