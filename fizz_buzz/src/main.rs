use std::io::{self, Write};

// Defining a trait for the different fizzbuzz functions
trait FizzBuzz {
    fn exec(&self, start: i32, stop: i32);
}

struct FizzBuzzNewline;
struct FizzBuzzComma;

// FizzBuzz implementation that prints each number on a new line
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

// FizzBuzz implementation that prints each number separated by a comma
impl FizzBuzz for FizzBuzzComma {
    fn exec(&self, start: i32, stop: i32) {
        let mut is_fb: bool;

        for num in start..stop+1 {
            is_fb = false;

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

fn handle_name() -> String {
    let mut name;

    println!("What is your name?");
    // This loop ensures that the user enters a proper ASCII name string
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

    return name;
}

fn handle_style() -> String {
    let mut style;

    println!("Do you want the output to be printed on a single comma-separated line (c), or each number on its own line (n)?");
    // This loop ensures that the user enters `c` or `n` to indicate the style that the
    // numbers should be printed
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

    return style;
}

fn handle_play_again() -> bool {
    let mut play_again;

    loop {
        play_again = String::new();
        println!("\nDo you want to play again?");
        print!("[y / n]: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read input from user");

        match play_again.trim() {
            "n" => {
                println!(); // Print an empty line, just for decoration
                println!("Bye!");
                return false;
            }
            "y" => {
                println!();
                return true;
            }
            _ => {}
        }
    }
}

fn main() {
    let name;
    let mut style;
    let mut stop;
    let mut successful: bool;
    let min: i32 = 1;
    let max: i32 = 1000;

    name = handle_name();
    println!("\nHi, {}!", name);
    println!("Lets play FizzBuzz. I will output the first `n` FizzBuzz numbers.");
    // This loop only breaks when the numbers are printed successfully
    loop {
        style = handle_style();

        // Select the function that will be used, based on the style chosen by the user
        let fizz_buzz: Box<dyn FizzBuzz> = match style.trim() {
            "c" => Box::new(FizzBuzzComma),
            "n" => Box::new(FizzBuzzNewline),
            _ => unreachable!(),
        };

        println!(); // Print an empty line, just for decoration
        // This loop ensures that the user enters a number within the specified range
        loop {
            stop = String::new();
            println!("Please enter a number between {min} and {max} (press Enter to go with 100).");
            print!("> ");
            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin()
                .read_line(&mut stop)
                .expect("Failed to read input from user");

            // Remove all leading and trailing whitespaces
            let trimmed_stop = stop.trim();

            // If user enters nothing, use the default of 100
            if trimmed_stop.is_empty() {
                fizz_buzz.exec(1, 100);
                successful = true;
                break;
            }

            // If the user enters something, try to parse it to an integer
            match trimmed_stop.parse::<i32>() {
                Ok(stop) => {
                    if (1..1000).contains(&stop) {
                        println!();
                        fizz_buzz.exec(1, stop);
                        successful = true;
                        break;
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

        // If the numbers have been printed successfully, ask the user if they want to play again
        if successful {
            if handle_play_again() {
                continue;
            }
            return;
        }
    }
}
