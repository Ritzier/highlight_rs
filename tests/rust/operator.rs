/// Tests the tokenization of Rust's range and range-inclusive operators,
#[test]
fn test_range_operators() {
    let code = ".. ..= 0..10 0..=10";
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "..",
                token_type: Operator
            },
            Token {
                content: "..=",
                token_type: Operator
            },
            Token {
                content: "0",
                token_type: Number
            },
            Token {
                content: "..",
                token_type: Operator
            },
            Token {
                content: "10",
                token_type: Number
            },
            Token {
                content: "0",
                token_type: Number
            },
            Token {
                content: "..=",
                token_type: Operator
            },
            Token {
                content: "10",
                token_type: Number
            }
        ]
    )
}

/// Verifies that the tokenizer recognizes all the standard Rust binary and composite operators.
#[test]
fn test_rust_operators() {
    let code = "+ - * / == != <= >= && || -> =>";
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "+",
                token_type: Operator
            },
            Token {
                content: "-",
                token_type: Operator
            },
            Token {
                content: "*",
                token_type: Operator
            },
            Token {
                content: "/",
                token_type: Operator
            },
            Token {
                content: "==",
                token_type: Operator
            },
            Token {
                content: "!=",
                token_type: Operator
            },
            Token {
                content: "<=",
                token_type: Operator
            },
            Token {
                content: ">=",
                token_type: Operator
            },
            Token {
                content: "&&",
                token_type: Operator
            },
            Token {
                content: "||",
                token_type: Operator
            },
            Token {
                content: "->",
                token_type: Operator
            },
            Token {
                content: "=>",
                token_type: Operator
            }
        ]
    )
}

/// Ensures that compound operators (e.g. '+=', '==') are matched before their single-character peers.
#[test]
fn test_operator_precedence() {
    let code = "+= == <= >= != && || -> =>";
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "+=",
                token_type: Operator
            },
            Token {
                content: "==",
                token_type: Operator
            },
            Token {
                content: "<=",
                token_type: Operator
            },
            Token {
                content: ">=",
                token_type: Operator
            },
            Token {
                content: "!=",
                token_type: Operator
            },
            Token {
                content: "&&",
                token_type: Operator
            },
            Token {
                content: "||",
                token_type: Operator
            },
            Token {
                content: "->",
                token_type: Operator
            },
            Token {
                content: "=>",
                token_type: Operator
            }
        ]
    )
}

#[test]
fn test_unary_operators() {
    let code = "-!~&*";
    assert_tokens!(
        code,
        [
            Token {
                content: "-",
                token_type: Operator
            },
            Token {
                content: "!",
                token_type: Operator
            },
            Token {
                content: "~",
                token_type: Operator
            },
            Token {
                content: "&",
                token_type: Operator
            },
            Token {
                content: "*",
                token_type: Operator
            },
        ]
    );
}

#[test]
fn test_assignment_operators() {
    let code = "= += -= *= /= %= ^= &= |= <<= >>=";
    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "=",
                token_type: Operator
            },
            Token {
                content: "+=",
                token_type: Operator
            },
            Token {
                content: "-=",
                token_type: Operator
            },
            Token {
                content: "*=",
                token_type: Operator
            },
            Token {
                content: "/=",
                token_type: Operator
            },
            Token {
                content: "%=",
                token_type: Operator
            },
            Token {
                content: "^=",
                token_type: Operator
            },
            Token {
                content: "&=",
                token_type: Operator
            },
            Token {
                content: "|=",
                token_type: Operator
            },
            Token {
                content: "<<=",
                token_type: Operator
            },
            Token {
                content: ">>=",
                token_type: Operator
            },
        ]
    );
}
