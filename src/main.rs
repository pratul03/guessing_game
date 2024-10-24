use core::num;
use std::{cmp::Ordering, io};

use colored::Colorize;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_num: i32 = rand::thread_rng().gen_range(1..101);
    println!("Secret number is: {}", secret_num);

    loop {
        println!("please input your guess.");

        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line!");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed : {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Equal => {
                println!("{}", "You guessed it right!".green());
                break;
            }
            Ordering::Greater => println!("{}", "Too big".red()),
        }
    }
}
