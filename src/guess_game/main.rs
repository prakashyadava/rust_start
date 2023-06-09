#![allow(dead_code)]
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");
    println!("Input your guess: ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Faild to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number");
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small!"),
        Ordering::Equal => println!("You Win!"),
        Ordering::Greater => println!("Too Big!")
    }

}

