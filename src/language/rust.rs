use regex::Regex;
use std::sync::LazyLock;

use super::*;

// --- Comments ---
static COMMENT_LINE: LazyLock<Regex> = rg!(r"//.*");
static COMMENT_BLOCK: LazyLock<Regex> = rg!(r"/\*[\s\S]*?\*/");

// --- Lifetimes ---
static LIFETIME: LazyLock<Regex> = rg!(r"'(_|[a-zA-Z][a-zA-Z0-9_]*)\b");
static LIFETIME_REF: LazyLock<Regex> = rg!(r"&'(_|[a-zA-Z][a-zA-Z0-9_]*)\b");

//  --- String ---
static STRING_DOUBLE: LazyLock<Regex> = rg!(r#""([^"\\]|\\.)*""#);

// --- Char ---
static CHAR_LITERAL: LazyLock<Regex> =
    rg!(r"'([^'\\]|\\[nrt0'\\]|\\x[0-9a-fA-F]{2}|\\u\{[0-9a-fA-F]{1,6}\})'");

// --- Number ---
static NUMBER_WITH_SUFFIX: LazyLock<Regex> =
    rg!(r"\b\d[\d_]*([eE][+-]?\d+)?(_?[a-zA-Z][a-zA-Z0-9]*)?\b");
static NUMBER_FLOAT: LazyLock<Regex> = rg!(
    r"\b((\d[\d_]*\.\d[\d_]*|\d[\d_]*\.|\.\d[\d_]*)([eE][+-]?\d[\d_]*)?(_?[a-zA-Z][a-zA-Z0-9]*)?)\b"
);

static NUMBER_HEX: LazyLock<Regex> = rg!(r"\b0[xX][0-9a-fA-F]+\b");
static NUMBER_OCTAL: LazyLock<Regex> = rg!(r"\b0[oO][0-7]+\b");
static NUMBER_BINARY: LazyLock<Regex> = rg!(r"\b0[bB][01]+\b");

// --- Attributes ---
static ATTRIBUTE: LazyLock<Regex> = rg!(r"#!?\[[\s\S]*?\]");

// --- Macro ---
static MACRO_CALL: LazyLock<Regex> = rg!(r"\b\w+!");

// --- Function calls ---
static FUNCTION_CALL: LazyLock<Regex> = rg!(r"\b[a-zA-Z_][a-zA-Z0-9_]*\s*(::\s*<.*?>)?\s*\(");

// --- Operator ---

// Triple
static OP_ADD_ASSIGN: LazyLock<Regex> = rg!(r"\+=");
static OP_SUB_ASSIGN: LazyLock<Regex> = rg!(r"-=");
static OP_MUL_ASSIGN: LazyLock<Regex> = rg!(r"\*=");
static OP_DIV_ASSIGN: LazyLock<Regex> = rg!(r"/=");
static OP_MOD_ASSIGN: LazyLock<Regex> = rg!(r"%=");
static OP_XOR_ASSIGN: LazyLock<Regex> = rg!(r"\^=");
static OP_AND_ASSIGN: LazyLock<Regex> = rg!(r"&=");
static OP_OR_ASSIGN: LazyLock<Regex> = rg!(r"\|=");
static OP_SHL_ASSIGN: LazyLock<Regex> = rg!(r"<<=");
static OP_SHR_ASSIGN: LazyLock<Regex> = rg!(r">>=");

// Double
static OP_EQ: LazyLock<Regex> = rg!(r"==");
static OP_NE: LazyLock<Regex> = rg!(r"!=");
static OP_LE: LazyLock<Regex> = rg!(r"<=");
static OP_GE: LazyLock<Regex> = rg!(r">=");
static OP_AND: LazyLock<Regex> = rg!(r"&&");
static OP_OR: LazyLock<Regex> = rg!(r"\|\|");
static OP_SHL: LazyLock<Regex> = rg!(r"<<");
static OP_SHR: LazyLock<Regex> = rg!(r">>");
static OP_RANGE: LazyLock<Regex> = rg!(r"\.\.");
static OP_RANGE_INCLUSIVE: LazyLock<Regex> = rg!(r"\.\.=");
static OP_ARROW: LazyLock<Regex> = rg!(r"=>");
static OP_THIN_ARROW: LazyLock<Regex> = rg!(r"->");
static OP_SCOPE: LazyLock<Regex> = rg!(r"::");

// Single
static OP_ADD: LazyLock<Regex> = rg!(r"\+");
static OP_SUB: LazyLock<Regex> = rg!(r"-");
static OP_MUL: LazyLock<Regex> = rg!(r"\*");
static OP_DIV: LazyLock<Regex> = rg!(r"/");
static OP_MOD: LazyLock<Regex> = rg!(r"%");
static OP_XOR: LazyLock<Regex> = rg!(r"\^");
static OP_BIT_AND: LazyLock<Regex> = rg!(r"&");
static OP_BIT_OR: LazyLock<Regex> = rg!(r"\|");
static OP_NOT: LazyLock<Regex> = rg!(r"!");
static OP_BIT_NOT: LazyLock<Regex> = rg!(r"~");
static OP_LT: LazyLock<Regex> = rg!(r"<");
static OP_GT: LazyLock<Regex> = rg!(r">");
static OP_ASSIGN: LazyLock<Regex> = rg!(r"=");
static OP_QUESTION: LazyLock<Regex> = rg!(r"\?");
static PUNCT_DOUBLE_COLON: LazyLock<Regex> = rg!(r"\:");

// --- Punctuation ---
static PUNCT_LPAREN: LazyLock<Regex> = rg!(r"\(");
static PUNCT_RPAREN: LazyLock<Regex> = rg!(r"\)");
static PUNCT_LBRACKET: LazyLock<Regex> = rg!(r"\[");
static PUNCT_RBRACKET: LazyLock<Regex> = rg!(r"\]");
static PUNCT_LBRACE: LazyLock<Regex> = rg!(r"\{");
static PUNCT_RBRACE: LazyLock<Regex> = rg!(r"\}");
static PUNCT_SEMICOLON: LazyLock<Regex> = rg!(r";");
static PUNCT_COMMA: LazyLock<Regex> = rg!(r",");
static PUNCT_DOT: LazyLock<Regex> = rg!(r"\.");

// --- Identifier ---
static IDENTIFIER: LazyLock<Regex> = rg!(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b");
static TYPE_IDENTIFIER: LazyLock<Regex> = rg!(r"\b[A-Z][a-zA-Z0-9_]*\b");

// --- Keyword ---
const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "union",
    "unsafe", "use", "where", "while", "yield",
];

// --- Type ---
const RUST_TYPES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32",
    "f64", "bool", "char", "str",
];

// --- HTML ---
static HTML_OPEN_TAG: LazyLock<Regex> = rg!(
    r#"<[a-zA-Z][a-zA-Z0-9\-]*(\s+[a-zA-Z_:][-a-zA-Z0-9_:.]*(\s*=\s*(".*?"|'.*?'|[^\s"'>=]+))?)*\s*?>"#
);
static HTML_CLOSE_TAG: LazyLock<Regex> = rg!(r"</[a-zA-Z][a-zA-Z0-9\-]*\s*>");
static HTML_SELF_CLOSE_TAG: LazyLock<Regex> = rg!(
    r#"<[a-zA-Z][a-zA-Z0-9\-]*(\s+[a-zA-Z_:][-a-zA-Z0-9_:.]*(\s*=\s*(".*?"|'.*?'|[^\s"'>=]+))?)*\s*/>"#
);

static PATTERNS: LazyLock<Vec<(Regex, TokenType)>> = LazyLock::new(|| {
    let static_patterns = vec![
        // Comment
        (COMMENT_LINE.clone(), TokenType::Comment),
        (COMMENT_BLOCK.clone(), TokenType::Comment),
        // Char
        (CHAR_LITERAL.clone(), TokenType::Char),
        // Lifetime
        (LIFETIME.clone(), TokenType::Lifetime),
        (LIFETIME_REF.clone(), TokenType::Lifetime),
        // String
        (STRING_DOUBLE.clone(), TokenType::String),
        // Number
        (NUMBER_FLOAT.clone(), TokenType::Number),
        // Number with suffix
        (NUMBER_WITH_SUFFIX.clone(), TokenType::Number),
        (NUMBER_HEX.clone(), TokenType::Number),
        (NUMBER_OCTAL.clone(), TokenType::Number),
        (NUMBER_BINARY.clone(), TokenType::Number),
        // Atribute and Macro
        (ATTRIBUTE.clone(), TokenType::Attribute),
        (MACRO_CALL.clone(), TokenType::Macro),
    ];

    // Generate keyword patterns
    let keyword_patterns = RUST_KEYWORDS.iter().map(|keyword| {
        (
            Regex::new(&format!(r"\b{keyword}\b")).unwrap(),
            TokenType::Keyword,
        )
    });

    // Generate type patterns
    let type_patterns = RUST_TYPES.iter().map(|type_name| {
        (
            Regex::new(&format!(r"\b{type_name}\b")).unwrap(),
            TokenType::Type,
        )
    });

    // More static patterns
    let operator_patterns = vec![
        (FUNCTION_CALL.clone(), TokenType::Function),
        // 3-char
        (OP_SHL_ASSIGN.clone(), TokenType::Operator), // <<=
        (OP_SHR_ASSIGN.clone(), TokenType::Operator), // >>=
        (OP_RANGE_INCLUSIVE.clone(), TokenType::Operator), // ..=
        // 2-char
        (OP_ADD_ASSIGN.clone(), TokenType::Operator), // +=
        (OP_SUB_ASSIGN.clone(), TokenType::Operator), // -=
        (OP_MUL_ASSIGN.clone(), TokenType::Operator), // *=
        (OP_DIV_ASSIGN.clone(), TokenType::Operator), // /=
        (OP_MOD_ASSIGN.clone(), TokenType::Operator), // %=
        (OP_XOR_ASSIGN.clone(), TokenType::Operator), // ^=
        (OP_AND_ASSIGN.clone(), TokenType::Operator), // &=
        (OP_OR_ASSIGN.clone(), TokenType::Operator),  // |=
        (OP_EQ.clone(), TokenType::Operator),         // ==
        (OP_NE.clone(), TokenType::Operator),         // !=
        (OP_LE.clone(), TokenType::Operator),         // <=
        (OP_GE.clone(), TokenType::Operator),         // >=
        (OP_AND.clone(), TokenType::Operator),        // &&
        (OP_OR.clone(), TokenType::Operator),         // ||
        (OP_SHL.clone(), TokenType::Operator),        // <=
        (OP_SHR.clone(), TokenType::Operator),        // >=
        (OP_RANGE.clone(), TokenType::Operator),      // ..
        (OP_ARROW.clone(), TokenType::Operator),      // =>
        (OP_THIN_ARROW.clone(), TokenType::Operator), // ->
        (OP_SCOPE.clone(), TokenType::Operator),      // ::
        // 1-char
        (OP_ADD.clone(), TokenType::Operator),      // +
        (OP_SUB.clone(), TokenType::Operator),      // -
        (OP_MUL.clone(), TokenType::Operator),      // *
        (OP_DIV.clone(), TokenType::Operator),      // /
        (OP_MOD.clone(), TokenType::Operator),      // %
        (OP_XOR.clone(), TokenType::Operator),      // ^
        (OP_BIT_AND.clone(), TokenType::Operator),  // &
        (OP_BIT_OR.clone(), TokenType::Operator),   // |
        (OP_NOT.clone(), TokenType::Operator),      // !
        (OP_BIT_NOT.clone(), TokenType::Operator),  // ~
        (OP_LT.clone(), TokenType::Manual),         // <
        (OP_GT.clone(), TokenType::Manual),         // >
        (OP_ASSIGN.clone(), TokenType::Operator),   // =
        (OP_QUESTION.clone(), TokenType::Operator), // ?
        // Punctation
        (PUNCT_LPAREN.clone(), TokenType::Punctuation),
        (PUNCT_RPAREN.clone(), TokenType::Punctuation),
        (PUNCT_LBRACKET.clone(), TokenType::Punctuation),
        (PUNCT_RBRACKET.clone(), TokenType::Punctuation),
        (PUNCT_LBRACE.clone(), TokenType::Punctuation),
        (PUNCT_RBRACE.clone(), TokenType::Punctuation),
        (PUNCT_SEMICOLON.clone(), TokenType::Punctuation),
        (PUNCT_COMMA.clone(), TokenType::Punctuation),
        (PUNCT_DOT.clone(), TokenType::Punctuation),
        (TYPE_IDENTIFIER.clone(), TokenType::Type),
        (IDENTIFIER.clone(), TokenType::Identifier),
        (PUNCT_DOUBLE_COLON.clone(), TokenType::Punctuation),
    ];

    // Chain all patterns together
    static_patterns
        .into_iter()
        .chain(keyword_patterns)
        .chain(type_patterns)
        .chain(operator_patterns)
        .collect()
});

// Util
static FUNC_NAME: LazyLock<Regex> = rg!(r"([a-zA-Z_][a-zA-Z0-9_]*)");

pub struct RustTokenizer {
    patterns: Vec<(Regex, TokenType)>,
    view_patterns: Vec<(Regex, TokenType)>,
    pos: usize,
    chars: Vec<char>,
    skip_unrecognized: bool,
}

impl RustTokenizer {
    pub fn tokenize(input: &str) -> Vec<Token> {
        let mut rust_tokenizer = RustTokenizer::new(input);

        rust_tokenizer.tokenize_all()
    }

    fn tokenize_all(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.pos < self.chars.len() {
            // White space
            if self.chars[self.pos].is_whitespace() {
                let mut end = self.pos + 1;
                while end < self.chars.len() && self.chars[end].is_whitespace() {
                    end += 1;
                }
                let content = self.chars[self.pos..end].iter().collect::<String>();
                tokens.push(Token::new(content, TokenType::Whitespace));
                self.pos = end;
                continue;
            }

            // Detect the "view!" macro call
            if self.match_view_macro() {
                // The macro matcher advances self.pos ‒ see below
                let view_tokens = self.tokenize_view_macro();
                tokens.extend(view_tokens);
                continue;
            }

            // Regular Rust lexing
            let content: String = self.chars[self.pos..].iter().collect();
            let ttoks = self.tokenize_once(&content);
            tokens.extend(ttoks);
        }

        tokens
    }

    fn tokenize_once(&mut self, content: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut found_match = false;

        for (pattern, token_type) in self.patterns.iter() {
            if let Some(mat) = pattern.find(content) {
                if mat.start() == 0 {
                    let content = mat.as_str().to_string();
                    let end = self.pos + content.chars().count();

                    match token_type {
                        TokenType::Function if content.ends_with('(') => {
                            if let Some(mat) = FUNC_NAME.find(&content) {
                                let func_name = mat.as_str(); // just the function name
                                tokens.push(Token::new(func_name.to_string(), TokenType::Function));
                                self.pos += func_name.len(); // Advance only by function name's length
                                found_match = true; // Don't advance by the regex total match length
                                break; // Go back to main loop, rest is handled by normal tokenization
                            }
                        }

                        // For condition '<', '>' TokenType
                        TokenType::Manual => {
                            let left_is_whitespace = if self.pos > 0 {
                                self.chars[self.pos - 1].is_whitespace()
                            } else {
                                false
                            };
                            let right_is_whitespace = self
                                .chars
                                .get(self.pos + 1)
                                .is_some_and(|c| c.is_whitespace());

                            let token_type = if left_is_whitespace && right_is_whitespace {
                                TokenType::Operator
                            } else {
                                TokenType::Punctuation
                            };
                            tokens.push(Token::new(self.chars[self.pos].to_string(), token_type));
                            self.pos += 1;
                            found_match = true;
                            break;
                        }

                        token_type => tokens.push(Token::new(content, *token_type)),
                    }

                    self.pos = end;
                    found_match = true;
                    break;
                }
            }
        }

        if !found_match {
            if self.skip_unrecognized {
                // Skip unrecognized characters
                self.pos += 1;
            } else {
                // Create default token for unrecognized character
                let content = self.chars[self.pos].to_string();
                tokens.push(Token::new(content, TokenType::Default));
                self.pos += 1;
            }
        }

        tokens
    }

    fn match_view_macro(&self) -> bool {
        // match "view!" at self.pos, followed by optional space, followed by '{'
        let remain: String = self.chars[self.pos..].iter().collect();
        let macro_re = Regex::new(r"^view!\s*\{").unwrap();
        macro_re.is_match(&remain)
    }

    fn tokenize_view_macro(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        // Match and step over "view!" and spaces, push as macro
        let remain: String = self.chars[self.pos..].iter().collect();
        let macro_re = Regex::new(r"^view!").unwrap();

        if let Some(mat) = macro_re.find(&remain) {
            tokens.push(Token::new("view!".to_string(), TokenType::Macro));
            self.pos += mat.end();
            // Skip whitespace after "view!"
            while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
                self.pos += 1;
            }
        } else {
            // Should not happen, fall back
            return tokens;
        }

        // Now should be positioned at '{'
        if self.pos >= self.chars.len() || self.chars[self.pos] != '{' {
            return tokens;
        }
        tokens.push(Token::new("{".to_string(), TokenType::Punctuation));
        let open_brace = self.pos;
        let block_end =
            find_balanced_brace_block(&self.chars, open_brace).unwrap_or(self.chars.len() - 1);
        // Step over '{'
        self.pos += 1;

        let mut view_pos = self.pos;
        while view_pos < block_end {
            // Whitespace in HTML block
            if self.chars[view_pos].is_whitespace() {
                let mut next = view_pos + 1;
                while next < block_end && self.chars[next].is_whitespace() {
                    next += 1;
                }
                let content = self.chars[view_pos..next].iter().collect::<String>();
                tokens.push(Token::new(content, TokenType::Whitespace));
                view_pos = next;
                continue;
            }
            let slice: String = self.chars[view_pos..block_end].iter().collect();
            let mut matched = false;

            // Try match `view_patternsx`
            for (pat, ttype) in self.view_patterns.iter() {
                if let Some(mat) = pat.find(&slice) {
                    if mat.start() == 0 {
                        let seg = mat.as_str().to_string();
                        view_pos += seg.chars().count();
                        tokens.push(Token::new(seg, *ttype));
                        matched = true;
                        break;
                    }
                }
            }

            // Fallback to regular Rust patterns
            if !matched {
                let rest = &self.chars[view_pos..block_end].iter().collect::<String>();
                let saved_pos = self.pos;
                self.pos = view_pos;
                let ttoks = self.tokenize_once(rest);
                // tokens.extend will usually add 1 token
                let new_pos = self.pos;
                // Restore outer self.pos for subsequent macro parsing
                self.pos = saved_pos;

                // Nothing matched, position + 1
                if ttoks.is_empty() {
                    tokens.push(Token::new(
                        self.chars[view_pos].to_string(),
                        TokenType::Default,
                    ));
                    view_pos += 1;
                // Push and jump to the position
                } else {
                    // tokens found, advance view_pos
                    tokens.extend(ttoks);
                    view_pos = new_pos;
                }
            }
        }

        // add closing '}'
        tokens.push(Token::new("}".to_string(), TokenType::Punctuation));
        self.pos = block_end + 1;

        tokens
    }

    fn new(input: &str) -> Self {
        let view_patterns = vec![
            (HTML_OPEN_TAG.clone(), TokenType::Tag),
            (HTML_CLOSE_TAG.clone(), TokenType::Tag),
            (HTML_SELF_CLOSE_TAG.clone(), TokenType::Tag),
        ];
        let chars = input.chars().collect();

        Self {
            patterns: PATTERNS.clone(),
            view_patterns,
            pos: 0,
            chars,
            skip_unrecognized: false,
        }
    }
}

fn find_balanced_brace_block(chars: &[char], start: usize) -> Option<usize> {
    let mut depth = 0;
    let mut started = false;
    for (i, &ch) in chars.iter().enumerate().skip(start) {
        match ch {
            '{' => {
                depth += 1;
                started = true;
            }
            '}' => {
                if depth > 0 {
                    depth -= 1;
                }
                if depth == 0 && started {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}
