use std::io::{self, stdin, stdout, Write};
use std::env::{current_dir, var,};
use std::fs::OpenOptions;

fn main() -> io::Result<()> {
    loop {
        print_user_and_directory();

        let mut input: String = String::new();

        stdin()
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

fn get_user_name() -> String {
    let username = if cfg!(target_os = "windows") {
        var("USERNAME")
    } else {
        var("USER")
    };

    match username {
        Ok(name) => {
            return name;
        }
        Err(error) => {
            eprintln!("Error: {error}");
        }
    };

    String::from("Unknow user")
}

fn get_current_directory() -> String {
    let current_path = current_dir();

    match current_path {
        Ok(path) => {
            return path.display().to_string();
        }
        Err(error) => eprintln!("Error: {error}"), 
    };

    String::from("Unknow directory")
}

fn print_user_and_directory() {
    let user: String = get_user_name();
    let directory: String = get_current_directory();

    print!("[{user}@{directory}] -> ");
    stdout().flush().unwrap();
}

fn command_handler(input: String) {
    let token_array: Vec<String> = input
        .split_whitespace()
        .map(|token| token.to_string())
        .collect();

    match token_array[0].as_str() {
        "echo" => echo_command(token_array),
        "cls" => clear_command(),
        _ => {
            println!("Error: command not found: {}", token_array[0]);
        }
    }
}

fn echo_command(token_array: Vec<String>) {
    for i in token_array {
        if i == "echo" {
            continue;
        }
        print!("{i} ");
        stdout().flush().unwrap();
    }
    println!()
}

fn clear_command()
{
    print!("\x1B[2J\x1B[1;1H");
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
