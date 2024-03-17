# rust_guide
Guide for Rust programming language
## cargo
### `cargo new <project-name>` - start a new project
### `cargo build` - compile code
### `cargo run` - execute code

## in-line comments and Doc comments
### Regular comments which are ignored by the compiler:
- `//` Line comments which go to the end of the line.
- `/*` Block comments which go to the closing delimiter. */
### Doc comments which are parsed into HTML library documentation:
- `///` Generate library docs for the following item.
- `//!` Generate library docs for the enclosing item.

## variables
### 1. variable assignments
- `let myvar = 8;` declare a variable
- `let mut myvar = 8;` declare a variable and make it mutable
- `const MYVAR: <data-type> = 8;` declare a constant that will be persisted for the duration of the program

