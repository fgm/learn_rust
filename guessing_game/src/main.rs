// cmp::Ordering is a enumeration of Less/Equal/Greater.
use std::cmp::Ordering;
use std::io;

// rang::Rng is a trait.
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess");

        // read_line appends to the passed variable reference, it does not replace it,
        // so we need to reinitialize it on every pass.
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // - This guess will now shadow the previous one, not replace it.
        // - We need to force the type
        // let guess: u32 = guess.trim().parse().expect("Please type an integer!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
