# Retarded Syntax Highlighter

A trash library for tokenize and highlights `Programming Language` syntax:

## Project Structure

```
src
├── errors.rs               # Error types and handling
├── language/               # Language-specific modules
│   ├── css/                # CSS tokenization
│   │   ├── pattern.rs      # Regex patterns for CSS
│   │   └── token.rs        # Token structs/enums for CSS
│   ├── css.rs              # CSS tokenizer engine
│   ├── language.rs         # Language enum and display implementation
│   ├── mod.rs              # Language abstraction, macros, dispatch
│   ├── rust/               # Rust tokenization
│   │   ├── pattern.rs      # Regex patterns for Rust
│   │   └── token.rs        # Token structs/enums for Rust
│   └── rust.rs             # Rust tokenizer engine
├── lib.rs                  # Library entrypoint, exports, glue
└── utils.rs                # Utility functions (highlight, html_escape, etc.)
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

## Supported Languages

| Language | Identifiers  | Features                                                                                    |
| -------- | ------------ | ------------------------------------------------------------------------------------------- |
| **Rust** | `rust`, `rs` | Keywords, types, lifetimes, macros, generics, functions, attributes, HTML in `view!` macros |
| **CSS**  | `css`        | Selectors, properties, at-rules, functions, custom properties, colors, units                |

## CSS Classes

The library generates HTML with CSS classes prefixed with `hl-`:

### Rust Classes

- `hl-keyword` - Keywords (`fn`, `let`, `if`, etc.)
- `hl-type` - Types (`i32`, `String`, `Vec`, etc.)
- `hl-function` - Function calls
- `hl-macro` - Macro invocations (`println!`, `vec!`, etc.)
- `hl-string` - String literals
- `hl-char` - Character literals
- `hl-number` - Numeric literals
- `hl-comment` - Comments
- `hl-lifetime` - Lifetime annotations (`'a`, `&'static`)
- `hl-attribute` - Attributes (`#[derive(Debug)]`)
- `hl-operator` - Operators (`+`, `->`, `::`)
- `hl-punctuation` - Punctuation (`()`, `{}`, `;`)
- `hl-identifier` - Variable names
- `hl-tag` - HTML tags (in `view!` macros)

### CSS Classes

- `hl-keyword` - Keywords (`important`, `inherit`)
- `hl-selector-class` - Class selectors (`.my-class`)
- `hl-selector-id` - ID selectors (`#my-id`)
- `hl-selector-tag` - Tag selectors (`div`, `body`)
- `hl-selector-pseudo` - Pseudo selectors (`:hover`, `::before`)
- `hl-selector-universal` - Universal selector (`*`)
- `hl-property` - CSS properties (`color`, `margin`)
- `hl-function` - CSS functions (`rgb()`, `calc()`)
- `hl-string` - String values
- `hl-number` - Numbers
- `hl-unit` - Numbers with units (`16px`, `100%`)
- `hl-literal` - Color literals (`#fff`, `rgb(255,0,0)`)
- `hl-comment` - Comments
- `hl-operator` - Operators
- `hl-punctuation` - Punctuation

### Example Styling

### CSS Variables (Catppuccin Mocha theme)

```scss
:root {
  /* Base colors */
  --text: #cdd6f4;
  --overlay2: #9399b2;
  --subtext1: #bac2de;

  /* Syntax highlighting colors */
  --mauve: #cba6f7; /* Keywords */
  --blue: #89b4fa; /* Functions, IDs */
  --green: #a6e3a1; /* Strings, attributes */
  --yellow: #f9e2af; /* Types, tag selectors */
  --red: #f38ba8; /* Macros, units */
  --peach: #fab387; /* Numbers, literals */
  --teal: #94e2d5; /* Properties, classes */
  --sky: #89dceb; /* Operators */
  --lavender: #b4befe; /* Lifetimes */
  --sapphire: #74c7ec; /* Universal selector */
}
```

```scss
@use "sass:color";
@use "sass:map";

$highlight-map: (
  "attribute": "green",
  "char": "peach",
  "comment": "overlay2",
  "function": "blue",
  "identifier": "text",
  "keyword": "mauve",
  "lifetime": "lavender",
  "macro": "red",
  "number": "peach",
  "operator": "sky",
  "punctuation": "subtext1",
  "string": "green",
  "tag": "teal",
  "type": "yellow",
  "selector-class": "teal",
  "selector-id": "blue",
  "selector-pseudo": "teal",
  "selector-tag": "yellow",
  "selector-universal": "sapphire",
  "unit": "red",
  "literal": "peach",
  "property": "teal",
);

$font-style-map: (
  "comment": italic,
  "char": bold,
);
@each $token, $color in $highlight-map {
  .hl-#{$token} {
    color: var(--#{$color});

    @if map.has-key($font-style-map, $token) {
      $style: map.get($font-style-map, $token);

      @if $style == bold {
        font-weight: bold;
      } @else if $style == italic {
        font-style: italic;
      }
    }
  }
}
```
