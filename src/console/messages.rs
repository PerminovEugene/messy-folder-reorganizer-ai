use std::io::{self, Write};
use std::path::Display;

use colored::Colorize;

use super::confirmation::ask_confirmation;

// Information Messages

pub fn print_parsing_destination_folder() {
    println!("{}", "📂 Parsing destination folders structure...".green());
}

pub fn print_parsing_sources() {
    println!("{}", "📂 Parsing source files...".green());
}

pub fn print_generating_embeddings_for_sources() {
    println!("{}", "🧠 Generating embeddings for source files...".green());
}

pub fn print_looking_for_suitable_destination() {
    println!(
        "{}",
        "📍 Searching for a suitable destination folder for each source file...".green()
    );
}

pub fn print_initial_message(version: &str) {
    println!();
    println!(
        "{} {}",
        "🚀 Messy-folder-reorganizer-ai - Version".green(),
        version
    );
}

pub fn print_migration_plan_saved() {
    println!("{}", "💾 Migration plan has been saved.".green());
    println!();
}

pub fn print_generate_config_file(config_file_path: String) {
    println!(
        "{} {:?}",
        "⚙️ Initialized configuration file:".green(),
        config_file_path
    );
}

pub fn print_source_files_metadata_saved() {
    println!("{}", "💾 Source file metadata has been saved.".green());
}

pub fn print_processing_directory(path: Display) {
    println!("{} {:?}", "🔍 Processing directory:".green(), path);
}

pub fn print_processing_file(file_name: &String) {
    println!("{} {:?}", "📄 Processing file:".blue(), file_name);
}

pub fn print_files_reorganization_done() {
    println!("{}", "✅ File migration completed.".green());
}

pub fn print_move_file(from: Display, to: Display) {
    print!(
        "{} {} {} {} ... ",
        "📦 Moving file".blue(),
        from,
        "to".blue(),
        to
    );
    io::stdout().flush().unwrap(); // flush so next text appears right after
}

pub fn print_done_to_same_string() {
    println!("{}", "Done".green());
}

pub fn print_creating_dest_embeddings() {
    println!(
        "{}",
        "🗂️ Creating embeddings for the destination folder structure...".green()
    );
}

pub fn print_clustering_unknown_vectors() {
    println!("{}", "🔢 Clustering unidentified vectors...".green());
}

pub fn print_asking_llm_for_new_folder_names() {
    println!(
      "{}", 
      "🤖 Requesting the LLM to generate folder names for clustered files... (This might take some time, please be patient.)".blue()
    );
}

pub fn print_saving_dest_embeddings() {
    println!(
        "{}",
        "💾 Saving destination embeddings to the database...".green()
    );
}

pub fn print_file_renamed(old_name: &String, new_name: String) {
    println!(
        "{} {} {} {}",
        "💾 Renaming file to avoid name collision. Initial name:".green(),
        old_name,
        "New name".green(),
        new_name,
    );
}

pub fn print_starting_rollack(time: String) {
    println!(
        "{} {}",
        "💾 Starting rollback files migrations from ".green(),
        time
    );
}

pub fn print_starting_apply_migrations(time: String) {
    println!(
        "{} {}",
        "💾 Starting apply files migrations generated from ".green(),
        time
    );
}

// Warning Messages

pub fn print_files_not_updated() {
    println!("{}", "⚠️ File locations were not updated.".yellow());
}

pub fn print_file_not_found(path: Display) {
    println!(
        "{} {} {}",
        "⚠️ File".yellow(),
        path,
        "not found. Skipped".yellow()
    );
}

pub fn print_skipped_failed_migration(from: Display, to: Display) {
    println!(
        "{} {} {} {} {}",
        "⚠️ Reason: Migration".yellow(),
        from,
        "to".yellow(),
        to,
        " was not successfull".yellow()
    );
}

pub fn print_skipped_to_same_string() {
    println!("{}", " Skipped".yellow(),);
}

pub fn print_ignoring_entry(is_file: bool, path: &str) {
    if is_file {
        print_ignoring_file(path);
    } else {
        print_ignoring_folder(path);
    }
}

pub fn print_ignoring_file(path: &str) {
    println!("{} {:?}", "⚠️ Ignoring file:".yellow(), path);
}

pub fn print_ignoring_folder(path: &str) {
    println!("{} {:?}", "⚠️ Ignoring folder:".yellow(), path);
}

// Confirmation Messages

pub fn ask_for_files_migration() -> bool {
    ask_confirmation(
        "❓ Are you satisfied with the file reorganization plan? Would you like to apply it?",
    )
}

pub fn print_invalid_input() {
    println!("{}", "❌ Invalid input. Please enter 'y' or 'n'.".yellow());
}
