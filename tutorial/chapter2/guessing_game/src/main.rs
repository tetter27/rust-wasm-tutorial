use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    // variable is immutable by default
    // mut make it mutable
    let mut guess = String::new();

    io::stdin()  // std::io::stdin()
        .read_line(&mut guess)  // & is reference for indicating that code access one piece of data without needing to copy that data into memory multiple times.
        .expect("Failed to read line");  // processing for Err case.

    println!("You guessed: {guess}");
}


// ToDo: Comparing the Guess to the Secret Number