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
    CustomProperty,
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
    SelectorClass,
    SelectorId,
    SelectorPseudo,
    SelectorTag,
    SelectorUniversal,
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
            CssTokenKind::CustomProperty => "property",
            CssTokenKind::Function => "function",
            CssTokenKind::Identifier => "identifier",
            CssTokenKind::Keyword => "keyword",
            CssTokenKind::Literal => "literal",
            CssTokenKind::Macro => "macro",
            CssTokenKind::Number => "number",
            CssTokenKind::Operator => "operator",
            CssTokenKind::Property => "property",
            CssTokenKind::Punctuation => "punctuation",
            CssTokenKind::SelectorClass => "selector-class",
            CssTokenKind::SelectorId => "selector-id",
            CssTokenKind::SelectorPseudo => "selector-pseudo",
            CssTokenKind::SelectorTag => "selector-tag",
            CssTokenKind::SelectorUniversal => "selector-universal",
            CssTokenKind::String => "string",
            CssTokenKind::Unit => "unit",
            CssTokenKind::Whitespace | CssTokenKind::Default => return content,
        };
        format!("<span class=\"{PREFIX}{class}\">{content}</span>")
    }
}
