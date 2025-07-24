#[test]
fn struct_with_lifetime() {
    let code = r#"struct Holder<'a, T> {
    content: &'a str,
    value: T
}"#;
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "struct",
                token_type: Keyword
            },
            Token {
                content: "Holder",
                token_type: Type
            },
            Token {
                content: "<",
                token_type: Punctuation
            },
            Token {
                content: "'a",
                token_type: Lifetime
            },
            Token {
                content: ",",
                token_type: Punctuation
            },
            Token {
                content: "T",
                token_type: Type
            },
            Token {
                content: ">",
                token_type: Punctuation
            },
            Token {
                content: "{",
                token_type: Punctuation
            },
            Token {
                content: "content",
                token_type: Identifier
            },
            Token {
                content: ":",
                token_type: Punctuation
            },
            Token {
                content: "&'a",
                token_type: Lifetime
            },
            Token {
                content: "str",
                token_type: Type
            },
            Token {
                content: ",",
                token_type: Punctuation
            },
            Token {
                content: "value",
                token_type: Identifier
            },
            Token {
                content: ":",
                token_type: Punctuation
            },
            Token {
                content: "T",
                token_type: Type
            },
            Token {
                content: "}",
                token_type: Punctuation
            }
        ]
    );
}

#[test]
fn function_with_lifetime() {
    let code = "fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {}";
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "fn",
                token_type: Keyword
            },
            Token {
                content: "longest",
                token_type: Identifier
            },
            Token {
                content: "<",
                token_type: Punctuation
            },
            Token {
                content: "'a",
                token_type: Lifetime
            },
            Token {
                content: ">",
                token_type: Punctuation
            },
            Token {
                content: "(",
                token_type: Punctuation
            },
            Token {
                content: "x",
                token_type: Identifier
            },
            Token {
                content: ":",
                token_type: Punctuation
            },
            Token {
                content: "&'a",
                token_type: Lifetime
            },
            Token {
                content: "str",
                token_type: Type
            },
            Token {
                content: ",",
                token_type: Punctuation
            },
            Token {
                content: "y",
                token_type: Identifier
            },
            Token {
                content: ":",
                token_type: Punctuation
            },
            Token {
                content: "&'a",
                token_type: Lifetime
            },
            Token {
                content: "str",
                token_type: Type
            },
            Token {
                content: ")",
                token_type: Punctuation
            },
            Token {
                content: "->",
                token_type: Operator
            },
            Token {
                content: "&'a",
                token_type: Lifetime
            },
            Token {
                content: "str",
                token_type: Type
            },
            Token {
                content: "{",
                token_type: Punctuation
            },
            Token {
                content: "}",
                token_type: Punctuation
            }
        ]
    );
}
