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
- Unused variables cause warnings
  - Prefixing a variable identifier with `_` makes the compiler not warn if the
    variable is not used.
- Assignement is called _binding a value to a name_
- NLL: non-lexical lifetime: since Rust 1.31, the borrow checker can determine
  when a reference is no longer used, and release the borrowing,
  before the end of the scope

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
    - Literals like `b'x'` express the `u8` representation of the character `x`,
      which MUST be ASCII: `b'é` doesn't work.
- Compound: tuple
  - Fixed size
  - Heterogeneous, the type of members is part of the tuple type, in order
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
- Slices: a `*struct{ ptr, len, capacity}` (vs `struct {ptr, offset, len}` in Go)
  describing a view into a piece of data
  - Slices of `String` can be created using `..` as the slicing operator,
    as in `let hello = &s[0..5]`.
  - Slice indexes are expressed in bytes, but String slicing MUST happen on
     valid UTF-8 character boundaries.
  - In `let s = "Hello"`, the string literal `s` is a `&str`: an immutable
    reference which is a slice pointing to the compiled-in data making up `Hello`.
  - Slices also apply to arrays, in the same way. 

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
  - As with Go `:`, either or both limits can be omitted.
- Arithmetic: `+|-|*|/|%`
- `&` produces a _reference_, not a pointer.
- monadic `*` dereferences a reference 

Reference: https://doc.rust-lang.org/book/appendix-02-operators.html

### If

- Traditional
```rust
fn main() {
    if expr {
        /* braces required */
    } else if expr {
        /* braces required except for `else if` */
        /* else is optional */
    } else {
        /* braces required */
        /* else could be on another line */
    }
}
```
- The `else` does not have to be on the same line as the previous closing brace
- Ternary because if is an expression: `let x = if condition { y } else { z }`
  - Both arms must evaluate to a value of the same type

### Loops: `loop` / `break` / labels / `while`

- labels are lines starting with a quote and ending with a colon, like `'id:`
- `loop` creates an infinite loop
- `break` exits it. 
  - May specify a target label, including the initial quote.
  - May specify a "return" value, because loops are expressions too
- `continue` skips to next iteration
- `while` evaluates to the unit value, so useless as an expression
- `for x in iterable { /* braces required */ }`

## Ownership

- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped, calling the `drop`
  method provided by the `Drop` trait. Related to the RAII pattern.
- https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
- The limitation of "one owner" ensures `drop` will never be called more than
  once, by the resource owner only.

### Shallow copy vs move

- In this example, the `s2 = s1` assignement does not perform a shallow copy, 
  but a _move_: ownership of the internal `str` pointer in the `String` is transferred
  to `s2`, and `s1` can no longer be used.

```rust
fn invalid() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}
```
- One consequence is that deep copies never happe, so automatic copies are cheap.
- This also happens on function calls (but not macro expansion like println!) and returns
- On function exit, ownership of the local heap vars and arguments is dropped
  See `ownership_demo`, `takes_ownership`, `makes_copy` in `ownership/src/main.rs`
- One way to avoid it is the use of references (`&foo` is a reference not a pointer)
- Moves 
  - can be prevented by implementing the `Copy` trait to allow values to remain
  valid after assignment to another variables
  - The trait is present for types allocated on the stack (integers, etc)
  - `Copy` cannot be added on types implementing the `Drop` trait.
- Example valid types for `Copy`
  - all integer, bool, float, char types
  - tuples only containing types implementing `Copy`

### Cloning

Conversely, `clone` performs a deep copy and does not transfer ownership.

```rust
fn valid() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

It means `clone` is a true deep copy, and can be expensive.

### References and borrowing

- A _reference_ is 
  - like a pointer in that it’s an address we can follow to access data
  - but the data stored at that address is owned by some other variable
  - this implies that the data is always valid
- Creating a reference (with `&`) is called _borrowing_.
  - Immutable by default
  - Mutable references are created by `&mut foo`, assuming `foo` is mutable
  - Only one mutable reference may exist at a time for a given piece of data
  - An immutable and a mutable reference to the same data cannot coexist
    (users of the immutable reference could get the data changed while they
    assume it is immutable)
- Because a function call using a reference passes a borrowed value,
  the argument will not cause a drop because it does not have ownershi

The restriction on multiple mutable references helps prevent _data races_ at
compile time. Data races happen when
- two or more pointers point to the same data at the same time
- at least one of them attempts to write the data
- there is no synchronization mechanism between them.

Dangling references are also impossible, as in this example:
```rust
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```
That code doesn't compile, because `s` no longer exists after the end of the function,
so the reference would be dangling.

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
