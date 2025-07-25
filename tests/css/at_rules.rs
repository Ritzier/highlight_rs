#[test]
fn test_at_rules() {
    let code = "@import @font-face";
    assert_fail_skip_whitespace!(code);
}
