use std::io;

fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = 0;
    {
        let x = 9;
        println!("Here the value of x is: {x}");
    }
    println!("The value of x is: {x}");
    const SECONDS_IN_DAY: u32 = 60 * 60 * 24;
    println!("here are {SECONDS_IN_DAY} seconds in a day.");

    let spaces = "    ";
    let spaces = spaces.len();
    let smile: char = '😻';
    println!("I am happy with my {smile} which has {spaces} spaces.");

    let tup = (12.3, 'v', false);
    let (x, y, z) = tup;
    println!("The {y}setup filesize is {x} and that is a {z} file.");

    choose()
}

fn choose() {
    let _arr = [1, 2, 3, 4, 5];

    println!("Pls enter a number from 1 to 5.");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to get input!");

    let choice_num: i32 = choice.trim().parse().expect("Should be a number between 1 to 5");
    println!("{choice_num}")
}
