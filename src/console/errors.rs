use colored::Colorize;

use crate::errors::app_error::AppError;

pub fn print_app_error(err_title: &str, e: AppError) {
    eprintln!("{}:  {}", err_title.red(), e);
}
