#[test]
fn test_comments() {
    let code = "/* comments */";
    assert_tokens!(
        code,
        [Token {
            content: "/* comments */",
            token_type: Comment
        }]
    );
}
