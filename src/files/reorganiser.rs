use colored::Colorize;
use path_clean::PathClean;
use std::{fs, path::PathBuf};

use crate::files::file_info::FilesReorganisationPlan;

pub fn apply_plan(path: String) -> std::io::Result<()> {
    let plan = fs::read_to_string("./plan.json").expect("Error reading plan file");

    let plan: Vec<FilesReorganisationPlan> = match serde_json::from_str(&plan) {
        Ok(parsed_plan) => parsed_plan,
        Err(err) => {
            eprintln!("Error parsing plan.json: {}", err);
            return Err(err.into()); // Proper error propagation
        }
    };

    for plan_item in plan {
        let base_path: PathBuf = PathBuf::from(&path);
        let original_path = base_path.join(&plan_item.original).clean();

        let new_path = base_path.join(plan_item.new_path).clean();

        println!(
            "Moving file {} to {:?}",
            original_path.to_string_lossy(),
            new_path
        );

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

    println!("{}", "Files reorganization is done".green());
    Ok(())
}
