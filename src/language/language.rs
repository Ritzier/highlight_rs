use std::fmt;

#[cfg_attr(feature = "strum", derive(strum::EnumIter))]
#[derive(Debug, PartialEq, Clone)]
pub enum Language {
    Rust,
    Css,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lang = match self {
            Self::Rust => "rust",
            Self::Css => "css",
        };

        write!(f, "{lang}")
    }
}
