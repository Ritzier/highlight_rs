use super::*;

/// Many keywords and identifier in keyword-like position
#[test]
fn test_common_rust_keywords() {
    let code = [
        "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
        "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move",
        "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait",
        "true", "type", "union", "unsafe", "use", "where", "while", "yield",
    ]
    .join(" ");

    assert_tokens_skip_whitespace!(
        &code,
        [
            Token {
                content: "as",
                token_type: Keyword
            },
            Token {
                content: "async",
                token_type: Keyword
            },
            Token {
                content: "await",
                token_type: Keyword
            },
            Token {
                content: "break",
                token_type: Keyword
            },
            Token {
                content: "const",
                token_type: Keyword
            },
            Token {
                content: "continue",
                token_type: Keyword
            },
            Token {
                content: "crate",
                token_type: Keyword
            },
            Token {
                content: "dyn",
                token_type: Keyword
            },
            Token {
                content: "else",
                token_type: Keyword
            },
            Token {
                content: "enum",
                token_type: Keyword
            },
            Token {
                content: "extern",
                token_type: Keyword
            },
            Token {
                content: "false",
                token_type: Keyword
            },
            Token {
                content: "fn",
                token_type: Keyword
            },
            Token {
                content: "for",
                token_type: Keyword
            },
            Token {
                content: "if",
                token_type: Keyword
            },
            Token {
                content: "impl",
                token_type: Keyword
            },
            Token {
                content: "in",
                token_type: Keyword
            },
            Token {
                content: "let",
                token_type: Keyword
            },
            Token {
                content: "loop",
                token_type: Keyword
            },
            Token {
                content: "match",
                token_type: Keyword
            },
            Token {
                content: "mod",
                token_type: Keyword
            },
            Token {
                content: "move",
                token_type: Keyword
            },
            Token {
                content: "mut",
                token_type: Keyword
            },
            Token {
                content: "pub",
                token_type: Keyword
            },
            Token {
                content: "ref",
                token_type: Keyword
            },
            Token {
                content: "return",
                token_type: Keyword
            },
            Token {
                content: "self",
                token_type: Keyword
            },
            Token {
                content: "Self",
                token_type: Keyword
            },
            Token {
                content: "static",
                token_type: Keyword
            },
            Token {
                content: "struct",
                token_type: Keyword
            },
            Token {
                content: "super",
                token_type: Keyword
            },
            Token {
                content: "trait",
                token_type: Keyword
            },
            Token {
                content: "true",
                token_type: Keyword
            },
            Token {
                content: "type",
                token_type: Keyword
            },
            Token {
                content: "union",
                token_type: Keyword
            },
            Token {
                content: "unsafe",
                token_type: Keyword
            },
            Token {
                content: "use",
                token_type: Keyword
            },
            Token {
                content: "where",
                token_type: Keyword
            },
            Token {
                content: "while",
                token_type: Keyword
            },
            Token {
                content: "yield",
                token_type: Keyword
            }
        ]
    );
}
