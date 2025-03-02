use ai::embeddings;
use clap::Parser;
use colored::Colorize;

mod ai;
mod bd;
mod configuration;
mod console;
mod files;

use ai::ai_request::ask_ai_for_reordering_plan;
use configuration::init::init;
use console::confirmation::ask_confirmation;
use files::create_file::create_plan_file;
use files::create_file::create_source_file;
use files::dirr_processing::fill_up_files_data_by_path;
use files::file_info;
use files::reorganiser::apply_plan;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    println!("Messy-folder-reorganizer-ai - Version {}", VERSION);

    init();
    let args = configuration::args::Args::parse();
    let config = configuration::read_config::read_config();

    let mut files_data: Vec<file_info::FileInfo> = Vec::new();

    fill_up_files_data_by_path(
        &args.path,
        "",
        args.recursive,
        args.skip_problematic_dir,
        &mut files_data,
    );
    create_source_file(&files_data);

    let file_names = files_data.iter().map(|d| &d.name).collect::<Vec<_>>();

    let embeddings =
        embeddings::get_embeddings(&file_names, args.model, args.ai_server_address, config).await;

    bd::quadrant::add_vectors(&file_names, embeddings.unwrap())
        .await
        .unwrap();

    print!("{}", "🚀 Done!".green());

    // let plan = ask_ai_for_reordering_plan(
    //     &files_data,
    //     args.model,
    //     args.show_ai_thinking,
    //     args.show_prompt,
    //     args.ai_server_address,
    //     config,
    // )
    // .await;

    // create_plan_file(plan.unwrap());

    // if args.force_apply
    //     || ask_confirmation(
    //         "❓ Are you satisfied with the file reorganization plan? Would you like to apply it?",
    //     )
    // {
    //     apply_plan(args.path).unwrap();
    // } else {
    //     println!("{}", "🚫 File locations were not updated.".yellow())
    // }
}
