use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut  counter: u32 = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                counter = counter + 1;
                println!("Too small!");
            },
            Ordering::Greater => {
                counter = counter +1;
                println!("Too big!");
            },
            Ordering::Equal => {
                println!("You win! after {} tryings", counter);
                break;
            }
        }
    }
}