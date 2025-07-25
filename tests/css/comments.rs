#[test]
fn test_comments() {
    let code = "/* comments */";
    assert_tokens!(
        code,
        [CssToken {
            content: "/* comments */",
            kind: Comment
        }]
    );
}
