use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // We generate a number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // We print this random generated number
    // println!("The secret number is: {}", secret_number);

    // We loop forever
    loop {
        println!("Please input your guess.");

        // Create a mutable variable initialize with an empty string
        // Note that it's a string
        let mut guess = String::new();

        // Read the user's input
        // If we can't read the line the program
        // will exit with the message 'Failed to read line'
        // We put the result in guess by passing a reference.
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Rust allows us to shadow the guess variable,
        // now it's a u32 (number).
        // If parse return Result::Ok then we returns the parsed value; otherwise we skip the rest of the loop.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // We test the guess value with the user's value.
        // match seems to work like a switch case.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // We quit the loop
                break;
            },
        }
    }
}
