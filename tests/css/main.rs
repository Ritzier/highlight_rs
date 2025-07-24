use highlight_rs::*;

macro_rules! assert_tokens {
    ($code:expr, [ $( Token { content: $content:expr, token_type: $token_type:ident } ),* $(,)? ]) => {{
        let actual = $crate::CssTokenizer::tokenize($code);
        let expected = vec![
            $(
                $crate::CssToken {
                    content: $content.to_string(),
                    kind: $crate::CssTokenKind::$token_type,
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
        let actual = $crate::CssTokenizer::tokenize($code);
        assert_eq!(actual, []);
    }};
}

macro_rules! assert_tokens_skip_whitespace {
    (
        $code:expr,
        [ $( Token { content: $content:expr, token_type: $token_type:ident } ),* $(,)? ]
    ) => {{
        let actual = $crate::CssTokenizer::tokenize($code)
            .into_iter()
            .filter(|tok| tok.kind != $crate::CssTokenKind::Whitespace)
            .collect::<Vec<_>>();
        let expected = vec![
            $(
                $crate::CssToken {
                    content: $content.to_string(),
                    kind: $crate::CssTokenKind::$token_type,
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
        let actual = $crate::CssTokenizer::tokenize($code)
            .into_iter()
            .filter(|tok| tok.token_type != $crate::TokenType::Whitespace)
            .collect::<Vec<_>>();
        assert_eq!(actual, []);
    }};
}

mod comments;
mod full;
mod keywords;
