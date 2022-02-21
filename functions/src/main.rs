fn main() {
    println!("Hello, world!");

    let (two, three) = another_function(24, 'w');
    println!("Two: {}, Three: {}", two, three);
}

fn another_function(x: i32, c: char) -> (i32, i32) {
    println!("Another function. {}{}", x, c);
    (2, 3) // Or return (2, 3);
}
