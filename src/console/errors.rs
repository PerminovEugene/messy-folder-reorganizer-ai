use colored::Colorize;

use crate::errors::app_error::AppError;

pub fn print_app_error(err_title: &str, e: AppError) {
    eprintln!("{}:  {}", err_title.red(), e);
}

pub fn print_error_message(message: String) {
    eprintln!("❌ {}", message.red());
}

pub fn print_error_to_same_string() {
    println!("{}", " Error".red(),);
}
