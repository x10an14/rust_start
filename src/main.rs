extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn guess_random_number(to: u32, from: u32) -> bool {
    let secret_number = rand::thread_rng().gen_range(to, from);

    loop {
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => {
                let input = guess.trim().to_string();
                println!("Received '{}' as your input string.", input);
                if input.to_lowercase().starts_with("q") {
                   return false;
                } else {
                    println!("Please type a number!\n");
                    continue;
                }
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
    println!("The secret number was: {}\n", secret_number);
    true
}

fn main() {
    println!("Guess the number(s)!");
    println!("--------------------");

    loop {
        println!("\nGuess the number(!):");
        if guess_random_number(1, 21) == false {
            println!("Quitting game");
            break;
        }
    }
}
