use std::io::{self, Write};
use std::path::Display;

use colored::Colorize;

use super::confirmation::ask_confirmation;

// ───────────── Information Messages ─────────────

pub fn print_initial_message(version: &str) {
    println!();
    println!(
        "{} {}",
        "🚀 Messy-folder-reorganizer-ai - version".green(),
        version
    );
}

pub fn print_parsing_destination_folder() {
    println!("{}", "📁 Parsing destination folder structure...".green());
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
        "📍 Searching for the best destination folder for each source file...".green()
    );
}

pub fn print_migration_plan_saved() {
    println!("{}", "💾 Migration plan saved.".green());
    println!();
}

pub fn print_generate_config_file(config_file_path: String) {
    println!(
        "{} {:?}",
        "⚙️  Configuration file created:".green(),
        config_file_path
    );
}

pub fn print_source_files_metadata_saved() {
    println!("{}", "💾 Source file metadata saved.".green());
}

pub fn print_processing_directory(path: Display) {
    print!("{} {:?} ", "📁 Processing directory:".blue(), path);
    io::stdout().flush().unwrap();
}

pub fn print_processing_file(file_name: &String) {
    print!("{} {:?}", "📄 Processing file:".blue(), file_name);
    io::stdout().flush().unwrap();
}

pub fn print_files_reorganization_done() {
    println!("{}", "✅ File migration completed successfully.".green());
}

pub fn print_move_file(from: Display, to: Display) {
    print!("{} {} {} {}... ", "📦 Moving".blue(), from, "to".blue(), to);
    io::stdout().flush().unwrap();
}

pub fn print_done_to_same_string() {
    println!("{}", "✅ Done".green());
}

pub fn print_creating_dest_embeddings() {
    println!(
        "{}",
        "🗂️ Generating embeddings for destination folders...".green()
    );
}

pub fn print_clustering_unknown_vectors() {
    println!(
        "{}",
        "🔢 Clustering unidentified file embeddings...".green()
    );
}

pub fn print_asking_llm_for_new_folder_names() {
    println!(
        "{}",
        "🤖 Asking the LLM to generate folder names for clusters... (This may take a moment.)"
            .blue()
    );
}

pub fn print_saving_dest_embeddings() {
    println!(
        "{}",
        "💾 Saving destination folder embeddings to database...".green()
    );
}

pub fn print_file_renamed(old_name: &String, new_name: String) {
    println!(
        "{} {} {} {}",
        "✏️  File renamed due to name conflict. Original:".green(),
        old_name,
        "New:".green(),
        new_name
    );
}

pub fn print_starting_rollack(time: String) {
    println!(
        "{} {}",
        "🔄 Starting rollback of file migrations from".green(),
        time
    );
}

pub fn print_starting_apply_migrations(time: String) {
    println!(
        "{} {}",
        "📤 Applying file migrations created at".green(),
        time
    );
}

pub fn print_reading_directory_entries(path: Display) {
    print!("{} {:?} ", "📂 Reading directory:".blue(), path);
    io::stdout().flush().unwrap();
}

// ───────────── Warning Messages ─────────────

pub fn print_files_not_updated() {
    println!("{}", "⚠️  File paths were not updated.".yellow());
}

pub fn print_file_not_found(path: Display) {
    println!(
        "{} {} {}",
        "⚠️  File".yellow(),
        path,
        "not found. Skipping.".yellow()
    );
}

pub fn print_skipped_failed_migration(from: Display, to: Display) {
    println!(
        "{} {} {} {} {}",
        "⚠️  Skipped: failed to migrate".yellow(),
        from,
        "to".yellow(),
        to,
        "(operation unsuccessful)".yellow()
    );
}

pub fn print_skipped_to_same_string() {
    println!("{}", "⚠️  Skipped.".yellow());
}

pub fn print_ignoring_entry(is_file: bool, is_symlink: bool, path: String) {
    if is_file {
        print_ignoring_file(path);
    } else if is_symlink {
        print_ignoring_symlink(path);
    } else {
        print_ignoring_folder(path);
    }
}

pub fn print_ignoring_unix_uniq_entry_type(path: String) {
    println!(
        "{} {:?}",
        "⚠️  Skipping unsupported special file type:".yellow(),
        path
    );
}

pub fn print_ignoring_file(path: String) {
    println!("{} {:?}", "⚠️ Ignoring file:".yellow(), path);
}

pub fn print_ignoring_symlink(path: String) {
    println!("{} {:?}", "⚠️ Ignoring symlink:".yellow(), path);
}

pub fn print_ignoring_folder(path: String) {
    println!("{} {:?}", "⚠️ Ignoring folder:".yellow(), path);
}

// ───────────── Confirmation / Error Messages ─────────────

pub fn ask_for_files_migration() -> bool {
    ask_confirmation(
        "❓ Are you satisfied with the proposed file organization? Apply the migration now?",
    )
}

pub fn print_invalid_input() {
    println!("{}", "❌ Invalid input. Please enter 'y' or 'n'.".yellow());
}
