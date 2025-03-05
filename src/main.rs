use std::env;

use ai::embeddings;
use bd::quadrant::add_vectors;
use bd::quadrant::find_closest_vectors;
use clap::Parser;
use colored::Colorize;

mod ai;
mod bd;
mod configuration;
mod console;
mod files;
mod workflow;

use configuration::init::init;
use files::file_info;
use workflow::index_destination;
use workflow::process_sources;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    println!("Messy-folder-reorganizer-ai - Version {}", VERSION);

    init();
    let args = configuration::args::Args::parse();
    let config = configuration::read_config::read_config();

    index_destination::index_destinations(&config, &args).await;

    process_sources::process_sources(&config, &args).await;

    // let mut dest_files_data: Vec<file_info::FileInfo> = Vec::new();

    // let dest = if args.destination != "home" {
    //     args.destination
    // } else {
    //     env::var("HOME").unwrap_or_else(|_| ".".to_string())
    // };

    // collect_files_metadata(
    //     &dest,
    //     "",
    //     args.skip_problematic_dir,
    //     &mut dest_files_data,
    //     &vec![Regex::new(r"^\..*").unwrap()],
    //     true,
    //     true,
    //     false,
    // );

    // let dest_file_names = dest_files_data.iter().map(|d| &d.name).collect::<Vec<_>>();

    // println!("{:?}", dest_file_names);

    // println!("Creating embeddings for destination collection");

    // let dest_embeddings = embeddings::get_embeddings(
    //     &dest_file_names,
    //     args.model.clone(), // TODO remove clones
    //     args.ai_server_address.clone(),
    //     config.clone(),
    // )
    // .await;

    // println!("Adding vectors to destination collection");

    // add_vectors(&dest_file_names, dest_embeddings.unwrap())
    //     .await
    //     .unwrap();

    // println!("Adding vectors to destination collection");

    // processing source

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
