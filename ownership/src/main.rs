fn invalid() {
    let s1 = String::from("hello");
    let _s2 = s1;

    // println!("{}, world!", s1); // Wrong: s1 was "moved into" s2
}

fn valid() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2)
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    // (s, length) also works: it's an expression
    // return (s, length) also works: it's an expression too
    return (s, length); // return as a statement works too.
}

fn main() {
    let mut s = String::from("thé");
    s.push_str(" au lait");
    println!("s: \"{}\" ptr: {:?} len: {}", s, s.as_ptr(), s.len());

    valid();
    invalid();

    let (s, l) = calculate_length(s);
    println!("s: {}, l: {}", s, l)
}

fn ownership_demo() {
    let s = String::from("hello"); // s comes into scope

    // s's value moves into the function...
    takes_ownership(s);
    // ... and so is no longer valid here

    let x = 5; // x comes into scope

    // x would move into the function, but i32 is Copy,
    // so it's okay to still use x afterward
    makes_copy(x);
} // Here, x goes out of scope, then s.
// But because s's value was moved, nothingspecial happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called.
// The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens
