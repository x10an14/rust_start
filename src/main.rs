extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn guess_random_number(to: u32, from: u32){
    let secret_number = rand::thread_rng().gen_range(to, from);

    loop {
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => {
                println!("Please type a number!\n");
                continue;
            },
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal   => {
                println!("Spot on!\n");
                break;
            },
        }
    }
    println!("The secret number was: {}", secret_number);
}

fn main() {
    println!("Guess the number(s)!");
    println!("--------------------\n");
    
    guess_random_number(1, 20);
}
