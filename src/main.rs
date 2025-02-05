use std::{
  fs::{self, File, Metadata},
  io::Write,
  path::Path,
  time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct FileInfo {
  name: String,
  size: u64,
  created_at: String,
  modified_at: String,
}

fn create_planner_file(files_data: &Vec<FileInfo>) {
  let file_name = "./source-files.json";
  let path = Path::new(file_name);

  if path.exists() {
      if let Err(err) = fs::remove_file(path) {
          println!("Error deleting old planner file: {:?}", err);
          return;
      }
  }

  match File::create(file_name) {
      Ok(mut file) => {
          println!("File created: {}", file_name);
          let json_data = match serde_json::to_string_pretty(&files_data) {
              Ok(json) => json,
              Err(err) => {
                  println!("Error serializing JSON: {:?}", err);
                  return;
              }
          };

          if let Err(err) = file.write_all(json_data.as_bytes()) {
              println!("Error writing to file: {:?}", err);
          }
      }
      Err(err) => println!("Error creating file: {:?}", err),
  }
  println!("Source files data is saved");

}

fn process_file(path: &Path, file_meta: Metadata) -> FileInfo {
  FileInfo {
      name: path.to_string_lossy().to_string(),
      size: file_meta.len(),
      created_at: get_system_time_string(file_meta.created().unwrap_or(SystemTime::now())),
      modified_at: get_system_time_string(file_meta.modified().unwrap_or(SystemTime::now())),
  }
}

fn get_system_time_string(time: SystemTime) -> String {
  time.duration_since(UNIX_EPOCH)
      .map(|dur| dur.as_secs().to_string())
      .unwrap_or_else(|_| "unknown".to_string())
}

fn process_dirr(path: &str, files_data: &mut Vec<FileInfo>) {
  match fs::read_dir(path) {
      Ok(read_dir_res) => {
          for entry in read_dir_res {
              if let Ok(dir) = entry {
                  let file_meta = match dir.metadata() {
                      Ok(meta) => meta,
                      Err(err) => {
                          println!("Error reading metadata for {:?}: {:?}", dir.path(), err);
                          continue;
                      }
                  };

                  if file_meta.is_file() {
                      let file_info = process_file(&dir.path(), file_meta);
                      files_data.push(file_info);
                  } else {
                      println!("Processing directory: {:?}", dir.path());
                      process_dirr(dir.path().to_str().unwrap(), files_data);
                  }
              }
          }
      }
      Err(err) => {
          println!("Error reading directory {:?}: {:?}", path, err);
      }
  }
}

fn main() {
  let mut files_data: Vec<FileInfo> = Vec::new();
  process_dirr(".", &mut files_data);
  create_planner_file(&files_data);
}
