use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    let secret_num = rand::thread_rng().random_range(1..=100);

    println!("The secret number is: {}", secret_num);
    println!("Guess the number!");

    loop {
        let mut guess = String::new();

        println!("Please enter your guess number: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input!");
        io::stdout().flush().unwrap(); 

        let parsed = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue; 
            }
        };
    
        println!("You guessed: {}", parsed); 
        match parsed.cmp(&secret_num) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            },
        }
    }
    
}
