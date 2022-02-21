fn if_demo() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn ternary_demo() {
    const CONDITION: bool = true;
    // Ternary
    let x = if CONDITION { 5 } else { 6 };
    println!("x: {}, inline: {}", x, if CONDITION { 5 } else { 6 });
}

fn loop_demo() {
    // Loop demo
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}

fn while_demo() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_demo() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

    let range = 1..4;
    println!("{:?}", range  );
    for i in range.rev() {
        println!("{}", i)
    }
}

fn main() {
    if_demo();
    ternary_demo();
    loop_demo();
    while_demo();
    for_demo();
}
