use crate::configuration::args::Args;
use crate::configuration::config::Config;
use crate::console::messages::{ask_for_files_migration, print_files_not_updated};
use crate::files::reorganiser::apply_plan;

use super::sources_processor::ProcessResult;

pub async fn migrate_files(config: &Config, args: &Args, files_data: &Vec<ProcessResult>) {
    if args.force_apply || ask_for_files_migration() {
        apply_plan().unwrap();
    } else {
        print_files_not_updated();
    }
}
