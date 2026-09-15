use std::{cmp::Ordering, io};
use rand::Rng;
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    println!("Input the number: ");

    // variables in rust are immutable by default : use the mut keyword to make them mutable
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess : i32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => println!("You win!"),
    }
}
