use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Sup, welcome to my lil guessing game");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter a number, I'll tell you if you guessed right");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error, while reading line");

        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue
            }

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            // meaning compare the value of guess to the value contained in secret number
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("it's perfect");
                break;
            }
            Ordering::Greater => println!("too big"),
        }
    }
}
