use highlight_rs::*;

macro_rules! assert_tokens {
    ($code:expr, [ $( Token { content: $content:expr, token_type: $token_type:ident } ),* $(,)? ]) => {{
        let actual = $crate::RustTokenizer::tokenize($code);
        let expected = vec![
            $(
                $crate::Token {
                    content: $content.to_string(),
                    token_type: $crate::TokenType::$token_type,
                }
            ),*
        ];
        assert_eq!(actual.len(), expected.len(), "Token lengths differ");
        for (i, (got, want)) in actual.into_iter().zip(expected.into_iter()).enumerate() {
            assert_eq!(got, want, "Token at index {} does not match", i);
        }
    }};
}

#[allow(unused_macros)]
macro_rules! assert_fail {
    ($code:expr) => {{
        let actual = $crate::RustTokenizer::tokenize($code);
        assert_eq!(actual, []);
    }};
}

macro_rules! assert_tokens_skip_whitespace {
    (
        $code:expr,
        [ $( Token { content: $content:expr, token_type: $token_type:ident } ),* $(,)? ]
    ) => {{
        let actual = $crate::RustTokenizer::tokenize($code)
            .into_iter()
            .filter(|tok| tok.token_type != $crate::TokenType::Whitespace)
            .collect::<Vec<_>>();
        let expected = vec![
            $(
                $crate::Token {
                    content: $content.to_string(),
                    token_type: $crate::TokenType::$token_type,
                }
            ),*
        ];
        assert_eq!(actual.len(), expected.len(), "Token lengths differ");
        for (i, (got, want)) in actual.into_iter().zip(expected.into_iter()).enumerate() {
            assert_eq!(got, want, "Token at index {} does not match", i);
        }
    }};
}

#[allow(unused_macros)]
macro_rules! assert_fail_skip_whitespace {
    ($code:expr) => {{
        let actual = $crate::RustTokenizer::tokenize($code)
            .into_iter()
            .filter(|tok| tok.token_type != $crate::TokenType::Whitespace)
            .collect::<Vec<_>>();
        assert_eq!(actual, []);
    }};
}

mod attribute;
mod comment;
mod function;
mod generic;
mod html;
mod identifier;
mod keywords;
mod lifetime_and_generic;
mod macros;
mod number;
mod operator;
mod punctation;
mod rust_tokenizer;
mod string;
mod types;
mod view_macro;
