use super::*;

pub fn highlight(input: &str, lang: &str) -> String {
    match Language::from(lang) {
        Ok(lang) => lang.highlight(input),
        Err(_) => html_escape(input),
    }
}

pub fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
