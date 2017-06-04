extern crate rand;

use std::io;


fn receive_number_from_stdin(print_string: &str) -> u32 {
    use std::io::Write;
    use std::process;

    print!("{}: ", print_string);
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let number: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_)  => {
            let guess = input.trim().to_string();
            println!("Received '{}' as your input string.", guess);
            if guess.to_lowercase().starts_with("q") {
                println!("Quitting game");
                process::exit(0);
            } else {
                println!("Unable to parse!\nTry again:");
                return receive_number_from_stdin(print_string);
            }
        },
    };

    number
}


fn guess_random_number(to: u32, from: u32) -> bool {
    use rand::Rng;
    use std::cmp::Ordering;

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
    println!("--------------------\n");

    let min_number = receive_number_from_stdin("Specify minimum number");
    let max_number = receive_number_from_stdin("Specify maximum number");

    loop {
        println!("\nGuess the number(!):");
        if guess_random_number(min_number, max_number + 1) == false {
            println!("Quitting game");
            break;
        }
    }
}
