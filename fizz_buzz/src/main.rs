use std::io;

fn main() {
    let mut name = String::with_capacity(20);

    println!("What is your name?");
    print!(">>> ");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input from user");

    println!("Hi, {name}!");
    println!("I am your FizzBuzz assistant. Enter a number between 2 and 1000.");
    fizz_buzz(0, 50);
}

fn fizz_buzz(start: i32, stop: i32) {
    let mut is_fb: bool;

    for num in start..stop {
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
