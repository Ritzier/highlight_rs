use highlight_rs::html_escape;

#[test]
fn test_html_secape() {
    assert_eq!(html_escape("hello"), "hello");
    assert_eq!(html_escape("&<>\"'"), "&amp;&lt;&gt;&quot;&#39;");
    assert_eq!(html_escape("<script>"), "&lt;script&gt;")
}

#[test]
fn test_html_escape_basic() {
    assert_eq!(
        html_escape(r#"<tag attr="foo&bar">'quote'"#),
        "&lt;tag attr=&quot;foo&amp;bar&quot;&gt;&#39;quote&#39;"
    );
}

#[test]
fn test_html_escape_amp() {
    assert_eq!(html_escape("&"), "&amp;");
}

#[test]
fn test_html_escape_quote() {
    assert_eq!(html_escape(r#""'"#), "&quot;&#39;");
}

#[test]
fn test_html_escape_sequence() {
    let original = r#"5 < 10 && a > 3"#;
    let escaped = html_escape(original);
    assert_eq!(escaped, "5 &lt; 10 &amp;&amp; a &gt; 3");
}
