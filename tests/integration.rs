use std::{fs, path::PathBuf};
use test_case::test_case;

use helpers::{
    check::check_expected_structure,
    cli::{run_reorganization, OutputMode},
    config::TestCase,
    prepare_fs::create_test_folders,
};

mod helpers;

// #[test_case("clustering")]
#[test_case("file_collision")]
#[test_case("with_ignored_items")]
#[test_case("use_dest_root")]
#[test_case("symlinks")]
#[test_case("embedding_only")]

fn integration_cli_file_organizer(case_folder: &str) -> Result<(), String> {
    let base_path_to_cases = "tests/test_cases";
    let config_name = "case.json";

    let path_to_case: PathBuf = PathBuf::from(base_path_to_cases).join(case_folder);
    let path_to_config = &path_to_case.join(config_name);

    let json_str = fs::read_to_string(path_to_config).unwrap();
    let test_case: TestCase = serde_json::from_str(&json_str).unwrap();

    create_test_folders(&test_case, &path_to_case).unwrap();

    let (source_root_folder_name, _) = test_case.source.iter().next().unwrap();
    let source_path = &path_to_case.join(source_root_folder_name);
    let source = source_path.to_str().unwrap();

    let (destination_root_folder_name, _) = test_case.destination.iter().next().unwrap();
    let destination_path = &path_to_case.join(destination_root_folder_name);
    let destination = destination_path.to_str().unwrap();

    let log_file_path = path_to_case.join("test_case.log");
    let mode = OutputMode::ToFile(log_file_path.to_string_lossy().to_string());

    run_reorganization(
        source,
        destination,
        "mxbai-embed-large",
        "deepseek-r1:latest",
        "http://localhost:11434",
        "http://localhost:6334",
        mode,
    )
    .expect("CLI process failed");

    check_expected_structure(&test_case, &path_to_case)?;

    Ok(())
}
