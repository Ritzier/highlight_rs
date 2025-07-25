#[test]
fn test_css_keyword() {
    let code = "important inherit initial unset revert auto none normal transparent currentColor";
    assert_tokens_skip_whitespace!(
        code,
        [
            CssToken {
                content: "important",
                kind: Keyword
            },
            CssToken {
                content: "inherit",
                kind: Keyword
            },
            CssToken {
                content: "initial",
                kind: Keyword
            },
            CssToken {
                content: "unset",
                kind: Keyword
            },
            CssToken {
                content: "revert",
                kind: Keyword
            },
            CssToken {
                content: "auto",
                kind: Keyword
            },
            CssToken {
                content: "none",
                kind: Keyword
            },
            CssToken {
                content: "normal",
                kind: Keyword
            },
            CssToken {
                content: "transparent",
                kind: Keyword
            },
            CssToken {
                content: "currentColor",
                kind: Keyword
            }
        ]
    )
}
