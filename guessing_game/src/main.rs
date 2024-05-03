use std::{cmp::Ordering, io};

use colored::Colorize;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Secret number: {}", secret_number);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Faliend ro read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            _ => continue, 
        };

        println!("You guessed: {}",  guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            },
            Ordering::Greater => println!("{}", "Too big!".red()),
        }
    }
}
