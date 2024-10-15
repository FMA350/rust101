use core::num;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the correct number!");
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Input: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small baby"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Just right");
                break;
            }
        }
    }
}
