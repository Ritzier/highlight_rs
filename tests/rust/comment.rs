use super::*;

/// Single-line (`// ...`) and block (`/* ... */`) comments
#[test]
fn test_rust_comments() {
    let code = "// line comment\n/* block comment */";
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "// line comment",
                token_type: Comment
            },
            Token {
                content: "/* block comment */",
                token_type: Comment
            }
        ]
    );
}

/// Multi-line block comment, which should be parsed as a single token
#[test]
fn test_multiline_comment() {
    let code = "/* this is a\n   multiline\n   comment */";
    assert_tokens_skip_whitespace!(
        code,
        [Token {
            content: "/* this is a\n   multiline\n   comment */",
            token_type: Comment
        }]
    );
}

/// Standard (///) and inner (//!) doc comments should both be recognized
#[test]
fn test_doc_comments_inner_and_outer() {
    let code = "\
/// This is an outer doc comment
//! This is an inner doc comment
";
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "/// This is an outer doc comment",
                token_type: Comment
            },
            Token {
                content: "//! This is an inner doc comment",
                token_type: Comment
            }
        ]
    );
}

/// Block doc comments (/** ... */ and /*! ... */) are also supported in Rust
#[test]
fn test_block_doc_comments() {
    let code = "\
/** This is a block doc comment */
/*! This is an inner block doc comment */
";
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "/** This is a block doc comment */",
                token_type: Comment
            },
            Token {
                content: "/*! This is an inner block doc comment */",
                token_type: Comment
            }
        ]
    );
}

/// Code with interspersed comments still properly tokenizes both
#[test]
fn test_comments_with_code() {
    let code = r#"
// Line comment before code
let x = 42; /* Trailing block comment */
"#;
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "// Line comment before code",
                token_type: Comment
            },
            Token {
                content: "let",
                token_type: Keyword
            },
            Token {
                content: "x",
                token_type: Identifier
            },
            Token {
                content: "=",
                token_type: Operator
            },
            Token {
                content: "42",
                token_type: Number
            },
            Token {
                content: ";",
                token_type: Punctuation
            },
            Token {
                content: "/* Trailing block comment */",
                token_type: Comment
            },
        ]
    );
}
