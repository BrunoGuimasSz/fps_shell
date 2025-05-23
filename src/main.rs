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

        input = input.trim().to_string();

        if input == "exit" {
            break;
        }
        command_handler(&input);

        save_command_in_history(input);
    }

    Ok(())
}

fn command_handler(input: &str) {
    let token_vec: Vec<String> = input
        .split_whitespace()
        .map(|token| token.to_string())
        .collect();

    let arg: &Option<&str> = &token_vec
        .iter()
        .find(|token| token.starts_with('-'))
        .map(|s| s.as_str());

    match token_vec[0].as_str() {
        "echo" => command::echo(token_vec),
        "cls" => command::clear(),
        "cat" => command::cat(token_vec),
        "cd" => command::cd(token_vec),
        "ls" => command::ls(token_vec.clone(), arg),
        "touch" => command::touch(token_vec),
        "rm" => command::rm(token_vec),
        _ => {
            println!("Error: command not found: {}", token_vec[0]);
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
