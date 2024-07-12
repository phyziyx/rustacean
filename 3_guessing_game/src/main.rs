use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing the number!");

    // ..= is the inclusive range operator (lower and upper bounds are included)
    // .. is the exclusive range operator (lower bound is included, upper bound is excluded)
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess_count = 0;

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess between 1 and 100:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Based off the example in the book, we used a match statement to compare the guess values
        // I've used that and added my own match statement to allow the user to exit the game
        // Each match statement has an arm (like an if-else statement chain)

        match guess.trim() {
            "exit" | "quit" => {
                println!("Exiting the game!");
                break;
            }
            _ => {
                let guess: u32 = match guess.trim().parse() {
                    Ok(num) => {
                        guess_count += 1;
                        num
                    }
                    Err(_) => continue,
                };

                println!("You guessed: {}", guess);

                // Using the match statement with the std's cmp ordering crate
                // This will compare the guess to the secret number and return an Ordering enum
                // Rust uses arms which help us not miss out on any possible cases

                match guess.cmp(&secret_number) {
                    Ordering::Less => println!("Too small!"),
                    Ordering::Greater => println!("Too big!"),
                    Ordering::Equal => {
                        println!("You win!  It took you {} guesses.", guess_count);
                        break;
                    }
                }
            }
        }
    }
}
