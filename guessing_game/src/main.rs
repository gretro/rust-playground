use std::{io, cmp};

use rand::Rng;

fn main() {
    println!("Guess the number");
    let to_guess = generate_random_number(0, 100);
    let mut is_game_over = false;

    while !is_game_over {
        println!("What is your guess?");
    
        let mut raw_guess = String::new();
        io::stdin()
            .read_line(&mut raw_guess)
            .expect("Failed to read the line");

        let maybe_guess = raw_guess.trim_end().parse::<u8>();
        match maybe_guess {
            Ok(guess) => {
                if guess > 100 {
                    println!("Input must be a number between 0 and 100. What is your guess?");
                    continue;
                }
        
                match to_guess.cmp(&guess) {
                    cmp::Ordering::Less => {
                        println!("The number to guess is less than {guess}")
                    },
                    cmp::Ordering::Equal => {
                        println!("Correct! The number was {to_guess}");
                        is_game_over = true
                    },
                    cmp::Ordering::Greater => {
                        println!("The number to guess is greater than {guess}")
                    }
                }
            },
            Err(ref e) => {
                println!("Could not parse the value. Did you enter a number? {raw_guess} {e}")
            },  
        }
    }

}

fn generate_random_number(min: u8, max: u8) -> u8 {
    let mut rng = rand::thread_rng();

    let generated = rng.gen_range(min..=max);
    return generated;
}