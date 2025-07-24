use super::*;

/// Identifiers with underscores, digits, or camel case should be valid
#[test]
fn test_basic_identifiers() {
    let code = "foo bar_baz _foobar _ x0 X_0 Y_Z a1b2c3";
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "foo",
                token_type: Identifier
            },
            Token {
                content: "bar_baz",
                token_type: Identifier
            },
            Token {
                content: "_foobar",
                token_type: Identifier
            },
            Token {
                content: "_",
                token_type: Identifier
            },
            Token {
                content: "x0",
                token_type: Identifier
            },
            Token {
                content: "X_0",
                token_type: Type
            },
            Token {
                content: "Y_Z",
                token_type: Type
            },
            Token {
                content: "a1b2c3",
                token_type: Identifier
            }
        ]
    );
}

/// Identifiers surrounded by punctuation
#[test]
fn test_identifiers_with_punctuation() {
    let code = "(foo),_bar;";
    assert_tokens!(
        code,
        [
            Token {
                content: "(",
                token_type: Punctuation
            },
            Token {
                content: "foo",
                token_type: Identifier
            },
            Token {
                content: ")",
                token_type: Punctuation
            },
            Token {
                content: ",",
                token_type: Punctuation
            },
            Token {
                content: "_bar",
                token_type: Identifier
            },
            Token {
                content: ";",
                token_type: Punctuation
            }
        ]
    );
}

/// Keywords should not be parsed as identifier
#[test]
fn test_keywords_vs_identifiers() {
    let code = "let let_ letx";
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "let",
                token_type: Keyword
            },
            Token {
                content: "let_",
                token_type: Identifier
            },
            Token {
                content: "letx",
                token_type: Identifier
            }
        ]
    );
}

/// Uppercase types and lowercase identifiers
#[test]
fn test_types_and_identifiers() {
    let code = "String str Vec vec";
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "String",
                token_type: Type
            },
            Token {
                content: "str",
                token_type: Type
            },
            Token {
                content: "Vec",
                token_type: Type
            },
            Token {
                content: "vec",
                token_type: Identifier
            }
        ]
    );
}

/// Identifier and function call (should distinguish)
#[test]
fn test_identifier_and_function() {
    let code = "f f()";
    assert_tokens!(
        code,
        [
            Token {
                content: "f",
                token_type: Identifier
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "f",
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

/// Long/complex identifier names
#[test]
fn test_complex_identifiers() {
    let code = "this_is_very_long_and_123_complex_identifier";
    assert_tokens!(
        code,
        [Token {
            content: "this_is_very_long_and_123_complex_identifier",
            token_type: Identifier
        }]
    );
}
