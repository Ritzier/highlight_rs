#[test]
fn test_at_rules() {
    let code = "@import @font-face";
    assert_tokens_skip_whitespace!(
        code,
        [
            CssToken {
                content: "@import",
                kind: AtRules
            },
            CssToken {
                content: "@font-face",
                kind: AtRules
            }
        ]
    );
}
