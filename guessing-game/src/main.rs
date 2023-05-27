use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("You're playing Guess the number");
    let mut upper_bound: String = String::new();

    println!("Enter an upper bound number to guess (inclusive)");
    io::stdin()
        .read_line(&mut upper_bound)
        .expect("Failed to read line");

    let upper_bound: u32 = match upper_bound.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, using 100 as default");
            100
        }
    };
    let secret_number: u32 = rand::thread_rng().gen_range(1..=upper_bound);
    // println!("DEBUG: secret_number - {secret_number}");

    loop {
        println!("Input your guess.");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
