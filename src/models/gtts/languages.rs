use std::str::FromStr;

/// Enum containing all the languages supported by the GTTS API
#[derive(Debug, Clone)]
pub enum Languages {
  /// ISO code: af
  Afrikaans,
  /// ISO code: ar       
  Arabic,
  /// ISO code: bg         
  Bulgarian,
  /// ISO code: bn
  Bengali,
  /// ISO code: bs
  Bosnian,
  /// ISO code: ca
  Catalan,
  /// ISO code: cs
  Czech,
  /// ISO code: cy
  Welsh,
  /// ISO code: da
  Danish,
  /// ISO code: de
  German,
  /// ISO code: el
  Greek,
  /// ISO code: en
  English,
  /// ISO code: eo
  Esperanto,
  /// ISO code: es
  Spanish,
  /// ISO code: et
  Estonian,
  /// ISO code: fi
  Finnish,
  /// ISO code: fr
  French,
  /// ISO code: gu
  Gujarati,
  /// ISO code: hi
  Hindi,
  /// ISO code: hr
  Croatian,
  /// ISO code: hu
  Hungarian,
  /// ISO code: hy
  Armenian,
  /// ISO code: id
  Indonesian,
  /// ISO code: is
  Icelandic,
  /// ISO code: it
  Italian,
  /// ISO code: ja
  Japanese,
  /// ISO code: jw
  Javanese,
  /// ISO code: km
  Khmer,
  /// ISO code: kn
  Kannada,
  /// ISO code: ko
  Korean,
  /// ISO code: la
  Latin,
  /// ISO code: lv
  Latvian,
  /// ISO code: mk
  Macedonian,
  /// ISO code: ml
  Malayalam,
  /// ISO code: mr
  Marathi,
  /// Myanmar (Burmese)
  ///
  /// ISO code: my
  MyanmarAKABurmese,
  /// ISO code: ne
  Nepali,
  /// ISO code: nl
  Dutch,
  /// ISO code: no
  Norwegian,
  /// ISO code: pl
  Polish,
  /// ISO code: pt
  Portuguese,
  /// ISO code: ro
  Romanian,
  /// ISO code: ru
  Russian,
  /// ISO code: si
  Sinhala,
  /// ISO code: sk
  Slovak,
  /// ISO code: sq
  Albanian,
  /// ISO code: sr
  Serbian,
  /// ISO code: su
  Sundanese,
  /// ISO code: sv
  Swedish,
  /// ISO code: sw
  Swahili,
  /// ISO code: ta
  Tamil,
  /// ISO code: te
  Telugu,
  /// ISO code: th
  Thai,
  /// ISO code: tl
  Filipino,
  /// ISO code: tr
  Turkish,
  /// ISO code: uk
  Ukrainian,
  /// ISO code: ur
  Urdu,
  /// ISO code: vi
  Vietnamese,
  /// ISO code: zh-CN
  Chinese,
}
impl FromStr for Languages {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "af" => Ok(Languages::Afrikaans),
      "ar" => Ok(Languages::Arabic),
      "bg" => Ok(Languages::Bulgarian),
      "bn" => Ok(Languages::Bengali),
      "bs" => Ok(Languages::Bosnian),
      "ca" => Ok(Languages::Catalan),
      "cs" => Ok(Languages::Czech),
      "cy" => Ok(Languages::Welsh),
      "da" => Ok(Languages::Danish),
      "de" => Ok(Languages::German),
      "el" => Ok(Languages::Greek),
      "en" => Ok(Languages::English),
      "eo" => Ok(Languages::Esperanto),
      "es" => Ok(Languages::Spanish),
      "et" => Ok(Languages::Estonian),
      "fi" => Ok(Languages::Finnish),
      "fr" => Ok(Languages::French),
      "gu" => Ok(Languages::Gujarati),
      "hi" => Ok(Languages::Hindi),
      "hr" => Ok(Languages::Croatian),
      "hu" => Ok(Languages::Hungarian),
      "hy" => Ok(Languages::Armenian),
      "id" => Ok(Languages::Indonesian),
      "is" => Ok(Languages::Icelandic),
      "it" => Ok(Languages::Italian),
      "ja" => Ok(Languages::Japanese),
      "jw" => Ok(Languages::Javanese),
      "km" => Ok(Languages::Khmer),
      "kn" => Ok(Languages::Kannada),
      "ko" => Ok(Languages::Korean),
      "la" => Ok(Languages::Latin),
      "lv" => Ok(Languages::Latvian),
      "mk" => Ok(Languages::Macedonian),
      "ml" => Ok(Languages::Malayalam),
      "mr" => Ok(Languages::Marathi),
      "my" => Ok(Languages::MyanmarAKABurmese),
      "ne" => Ok(Languages::Nepali),
      "nl" => Ok(Languages::Dutch),
      "no" => Ok(Languages::Norwegian),
      "pl" => Ok(Languages::Polish),
      "pt" => Ok(Languages::Portuguese),
      "ro" => Ok(Languages::Romanian),
      "ru" => Ok(Languages::Russian),
      "si" => Ok(Languages::Sinhala),
      "sk" => Ok(Languages::Slovak),
      "sq" => Ok(Languages::Albanian),
      "sr" => Ok(Languages::Serbian),
      "su" => Ok(Languages::Sundanese),
      "sv" => Ok(Languages::Swedish),
      "sw" => Ok(Languages::Swahili),
      "ta" => Ok(Languages::Tamil),
      "te" => Ok(Languages::Telugu),
      "th" => Ok(Languages::Thai),
      "tl" => Ok(Languages::Filipino),
      "tr" => Ok(Languages::Turkish),
      "uk" => Ok(Languages::Ukrainian),
      "ur" => Ok(Languages::Urdu),
      "vi" => Ok(Languages::Vietnamese),
      "zh-CN" => Ok(Languages::Chinese),
      _ => Err(format!(
        "Unknown language: {}. Make sure to use all the supported languages",
        s
      )),
    }
  }
}

impl Languages {
  pub fn as_code(&self) -> &'static str {
    match self {
      Languages::Afrikaans => "af",
      Languages::Albanian => "sq",
      Languages::Arabic => "ar",
      Languages::Armenian => "hy",
      Languages::Bengali => "bn",
      Languages::Bosnian => "bs",
      Languages::Bulgarian => "bg",
      Languages::Catalan => "ca",
      Languages::Chinese => "zh-CN",
      Languages::Croatian => "hr",
      Languages::Czech => "cs",
      Languages::Danish => "da",
      Languages::Dutch => "nl",
      Languages::English => "en",
      Languages::Esperanto => "eo",
      Languages::Estonian => "et",
      Languages::Filipino => "tl",
      Languages::Finnish => "fi",
      Languages::French => "fr",
      Languages::German => "de",
      Languages::Greek => "el",
      Languages::Gujarati => "gu",
      Languages::Hindi => "hi",
      Languages::Hungarian => "hu",
      Languages::Icelandic => "is",
      Languages::Indonesian => "id",
      Languages::Italian => "it",
      Languages::Japanese => "ja",
      Languages::Javanese => "jw",
      Languages::Kannada => "kn",
      Languages::Khmer => "km",
      Languages::Korean => "ko",
      Languages::Latin => "la",
      Languages::Latvian => "lv",
      Languages::Macedonian => "mk",
      Languages::Marathi => "mr",
      Languages::Nepali => "ne",
      Languages::Norwegian => "no",
      Languages::Polish => "pl",
      Languages::Portuguese => "pt",
      Languages::Romanian => "ro",
      Languages::Russian => "ru",
      Languages::Serbian => "sr",
      Languages::Sinhala => "si",
      Languages::Slovak => "sk",
      Languages::Spanish => "es",
      Languages::Swahili => "sw",
      Languages::Swedish => "sv",
      Languages::Tamil => "ta",
      Languages::Telugu => "te",
      Languages::Thai => "th",
      Languages::Turkish => "tr",
      Languages::Ukrainian => "uk",
      Languages::Urdu => "ur",
      Languages::Vietnamese => "vi",
      Languages::Welsh => "cy",
      Languages::MyanmarAKABurmese => "my",
      Languages::Malayalam => "ml",
      Languages::Sundanese => "su",
    }
  }
}
