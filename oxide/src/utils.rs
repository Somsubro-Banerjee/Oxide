use colored::*;
use std::io::{self, Write};

pub fn ask(prompt: &str) -> String {
    print!("{} ", prompt.bright_yellow());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn ask_optional(prompt: &str) -> Option<String> {
    let response = ask(prompt);
    if response.is_empty() {
        None
    } else {
        Some(response)
    }
}

pub fn ask_with_options(prompt: &str, options: &[&str]) -> String {
    println!("{}", prompt.bright_yellow());
    for (i, option) in options.iter().enumerate() {
        println!("{}: {}", i + 1, option);
    }

    loop {
        let response = ask("Enter your choice:");
        if let Ok(index) = response.parse::<usize>() {
            if index > 0 && index <= options.len() {
                return options[index - 1].to_string();
            }
        }
        println!("{}", "Invalid choice. Try again.".red());
    }
}
