use super::*;

/// Standalone function call without arguments
#[test]
fn test_simple_function_call() {
    let code = "main()";
    assert_tokens!(
        code,
        [
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
            }
        ]
    );
}

/// Function call with a string literal as argument
#[test]
fn test_function_with_string_argument() {
    let code = "greet_user(\"Ritzier\")";
    assert_tokens!(
        code,
        [
            Token {
                content: "greet_user",
                token_type: Function
            },
            Token {
                content: "(",
                token_type: Punctuation
            },
            Token {
                content: "\"Ritzier\"",
                token_type: String
            },
            Token {
                content: ")",
                token_type: Punctuation
            }
        ]
    );
}

/// Object method call with a simple argument
#[test]
fn test_method_call_on_identifier() {
    let code = "variable.get(1)";
    assert_tokens!(
        code,
        [
            Token {
                content: "variable",
                token_type: Identifier
            },
            Token {
                content: ".",
                token_type: Punctuation
            },
            Token {
                content: "get",
                token_type: Function
            },
            Token {
                content: "(",
                token_type: Punctuation
            },
            Token {
                content: "1",
                token_type: Number
            },
            Token {
                content: ")",
                token_type: Punctuation
            }
        ]
    );
}

/// Method call with multiple arguments including named and literal
#[test]
fn test_method_with_multiple_arguments() {
    let code = "foo.bar(123, \"baz\")";
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "foo",
                token_type: Identifier
            },
            Token {
                content: ".",
                token_type: Punctuation
            },
            Token {
                content: "bar",
                token_type: Function
            },
            Token {
                content: "(",
                token_type: Punctuation
            },
            Token {
                content: "123",
                token_type: Number
            },
            Token {
                content: ",",
                token_type: Punctuation
            },
            Token {
                content: "\"baz\"",
                token_type: String
            },
            Token {
                content: ")",
                token_type: Punctuation
            }
        ]
    );
}

/// Associated function call (e.g. static method)
#[test]
fn test_associated_function_call() {
    let code = "String::from(\"hi\")";
    assert_tokens!(
        code,
        [
            Token {
                content: "String",
                token_type: Type
            },
            Token {
                content: "::",
                token_type: Operator
            },
            Token {
                content: "from",
                token_type: Function
            },
            Token {
                content: "(",
                token_type: Punctuation
            },
            Token {
                content: "\"hi\"",
                token_type: String
            },
            Token {
                content: ")",
                token_type: Punctuation
            }
        ]
    );
}

/// Nested function and method calls
#[test]
fn test_nested_function_and_method_calls() {
    let code = "foo(bar(baz()))";
    assert_tokens!(
        code,
        [
            Token {
                content: "foo",
                token_type: Function
            },
            Token {
                content: "(",
                token_type: Punctuation
            },
            Token {
                content: "bar",
                token_type: Function
            },
            Token {
                content: "(",
                token_type: Punctuation
            },
            Token {
                content: "baz",
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
                content: ")",
                token_type: Punctuation
            },
            Token {
                content: ")",
                token_type: Punctuation
            }
        ]
    );
}

/// Function call using turbo fish (::<>) generic arguments
#[test]
fn test_function_call_with_turbo_fish() {
    let code = "parse::<usize>(\"42\")";
    assert_tokens!(
        code,
        [
            Token {
                content: "parse",
                token_type: Function
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
                content: "usize",
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
                content: "\"42\"",
                token_type: String
            },
            Token {
                content: ")",
                token_type: Punctuation
            }
        ]
    );
}

/// Call to a generic function with multiple generic arguments
#[test]
fn test_function_call_with_multiple_generics() {
    let code = "foo::<Result<i32, E>, Option<String>>(input)";
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "foo",
                token_type: Function
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
                content: "Result",
                token_type: Type
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
                content: ",",
                token_type: Punctuation
            },
            Token {
                content: "E",
                token_type: Type
            },
            Token {
                content: ">",
                token_type: Punctuation
            },
            Token {
                content: ",",
                token_type: Punctuation
            },
            Token {
                content: "Option",
                token_type: Type
            },
            Token {
                content: "<",
                token_type: Punctuation
            },
            Token {
                content: "String",
                token_type: Type
            },
            Token {
                content: ">>",
                token_type: Operator
            },
            Token {
                content: "(",
                token_type: Punctuation
            },
            Token {
                content: "input",
                token_type: Identifier
            },
            Token {
                content: ")",
                token_type: Punctuation
            }
        ]
    )
}
