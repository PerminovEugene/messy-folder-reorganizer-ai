use std::{fs, path::PathBuf};

use helpers::{
    check::check_expected_structure, cli::run_reorganization, config::TestCase,
    prepare_fs::create_test_folders,
};

mod helpers;

#[test]
fn integration_cli_file_organizer() {
    let base = "tests/file_collision";

    let config_name = "case.json";

    let path_to_case: PathBuf = PathBuf::from(base);
    let path_to_config = &path_to_case.join(config_name);

    let json_str = fs::read_to_string(path_to_config).unwrap();
    let test_case: TestCase = serde_json::from_str(&json_str).unwrap();

    create_test_folders(&test_case, &path_to_case).unwrap();

    let source_path = &path_to_case.join(&test_case.source.folder);
    let source = source_path.to_str().unwrap();

    let destination_path = &path_to_case.join(&test_case.destination.folder);
    let destination = destination_path.to_str().unwrap();

    println!("{}", source);
    println!("{}", destination);
    run_reorganization(
        source,
        destination,
        "mxbai-embed-large",
        "deepseek-r1:latest",
        "http://localhost:11434/",
        "http://localhost:6334/",
    );

    match check_expected_structure(&test_case, &path_to_case) {
        Ok(_) => println!("✅ Folder structure matches expected!"),
        Err(e) => eprintln!("❌ Folder structure check failed: {e}"),
    }
}
