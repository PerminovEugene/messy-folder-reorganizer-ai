use crate::configuration::args::Args;
use crate::console::messages::{ask_for_files_migration, print_files_not_updated};
use crate::files::reorganiser::apply_plan;

pub async fn migrate_files(args: &Args) {
    if args.force_apply || ask_for_files_migration() {
        apply_plan().unwrap();
    } else {
        print_files_not_updated();
    }
}
