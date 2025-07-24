use super::*;

#[test]
fn test_rust_numbers() {
    let code = "\
        123
        0xFF
        0o755
        0b1010
        3.14
        1e5
        3.14e-2
        42u8
        999_999
        6.022e23f64
        1_2_3.4_5_6
    ";

    assert_tokens_skip_whitespace!(
        code,
        [
            Token {
                content: "123",
                token_type: Number
            },
            Token {
                content: "0xFF",
                token_type: Number
            },
            Token {
                content: "0o755",
                token_type: Number
            },
            Token {
                content: "0b1010",
                token_type: Number
            },
            Token {
                content: "3.14",
                token_type: Number
            },
            Token {
                content: "1e5",
                token_type: Number
            },
            Token {
                content: "3.14e-2",
                token_type: Number
            },
            Token {
                content: "42u8",
                token_type: Number
            },
            Token {
                content: "999_999",
                token_type: Number
            },
            Token {
                content: "6.022e23f64",
                token_type: Number
            },
            Token {
                content: "1_2_3.4_5_6",
                token_type: Number
            },
        ]
    );
}
