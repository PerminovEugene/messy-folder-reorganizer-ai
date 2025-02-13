use crate::{file_info::FileInfo, promt::PROMT};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_util::io::StreamReader;

use tokio_stream::StreamExt;
use futures_util::stream::TryStreamExt;
use std::error::Error;

pub async fn create_reordering_plan(
    files_data: &Vec<FileInfo>,
    model_path: &str,
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();

    // Convert file metadata to JSON
    let file_names = files_data.iter().map(|d| &d.name).collect::<Vec<_>>();
    let file_data_json = serde_json::to_string_pretty(&file_names)?;

    // Define the AI prompt
    let prompt = format!(
        "{}\n{}",
        PROMT,
        file_data_json
    );

    print!("my promt---------> {}", prompt);

    let request_body = OllamaRequest {
        model: "llama3.2:1b".to_string(),
        prompt,
        stream: true, // Enable streaming
    };

    let mut response = client
        .post("http://localhost:11434/api/generate")
        .json(&request_body)
        .send()
        .await?;

    let mut response_text = String::new();
    // let mut stream = response.bytes();

    while let Some(chunk) = response.chunk().await? {
      let olama_response_token = serde_json::from_slice::<OllamaResponse>(&chunk)?;
      // println!("Ollama Response: {}", olama_response_token.response);

      response_text.push_str(&olama_response_token.response);
    }

    // while let Some(chunk) = stream.poll() {
    //     response_text.push_str(&String::from_utf8_lossy(&chunk));
    // }

    // Process the response as a stream (chunking)
    // let body_stream = response.bytes();
    // let reader = BufReader::new(body_stream.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)).into_async_read());
    // let mut lines = reader.lines();

    // while let Some(line) = lines.next_line().await? {
    //     if let Ok(data) = serde_json::from_str::<OllamaResponse>(&line) {
    //         response_text.push_str(&data.response);
    //     }
    // }

    println!("Ollama Response: {}", response_text);
    Ok(response_text)
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
    model: String,
    created_at: String,
    done: bool,
}
