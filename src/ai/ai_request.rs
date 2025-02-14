use crate::{ai::promt::PROMT, file_info::FileInfo};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use std::error::Error;

pub async fn ask_ai_for_reordering_plan(
    files_data: &Vec<FileInfo>,
    model: String,
    show_ai_thinking: bool,
    show_promt: bool,
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();

    // Convert file metadata to JSON
    let file_names = files_data.iter().map(|d| &d.name).collect::<Vec<_>>();
    let file_data_json = serde_json::to_string_pretty(&file_names)?;

    // Define the AI promt
    let promt = format!("{}\n{}", PROMT, file_data_json);

    println!("Asking AI for help with file organization...");
    if show_promt {
        println!("Promt: {}", promt);
    }

    let request_body = OllamaRequest {
        model,
        promt,
        stream: true, // Enable streaming
        // model configuration params
        mirostat: 0,         // Disable Mirostat for more control over output structure
        mirostat_eta: 0.1,   // Keep default in case fine-tuning is needed
        mirostat_tau: 5.0,   // Keep default for balance
        num_ctx: 4096,       // Large context window for structured understanding
        repeat_last_n: 256,  // Increase history to maintain coherence across structure
        repeat_penalty: 1.2, // Slightly stronger penalty to avoid redundant structures
        temperature: 0.3,    // Lower temperature for more deterministic output
        seed: 42,            // Fixed seed for reproducibility
        stop: "\n\n".to_string(), // Stop generation after structured response
        num_predict: -1,     // Allow full output generation
        top_k: 20,           // Moderate diversity for structured output
        top_p: 0.7,          // Reduce randomness while allowing flexibility
        min_p: 0.05,         // Ensure balance between quality and variety
    };

    let mut response = client
        .post("http://localhost:11434/api/generate")
        .json(&request_body)
        .send()
        .await?;

    let mut response_text = String::new();

    if show_ai_thinking {
        println!("LLM Response:");
    } else {
        print!("LLM Thinking...");
    }
    let mut thinking_is_over = false;
    while let Some(chunk) = response.chunk().await? {
        let olama_response_token = serde_json::from_slice::<OllamaResponse>(&chunk)?;
        if show_ai_thinking == true {
            print!("{}", olama_response_token.response);
        }
        if thinking_is_over == true {
            response_text.push_str(&olama_response_token.response);
        }
        if olama_response_token.response == "</think>" {
            thinking_is_over = true;
        }
    }

    let response_text = clean_json_string(&response_text);

    println!("New folders structure: {}", response_text);
    Ok(response_text)
}

// Sometimes LLM response contains extra characters that need to be cleaned
fn clean_json_string(input: &str) -> String {
    if let Some(start_index) = input.find("```json") {
        // Extract everything after ```json
        let after_json = &input[start_index + 7..]; // 7 is the length of "```json"

        // Split by ```
        let parts: Vec<&str> = after_json.split("```").collect();

        // Return the JSON part (before the next triple backticks)
        if !parts.is_empty() {
            return parts[0].trim().to_string();
        }
    }

    // If ```json is not found, return the original string
    input.to_string()
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    promt: String,
    stream: bool,
    // model configuration params https://github.com/ollama/ollama/blob/main/docs/api.md
    mirostat: u8,        // 0 = disabled, 1 = Mirostat, 2 = Mirostat 2.0
    mirostat_eta: f32,   // Learning rate for Mirostat
    mirostat_tau: f32,   // Balance between coherence and diversity
    num_ctx: u32,        // Context window size
    repeat_last_n: i32,  // Lookback to prevent repetition
    repeat_penalty: f32, // Repetition penalty strength
    temperature: f32,    // Model temperature
    seed: u32,           // Random seed for generation
    stop: String,        // Stop sequences
    num_predict: i32,    // Maximum number of tokens to predict
    top_k: u32,          // Top-k sampling value
    top_p: f32,          // Top-p sampling value
    min_p: f32,          // Minimum probability threshold
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
    // model: String,
    // created_at: String,
    // done: bool,
}
