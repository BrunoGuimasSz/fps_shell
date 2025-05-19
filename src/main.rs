use std::fs::OpenOptions;
use std::io::{self, Write};

pub mod command;

fn main() -> io::Result<()> {
    loop {
        command::print_user_and_directory();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error while reading line");

        let cleaned_input = input.trim().to_string();

        if cleaned_input == "exit" || cleaned_input == "quit" {
            break;
        }
        command_handler(cleaned_input);

        save_command_in_history(input);
    }

    Ok(())
}

fn command_handler(input: String) {
    let token_array: Vec<String> = input
        .split_whitespace()
        .map(|token| token.to_string())
        .collect();

    match token_array[0].as_str() {
        "echo" => command::echo(token_array),
        "cls" => command::clear(),
        "cat" => command::cat(token_array),
        _ => {
            println!("EWrror: command not found: {}", token_array[0]);
        }
    }
}

fn save_command_in_history(input: String) {
    let history_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("history.fps");

    if let Ok(mut file) = history_file {
        let _ = writeln!(file, "{}", input.trim());
    }
}
