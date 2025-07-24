macro_rules! rg {
    ($re:expr) => {
        LazyLock::new(|| Regex::new($re).unwrap())
    };
}

mod rust;
pub use rust::*;
mod css;
pub use css::*;

use super::{Error, Result, Token, TokenType};

#[derive(Debug, PartialEq)]
pub enum Language {
    Rust,
    Css,
}

impl Language {
    pub fn from(lang: &str) -> Result<Self> {
        match lang.to_lowercase().as_str() {
            "rust" | "rs" => Ok(Self::Rust),
            "css" => Ok(Self::Css),
            _ => Err(Error::Unsupported(lang.to_string())),
        }
    }

    pub fn tokenize(&self, input: &str) -> Vec<Token> {
        match self {
            Self::Rust => RustTokenizer::tokenize(input),
            Self::Css => CssTokenizer::tokenize(input),
        }
    }

    pub fn highlight(&self, input: &str) -> String {
        let tokens = self.tokenize(input);
        let mut result = String::new();

        for token in tokens {
            result.push_str(&token.to_html());
        }

        result
    }
}
