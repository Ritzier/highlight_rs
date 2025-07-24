use highlight_rs::*;

#[test]
fn test_unsupported_languge() {
    let result = Language::from("wtf");
    assert!(result.is_err());
    assert_eq!(result, Err(Error::Unsupported("wtf".to_string())));
}

#[test]
fn test_supported_languge() {
    let supported_language = [
        // Rust
        ("rust", Language::Rust),
        ("RUST", Language::Rust),
        ("rs", Language::Rust),
        ("RS", Language::Rust),
        // Css
        ("css", Language::Css),
        ("CSS", Language::Css),
    ];
    for (lang, language) in supported_language {
        let result = Language::from(lang);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), language)
    }
}
