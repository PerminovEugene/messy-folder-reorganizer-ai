use std::env;
use std::path::PathBuf;

use regex::Regex;

use crate::ai::embeddings::get_embeddings;
use crate::bd::quadrant::add_vectors;
use crate::configuration::args::Args;
use crate::configuration::config::Config;
use crate::files::dirr_processing::collect_files_metadata;
use crate::files::file_info::{self, convert_path_meta_to_file_info, FileInfo};

pub async fn index_destinations(config: &Config, args: &Args) {
    let mut dest_files_data: Vec<file_info::FileInfo> = Vec::new();

    let dest = if args.destination != "home" {
        args.destination.clone()
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

    if args.destination != "home" {
        let destination_base_folder = PathBuf::from(args.destination.clone());
        let file_name = destination_base_folder.file_name().unwrap();
        println!("file_name: {:?}", file_name);

        let destination_base_folder_2 = PathBuf::from(file_name);

        let dest_file_info = convert_path_meta_to_file_info(
            &destination_base_folder_2,
            destination_base_folder.metadata().unwrap(),
        );
        dest_files_data.push(dest_file_info);
    }
    let dest_file_names = dest_files_data
        .iter()
        .map(|d| d.name.clone())
        .collect::<Vec<_>>();

    println!("{:?}", dest_file_names);

    println!("Creating embeddings for destination collection");

    let dest_embeddings = get_embeddings(
        &dest_file_names,
        args.embedding_model.clone(), // TODO remove clones
        args.ai_server_address.clone(),
        config.clone(),
    )
    .await;

    println!("Adding vectors to destination collection");

    add_vectors(&dest_file_names, dest_embeddings.unwrap())
        .await
        .unwrap();
}
