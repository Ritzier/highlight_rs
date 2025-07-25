use crate::{PREFIX, html_escape};

#[derive(Debug, PartialEq)]
pub struct CssToken {
    pub content: String,
    pub kind: CssTokenKind,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CssTokenKind {
    AtRules,
    Comment,
    Default,
    Function,
    Identifier,
    Keyword,
    Literal,
    Macro,
    Number,
    Operator,
    Property,
    Punctuation,
    Selector,
    String,
    Unit,
    Whitespace,
}

impl CssToken {
    pub fn new(content: String, kind: CssTokenKind) -> Self {
        Self { content, kind }
    }
    pub fn to_html(&self) -> String {
        let content = html_escape(&self.content);

        let class = match self.kind {
            CssTokenKind::AtRules => "keyword",
            CssTokenKind::Comment => "comment",
            CssTokenKind::Function => "function",
            CssTokenKind::Identifier => "identifier",
            CssTokenKind::Keyword => "keyword",
            CssTokenKind::Literal => "literal",
            CssTokenKind::Macro => "macro",
            CssTokenKind::Number => "number",
            CssTokenKind::Operator => "operator",
            CssTokenKind::Property => "property",
            CssTokenKind::Punctuation => "punctuation",
            CssTokenKind::Selector => "selector",
            CssTokenKind::String => "string",
            CssTokenKind::Unit => "unit",
            CssTokenKind::Whitespace | CssTokenKind::Default => return content,
        };
        format!("<span class=\"{PREFIX}{class}\">{content}</span>")
    }
}
