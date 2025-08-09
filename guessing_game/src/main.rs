use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the number");
    let secret = rand::rng().random_range(1..=100);
    println!("Secret : {secret}");
    println!("Please input your guess");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32= guess.trim().parse().expect("please enter no");
    println!("You guessed : {guess}");
    match guess.cmp(&secret){
        Ordering::Less => println!("guessed low"),
        Ordering::Greater => println!("guessed high"),
        Ordering::Equal => println!("good"),
    }
}
