use std::sync::LazyLock;

use super::RustTokenKind;
use crate::regex::*;

// --- Comments ---
pub static COMMENT_LINE: LazyLock<Regex> = rg!(r"//.*");
pub static COMMENT_BLOCK: LazyLock<Regex> = rg!(r"/\*[\s\S]*?\*/");

// --- Lifetimes ---
pub static LIFETIME: LazyLock<Regex> = rg!(r"'(_|[a-zA-Z][a-zA-Z0-9_]*)\b");
pub static LIFETIME_REF: LazyLock<Regex> = rg!(r"&'(_|[a-zA-Z][a-zA-Z0-9_]*)\b");

//  --- String ---
pub static STRING_DOUBLE: LazyLock<Regex> = rg!(r#""([^"\\]|\\.)*""#);

// --- Char ---
static CHAR_LITERAL: LazyLock<Regex> =
    rg!(r"'([^'\\]|\\[nrt0'\\]|\\x[0-9a-fA-F]{2}|\\u\{[0-9a-fA-F]{1,6}\})'");

// --- Number ---
pub static NUMBER_WITH_SUFFIX: LazyLock<Regex> =
    rg!(r"\b\d[\d_]*([eE][+-]?\d+)?(_?[a-zA-Z][a-zA-Z0-9]*)?\b");
pub static NUMBER_FLOAT: LazyLock<Regex> = rg!(
    r"\b((\d[\d_]*\.\d[\d_]*|\d[\d_]*\.|\.\d[\d_]*)([eE][+-]?\d[\d_]*)?(_?[a-zA-Z][a-zA-Z0-9]*)?)\b"
);

pub static NUMBER_HEX: LazyLock<Regex> = rg!(r"\b0[xX][0-9a-fA-F]+\b");
pub static NUMBER_OCTAL: LazyLock<Regex> = rg!(r"\b0[oO][0-7]+\b");
pub static NUMBER_BINARY: LazyLock<Regex> = rg!(r"\b0[bB][01]+\b");

// --- Attributes ---
pub static ATTRIBUTE: LazyLock<Regex> = rg!(r"#!?\[[\s\S]*?\]");

// --- Macro ---
pub static MACRO_CALL: LazyLock<Regex> = rg!(r"\b\w+!");

// --- Function calls ---
pub static FUNCTION_CALL: LazyLock<Regex> = rg!(r"\b[a-zA-Z_][a-zA-Z0-9_]*\s*(::\s*<.*?>)?\s*\(");

// --- Operator ---

// Triple
pub static OP_ADD_ASSIGN: LazyLock<Regex> = rg!(r"\+=");
pub static OP_SUB_ASSIGN: LazyLock<Regex> = rg!(r"-=");
pub static OP_MUL_ASSIGN: LazyLock<Regex> = rg!(r"\*=");
pub static OP_DIV_ASSIGN: LazyLock<Regex> = rg!(r"/=");
pub static OP_MOD_ASSIGN: LazyLock<Regex> = rg!(r"%=");
pub static OP_XOR_ASSIGN: LazyLock<Regex> = rg!(r"\^=");
pub static OP_AND_ASSIGN: LazyLock<Regex> = rg!(r"&=");
pub static OP_OR_ASSIGN: LazyLock<Regex> = rg!(r"\|=");
pub static OP_SHL_ASSIGN: LazyLock<Regex> = rg!(r"<<=");
pub static OP_SHR_ASSIGN: LazyLock<Regex> = rg!(r">>=");

// Double
pub static OP_EQ: LazyLock<Regex> = rg!(r"==");
pub static OP_NE: LazyLock<Regex> = rg!(r"!=");
pub static OP_LE: LazyLock<Regex> = rg!(r"<=");
pub static OP_GE: LazyLock<Regex> = rg!(r">=");
pub static OP_AND: LazyLock<Regex> = rg!(r"&&");
pub static OP_OR: LazyLock<Regex> = rg!(r"\|\|");
pub static OP_SHL: LazyLock<Regex> = rg!(r"<<");
pub static OP_SHR: LazyLock<Regex> = rg!(r">>");
pub static OP_RANGE: LazyLock<Regex> = rg!(r"\.\.");
pub static OP_RANGE_INCLUSIVE: LazyLock<Regex> = rg!(r"\.\.=");
pub static OP_ARROW: LazyLock<Regex> = rg!(r"=>");
pub static OP_THIN_ARROW: LazyLock<Regex> = rg!(r"->");
pub static OP_SCOPE: LazyLock<Regex> = rg!(r"::");

// Single
pub static OP_ADD: LazyLock<Regex> = rg!(r"\+");
pub static OP_SUB: LazyLock<Regex> = rg!(r"-");
pub static OP_MUL: LazyLock<Regex> = rg!(r"\*");
pub static OP_DIV: LazyLock<Regex> = rg!(r"/");
pub static OP_MOD: LazyLock<Regex> = rg!(r"%");
pub static OP_XOR: LazyLock<Regex> = rg!(r"\^");
pub static OP_BIT_AND: LazyLock<Regex> = rg!(r"&");
pub static OP_BIT_OR: LazyLock<Regex> = rg!(r"\|");
pub static OP_NOT: LazyLock<Regex> = rg!(r"!");
pub static OP_BIT_NOT: LazyLock<Regex> = rg!(r"~");
pub static OP_LT: LazyLock<Regex> = rg!(r"<");
pub static OP_GT: LazyLock<Regex> = rg!(r">");
pub static OP_ASSIGN: LazyLock<Regex> = rg!(r"=");
pub static OP_QUESTION: LazyLock<Regex> = rg!(r"\?");
pub static PUNCT_DOUBLE_COLON: LazyLock<Regex> = rg!(r"\:");

// --- Punctuation ---
pub static PUNCT_LPAREN: LazyLock<Regex> = rg!(r"\(");
pub static PUNCT_RPAREN: LazyLock<Regex> = rg!(r"\)");
pub static PUNCT_LBRACKET: LazyLock<Regex> = rg!(r"\[");
pub static PUNCT_RBRACKET: LazyLock<Regex> = rg!(r"\]");
pub static PUNCT_LBRACE: LazyLock<Regex> = rg!(r"\{");
pub static PUNCT_RBRACE: LazyLock<Regex> = rg!(r"\}");
pub static PUNCT_SEMICOLON: LazyLock<Regex> = rg!(r";");
pub static PUNCT_COMMA: LazyLock<Regex> = rg!(r",");
pub static PUNCT_DOT: LazyLock<Regex> = rg!(r"\.");

// --- Identifier ---
pub static IDENTIFIER: LazyLock<Regex> = rg!(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b");
pub static TYPE_IDENTIFIER: LazyLock<Regex> = rg!(r"\b[A-Z][a-zA-Z0-9_]*\b");

// --- Keyword ---
pub const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "union",
    "unsafe", "use", "where", "while", "yield",
];

// --- Type ---
pub const RUST_TYPES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32",
    "f64", "bool", "char", "str",
];

// --- HTML ---
pub static HTML_OPEN_TAG: LazyLock<Regex> = rg!(
    r#"<[a-zA-Z][a-zA-Z0-9\-]*(\s+[a-zA-Z_:][-a-zA-Z0-9_:.]*(\s*=\s*(".*?"|'.*?'|[^\s"'>=]+))?)*\s*?>"#
);
pub static HTML_CLOSE_TAG: LazyLock<Regex> = rg!(r"</[a-zA-Z][a-zA-Z0-9\-]*\s*>");
pub static HTML_SELF_CLOSE_TAG: LazyLock<Regex> = rg!(
    r#"<[a-zA-Z][a-zA-Z0-9\-]*(\s+[a-zA-Z_:][-a-zA-Z0-9_:.]*(\s*=\s*(".*?"|'.*?'|[^\s"'>=]+))?)*\s*/>"#
);

pub static PATTERNS: LazyLock<Vec<(Regex, RustTokenKind)>> = LazyLock::new(|| {
    let static_patterns = vec![
        // Comment
        (COMMENT_LINE.clone(), RustTokenKind::Comment),
        (COMMENT_BLOCK.clone(), RustTokenKind::Comment),
        // Char
        (CHAR_LITERAL.clone(), RustTokenKind::Char),
        // Lifetime
        (LIFETIME.clone(), RustTokenKind::Lifetime),
        (LIFETIME_REF.clone(), RustTokenKind::Lifetime),
        // String
        (STRING_DOUBLE.clone(), RustTokenKind::String),
        // Number
        (NUMBER_FLOAT.clone(), RustTokenKind::Number),
        // Number with suffix
        (NUMBER_WITH_SUFFIX.clone(), RustTokenKind::Number),
        (NUMBER_HEX.clone(), RustTokenKind::Number),
        (NUMBER_OCTAL.clone(), RustTokenKind::Number),
        (NUMBER_BINARY.clone(), RustTokenKind::Number),
        // Atribute and Macro
        (ATTRIBUTE.clone(), RustTokenKind::Attribute),
        (MACRO_CALL.clone(), RustTokenKind::Macro),
    ];

    // Generate keyword patterns
    let keyword_patterns = RUST_KEYWORDS.iter().map(|keyword| {
        (
            Regex::new(&format!(r"\b{keyword}\b")).unwrap(),
            RustTokenKind::Keyword,
        )
    });

    // Generate type patterns
    let type_patterns = RUST_TYPES.iter().map(|type_name| {
        (
            Regex::new(&format!(r"\b{type_name}\b")).unwrap(),
            RustTokenKind::Type,
        )
    });

    // More static patterns
    let operator_patterns = vec![
        (FUNCTION_CALL.clone(), RustTokenKind::Function),
        // 3-char
        (OP_SHL_ASSIGN.clone(), RustTokenKind::Operator), // <<=
        (OP_SHR_ASSIGN.clone(), RustTokenKind::Operator), // >>=
        (OP_RANGE_INCLUSIVE.clone(), RustTokenKind::Operator), // ..=
        // 2-char
        (OP_ADD_ASSIGN.clone(), RustTokenKind::Operator), // +=
        (OP_SUB_ASSIGN.clone(), RustTokenKind::Operator), // -=
        (OP_MUL_ASSIGN.clone(), RustTokenKind::Operator), // *=
        (OP_DIV_ASSIGN.clone(), RustTokenKind::Operator), // /=
        (OP_MOD_ASSIGN.clone(), RustTokenKind::Operator), // %=
        (OP_XOR_ASSIGN.clone(), RustTokenKind::Operator), // ^=
        (OP_AND_ASSIGN.clone(), RustTokenKind::Operator), // &=
        (OP_OR_ASSIGN.clone(), RustTokenKind::Operator),  // |=
        (OP_EQ.clone(), RustTokenKind::Operator),         // ==
        (OP_NE.clone(), RustTokenKind::Operator),         // !=
        (OP_LE.clone(), RustTokenKind::Operator),         // <=
        (OP_GE.clone(), RustTokenKind::Operator),         // >=
        (OP_AND.clone(), RustTokenKind::Operator),        // &&
        (OP_OR.clone(), RustTokenKind::Operator),         // ||
        (OP_SHL.clone(), RustTokenKind::Operator),        // <=
        (OP_SHR.clone(), RustTokenKind::Operator),        // >=
        (OP_RANGE.clone(), RustTokenKind::Operator),      // ..
        (OP_ARROW.clone(), RustTokenKind::Operator),      // =>
        (OP_THIN_ARROW.clone(), RustTokenKind::Operator), // ->
        (OP_SCOPE.clone(), RustTokenKind::Operator),      // ::
        // 1-char
        (OP_ADD.clone(), RustTokenKind::Operator),      // +
        (OP_SUB.clone(), RustTokenKind::Operator),      // -
        (OP_MUL.clone(), RustTokenKind::Operator),      // *
        (OP_DIV.clone(), RustTokenKind::Operator),      // /
        (OP_MOD.clone(), RustTokenKind::Operator),      // %
        (OP_XOR.clone(), RustTokenKind::Operator),      // ^
        (OP_BIT_AND.clone(), RustTokenKind::Operator),  // &
        (OP_BIT_OR.clone(), RustTokenKind::Operator),   // |
        (OP_NOT.clone(), RustTokenKind::Operator),      // !
        (OP_BIT_NOT.clone(), RustTokenKind::Operator),  // ~
        (OP_LT.clone(), RustTokenKind::Manual),         // <
        (OP_GT.clone(), RustTokenKind::Manual),         // >
        (OP_ASSIGN.clone(), RustTokenKind::Operator),   // =
        (OP_QUESTION.clone(), RustTokenKind::Operator), // ?
        // Punctation
        (PUNCT_LPAREN.clone(), RustTokenKind::Punctuation),
        (PUNCT_RPAREN.clone(), RustTokenKind::Punctuation),
        (PUNCT_LBRACKET.clone(), RustTokenKind::Punctuation),
        (PUNCT_RBRACKET.clone(), RustTokenKind::Punctuation),
        (PUNCT_LBRACE.clone(), RustTokenKind::Punctuation),
        (PUNCT_RBRACE.clone(), RustTokenKind::Punctuation),
        (PUNCT_SEMICOLON.clone(), RustTokenKind::Punctuation),
        (PUNCT_COMMA.clone(), RustTokenKind::Punctuation),
        (PUNCT_DOT.clone(), RustTokenKind::Punctuation),
        (TYPE_IDENTIFIER.clone(), RustTokenKind::Type),
        (IDENTIFIER.clone(), RustTokenKind::Identifier),
        (PUNCT_DOUBLE_COLON.clone(), RustTokenKind::Punctuation),
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
pub static FUNC_NAME: LazyLock<Regex> = rg!(r"([a-zA-Z_][a-zA-Z0-9_]*)");
