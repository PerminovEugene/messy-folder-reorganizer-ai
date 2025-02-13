mod create_source_file;
mod dirr_processing;
mod file_info;
mod files_sorting;
// mod olama_responce;
mod promt;

use create_source_file::create_source_file;
use dirr_processing::fill_up_files_data_by_path;
use files_sorting::create_reordering_plan;

#[tokio::main]
async fn main() {
    let mut files_data: Vec<file_info::FileInfo> = Vec::new();
    fill_up_files_data_by_path("./test_cases/messy-folder", &mut files_data);
    let source_file_path = create_source_file(&files_data);

    // http://huggingface.co/TheBloke/Llama-2-7B-Chat-GGUF/blob/main/llama-2-7b-chat.Q4_K_M.gguf
    // let model_path = "./models/llama-2-7b-chat.Q4_K_M.gguf"; // Path to your LLaMA model
    create_reordering_plan(&files_data, "").await;

  //   match create_reordering_plan(&files_data, "").await {
  //     Ok(response) => println!("Ollama Response: {}", response),
  //     Err(err) => eprintln!("Error: {}", err),
  // }
}
