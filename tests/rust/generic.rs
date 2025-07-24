#[test]
fn test_struct_turbo_fish() {
    let code = "Vec::<String>::new()";
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
                content: "String",
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
}

#[test]
fn test_simple_generic_function() {
    let code = "fn get<T>(t: T) {}";
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
                content: "t",
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
            },
        ]
    );
}

#[test]
fn test_nested_generics() {
    let code = "Option<Result<Vec<u8>, String>>";
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "Option",
                token_type: Type
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
                content: "Vec",
                token_type: Type
            },
            Token {
                content: "<",
                token_type: Punctuation
            },
            Token {
                content: "u8",
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
                content: "String",
                token_type: Type
            },
            Token {
                content: ">>",
                token_type: Operator
            }
        ]
    );
}

#[test]
fn test_turbofish_method_with_generic() {
    let code = "my_str.parse::<i32>().unwrap()";
    assert_tokens!(
        code,
        [
            Token {
                content: "my_str",
                token_type: Identifier
            },
            Token {
                content: ".",
                token_type: Punctuation
            },
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
                content: "i32",
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
                content: ")",
                token_type: Punctuation
            },
            Token {
                content: ".",
                token_type: Punctuation
            },
            Token {
                content: "unwrap",
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

#[test]
fn test_lifetime_and_generics() {
    let code = "fn foo<'a, T>(x: &'a T) -> &'a T {}";
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "fn",
                token_type: Keyword
            },
            Token {
                content: "foo",
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
                content: "T",
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
                content: "T",
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
    )
}

#[test]
fn test_array_generic() {
    let code = "let buf: [u8; 32] = [0; 32];";
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "let",
                token_type: Keyword
            },
            Token {
                content: "buf",
                token_type: Identifier
            },
            Token {
                content: ":",
                token_type: Punctuation
            },
            Token {
                content: "[",
                token_type: Punctuation
            },
            Token {
                content: "u8",
                token_type: Type
            },
            Token {
                content: ";",
                token_type: Punctuation
            },
            Token {
                content: "32",
                token_type: Number
            },
            Token {
                content: "]",
                token_type: Punctuation
            },
            Token {
                content: "=",
                token_type: Operator
            },
            Token {
                content: "[",
                token_type: Punctuation
            },
            Token {
                content: "0",
                token_type: Number
            },
            Token {
                content: ";",
                token_type: Punctuation
            },
            Token {
                content: "32",
                token_type: Number
            },
            Token {
                content: "]",
                token_type: Punctuation
            },
            Token {
                content: ";",
                token_type: Punctuation
            }
        ]
    )
}

#[test]
fn test_associated_type() {
    let code = "<Vec<u8> as SomeTrait>::Item";
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "<",
                token_type: Punctuation
            },
            Token {
                content: "Vec",
                token_type: Type
            },
            Token {
                content: "<",
                token_type: Punctuation
            },
            Token {
                content: "u8",
                token_type: Type
            },
            Token {
                content: ">",
                token_type: Punctuation
            },
            Token {
                content: "as",
                token_type: Keyword
            },
            Token {
                content: "SomeTrait",
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
                content: "Item",
                token_type: Type
            }
        ]
    );
}

#[test]
fn test_where_clause() {
    let code = "fn foo<T>() where T: Ord {}";
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "fn",
                token_type: Keyword
            },
            Token {
                content: "foo",
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
                content: ")",
                token_type: Punctuation
            },
            Token {
                content: "where",
                token_type: Keyword
            },
            Token {
                content: "T",
                token_type: Type
            },
            Token {
                content: ":",
                token_type: Punctuation
            },
            Token {
                content: "Ord",
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
    )
}

#[test]
fn test_const_generic() {
    let code = "struct ArrayVec<T, const N: usize> {}";
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "struct",
                token_type: Keyword
            },
            Token {
                content: "ArrayVec",
                token_type: Type
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
                content: ",",
                token_type: Punctuation
            },
            Token {
                content: "const",
                token_type: Keyword
            },
            Token {
                content: "N",
                token_type: Type
            },
            Token {
                content: ":",
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
                content: "{",
                token_type: Punctuation
            },
            Token {
                content: "}",
                token_type: Punctuation
            }
        ]
    )
}
