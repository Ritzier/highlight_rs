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
            CssToken {
                content: "/* comment */",
                kind: Comment
            },
            CssToken {
                content: "\n",
                kind: Whitespace
            },
            CssToken {
                content: "@media",
                kind: AtRules
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "screen",
                kind: SelectorTag
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "and",
                kind: Function
            },
            CssToken {
                content: "(",
                kind: Punctuation
            },
            CssToken {
                content: "min-width",
                kind: Property
            },
            CssToken {
                content: ":",
                kind: Punctuation
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "600px",
                kind: Unit
            },
            CssToken {
                content: ")",
                kind: Punctuation
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "{",
                kind: Punctuation
            },
            CssToken {
                content: "\n  ",
                kind: Whitespace
            },
            CssToken {
                content: "body",
                kind: SelectorTag
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "{",
                kind: Punctuation
            },
            CssToken {
                content: "\n    ",
                kind: Whitespace
            },
            CssToken {
                content: "color",
                kind: Property
            },
            CssToken {
                content: ":",
                kind: Punctuation
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "#fff",
                kind: Literal
            },
            CssToken {
                content: ";",
                kind: Punctuation
            },
            CssToken {
                content: "\n    ",
                kind: Whitespace
            },
            CssToken {
                content: "--main-bg",
                kind: CustomProperty
            },
            CssToken {
                content: ":",
                kind: Punctuation
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "linear-gradient",
                kind: Function
            },
            CssToken {
                content: "(",
                kind: Punctuation
            },
            CssToken {
                content: "to",
                kind: SelectorTag
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "bottom",
                kind: SelectorTag
            },
            CssToken {
                content: ",",
                kind: Punctuation
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "#fff",
                kind: Literal
            },
            CssToken {
                content: ",",
                kind: Punctuation
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "#000",
                kind: Literal
            },
            CssToken {
                content: ")",
                kind: Punctuation
            },
            CssToken {
                content: ";",
                kind: Punctuation
            },
            CssToken {
                content: "\n    ",
                kind: Whitespace
            },
            CssToken {
                content: "margin",
                kind: Property
            },
            CssToken {
                content: ":",
                kind: Punctuation
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "0",
                kind: Unit
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "auto",
                kind: Keyword
            },
            CssToken {
                content: ";",
                kind: Punctuation
            },
            CssToken {
                content: "\n    ",
                kind: Whitespace
            },
            CssToken {
                content: "font-size",
                kind: Property
            },
            CssToken {
                content: ":",
                kind: Punctuation
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "16px",
                kind: Unit
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "!important",
                kind: Keyword
            },
            CssToken {
                content: ";",
                kind: Punctuation
            },
            CssToken {
                content: "\n    ",
                kind: Whitespace
            },
            CssToken {
                content: "background",
                kind: Property
            },
            CssToken {
                content: ":",
                kind: Punctuation
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "url",
                kind: Function
            },
            CssToken {
                content: "(",
                kind: Punctuation
            },
            CssToken {
                content: "\"bg.png\"",
                kind: String
            },
            CssToken {
                content: ")",
                kind: Punctuation
            },
            CssToken {
                content: ";",
                kind: Punctuation
            },
            CssToken {
                content: "\n  ",
                kind: Whitespace
            },
            CssToken {
                content: "}",
                kind: Punctuation
            },
            CssToken {
                content: "\n",
                kind: Whitespace
            },
            CssToken {
                content: "}",
                kind: Punctuation
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
            CssToken {
                content: ":root",
                kind: SelectorPseudo
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "{",
                kind: Punctuation
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "--primary-color",
                kind: CustomProperty
            },
            CssToken {
                content: ":",
                kind: Punctuation
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "#123456",
                kind: Literal
            },
            CssToken {
                content: ";",
                kind: Punctuation
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "}",
                kind: Punctuation
            }
        ]
    )
}

#[test]
fn test_custome_properties() {
    let code = r#"#demo > .notification > div > p, p ~ ul, input [type="radio"] {
  color: green;
  width: 80%;
}
"#;
    assert_tokens!(
        code,
        [
            CssToken {
                content: "#demo",
                kind: SelectorId
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: ">",
                kind: Default
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: ".notification",
                kind: SelectorClass
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: ">",
                kind: Default
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "div",
                kind: SelectorTag
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: ">",
                kind: Default
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "p",
                kind: SelectorTag
            },
            CssToken {
                content: ",",
                kind: Punctuation
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "p",
                kind: SelectorTag
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "~",
                kind: Operator
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "ul",
                kind: SelectorTag
            },
            CssToken {
                content: ",",
                kind: Punctuation
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "input",
                kind: SelectorTag
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "[",
                kind: Default
            },
            CssToken {
                content: "type",
                kind: SelectorTag
            },
            CssToken {
                content: "=",
                kind: Default
            },
            CssToken {
                content: "\"radio\"",
                kind: String
            },
            CssToken {
                content: "]",
                kind: Default
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "{",
                kind: Punctuation
            },
            CssToken {
                content: "\n  ",
                kind: Whitespace
            },
            CssToken {
                content: "color",
                kind: Property
            },
            CssToken {
                content: ":",
                kind: Punctuation
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "green",
                kind: SelectorTag
            },
            CssToken {
                content: ";",
                kind: Punctuation
            },
            CssToken {
                content: "\n  ",
                kind: Whitespace
            },
            CssToken {
                content: "width",
                kind: Property
            },
            CssToken {
                content: ":",
                kind: Punctuation
            },
            CssToken {
                content: " ",
                kind: Whitespace
            },
            CssToken {
                content: "80",
                kind: Unit
            },
            CssToken {
                content: "%",
                kind: Default
            },
            CssToken {
                content: ";",
                kind: Punctuation
            },
            CssToken {
                content: "\n",
                kind: Whitespace
            },
            CssToken {
                content: "}",
                kind: Punctuation
            },
            CssToken {
                content: "\n",
                kind: Whitespace
            }
        ]
    );
}

#[test]
fn test_selectors_with_pseudo_and_class() {
    let code = "h1.mystyle:lang(en)";
    assert_tokens!(
        code,
        [
            CssToken {
                content: "h1",
                kind: SelectorTag
            },
            CssToken {
                content: ".mystyle",
                kind: SelectorClass
            },
            CssToken {
                content: ":lang(en)",
                kind: SelectorPseudo
            }
        ]
    )
}
