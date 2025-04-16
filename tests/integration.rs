use std::{
    fs,
    path::{Path, PathBuf},
};
use test_case::test_case;

use helpers::{
    check::assert_fs_structure,
    cli::{run_apply, run_reorganization, run_rollback, OutputMode},
    config::TestCase,
    prepare_fs::create_test_folders,
};

mod helpers;

// #[test_case("clustering")]
// #[test_case("deep_folders_structure")]
#[test_case("embedding_only")]
// #[test_case("embedding_partial")]
// #[test_case("failed_migration")]
#[test_case("file_collision")]
// #[test_case("llm_only")]
#[test_case("symlinks")]
#[test_case("use_dest_root")]
#[test_case("with_ignored_items")]

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

    let mode = setup_log_mode(&path_to_case, "process.log");
    // process

    let session_id = run_reorganization(
        source,
        destination,
        "mxbai-embed-large",
        "deepseek-r1:latest",
        "http://localhost:11434",
        "http://localhost:6334",
        &mode,
    )
    .expect("CLI process failed")
    .expect("Session ID not captured");

    assert_fs_structure(&test_case, &path_to_case, "expected")?;

    // rolback

    let mode = setup_log_mode(&path_to_case, "rollback.log");
    run_rollback(&mode, &session_id).expect("CLI rollback failed");
    assert_fs_structure(&test_case, &path_to_case, "source")?;

    // apply

    let mode = setup_log_mode(&path_to_case, "apply.log");
    run_apply(&mode, &session_id).expect("CLI apply failed");
    assert_fs_structure(&test_case, &path_to_case, "expected")?;

    Ok(())
}

fn setup_log_mode(path: &Path, log_file_name: &str) -> OutputMode {
    let log_file_path = path.join(log_file_name);
    OutputMode::ToFile(log_file_path.to_string_lossy().to_string())
}
