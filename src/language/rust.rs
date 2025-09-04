use crate::regex::*;

mod token;
pub use token::{RustToken, RustTokenKind};
mod pattern;
use pattern::*;

pub struct RustTokenizer {
    patterns: Vec<(Regex, RustTokenKind)>,
    view_patterns: Vec<(Regex, RustTokenKind)>,
    pos: usize,
    chars: Vec<char>,
    skip_unrecognized: bool,
}

impl RustTokenizer {
    pub fn tokenize(input: &str) -> Vec<RustToken> {
        let mut rust_tokenizer = RustTokenizer::new(input);

        rust_tokenizer.tokenize_all()
    }

    fn tokenize_all(&mut self) -> Vec<RustToken> {
        let mut tokens = Vec::new();

        while self.pos < self.chars.len() {
            // White space
            if self.chars[self.pos].is_whitespace() {
                let mut end = self.pos + 1;
                while end < self.chars.len() && self.chars[end].is_whitespace() {
                    end += 1;
                }
                let content = self.chars[self.pos..end].iter().collect::<String>();
                tokens.push(RustToken::new(content, RustTokenKind::Whitespace));
                self.pos = end;
                continue;
            }

            // Detect the "view!" macro call
            if self.match_view_macro() {
                // The macro matcher advances self.pos ‒ see below
                let view_tokens = self.tokenize_view_macro();
                tokens.extend(view_tokens);
                continue;
            }

            // Regular Rust lexing
            let content: String = self.chars[self.pos..].iter().collect();
            let ttoks = self.tokenize_once(&content);
            tokens.extend(ttoks);
        }

        tokens
    }

    fn tokenize_once(&mut self, content: &str) -> Vec<RustToken> {
        let mut tokens = Vec::new();
        let mut found_match = false;

        for (pattern, token_type) in self.patterns.iter() {
            if let Some(mat) = pattern.find(content) {
                if mat.start() == 0 {
                    let content = mat.as_str().to_string();
                    let end = self.pos + content.chars().count();

                    match token_type {
                        RustTokenKind::Function if content.ends_with('(') => {
                            if let Some(mat) = FUNC_NAME.find(&content) {
                                let func_name = mat.as_str(); // just the function name
                                tokens.push(RustToken::new(
                                    func_name.to_string(),
                                    RustTokenKind::Function,
                                ));
                                self.pos += func_name.len(); // Advance only by function name's length
                                found_match = true; // Don't advance by the regex total match length
                                break; // Go back to main loop, rest is handled by normal tokenization
                            }
                        }

                        // For condition '<', '>' TokenType
                        RustTokenKind::Manual => {
                            let left_is_whitespace = if self.pos > 0 {
                                self.chars[self.pos - 1].is_whitespace()
                            } else {
                                false
                            };
                            let right_is_whitespace = self
                                .chars
                                .get(self.pos + 1)
                                .is_some_and(|c| c.is_whitespace());

                            let token_type = if left_is_whitespace && right_is_whitespace {
                                RustTokenKind::Operator
                            } else {
                                RustTokenKind::Punctuation
                            };
                            tokens
                                .push(RustToken::new(self.chars[self.pos].to_string(), token_type));
                            self.pos += 1;
                            found_match = true;
                            break;
                        }

                        token_type => tokens.push(RustToken::new(content, *token_type)),
                    }

                    self.pos = end;
                    found_match = true;
                    break;
                }
            }
        }

        if !found_match {
            if self.skip_unrecognized {
                // Skip unrecognized characters
                self.pos += 1;
            } else {
                // Create default token for unrecognized character
                let content = self.chars[self.pos].to_string();
                tokens.push(RustToken::new(content, RustTokenKind::Default));
                self.pos += 1;
            }
        }

        tokens
    }

    fn match_view_macro(&self) -> bool {
        // match "view!" at self.pos, followed by optional space, followed by '{'
        let remain: String = self.chars[self.pos..].iter().collect();
        let macro_re = Regex::new(r"^view!\s*\{").unwrap();
        macro_re.is_match(&remain)
    }

    fn tokenize_view_macro(&mut self) -> Vec<RustToken> {
        let mut tokens = Vec::new();
        // Match and step over "view!" and spaces, push as macro
        let remain: String = self.chars[self.pos..].iter().collect();
        let macro_re = Regex::new(r"^view!").unwrap();

        if let Some(mat) = macro_re.find(&remain) {
            tokens.push(RustToken::new("view!".to_string(), RustTokenKind::Macro));
            self.pos += mat.end();
            // Skip whitespace after "view!"
            while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
                self.pos += 1;
            }
        } else {
            // Should not happen, fall back
            return tokens;
        }

        // Now should be positioned at '{'
        if self.pos >= self.chars.len() || self.chars[self.pos] != '{' {
            return tokens;
        }
        tokens.push(RustToken::new("{".to_string(), RustTokenKind::Punctuation));
        let open_brace = self.pos;
        let block_end =
            find_balanced_brace_block(&self.chars, open_brace).unwrap_or(self.chars.len() - 1);
        // Step over '{'
        self.pos += 1;

        let mut view_pos = self.pos;
        while view_pos < block_end {
            // Whitespace in HTML block
            if self.chars[view_pos].is_whitespace() {
                let mut next = view_pos + 1;
                while next < block_end && self.chars[next].is_whitespace() {
                    next += 1;
                }
                let content = self.chars[view_pos..next].iter().collect::<String>();
                tokens.push(RustToken::new(content, RustTokenKind::Whitespace));
                view_pos = next;
                continue;
            }
            let slice: String = self.chars[view_pos..block_end].iter().collect();
            let mut matched = false;

            // Try match `view_patternsx`
            for (pat, ttype) in self.view_patterns.iter() {
                if let Some(mat) = pat.find(&slice) {
                    if mat.start() == 0 {
                        let seg = mat.as_str().to_string();
                        view_pos += seg.chars().count();
                        tokens.push(RustToken::new(seg, *ttype));
                        matched = true;
                        break;
                    }
                }
            }

            // Fallback to regular Rust patterns
            if !matched {
                let rest = &self.chars[view_pos..block_end].iter().collect::<String>();
                let saved_pos = self.pos;
                self.pos = view_pos;
                let ttoks = self.tokenize_once(rest);
                // tokens.extend will usually add 1 token
                let new_pos = self.pos;
                // Restore outer self.pos for subsequent macro parsing
                self.pos = saved_pos;

                // Nothing matched, position + 1
                if ttoks.is_empty() {
                    tokens.push(RustToken::new(
                        self.chars[view_pos].to_string(),
                        RustTokenKind::Default,
                    ));
                    view_pos += 1;
                // Push and jump to the position
                } else {
                    // tokens found, advance view_pos
                    tokens.extend(ttoks);
                    view_pos = new_pos;
                }
            }
        }

        // add closing '}'
        tokens.push(RustToken::new("}".to_string(), RustTokenKind::Punctuation));
        self.pos = block_end + 1;

        tokens
    }

    fn new(input: &str) -> Self {
        let view_patterns = vec![
            (HTML_OPEN_TAG.clone(), RustTokenKind::Tag),
            (HTML_CLOSE_TAG.clone(), RustTokenKind::Tag),
            (HTML_SELF_CLOSE_TAG.clone(), RustTokenKind::Tag),
        ];
        let chars = input.chars().collect();

        Self {
            patterns: PATTERNS.clone(),
            view_patterns,
            pos: 0,
            chars,
            skip_unrecognized: false,
        }
    }
}

fn find_balanced_brace_block(chars: &[char], start: usize) -> Option<usize> {
    let mut depth = 0;
    let mut started = false;
    for (i, &ch) in chars.iter().enumerate().skip(start) {
        match ch {
            '{' => {
                depth += 1;
                started = true;
            }
            '}' => {
                if depth > 0 {
                    depth -= 1;
                }
                if depth == 0 && started {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}
