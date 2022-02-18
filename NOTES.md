## Code

- `let` defines and may initialize a variable, which is immutable by default
  - `let mut x = 25` declares x to be a mutable integer initialized with the value 25 
  - `let x: u32` declares x to be an immutable `u32`
- `loop` creates an infinite loop 
- `println!` is a _macro_, not a _function_.
  - We know this because it finishes with a `!`
- _trait_ defines an extended notion of an interface which can include
  - functions and methods
  - types
  - constants
- `Result` in many package is a enum with variants `Ok` and `Err`
  - `Ok` contains the result value
  - `Err` contains the error met obtaining the value
- Reassigning a variable in the same scope shadows the previous version instead of replacing it
- Unused imports cause warnings

### Operators

- `..` range definition operator, as in `1..101`.
  - Lower bound is inclusive 
  - Upper bound is exclusive

## Tools
### `cargo` package manager

- packages are called _crates_
- config file is `Cargo.toml`
  - it contains the dependencies and package info
    - `package.edition` is the Rust edition
- lock file is `Cargo.lock` and contains reference version info
- `cargo new --vcs=git` will create a new repo even if already within a repo. Default off.
- `cargo check` verifies syntax but does not build. Often much faster than a build.
- `cargo build` compiles and links
  - builds below `$PWD/target/(debug|release)` 
  - `--release` compiles with optimizations. Compile is slower but code is faster.
- `cargo run` compiles, links and runs
- `cargo doc --open` builds the doc for a crate, including its imports,
  and opens it in the default browser

### Others

- `rustc` compiler
- `rustfmt` formatter

## Formatting

- C-shaped (one true brace)
- indents: 4 spaces, no tabs
- final `;` may sometimes be omitted (as in this example),
  - not recommended

## Project layout (with Cargo)

- `(project)/` contains README, license, config, assets, etc; but no Rust code
- `(project)/src/` contains Rust source code
