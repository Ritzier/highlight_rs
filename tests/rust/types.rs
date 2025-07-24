#[test]
fn test_keywords() {
    let code = [
        "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
        "f32", "f64", "bool", "char", "str",
    ]
    .join(" ");

    assert_tokens_skip_whitespace!(
        &code,
        [
            Token {
                content: "i8",
                token_type: Type
            },
            Token {
                content: "i16",
                token_type: Type
            },
            Token {
                content: "i32",
                token_type: Type
            },
            Token {
                content: "i64",
                token_type: Type
            },
            Token {
                content: "i128",
                token_type: Type
            },
            Token {
                content: "isize",
                token_type: Type
            },
            Token {
                content: "u8",
                token_type: Type
            },
            Token {
                content: "u16",
                token_type: Type
            },
            Token {
                content: "u32",
                token_type: Type
            },
            Token {
                content: "u64",
                token_type: Type
            },
            Token {
                content: "u128",
                token_type: Type
            },
            Token {
                content: "usize",
                token_type: Type
            },
            Token {
                content: "f32",
                token_type: Type
            },
            Token {
                content: "f64",
                token_type: Type
            },
            Token {
                content: "bool",
                token_type: Type
            },
            Token {
                content: "char",
                token_type: Type
            },
            Token {
                content: "str",
                token_type: Type
            }
        ]
    );
}
