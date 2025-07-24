#[test]
fn test_css_keyword() {
    let code = "important inherit initial unset revert auto none normal transparent currentColor";
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "important",
                token_type: Keyword
            },
            Token {
                content: "inherit",
                token_type: Keyword
            },
            Token {
                content: "initial",
                token_type: Keyword
            },
            Token {
                content: "unset",
                token_type: Keyword
            },
            Token {
                content: "revert",
                token_type: Keyword
            },
            Token {
                content: "auto",
                token_type: Keyword
            },
            Token {
                content: "none",
                token_type: Keyword
            },
            Token {
                content: "normal",
                token_type: Keyword
            },
            Token {
                content: "transparent",
                token_type: Keyword
            },
            Token {
                content: "currentColor",
                token_type: Keyword
            }
        ]
    );
}
