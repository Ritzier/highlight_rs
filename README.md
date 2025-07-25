# Retarded Syntax Highlighter

A trash library for tokenize and highlights `Programming Language` syntax:

## Project Structure

```
src
├── errors.rs                # Error types and handling
├── language/                # Language-specific modules
│   ├── css/                 # CSS tokenization
│   │   ├── pattern.rs       # Regex patterns for CSS
│   │   └── token.rs         # Token structs/enums for CSS
│   ├── css.rs               # CSS tokenizer engine
│   ├── mod.rs               # Language abstraction, macros, dispatch
│   ├── rust/                # Rust tokenization
│   │   ├── pattern.rs       # Regex patterns for Rust
│   │   └── token.rs         # Token structs/enums for Rust
│   └── rust.rs              # Rust tokenizer engine
├── lib.rs                   # Library entrypoint, exports, glue
└── utils.rs                 # Utility functions (highlight, html_escape, etc.)
```

## Getting Started

1. Add Cargo dependencies

```toml
[dependencies]
highlight_rs = { git = "https://github.com/ritzier/highlight_rs" }
```

2. Usage

```rust
use highlight_rs::{highlight, Language};

// Highlight as Rust
let rust_code = r#"
fn main() {
    println!("Hello, world!");
}
"#;
let html = highlight(rust_code, "rust");

// Highlight as CSS
let css_code = r#"
:root { --primary: #aaa; }
body { background: var(--primary); }
"#;
let html = highlight(css_code, "css");

// Direct usage, with token types
let tokens = Language::Css.tokenize(css_code);
for token in tokens {
    println!("{}: {:?}", token.to_html(), token);
}
```
