use colored::Colorize;

use super::confirmation::ask_confirmation;

pub fn print_files_not_updated() {
    println!("{}", "🚫 File locations were not updated.".yellow())
}

pub fn ask_for_files_migration() -> bool {
    ask_confirmation(
        "❓ Are you satisfied with the file reorganization plan? Would you like to apply it?",
    )
}
