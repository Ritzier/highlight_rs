use regex::Regex;
use std::sync::LazyLock;

use super::*;

// --- Comments ---
static CSS_COMMENT: LazyLock<Regex> = rg!(r"/\*[\s\S]*?\*/");

// --- Whitespace ---
static CSS_WHITESPACE: LazyLock<Regex> = rg!(r"\s+");

// --- At-rules ---
static CSS_AT_RULES: LazyLock<Regex> = rg!(r"@([a-zA-Z_-]+)");

// --- Keywords and common special values ---
static CSS_KEYWORDS: LazyLock<Regex> =
    rg!(r"\b(important|inherit|initial|unset|revert|auto|none|normal|transparent|currentColor)\b");

// --- Custom Properties / CSS Variables (match --var-name:) ---
static CSS_CUSTOM_PROPERTY: LazyLock<Regex> = rg!(r"--[a-zA-Z0-9\-_]+");

// --- Functions ---
static CSS_FUNCTION: LazyLock<Regex> = rg!(r"\b[a-zA-Z_][a-zA-Z0-9_-]*\s*\("); // covers url(, rgb(, calc(, etc

// --- Strings ---
static CSS_STRING_LITERAL: LazyLock<Regex> = rg!(r#""([^"\\]|\\.)*"|'([^'\\]|\\.)*'"#);

// --- Numbers with units and percentage ---
static CSS_NUMBER_WITH_UNIT: LazyLock<Regex> = rg!(
    r"\b\d*\.?\d+(%|px|em|rem|vh|vw|vmin|vmax|ex|ch|cm|mm|in|pt|pc|deg|rad|grad|turn|s|ms|Hz|kHz|dpi|dpcm|dppx|fr)?\b"
);

// --- Numbers ---
static CSS_NUMBER: LazyLock<Regex> = rg!(r"\b\d*\.?\d+\b");

// --- Hex colors ---
static CSS_HEX_COLOR: LazyLock<Regex> = rg!(r"#[0-9a-fA-F]{3,8}\b");

// --- Selectors: id, class, pseudo(class/element), element, universal ---
static CSS_SELECTOR_ID: LazyLock<Regex> = rg!(r"#[a-zA-Z_][\w\-]*");
static CSS_SELECTOR_CLASS: LazyLock<Regex> = rg!(r"\.[a-zA-Z_][\w\-]*");
static CSS_SELECTOR_PSEUDO: LazyLock<Regex> = rg!(r"::?[a-zA-Z_][\w\-]*(?:\([^)]+\))?"); // handles pseudo-classes and elements
static CSS_SELECTOR_ELEM: LazyLock<Regex> = rg!(r"\b[a-zA-Z_][\w\-]*\b");
static CSS_SELECTOR_UNIVERSAL: LazyLock<Regex> = rg!(r"\*");

// --- Properties ---
static CSS_PROPERTY: LazyLock<Regex> = rg!(
    r"\b([a-zA-Z][a-zA-Z0-9-]*|[*]|[.][a-zA-Z][a-zA-Z0-9-]*|[#][a-zA-Z][a-zA-Z0-9-]*|::[a-zA-Z][a-zA-Z0-9-]*|:[a-zA-Z][a-zA-Z0-9-]*(?:\([^)]*\))?)\b"
);

// --- Operators and Punctuation ---
static CSS_OPERATOR: LazyLock<Regex> = rg!(r"[+\-*/^~|]");
static CSS_PUNCTUATION: LazyLock<Regex> = rg!(r"[{}();,:]");

// --- Identifiers  ---
static CSS_IDENTIFIER: LazyLock<Regex> = rg!(r"[a-zA-Z_][\w\-]*");

// --- Patterns ---
static PATTERNS: LazyLock<Vec<(Regex, TokenType)>> = LazyLock::new(|| {
    vec![
        // Comments and whitespace
        (CSS_COMMENT.clone(), TokenType::Comment),
        (CSS_WHITESPACE.clone(), TokenType::Whitespace),
        // At-rules
        (CSS_AT_RULES.clone(), TokenType::Macro),
        // String literals, colors, functions
        (CSS_STRING_LITERAL.clone(), TokenType::String),
        (CSS_HEX_COLOR.clone(), TokenType::Literal),
        (CSS_FUNCTION.clone(), TokenType::Function),
        // !important, keywords, custom property
        // (Regex::new(r"!important\b").unwrap(), TokenType::Keyword),
        (CSS_KEYWORDS.clone(), TokenType::Keyword),
        (CSS_CUSTOM_PROPERTY.clone(), TokenType::Property),
        // Properties (should come before generic identifiers!)
        (CSS_PROPERTY.clone(), TokenType::Property),
        // Selectors (ordered from most to least specific to avoid overlaps)
        (CSS_SELECTOR_ID.clone(), TokenType::Selector),
        (CSS_SELECTOR_CLASS.clone(), TokenType::Selector),
        (CSS_SELECTOR_PSEUDO.clone(), TokenType::Selector),
        (CSS_SELECTOR_UNIVERSAL.clone(), TokenType::Selector),
        (CSS_SELECTOR_ELEM.clone(), TokenType::Selector),
        // Numbers/Units
        (CSS_NUMBER_WITH_UNIT.clone(), TokenType::Unit),
        (CSS_NUMBER.clone(), TokenType::Number),
        // Operators and punctuation
        (CSS_OPERATOR.clone(), TokenType::Operator),
        (CSS_PUNCTUATION.clone(), TokenType::Punctuation),
        // Fallback identifier
        (CSS_IDENTIFIER.clone(), TokenType::Identifier),
    ]
});

pub struct CssTokenizer {
    pos: usize,
    chars: Vec<char>,
}

impl CssTokenizer {
    fn new(code: &str) -> Self {
        Self {
            pos: 0,
            chars: code.chars().collect(),
        }
    }

    pub fn tokenize(code: &str) -> Vec<Token> {
        CssTokenizer::new(code).tokenize_all()
    }

    fn tokenize_all(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let len = self.chars.len();

        while self.pos < len {
            let code = &self.chars[self.pos..].iter().collect::<String>();
            let mut found = false;

            for (regex, typ) in PATTERNS.iter() {
                if let Some(mat) = regex.find(code) {
                    if mat.start() == 0 {
                        let content = mat.as_str().to_string();

                        match typ {
                            TokenType::Function => match content.find('(') {
                                Some(parent_start) => {
                                    let fname = content[..parent_start].trim_end();
                                    tokens.push(Token::new(fname.to_string(), TokenType::Function));
                                    tokens
                                        .push(Token::new("(".to_string(), TokenType::Punctuation));
                                }
                                None => {
                                    tokens.push(Token::new(content.clone(), *typ));
                                }
                            },

                            _ => {
                                tokens.push(Token::new(content.clone(), *typ));
                            }
                        }

                        self.pos += content.chars().count();
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                // Unrecognized: treat as single default token
                let content = self.chars[self.pos].to_string();
                tokens.push(Token::new(content, TokenType::Default));
                self.pos += 1;
            }
        }
        tokens
    }
}
