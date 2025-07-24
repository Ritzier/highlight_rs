use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Regex: {0:?}")]
    Regex(#[from] regex::Error),
    #[error("Unsupported langugae: {0}")]
    Unsupported(String),
}
