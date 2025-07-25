use regex::Regex;
use std::sync::LazyLock;

use super::CssTokenKind;

// --- Comments ---
pub static CSS_COMMENT: LazyLock<Regex> = rg!(r"/\*[\s\S]*?\*/");

// --- Whitespace ---
pub static CSS_WHITESPACE: LazyLock<Regex> = rg!(r"\s+");

// --- At-rules ---
pub static CSS_AT_RULES: LazyLock<Regex> = rg!(r"@([a-zA-Z_-]+)");

// --- !important ---
pub static CSS_IMPORTANT: LazyLock<Regex> = rg!(r"!\s*important\b");

// --- Keywords and common special values ---
pub static CSS_KEYWORDS: LazyLock<Regex> =
    rg!(r"\b(important|inherit|initial|unset|revert|auto|none|normal|transparent|currentColor)\b");

// --- Custom Properties / CSS Variables (match --var-name:) ---
pub static CSS_CUSTOM_PROPERTY: LazyLock<Regex> = rg!(r"--[a-zA-Z0-9\-_]+");

// --- Functions ---
pub static CSS_FUNCTION: LazyLock<Regex> = rg!(r"\b[a-zA-Z_][a-zA-Z0-9_-]*\s*\("); // covers url(, rgb(, calc(, etc

// --- Strings ---
pub static CSS_STRING_LITERAL: LazyLock<Regex> = rg!(r#""([^"\\]|\\.)*"|'([^'\\]|\\.)*'"#);

// --- Numbers with units and percentage ---
pub static CSS_NUMBER_WITH_UNIT: LazyLock<Regex> = rg!(
    r"\b\d*\.?\d+(%|px|em|rem|vh|vw|vmin|vmax|ex|ch|cm|mm|in|pt|pc|deg|rad|grad|turn|s|ms|Hz|kHz|dpi|dpcm|dppx|fr)?\b"
);

// --- Numbers ---
pub static CSS_NUMBER: LazyLock<Regex> = rg!(r"\b\d*\.?\d+\b");

// --- Hex colors ---
pub static CSS_HEX_COLOR: LazyLock<Regex> = rg!(r"#[0-9a-fA-F]{3,8}\b");

// --- Selectors: id, class, pseudo(class/element), element, universal ---
pub static CSS_SELECTOR_ID: LazyLock<Regex> = rg!(r"#[a-zA-Z_][\w\-]*");
pub static CSS_SELECTOR_CLASS: LazyLock<Regex> = rg!(r"\.[a-zA-Z_][\w\-]*");
pub static CSS_SELECTOR_PSEUDO: LazyLock<Regex> = rg!(r"::?[a-zA-Z_][\w\-]*(?:\([^)]+\))?"); // handles pseudo-classes and elements
pub static CSS_SELECTOR_ELEM: LazyLock<Regex> = rg!(r"\b[a-zA-Z_][\w\-]*\b");
pub static CSS_SELECTOR_UNIVERSAL: LazyLock<Regex> = rg!(r"\*");

// --- Properties ---
pub static CSS_PROPERTY: LazyLock<Regex> = rg!(
    r"\b([a-zA-Z][a-zA-Z0-9-]*|[*]|[.][a-zA-Z][a-zA-Z0-9-]*|[#][a-zA-Z][a-zA-Z0-9-]*|::[a-zA-Z][a-zA-Z0-9-]*|:[a-zA-Z][a-zA-Z0-9-]*(?:\([^)]*\))?)\b"
);

// --- Operators and Punctuation ---
pub static CSS_OPERATOR: LazyLock<Regex> = rg!(r"[+\-*/^~|]");
pub static CSS_PUNCTUATION: LazyLock<Regex> = rg!(r"[{}();,:]");

// --- Identifiers  ---
pub static CSS_IDENTIFIER: LazyLock<Regex> = rg!(r"[a-zA-Z_][\w\-]*");

// --- Patterns ---
pub static PATTERNS: LazyLock<Vec<(Regex, CssTokenKind)>> = LazyLock::new(|| {
    vec![
        // Comments and whitespace
        (CSS_COMMENT.clone(), CssTokenKind::Comment),
        (CSS_WHITESPACE.clone(), CssTokenKind::Whitespace),
        // At-rules
        (CSS_AT_RULES.clone(), CssTokenKind::AtRules),
        // String literals, colors, functions
        (CSS_STRING_LITERAL.clone(), CssTokenKind::String),
        (CSS_HEX_COLOR.clone(), CssTokenKind::Literal),
        (CSS_FUNCTION.clone(), CssTokenKind::Function),
        // !important, keywords, custom property
        (CSS_IMPORTANT.clone(), CssTokenKind::Keyword),
        (CSS_KEYWORDS.clone(), CssTokenKind::Keyword),
        (CSS_CUSTOM_PROPERTY.clone(), CssTokenKind::Property),
        // Properties (should come before generic identifiers!)
        (CSS_PROPERTY.clone(), CssTokenKind::Property),
        // Selectors (ordered from most to least specific to avoid overlaps)
        (CSS_SELECTOR_ID.clone(), CssTokenKind::Selector),
        (CSS_SELECTOR_CLASS.clone(), CssTokenKind::Selector),
        (CSS_SELECTOR_PSEUDO.clone(), CssTokenKind::Selector),
        (CSS_SELECTOR_UNIVERSAL.clone(), CssTokenKind::Selector),
        (CSS_SELECTOR_ELEM.clone(), CssTokenKind::Selector),
        // Numbers/Units
        (CSS_NUMBER_WITH_UNIT.clone(), CssTokenKind::Unit),
        (CSS_NUMBER.clone(), CssTokenKind::Number),
        // Operators and punctuation
        (CSS_OPERATOR.clone(), CssTokenKind::Operator),
        (CSS_PUNCTUATION.clone(), CssTokenKind::Punctuation),
        // Fallback identifier
        (CSS_IDENTIFIER.clone(), CssTokenKind::Identifier),
    ]
});
