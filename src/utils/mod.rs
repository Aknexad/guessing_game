use std::io;

pub fn read_terminal() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    return input.trim().to_string();
}
