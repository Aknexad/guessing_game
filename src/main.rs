use colored::Colorize;
mod utils;

fn main() {
    println!(
        "
 ╔═══════════════════════╗
 ║ 🎯  Number Guesser 🎯 ║
 ╚═══════════════════════╝
"
    );

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
        println!(
            "{} {}",
            Colorize::blue("You Guessed"),
            Colorize::blue(guess.to_string().as_str())
        );

        let result = utils::comparing_numbers(secret_number, guess);

        if result {
            break;
        }

        //match guess.cmp(&secret_number) {
        //    Ordering::Less => println!("{}", Colorize::red("Too Small!")),
        //    Ordering::Greater => println!("{}", Colorize::red("Too Big")),
        //    Ordering::Equal => {
        //        println!("{}", Colorize::green("You WIn!"));
        //        break;
        //    }
        //}
    }
}
