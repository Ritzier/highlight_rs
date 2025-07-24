#[test]
fn test_full_css() {
    let code = r#"/* comment */
@media screen and (min-width: 600px) {
  body {
    color: #fff;
    --main-bg: linear-gradient(to bottom, #fff, #000);
    margin: 0 auto;
    font-size: 16px !important;
    background: url("bg.png");
  }
}"#;
    assert_tokens!(
        code,
        [
            Token {
                content: "/* comment */",
                token_type: Comment
            },
            Token {
                content: "\n",
                token_type: Whitespace
            },
            Token {
                content: "@media",
                token_type: Macro
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "screen",
                token_type: Property
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "and",
                token_type: Function
            },
            Token {
                content: "(",
                token_type: Punctuation
            },
            Token {
                content: "min-width",
                token_type: Property
            },
            Token {
                content: ":",
                token_type: Punctuation
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "600px",
                token_type: Unit
            },
            Token {
                content: ")",
                token_type: Punctuation
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "{",
                token_type: Punctuation
            },
            Token {
                content: "\n  ",
                token_type: Whitespace
            },
            Token {
                content: "body",
                token_type: Property
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "{",
                token_type: Punctuation
            },
            Token {
                content: "\n    ",
                token_type: Whitespace
            },
            Token {
                content: "color",
                token_type: Property
            },
            Token {
                content: ":",
                token_type: Punctuation
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "#fff",
                token_type: Literal
            },
            Token {
                content: ";",
                token_type: Punctuation
            },
            Token {
                content: "\n    ",
                token_type: Whitespace
            },
            Token {
                content: "--main-bg",
                token_type: Property
            },
            Token {
                content: ":",
                token_type: Punctuation
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "linear-gradient",
                token_type: Function
            },
            Token {
                content: "(",
                token_type: Punctuation
            },
            Token {
                content: "to",
                token_type: Property
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "bottom",
                token_type: Property
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
                content: "#fff",
                token_type: Literal
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
                content: "#000",
                token_type: Literal
            },
            Token {
                content: ")",
                token_type: Punctuation
            },
            Token {
                content: ";",
                token_type: Punctuation
            },
            Token {
                content: "\n    ",
                token_type: Whitespace
            },
            Token {
                content: "margin",
                token_type: Property
            },
            Token {
                content: ":",
                token_type: Punctuation
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "0",
                token_type: Unit
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "auto",
                token_type: Keyword
            },
            Token {
                content: ";",
                token_type: Punctuation
            },
            Token {
                content: "\n    ",
                token_type: Whitespace
            },
            Token {
                content: "font-size",
                token_type: Property
            },
            Token {
                content: ":",
                token_type: Punctuation
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "16px",
                token_type: Unit
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "!",
                token_type: Default
            },
            Token {
                content: "important",
                token_type: Keyword
            },
            Token {
                content: ";",
                token_type: Punctuation
            },
            Token {
                content: "\n    ",
                token_type: Whitespace
            },
            Token {
                content: "background",
                token_type: Property
            },
            Token {
                content: ":",
                token_type: Punctuation
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "url",
                token_type: Function
            },
            Token {
                content: "(",
                token_type: Punctuation
            },
            Token {
                content: "\"bg.png\"",
                token_type: String
            },
            Token {
                content: ")",
                token_type: Punctuation
            },
            Token {
                content: ";",
                token_type: Punctuation
            },
            Token {
                content: "\n  ",
                token_type: Whitespace
            },
            Token {
                content: "}",
                token_type: Punctuation
            },
            Token {
                content: "\n",
                token_type: Whitespace
            },
            Token {
                content: "}",
                token_type: Punctuation
            }
        ]
    )
}

#[test]
fn test_custom_properties_and_vars() {
    let css = r#":root { --primary-color: #123456; }"#;
    assert_tokens!(
        css,
        [
            Token {
                content: ":root",
                token_type: Selector
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "{",
                token_type: Punctuation
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "--primary-color",
                token_type: Property
            },
            Token {
                content: ":",
                token_type: Punctuation
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "#123456",
                token_type: Literal
            },
            Token {
                content: ";",
                token_type: Punctuation
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "}",
                token_type: Punctuation
            }
        ]
    );
}
