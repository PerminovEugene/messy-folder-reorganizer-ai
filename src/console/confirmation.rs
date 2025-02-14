use colored::Colorize;
use std::io::{self, Write};

pub fn ask_confirmation(prompt: &str) -> bool {
    print!("{} (y/n): ", prompt.green());
    io::stdout().flush().unwrap(); // Ensure prompt is displayed immediately

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => true,
        "n" | "no" => false,
        _ => {
            println!("Invalid input. Please enter 'y' or 'n'.");
            ask_confirmation(prompt) // Recursively ask again
        }
    }
}
