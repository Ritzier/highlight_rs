use crate::{PREFIX, html_escape};

#[derive(Debug, PartialEq)]
pub struct Token {
    pub content: String,
    pub token_type: TokenType,
}

impl Token {
    pub fn new(content: String, token_type: TokenType) -> Self {
        Self {
            content,
            token_type,
        }
    }

    pub fn to_html(&self) -> String {
        let escaped_content = html_escape(&self.content);
        match &self.token_type {
            TokenType::Default | TokenType::Whitespace => escaped_content,
            token_type => format!(
                "<span class=\"{PREFIX}{}\">{}</span>",
                token_type.class_name(),
                escaped_content
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    Attribute,
    Char,
    Comment,
    Default,
    Enum,
    Function,
    Generic,
    Identifier,
    Keyword,
    Lifetime,
    Literal,
    Macro,
    Manual,
    Number,
    Operator,
    Property,
    Punctuation,
    Selector,
    String,
    Struct,
    Tag,
    Type,
    Unit,
    Whitespace,
}

impl TokenType {
    pub fn class_name(&self) -> &'static str {
        match self {
            Self::Attribute => "attribute",
            Self::Char => "char",
            Self::Comment => "comment",
            Self::Default => "default",
            Self::Enum => "enum",
            Self::Function => "function",
            Self::Generic => "generic",
            Self::Identifier => "identifier",
            Self::Keyword => "keyword",
            Self::Lifetime => "lifetime",
            Self::Literal => "literal",
            Self::Macro => "macro",
            Self::Manual => "error",
            Self::Number => "number",
            Self::Operator => "operator",
            Self::Property => "property",
            Self::Punctuation => "punctuation",
            Self::Selector => "selector",
            Self::String => "string",
            Self::Struct => "struct",
            Self::Tag => "tag",
            Self::Type => "type",
            Self::Unit => "unit",
            Self::Whitespace => "whitespace",
        }
    }
}
