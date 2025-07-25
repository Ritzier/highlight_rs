use crate::{PREFIX, html_escape};

#[derive(Debug, PartialEq)]
pub struct RustToken {
    pub content: String,
    pub kind: RustTokenKind,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RustTokenKind {
    Attribute,
    Char,
    Comment,
    Default,
    Function,
    Identifier,
    Keyword,
    Lifetime,
    Macro,
    Manual, // For manually handle < > char
    Number,
    Operator,
    Punctuation,
    String,
    Tag,
    Type,
    Whitespace,
}

impl RustToken {
    pub fn new(content: String, kind: RustTokenKind) -> Self {
        Self { content, kind }
    }
    pub fn to_html(&self) -> String {
        let content = html_escape(&self.content);

        let class = match self.kind {
            RustTokenKind::Attribute => "attribute",
            RustTokenKind::Char => "char",
            RustTokenKind::Comment => "comment",
            RustTokenKind::Function => "function",
            RustTokenKind::Identifier => "identifier",
            RustTokenKind::Keyword => "keyword",
            RustTokenKind::Lifetime => "lifetime",
            RustTokenKind::Macro => "macro",
            RustTokenKind::Number => "number",
            RustTokenKind::Operator => "operator",
            RustTokenKind::Punctuation => "punctuation",
            RustTokenKind::String => "string",
            RustTokenKind::Tag => "tag",
            RustTokenKind::Type => "type",
            RustTokenKind::Default | RustTokenKind::Manual | RustTokenKind::Whitespace => {
                return content;
            }
        };
        format!("<span class=\"{PREFIX}{class}\">{content}</span>")
    }
}
