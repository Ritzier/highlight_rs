use highlight_rs::*;

#[test]
fn test_lt() {
    let code = "if 10 > 5";
    assert_tokens!(
        code,
        [
            Token {
                content: "if",
                token_type: Keyword
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "10",
                token_type: Number
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: ">",
                token_type: Operator
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "5",
                token_type: Number
            }
        ]
    );
    let code = "<i32>";
    assert_tokens!(
        code,
        [
            Token {
                content: "<",
                token_type: Punctuation
            },
            Token {
                content: "i32",
                token_type: Type
            },
            Token {
                content: ">",
                token_type: Punctuation
            }
        ]
    );
    let code = "Vec::<i32>::new()";
    assert_tokens!(
        code,
        [
            Token {
                content: "Vec",
                token_type: Type
            },
            Token {
                content: "::",
                token_type: Operator
            },
            Token {
                content: "<",
                token_type: Punctuation
            },
            Token {
                content: "i32",
                token_type: Type
            },
            Token {
                content: ">",
                token_type: Punctuation
            },
            Token {
                content: "::",
                token_type: Operator
            },
            Token {
                content: "new",
                token_type: Function
            },
            Token {
                content: "(",
                token_type: Punctuation
            },
            Token {
                content: ")",
                token_type: Punctuation
            }
        ]
    );
    let code = "fn get<T>(value: T){}";
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "fn",
                token_type: Keyword
            },
            Token {
                content: "get",
                token_type: Identifier
            },
            Token {
                content: "<",
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
                content: "(",
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
                content: ")",
                token_type: Punctuation
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

#[test]
fn test_complex_rust_code() {
    let code = r#"
fn main() {
    let x: i32 = 42;
    println!("Hello, {}!", x);
    // This is a comment
    if x > 0 {
        return;
    }
}
"#;
    assert_tokens!(
        code,
        [
            Token {
                content: "\n",
                token_type: Whitespace
            },
            Token {
                content: "fn",
                token_type: Keyword
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "main",
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
                content: "let",
                token_type: Keyword
            },
            Token {
                content: " ",
                token_type: Whitespace
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
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "i32",
                token_type: Type
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "=",
                token_type: Operator
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "42",
                token_type: Number
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
                content: "println!",
                token_type: Macro
            },
            Token {
                content: "(",
                token_type: Punctuation
            },
            Token {
                content: "\"Hello, {}!\"",
                token_type: String
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
                content: "x",
                token_type: Identifier
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
                content: "// This is a comment",
                token_type: Comment
            },
            Token {
                content: "\n    ",
                token_type: Whitespace
            },
            Token {
                content: "if",
                token_type: Keyword
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "x",
                token_type: Identifier
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: ">",
                token_type: Operator
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "0",
                token_type: Number
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
                content: "\n        ",
                token_type: Whitespace
            },
            Token {
                content: "return",
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
            },
            Token {
                content: "\n",
                token_type: Whitespace
            }
        ]
    );
}

// TODO:
// #[test]
// fn test_token_type_class_names() {
//     assert_eq!(RustTokenKind::Keyword.class_name(), "keyword");
//     assert_eq!(TokenType::String.class_name(), "string");
//     assert_eq!(TokenType::Number.class_name(), "number");
//     assert_eq!(TokenType::Comment.class_name(), "comment");
//     assert_eq!(TokenType::Identifier.class_name(), "identifier");
//     assert_eq!(TokenType::Operator.class_name(), "operator");
//     assert_eq!(TokenType::Punctuation.class_name(), "punctuation");
//     assert_eq!(TokenType::Type.class_name(), "type");
//     assert_eq!(TokenType::Function.class_name(), "function");
//     assert_eq!(TokenType::Macro.class_name(), "macro");
//     assert_eq!(TokenType::Attribute.class_name(), "attribute");
// }

#[test]
fn test_empty_code() {
    let tokens = RustTokenizer::tokenize("");
    assert_eq!(tokens.len(), 0);
}

#[test]
fn test_whitespace_only() {
    let code = "    \n\t  ";
    assert_tokens!(
        code,
        [Token {
            content: "    \n\t  ",
            token_type: Whitespace
        }]
    );
}
