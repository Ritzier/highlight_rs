mod pattern;
mod token;
use pattern::*;
pub use token::*;

pub struct CssTokenizer {
    pos: usize,
    chars: Vec<char>,
}

impl CssTokenizer {
    fn new(code: &str) -> Self {
        Self {
            pos: 0,
            chars: code.chars().collect(),
        }
    }

    pub fn tokenize(code: &str) -> Vec<CssToken> {
        CssTokenizer::new(code).tokenize_all()
    }

    fn tokenize_all(&mut self) -> Vec<CssToken> {
        let mut tokens = Vec::new();
        let len = self.chars.len();

        while self.pos < len {
            let code = &self.chars[self.pos..].iter().collect::<String>();
            let mut found = false;

            for (regex, typ) in PATTERNS.iter() {
                if let Some(mat) = regex.find(code) {
                    if mat.start() == 0 {
                        let content = mat.as_str().to_string();

                        match typ {
                            CssTokenKind::Function => match content.find('(') {
                                Some(parent_start) => {
                                    let fname = content[..parent_start].trim_end();
                                    tokens.push(CssToken::new(
                                        fname.to_string(),
                                        CssTokenKind::Function,
                                    ));
                                    tokens.push(CssToken::new(
                                        "(".to_string(),
                                        CssTokenKind::Punctuation,
                                    ));
                                }
                                None => {
                                    tokens.push(CssToken::new(content.clone(), *typ));
                                }
                            },

                            _ => {
                                tokens.push(CssToken::new(content.clone(), *typ));
                            }
                        }

                        self.pos += content.chars().count();
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                // Unrecognized: treat as single default token
                let content = self.chars[self.pos].to_string();
                tokens.push(CssToken::new(content, CssTokenKind::Default));
                self.pos += 1;
            }
        }
        tokens
    }
}
