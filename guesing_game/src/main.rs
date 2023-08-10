use std::{io, cmp::Ordering};

use rand::Rng;

fn main() {
    println!("############# We're going to play a guessing game. ##############");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}.");

    loop {
        println!("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
        println!("Pls enter a number between 1 and 100.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.contains("quit") {
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(numb) => numb,
            Err(_) => {
                println!("Pls enter a valid number, only digits");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Correct");
                break;
            }
        }
    }
}
