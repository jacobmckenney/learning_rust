use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let to_guess: u32 = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Guess a number: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Not a valid value!");
                continue;
            }
        };

        match guess.cmp(&to_guess) {
            Ordering::Less => println!("Higher!"),
            Ordering::Greater => println!("Lower!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
