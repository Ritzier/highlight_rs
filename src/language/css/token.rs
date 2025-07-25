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
    String,
    Unit,
    Whitespace,
    SelectorId,
    SelectorTag,
    SelectorClass,
    SelectorUniversal,
    SelectorPseudo,
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
            CssTokenKind::String => "string",
            CssTokenKind::Unit => "unit",
            CssTokenKind::Whitespace | CssTokenKind::Default => return content,
            CssTokenKind::SelectorId => "selector-id",
            CssTokenKind::SelectorTag => "selector-tag",
            CssTokenKind::SelectorClass => "selector-class",
            CssTokenKind::SelectorPseudo => "selector-pseudo",
            CssTokenKind::SelectorUniversal => "selector-universal",
        };
        format!("<span class=\"{PREFIX}{class}\">{content}</span>")
    }
}
