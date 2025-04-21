use std::{
    fs,
    path::{Path, PathBuf},
};
use test_case::test_case;

use helpers::{
    check::assert_fs_structure,
    cli::{run_apply, run_reorganization, run_rollback, OutputMode},
    prepare_fs::create_test_folders,
    test_case::TestCase,
};

mod helpers;

#[test_case("embedding_only")]
#[test_case("file_collision")]
#[test_case("symlinks")]
#[test_case("use_dest_root")]
#[test_case("with_ignored_items")]
// TODO migrate test to new implementation
// #[test_case("failed_migration")]
fn test_with_embeddings_only(case_folder: &str) -> Result<(), String> {
    assert_commands(case_folder, false)
}

// TODO migrate previous implementations to lates
// #[test_case("clustering")]
// #[test_case("deep_folders_structure")]
// #[test_case("llm_only")]

// Updated commands.
// Commented because deepseek is always not deterministic and simple hadrdcode assert doesn't work with it.
// TODO: change deepseek to better model OR use similarity for generated folder names
// #[test_case("embedding_partial")]
// #[test_case("embedding_partial_copy")]
// fn test_with_folder_generation(case_folder: &str) -> Result<(), String> {
//     assert_commands(case_folder, true)?;
//     Ok(())
// }

fn setup_output_mode(path: &Path, log_file_name: &str) -> OutputMode {
    let log_file_path = path.join(log_file_name);
    OutputMode::ToFile(log_file_path.to_string_lossy().to_string())
}

fn assert_commands(case_folder: &str, use_test_configs: bool) -> Result<(), String> {
    let base_path_to_cases = "tests/test_cases";
    let path_to_case: PathBuf = PathBuf::from(base_path_to_cases).join(case_folder);

    let test_case_config_name = "case.json";
    let path_to_config = &path_to_case.join(test_case_config_name);

    let json_str = fs::read_to_string(path_to_config).unwrap();
    let test_case: TestCase = serde_json::from_str(&json_str).unwrap();

    let fixtures_path = &path_to_case.join("fixtures");

    create_test_folders(&test_case, fixtures_path).unwrap();

    let (source_root_folder_name, _) = test_case.source.iter().next().unwrap();
    let source_path = &fixtures_path.join(source_root_folder_name);
    let source = source_path.to_str().unwrap();

    let (destination_root_folder_name, _) = test_case.destination.iter().next().unwrap();
    let destination_path = &fixtures_path.join(destination_root_folder_name);
    let destination = destination_path.to_str().unwrap();

    let path_to_app_folder: String = if use_test_configs {
        "tests/configs/".to_string()
    } else {
        path_to_case.as_os_str().to_string_lossy().into_owned()
    };
    let session_id = assert_process_command(
        &path_to_case,
        &path_to_app_folder,
        source,
        destination,
        &test_case,
        fixtures_path,
    )?;

    assert_rollback_command(
        &path_to_case,
        &path_to_app_folder,
        &session_id,
        &test_case,
        fixtures_path,
    )?;

    assert_apply_command(
        &path_to_case,
        &path_to_app_folder,
        &session_id,
        &test_case,
        fixtures_path,
    )?;
    Ok(())
}

fn assert_process_command(
    path_to_case: &Path,
    path_to_app_folder: &str,
    source: &str,
    destination: &str,
    test_case: &TestCase,
    fixtures_path: &Path,
) -> Result<String, String> {
    let mode = setup_output_mode(path_to_case, "process.log");
    let session_id = run_reorganization(
        path_to_app_folder,
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

    assert_fs_structure(test_case, fixtures_path, "expected")?;
    Ok(session_id)
}

fn assert_rollback_command(
    path_to_case: &Path,
    path_to_app_folder: &str,
    session_id: &str,
    test_case: &TestCase,
    fixtures_path: &Path,
) -> Result<(), String> {
    let mode = setup_output_mode(path_to_case, "rollback.log");
    run_rollback(path_to_app_folder, &mode, session_id).expect("CLI rollback failed");
    assert_fs_structure(test_case, fixtures_path, "source")
}

fn assert_apply_command(
    path_to_case: &Path,
    path_to_app_folder: &str,
    session_id: &str,
    test_case: &TestCase,
    fixtures_path: &Path,
) -> Result<(), String> {
    let mode = setup_output_mode(path_to_case, "apply.log");
    run_apply(path_to_app_folder, &mode, session_id).expect("CLI apply failed");
    assert_fs_structure(test_case, fixtures_path, "expected")
}
