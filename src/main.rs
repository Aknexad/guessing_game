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

    println!(
        "{}",
        Colorize::truecolor(
            "
 ╔═══════════════════════╗
 ║  🎯  Color Guide 🎯   ║
 ╚═══════════════════════╝
",
            150,
            0,
            237
        )
    );

    println!("{}", Colorize::green("██████████ => Win"));
    println!("{}", Colorize::bright_green("██████████ => less then 5"));
    println!("{}", Colorize::yellow("██████████ => less then 10"));
    println!("{}", Colorize::bright_red("██████████ => less then 20"));
    println!("{}", Colorize::red("██████████ => more then 20"));
    let mut score = 0;
    let secret_number = rand::random_range(1..=101);
    //println!("secret_number is {}", secret_number);
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

        score += 1;
        if result {
            break;
        }
    }

    println!("Your Score is {}", score);
}
