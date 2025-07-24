/// Verifies Rust macro invocation tokenization.
#[test]
fn test_rust_macros() {
    // println! macro call
    assert_tokens!(
        "println!(\"{variable}\")",
        [
            Token {
                content: "println!",
                token_type: Macro
            },
            Token {
                content: "(",
                token_type: Punctuation
            },
            Token {
                content: "\"{variable}\"",
                token_type: String
            },
            Token {
                content: ")",
                token_type: Punctuation
            },
        ]
    );

    // vec! macro with brackets
    assert_tokens!(
        "vec![1,2,3]",
        [
            Token {
                content: "vec!",
                token_type: Macro
            },
            Token {
                content: "[",
                token_type: Punctuation
            },
            Token {
                content: "1",
                token_type: Number
            },
            Token {
                content: ",",
                token_type: Punctuation
            },
            Token {
                content: "2",
                token_type: Number
            },
            Token {
                content: ",",
                token_type: Punctuation
            },
            Token {
                content: "3",
                token_type: Number
            },
            Token {
                content: "]",
                token_type: Punctuation
            },
        ]
    );

    // Macro with braces
    assert_tokens!(
        "foo!{x, y}",
        [
            Token {
                content: "foo!",
                token_type: Macro
            },
            Token {
                content: "{",
                token_type: Punctuation
            },
            Token {
                content: "x",
                token_type: Identifier
            },
            Token {
                content: ",",
                token_type: Punctuation
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "y",
                token_type: Identifier
            },
            Token {
                content: "}",
                token_type: Punctuation
            },
        ]
    );

    // Macro with multiple tokens and whitespace
    assert_tokens!(
        "bar! ( 123 )",
        [
            Token {
                content: "bar!",
                token_type: Macro
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "(",
                token_type: Punctuation
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "123",
                token_type: Number
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: ")",
                token_type: Punctuation
            },
        ]
    );
}
