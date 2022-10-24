use std::io;
use rand::Rng;
// use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Incorrect type!");
                continue
            },
        };

    println!("You guessed: {guess}");

    if guess < secret_number {
        println!("Too Small");
        continue;
    }

    if guess > secret_number {
        println!("Too big");
        continue;
    }

    if guess == secret_number {
        println!("You won!");
        break;
    }

        // match guess.cmp(&secret_number) {
        //     Ordering::Less => println!("Too small!"),
        //     Ordering::Greater => println!("Too big!"),
        //     Ordering::Equal => {
        //         println!("You win!");
        //         break;
        //     },
        // }
    }
}