// TODO Add format and content based context

pub fn add_context_to_files_input(file_names: &[String]) -> Vec<String> {
    file_names
        .iter()
        .map(|name| format!("This is a file name: {name}"))
        .collect::<Vec<_>>()
}

pub fn add_context_to_folders_input(file_names: &[String]) -> Vec<String> {
    file_names
        .iter()
        .map(|name| format!("This is a folder name: {name}"))
        .collect::<Vec<_>>()
}
