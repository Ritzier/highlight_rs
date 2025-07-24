const PREFIX: &str = "hl-";
type Result<T> = std::result::Result<T, Error>;

mod errors;
mod language;
mod token;
mod utils;

pub use errors::Error;
pub use language::{Language, RustTokenizer};
pub use token::{Token, TokenType};
pub use utils::{higlight, html_escape};
