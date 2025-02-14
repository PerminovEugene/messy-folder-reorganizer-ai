use clap::Parser;

mod ai;
mod args;
mod console;
mod files;

use ai::ai_request::ask_ai_for_reordering_plan;
use console::confirmation::ask_confirmation;
use files::create_file::create_plan_file;
use files::create_file::create_source_file;
use files::dirr_processing::fill_up_files_data_by_path;
use files::file_info;
use files::reorganiser::apply_plan;

#[tokio::main]
async fn main() {
    let args = args::Args::parse();

    let mut files_data: Vec<file_info::FileInfo> = Vec::new();

    fill_up_files_data_by_path(&args.path, args.recursive, &mut files_data);
    create_source_file(&files_data);

    let plan = ask_ai_for_reordering_plan(
        &files_data,
        args.model,
        args.show_ai_thinking,
        args.show_promt,
    )
    .await;

    create_plan_file(plan.unwrap());

    if args.force_apply
        || ask_confirmation(
            "Are you satisfied with files reoraganization pland and want to apply it?",
        )
    {
        apply_plan(args.path).unwrap();
    } else {
        println!("Files location were not updated")
    }
}
