<<<<<<< HEAD
use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

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
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
=======
mod variables;
mod functions;
mod loops;
mod ownership;
mod borrowing;
mod structs;
mod enums;
mod vectors;
mod strings;
mod option_result;
mod practice;

fn main() {
    println!("Rust Beginner Basics Repository");
    
    variables::run();
    structs::run();
}
>>>>>>> 0e0d91eef73e7c2b038ad461526160c19ed072b1
