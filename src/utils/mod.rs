use std::io;

use colored::*;

pub fn read_terminal() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    return input.trim().to_string();
}

fn calculating_proximity_coler(sub_result: u32) -> String {
    match sub_result {
        0 => "You Win".green().to_string(),
        number if (1..5).contains(&number) => "You're almost there".bright_green().to_string(),
        number if (5..=10).contains(&number) => "So Close".yellow().to_string(),
        number if (10..=20).contains(&number) => "Not To Far".bright_red().to_string(),
        _ => "To Fat".red().to_string().bold().to_string(),
    }
}

pub fn comparing_numbers(system_number: u32, user_number: u32) -> bool {
    if system_number == user_number {
        let win = "You Win".green().to_string();
        println!("{}", win);
        return true;
    };

    let subtraction = system_number.abs_diff(user_number);
    //println!("subtraction result {}", subtraction);

    let output_message = calculating_proximity_coler(subtraction);
    println!("{}", output_message);

    false
}
