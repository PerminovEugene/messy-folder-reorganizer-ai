use path_clean::PathClean;
use std::{fs, path::PathBuf};

use crate::{
    configuration::init::get_app_config_folder,
    console::messages::{print_files_reorganization_done, print_move_file},
    files::{consts::PLAN_FILE_NAME, file_info::FilesReorganisationPlan},
};

pub fn apply_plan() -> std::io::Result<()> {
    let plan_file_path = get_app_config_folder().join(PLAN_FILE_NAME);
    let plan = fs::read_to_string(plan_file_path).expect("Error reading plan file");

    let plan: Vec<FilesReorganisationPlan> = match serde_json::from_str(&plan) {
        Ok(parsed_plan) => parsed_plan,
        Err(err) => {
            eprintln!("Error parsing plan.json: {}", err);
            return Err(err.into()); // Proper error propagation
        }
    };

    for file_migration_plan in plan {
        let source_base_path: PathBuf = PathBuf::from(file_migration_plan.source);
        let destination_base_path: PathBuf = PathBuf::from(file_migration_plan.destination);

        let original_path = source_base_path
            .join(file_migration_plan.source_inner_path)
            .join(&file_migration_plan.file_name)
            .clean();

        let new_path = destination_base_path
            .join(file_migration_plan.destination_inner_path)
            .join(&file_migration_plan.file_name)
            .clean();

        print_move_file(original_path.to_str().unwrap(), new_path.to_str().unwrap());

        // Ensure the parent directory of the new path exists
        if let Some(parent) = new_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Attempt renaming and handle errors
        if let Err(err) = fs::rename(&original_path, &new_path) {
            eprintln!(
                "Error renaming {:?} to {:?}: {}",
                original_path, new_path, err
            );
        }
    }

    print_files_reorganization_done();
    Ok(())
}
