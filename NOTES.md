## Code

- `println!` is a _macro_, not a _function_.
  - We know this because it finishes with a `!` 

## Tools

- `cargo` package manager
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
