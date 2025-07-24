use super::*;

#[test]
fn test_rust_strings() {
    let code = r#""hello world" 'c'"#;
    assert_tokens!(
        code,
        [
            Token {
                content: "\"hello world\"",
                token_type: String
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "'c'",
                token_type: Char
            }
        ]
    );
}

#[test]
fn test_string_escapes() {
    let code = r#""hello\nworld" "with\"quotes""#;
    assert_tokens!(
        code,
        [
            Token {
                content: "\"hello\\nworld\"",
                token_type: String
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "\"with\\\"quotes\"",
                token_type: String
            }
        ]
    );
}
