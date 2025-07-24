use super::*;

#[test]
fn test_rust_punctuation() {
    let code = "()[]{};,.";
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "(",
                token_type: Punctuation
            },
            Token {
                content: ")",
                token_type: Punctuation
            },
            Token {
                content: "[",
                token_type: Punctuation
            },
            Token {
                content: "]",
                token_type: Punctuation
            },
            Token {
                content: "{",
                token_type: Punctuation
            },
            Token {
                content: "}",
                token_type: Punctuation
            },
            Token {
                content: ";",
                token_type: Punctuation
            },
            Token {
                content: ",",
                token_type: Punctuation
            },
            Token {
                content: ".",
                token_type: Punctuation
            }
        ]
    );
}
