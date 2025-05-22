use std::io::{self, Write};
use std::{env, fs};
use std::path::Path;
use std::fs::File;

fn get_user_name() -> String {
    let username = if cfg!(target_os = "windows") {
        env::var("USERNAME")
    } else {
        env::var("USER")
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
    let current_path = env::current_dir();

    match current_path {
        Ok(path) => {
            return path.display().to_string();
        }
        Err(error) => eprintln!("Error: {error}"), 
    };

    String::from("Unknow directory")
}

pub fn print_user_and_directory() {
    let user: String = get_user_name();
    let directory: String = get_current_directory();

    print!("[{user}@{directory}] -> ");
    io::stdout().flush().unwrap();
}

pub fn echo(token_array: Vec<String>) {
    for i in token_array {
        if i == "echo" {
            continue;
        }
        print!("{i} ");
        io::stdout().flush().unwrap();
    }
    println!()
}

pub fn clear()
{
    print!("\x1B[2J\x1B[1;1H");
}

pub fn cat(token_array: Vec<String>) {
    if token_array.len() < 2 {
        println!("Error: Type the file path");
        return;
    }
    let file_path = &token_array[1];

    match fs::read_to_string(file_path) {
        Ok(file_content) => {
            print!("{}", file_content);
            io::stdout().flush().unwrap();
        },
        Err(error) => eprintln!("Error: File {} don't exist: {}", file_path, error)
    };
}

pub fn cd(token_array: Vec<String>) {
    let path = Path::new(&token_array[1]);

    match env::set_current_dir(path) {
        Ok(_path) => (),
        Err(error) => eprintln!("Error: Directory not found: {}", error),
    }
}

pub fn ls(token_array: Vec<String>) {
    let dir = if token_array.len() > 1 {
        &token_array[1]
    } else {
        "."
    };

    let path = fs::read_dir(dir).unwrap();

    for entry in path {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        println!("{}", file_name.to_string_lossy());
    }
}

pub fn touch(token_array: Vec<String>) {
    let _file = File::create(&token_array[1]).unwrap();
}
