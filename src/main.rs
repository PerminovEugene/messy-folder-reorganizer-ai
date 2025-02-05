
mod create_source_file;
mod file_info;
mod dirr_processing;

use dirr_processing::fill_up_files_data_by_path;
use create_source_file::create_source_file;

fn main() {
  let mut files_data: Vec<file_info::FileInfo> = Vec::new();
  fill_up_files_data_by_path(".", &mut files_data);
  create_source_file(&files_data);
}
