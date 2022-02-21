## Code

- "Rust is an expression-based language". See example:
```rust
fn main() {
    let y = {
        let x = 3;
        x + 1 // Note: no terminal ";" or it won't work
        // x+1 is an expression, but x+1; is a statement
    };
    println!("{}", y) // 4
}
```
- comments 
  - use `//` idiomatically. 
  - `/* ... */` is also supported
  - commenting is idiomatically above the line, not after its end on the right
- assignement is a statement, not an expression (like Go, unlike C/Ruby)
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
- Unused imports cause warnings
- Assignement is called _binding a value to a name_

### Types

Every value is of a certain _data type_, unlike Go where they may only have a _kind_
in the case of constant expressions.

Types must be known at compile time, and can often be inferred, so writing them
out is not always required.

The type annotation for a variable `: <type>`, like `let x: u32;`

- Scalars
  - Integers: `(i|u)(8|16|32|64|128|size)`. 
    - Default is `i32`
    - The `(i|u)size` variants are sized for the target architecture (not pointer size)
      - mostly useful as indexes into collections
    - Integer overflow 
      - causes _panics_ in dev mode,
      - causes two's complement wrapping in release mode.
      - relying on it is considered an error
      - can be handled by stdlib methods: `wrapping_(op)`, `checked_(op)`, `overflowing_(op)`, `saturate_(op)`.
    - Literals: `nn_nnn`, `0xnn`, `0onn`, `0b1111_0000`, and for `u8` only: `b'A'`
      - May be suffixed by the type, like `23u8`
  - Floating-point: `f(32|64)`, using IEEE754
    - Default is `f64` as it is about as fast as `f32` on current CPUs 
  - Booleans: `bool`
  - Characters: 4-byte Unicode, from `U+0000` to `U+D7FF` and `U+E000` to `E+10FFFF`
    - The `U+D800`..`U+DFFF` range is reserved for surrogate pair halfs,
      which are not characters on their own
- Compound: tuple
  - Fixed size
  - Heterogeneous
  - Literal: `let tup = (value, value, ....)`
  - Explicit variable typing: `let tup: (i32, f64, u8) = (500, 6.4, 1)`
  - Item access
    - Destructuring: `let (x, y, z) = tup;`
    - Indexing: `let n = tup.0; let y = tup.1; let z = tup.2`
  - Empty tuple is `()`. Its type is called _unit type_ and value _unit value_,
    which is the default for tuple expressions without any other value.
- Compound: array
  - Fixed length. For variable length, use a _vector_ which is stdlib, not builtin
  - Homogeneous, indexed from 0 to len-1.
  - On the stack, not on the heap
  - Literal: 
    - Simple, inferred type: `let arr = [1, 2, 3, 4, 5]`
    - Type annotation is optional, includes length: `let arr: [i32; 5] = [1, 2, 3, 4, 5]`
    - Repeated initial value `let arr = [3; 5]` gives `[3, 3, 3, 3, 3]`
  - Out of range indexing panics.

### Variables and constants

- `let` defines and may initialize a variable, which is immutable by default
  - `let mut x = 25` declares x to be a mutable integer initialized with the value 25
  - `let x: u32` declares x to be an immutable `u32`
  - unlike constants, variables can be initialized by a runtime computation
- `const` defines constants, like `const DAY: u32 = 24*60*60;`
  - constants are always immutable
  - the type annotation is required
  - unlike variables, constants can only be initialized by a constant expression
- Reassigning a variable in the same scope
  - shadows the previous version instead of replacing it, see `variables/` example.
  - because it creates a new variable, it allows it to have a different type
  - Unlike Go, a variable may be shadowed in the same scope.

### Functions

- Keyword `fn`
- Parameters are comma-separated, typed with a `:`, like `fn foo(x: i32, c: char)`
- Function calls are expressions
- Return is described by a `-> <type>` annotation, like `fn bar() -> i32`
- To return multiple values, return a tuple and destructure the results
  - stable in a let, like `let (two, three) = another_function(24, 'w');`
  - unstable in an assignment: don't use yet (22B)
- If a function ends with an expression (without the `;` that would make it a statement)
  it returns the value of that expression.
- `return` can also be used.

### Operators

- `..` range definition operator, as in `1..101`.
  - Lower bound is inclusive
  - Upper bound is exclusive
- Arithmetic: `+|-|*|/|%`

Reference: https://doc.rust-lang.org/book/appendix-02-operators.html

### If/then/else



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

- constants are in `UPPER_SNAKE_CASE`
- functions and variables are in `lower_snake_case`
- C-shaped (one true brace)
- indents: 4 spaces, no tabs
- final `;` may sometimes be omitted (as in this example),
  - not recommended

## Project layout (with Cargo)

- `(project)/` contains README, license, config, assets, etc; but no Rust code
- `(project)/src/` contains Rust source code
