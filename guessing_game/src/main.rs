use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::{self, Colorize};

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The super duper secret_number is {}", secret_number);

    //this is something like a while true, or an empty for in Golang (for {...})
    loop {
        println!("Plesase input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Something bad happened");

        //Por que es permitido redeclarar una variable en el mismo scope ???
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Oh, that does not look as a number");
                continue; //Como en python, pasa a la siguiente iteracion.
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Par abajo".blue()),
            Ordering::Greater => println!("{}", "Por arriba".red()),
            Ordering::Equal => {
                println!("{}", "Ese mimo !!!".green());
                break;
            }
        }
    }
}
