#[test]
fn test_full_view_macro() {
    let code = r#"#[component]
fn HomePage() {
    view! {
        <h1>"Home Page"</h1>
        <p>"Welcome to my home page..."</p>
    }
}"#;
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "#[component]",
                token_type: Attribute
            },
            Token {
                content: "fn",
                token_type: Keyword
            },
            Token {
                content: "HomePage",
                token_type: Function
            },
            Token {
                content: "(",
                token_type: Punctuation
            },
            Token {
                content: ")",
                token_type: Punctuation
            },
            Token {
                content: "{",
                token_type: Punctuation
            },
            Token {
                content: "view!",
                token_type: Macro
            },
            Token {
                content: "{",
                token_type: Punctuation
            },
            Token {
                content: "<h1>",
                token_type: Tag
            },
            Token {
                content: "\"Home Page\"",
                token_type: String
            },
            Token {
                content: "</h1>",
                token_type: Tag
            },
            Token {
                content: "<p>",
                token_type: Tag
            },
            Token {
                content: "\"Welcome to my home page...\"",
                token_type: String
            },
            Token {
                content: "</p>",
                token_type: Tag
            },
            Token {
                content: "}",
                token_type: Punctuation
            },
            Token {
                content: "}",
                token_type: Punctuation
            }
        ]
    );
}

#[test]
fn test_a() {
    let code = r#"view!{
    <button on:click=move|_|{
        leptos::logging::log!("Logging...");
    }>"Log"</button>
}"#;
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "view!",
                token_type: Macro
            },
            Token {
                content: "{",
                token_type: Punctuation
            },
            Token {
                content: "<",
                token_type: Punctuation
            },
            Token {
                content: "button",
                token_type: Identifier
            },
            Token {
                content: "on",
                token_type: Identifier
            },
            Token {
                content: ":",
                token_type: Punctuation
            },
            Token {
                content: "click",
                token_type: Identifier
            },
            Token {
                content: "=",
                token_type: Operator
            },
            Token {
                content: "move",
                token_type: Keyword
            },
            Token {
                content: "|",
                token_type: Operator
            },
            Token {
                content: "_",
                token_type: Identifier
            },
            Token {
                content: "|",
                token_type: Operator
            },
            Token {
                content: "{",
                token_type: Punctuation
            },
            Token {
                content: "leptos",
                token_type: Identifier
            },
            Token {
                content: "::",
                token_type: Operator
            },
            Token {
                content: "logging",
                token_type: Identifier
            },
            Token {
                content: "::",
                token_type: Operator
            },
            Token {
                content: "log!",
                token_type: Macro
            },
            Token {
                content: "(",
                token_type: Punctuation
            },
            Token {
                content: "\"Logging...\"",
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
                content: "}",
                token_type: Punctuation
            },
            Token {
                content: ">",
                token_type: Punctuation
            },
            Token {
                content: "\"Log\"",
                token_type: String
            },
            Token {
                content: "</button>",
                token_type: Tag
            },
            Token {
                content: "}",
                token_type: Punctuation
            }
        ]
    );
}
