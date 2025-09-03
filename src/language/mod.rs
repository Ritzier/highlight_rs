use super::{Error, Result};

macro_rules! rg {
    ($re:expr) => {
        LazyLock::new(|| Regex::new($re).unwrap())
    };
}

mod language;
pub use language::Language;

mod rust;
pub use rust::*;
mod css;
pub use css::*;

pub enum HighlightToken {
    Rust(RustToken),
    Css(CssToken),
}

impl HighlightToken {
    pub fn to_html(&self) -> String {
        match self {
            HighlightToken::Rust(rt) => rt.to_html(),
            HighlightToken::Css(ct) => ct.to_html(),
        }
    }
}

impl From<RustToken> for HighlightToken {
    fn from(token: rust::RustToken) -> Self {
        HighlightToken::Rust(token)
    }
}

impl From<CssToken> for HighlightToken {
    fn from(token: CssToken) -> Self {
        HighlightToken::Css(token)
    }
}

impl Language {
    pub fn from(lang: &str) -> Result<Self> {
        match lang.to_lowercase().as_str() {
            "rust" | "rs" => Ok(Self::Rust),
            "css" => Ok(Self::Css),
            _ => Err(Error::Unsupported(lang.to_string())),
        }
    }

    pub fn tokenize(&self, input: &str) -> Vec<HighlightToken> {
        match self {
            Self::Rust => RustTokenizer::tokenize(input)
                .into_iter()
                .map(HighlightToken::from)
                .collect(),
            Self::Css => CssTokenizer::tokenize(input)
                .into_iter()
                .map(HighlightToken::from)
                .collect(),
        }
    }

    pub fn highlight(&self, input: &str) -> String {
        let tokens = self.tokenize(input);
        tokens.into_iter().map(|t| t.to_html()).collect()
    }
}
