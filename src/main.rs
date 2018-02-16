extern crate rand;

use std::io::stdin;

fn main() {
    println!("Hello, world");
    guessing_game()
}

fn guessing_game() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess)
}