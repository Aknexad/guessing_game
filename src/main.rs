use std::cmp::Ordering;
mod utils;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::random_range(1..=101);
    println!("secret_number is {}", secret_number);
    loop {
        println!("Please input your guess.");
        let user_guess = utils::read_terminal();

        let guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a Number");
                continue;
            }
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
