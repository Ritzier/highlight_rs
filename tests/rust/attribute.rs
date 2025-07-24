use super::*;

/// Single-line outer attribute
#[test]
fn test_rust_attributes_simple() {
    assert_tokens!(
        "#[test]",
        [Token {
            content: "#[test]",
            token_type: Attribute
        }]
    );
}

/// Outer and inner attributes used together
#[test]
fn test_rust_attributes_inner_outer() {
    assert_tokens!(
        "#![feature(never_type)] #[cfg(test)]",
        [
            Token {
                content: "#![feature(never_type)]",
                token_type: Attribute
            },
            Token {
                content: " ",
                token_type: Whitespace
            },
            Token {
                content: "#[cfg(test)]",
                token_type: Attribute
            },
        ]
    );
}

/// Attribute with several arguments in a meta-list (e.g. typical `derive`)
#[test]
fn test_rust_attributes_with_multiple_meta() {
    assert_tokens!(
        "#[derive(Copy, Clone, Debug)]",
        [Token {
            content: "#[derive(Copy, Clone, Debug)]",
            token_type: Attribute
        }]
    );
}

/// Attribute with a key-value argument, such as doc comments.
#[test]
fn test_rust_attributes_with_key_value() {
    assert_tokens!(
        "#[doc = \"test docs\"]",
        [Token {
            content: "#[doc = \"test docs\"]",
            token_type: Attribute
        }]
    );
}

/// Attribute directly preceding a function (checks code right after attribute)
#[test]
fn test_rust_multiple_attributes_and_code() {
    let code = "#[inline]\npub fn f() {}";
    assert_tokens!(
        code,
        [
            Token {
                content: "#[inline]",
                token_type: Attribute
            },
            Token {
                content: "\n",
                token_type: Whitespace
            },
            Token {
                content: "pub",
                token_type: Keyword
            },
            Token {
                content: " ",
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
                content: "}",
                token_type: Punctuation
            }
        ]
    );
}

/// Custom attribute with a namespaced (path) identifier and meta-list
#[test]
fn test_rust_attribute_custom_path() {
    assert_tokens!(
        "#[my_crate::some_attribute(foo = \"bar\")]",
        [Token {
            content: "#[my_crate::some_attribute(foo = \"bar\")]",
            token_type: Attribute
        }]
    );
}
