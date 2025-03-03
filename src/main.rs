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

use ai::ai_request::ask_ai_for_reordering_plan;
use configuration::init::init;
use console::confirmation::ask_confirmation;
use files::create_file::create_plan_file;
use files::create_file::create_source_file;
use files::dirr_processing::collect_files_metadata;
use files::file_info;
use files::reorganiser::apply_plan;
use regex::Regex;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    println!("Messy-folder-reorganizer-ai - Version {}", VERSION);

    init();
    let args = configuration::args::Args::parse();
    let config = configuration::read_config::read_config();

    // processing destination

    let mut dest_files_data: Vec<file_info::FileInfo> = Vec::new();

    let dest = if args.destination != "home" {
        args.destination
    } else {
        env::var("HOME").unwrap_or_else(|_| ".".to_string())
    };

    collect_files_metadata(
        &dest,
        "",
        args.skip_problematic_dir,
        &mut dest_files_data,
        &vec![Regex::new(r"^\..*").unwrap()],
        true,
        true,
        false,
    );

    let dest_file_names = dest_files_data.iter().map(|d| &d.name).collect::<Vec<_>>();

    println!("{:?}", dest_file_names);

    println!("Creating embeddings for destination collection");

    let dest_embeddings = embeddings::get_embeddings(
        &dest_file_names,
        args.model.clone(), // TODO remove clones
        args.ai_server_address.clone(),
        config.clone(),
    )
    .await;

    println!("Adding vectors to destination collection");

    add_vectors(&dest_file_names, dest_embeddings.unwrap())
        .await
        .unwrap();

    println!("Adding vectors to destination collection");

    // processing source

    let mut files_data: Vec<file_info::FileInfo> = Vec::new();

    collect_files_metadata(
        &args.path,
        "",
        args.skip_problematic_dir,
        &mut files_data,
        &vec![],
        args.recursive,
        false,
        true,
    );
    create_source_file(&files_data);

    let file_names = files_data.iter().map(|d| &d.name).collect::<Vec<_>>();

    let embeddings =
        embeddings::get_embeddings(&file_names, args.model, args.ai_server_address, config)
            .await
            .unwrap();

    let closest_pathes = find_closest_vectors(embeddings).await.unwrap();
    // let merged = embeddings
    //     .into_iter()
    //     .zip(file_names.into_iter())
    //     .for_each(|e| {
    //         let path = find_closest_vector(&e).await.unwrap();
    //         println!("{:?}", path);
    //     });

    // add_vectors(&file_names, embeddings.unwrap()).await.unwrap();

    println!("possible dests {:?}", dest_file_names);
    println!("files to sort out: {:?}", file_names);
    println!("result pathes: {:?}", closest_pathes);

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
