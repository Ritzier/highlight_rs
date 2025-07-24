use crate::{PREFIX, html_escape};

#[derive(Debug, PartialEq)]
pub struct RustToken {
    pub content: String,
    pub kind: RustTokenKind,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RustTokenKind {
    Comment,
    Char,
    Lifetime,
    String,
    Number,
    Attribute,
    Macro,
    Keyword,
    Type,
    Function,
    Operator,
    Manual, // For manually handle < > char
    Punctuation,
    Identifier,
    Whitespace,
    Tag,
    Default,
}

impl RustToken {
    pub fn new(content: String, kind: RustTokenKind) -> Self {
        Self { content, kind }
    }
    pub fn to_html(&self) -> String {
        let content = html_escape(&self.content);

        let class = match self.kind {
            RustTokenKind::Keyword => "keyword",
            RustTokenKind::Identifier => "identifier",
            RustTokenKind::Number => "number",
            RustTokenKind::String => "string",
            RustTokenKind::Char => "char",
            RustTokenKind::Type => "type",
            RustTokenKind::Macro => "macro",
            RustTokenKind::Comment => "comment",
            RustTokenKind::Lifetime => "lifetime",
            RustTokenKind::Function => "function",
            RustTokenKind::Operator => "operator",
            RustTokenKind::Attribute => "attribute",
            RustTokenKind::Punctuation => "punctuation",
            RustTokenKind::Tag => "tag",
            RustTokenKind::Default | RustTokenKind::Manual | RustTokenKind::Whitespace => {
                return content;
            }
        };
        format!("<span class=\"{PREFIX}{class}\">{content}</span>")
    }
}
