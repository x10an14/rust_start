extern crate rand;

use std::io;
use std::io::Write;
use std::cmp::Ordering;


fn receive_number_from_stdin(print_string: &str) -> i32 {
    use std::process;

    print!("{}: ", print_string);
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_)  => {
            let guess = input.trim().to_string();
            println!("\tReceived '{}' as your input string.", guess);
            if guess.to_lowercase().starts_with("q") ||
               guess.to_lowercase().starts_with("exit") {
                println!("\nQuitting game");
                process::exit(0);
            } else {
                println!("\tUnable to parse '{}'! Try again!", input.trim());
                return receive_number_from_stdin(print_string);
            }
        },
    };

    number
}


fn guess_random_number(to: i32, from: i32) -> bool {
    use rand::Rng;

    let secret_number = rand::thread_rng().gen_range(to, from);
    loop {
        let guess = receive_number_from_stdin("Please input your guess");
        print!("\tYou guessed: {} => ", guess);
        io::stdout().flush().ok().expect("Could not flush stdout");

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("Spot on! You won! =DDD");
                break;
            },
        }
    }
    println!("\tThe secret number was: {}\n", secret_number);
    true
}


fn main() {
    println!("Guess the number(s)!");
    println!("--------------------\n");

    let min_number;
    let max_number;

    // Get a minimum and a maximum number from stdin
    loop {
        let temp1 = receive_number_from_stdin("Specify minimum number");
        let temp2 = receive_number_from_stdin("Specify maximum number");
        match temp1.cmp(&temp2) {
            Ordering::Less    => {},
            Ordering::Equal   => {},
            Ordering::Greater => {
                println!("\t\
            Please enter a 'minimum number\
            ' no larger than the given 'maximum number'...");
                continue;
            }
        }
        min_number = temp1;
        max_number = temp2;
        break;
    }

    // Main loop
    loop {
        println!("\nGuess the number(!):");
        if guess_random_number(min_number, max_number + 1) == false {
            println!("\nQuitting game");
            break;
        }
    }
}
