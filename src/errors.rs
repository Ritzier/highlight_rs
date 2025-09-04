use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[cfg(feature = "full-regex")]
    #[error("Regex: {0:?}")]
    Regex(#[from] regex::Error),

    #[cfg(feature = "lite-regex")]
    #[error("Regex: {0:?}")]
    RegexLite(#[from] regex_lite::Error),

    #[error("Unsupported langugae: {0}")]
    Unsupported(String),
}
