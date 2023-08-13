use std::io::{self, Write};

trait FizzBuzz {
    fn exec(&self, start: i32, stop: i32);
}

struct FizzBuzzNewline;
struct FizzBuzzComma;

impl FizzBuzz for FizzBuzzNewline {
    fn exec(&self, start: i32, stop: i32) {
        let mut is_fb: bool;

        for num in start..stop+1 {
            is_fb = false;

            print!("{num} -> ");
            if num % 3 == 0 {
                print!("Fizz");
                is_fb = true;
            }
            if num % 5 == 0 {
                print!("Buzz");
                is_fb = true;
            }
            if !is_fb {
                print!("{num}");
            }
            println!();
        }
    }
}

impl FizzBuzz for FizzBuzzComma {
    fn exec(&self, start: i32, stop: i32) {
        let mut is_fb: bool;

        for num in start..stop+1 {
            is_fb = false;

            // print!("{num} -> ");
            if num % 3 == 0 {
                print!("Fizz");
                is_fb = true;
            }
            if num % 5 == 0 {
                print!("Buzz");
                is_fb = true;
            }
            if !is_fb {
                print!("{num}");
            }
            print!(", ");
        }
        println!();
    }
}

fn main() {
    let mut name;
    let mut style;
    let mut stop;
    let min: i32 = 1;
    let max: i32 = 1000;

    println!("What is your name?");
    loop {
        name = String::new();
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input from user");

        let trimmed_name = name.trim();
        if !trimmed_name.is_empty() && name.is_ascii() {
            name = trimmed_name.to_string();
            break;
        }
        println!("Please enter a valid name.");
    }

    println!("\nHi, {}!", name);
    println!("Lets play FizzBuzz. I will output the first `n` FizzBuzz numbers.");
    loop {
        println!("Do you want the output to be printed on a single comma-separated line, or each number on its own line?");
        loop {
            style = String::new();
            print!("[c / n]: ");
            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin()
                .read_line(&mut style)
                .expect("Failed to read input from user");

            if style.trim() == "c" || style.trim() == "n" {
                break;
            }
        };

        let fizz_buzz: Box<dyn FizzBuzz> = match style.trim() {
            "c" => Box::new(FizzBuzzComma),
            "n" | _ => Box::new(FizzBuzzNewline),
        };

        loop {
            stop = String::new();
            println!("Please enter a number between {min} and {max} (press Enter to go with 100).");
            print!("> ");
            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin()
                .read_line(&mut stop)
                .expect("Failed to read input from user");

            /* Remove all leading and trailing whitespaces */
            let trimmed_stop = stop.trim();

            /* If user enters nothing, use the default of 100 */
            if trimmed_stop.is_empty() {
                fizz_buzz.exec(1, 100);
                return;
            }

            /* If the user enters something, try to parse it to an integer */
            match trimmed_stop.parse::<i32>() {
                Ok(stop) => {
                    if (1..1000).contains(&stop) {
                        println!();
                        fizz_buzz.exec(1, stop);
                        return;
                    }
                    else {
                        println!("Number should be between {min} and {max}!");
                    }
                }
                Err(_) => {
                    println!("Invalid number!");
                }
            }
        }
    }
}
