use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess number app!");

    let secret: u8 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Enter number: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u8 = match guess.trim().parse() {
            Err(_) => {
                println!("You must enter a number!");
                continue;
            }
            Ok(num) => num,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
