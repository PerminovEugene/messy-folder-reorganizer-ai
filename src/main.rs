mod create_source_file;
mod dirr_processing;
mod file_info;

use create_source_file::create_source_file;
use dirr_processing::fill_up_files_data_by_path;

fn main() {
    let mut files_data: Vec<file_info::FileInfo> = Vec::new();
    fill_up_files_data_by_path(".", &mut files_data);
    create_source_file(&files_data);
}
