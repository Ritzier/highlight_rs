mod rust;
pub use rust::*;

use super::{Error, Result, Token, TokenType};

#[derive(Debug, PartialEq)]
pub enum Language {
    Rust,
}

impl Language {
    pub fn from(lang: &str) -> Result<Self> {
        match lang.to_lowercase().as_str() {
            "rust" | "rs" => Ok(Self::Rust),
            _ => Err(Error::Unsupported(lang.to_string())),
        }
    }

    pub fn tokenize(&self, input: &str) -> Vec<Token> {
        match self {
            Self::Rust => RustTokenizer::tokenize(input),
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
