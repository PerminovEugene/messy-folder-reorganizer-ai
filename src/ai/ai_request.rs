use reqwest::Client;
use std::error::Error;

use crate::{
    ai::{
        ollama_protocol::{OllamaGenerateRequest, OllamaOptions, OllamaResponse},
        prompt::read_initial_prompt,
    },
    configuration::config::Config,
};

pub async fn ask_ai_for_reordering_plan(
    file_names: Vec<&String>,
    model: String,
    ai_server_address: String,
    config: Config,
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();

    let file_data_json = serde_json::to_string_pretty(&file_names)?;

    let initial_prompt = read_initial_prompt();
    let prompt_with_input = format!("{}\n{}", initial_prompt, file_data_json);

    let options = OllamaOptions {
        mirostat: config.mirostat,
        mirostat_tau: config.mirostat_tau,
        mirostat_eta: config.mirostat_eta,
        num_ctx: config.num_ctx,
        repeat_last_n: config.repeat_last_n,
        repeat_penalty: config.repeat_penalty,
        temperature: config.temperature,
        seed: config.seed,
        stop: config.stop,
        num_predict: config.num_predict,
        top_k: config.top_k,
        top_p: config.top_p,
        min_p: config.min_p,
    };

    let request_body = OllamaGenerateRequest {
        model,
        prompt: prompt_with_input,
        stream: true,
        options: &options,
    };

    let mut response_text = String::new();

    let mut thinking_is_over = false;
    let mut endpoint = ai_server_address.clone();
    endpoint.push_str("api/generate");
    match client.post(endpoint).json(&request_body).send().await {
        Ok(mut response) => {
            while let Some(chunk) = response.chunk().await? {
                let olama_response_token: OllamaResponse =
                    serde_json::from_slice::<OllamaResponse>(&chunk)?;
                if olama_response_token.response.is_empty() {
                    continue;
                }
                if thinking_is_over {
                    response_text.push_str(&olama_response_token.response);
                }
                if olama_response_token.response == "</think>" {
                    thinking_is_over = true;
                }
            }
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            panic!("Error from request to LLM")
        }
    }

    let response_text = clean_json_string(&response_text);

    Ok(response_text)
}

// Sometimes LLM response contains extra characters that need to be cleaned
fn clean_json_string(input: &str) -> String {
    if let Some(start_index) = input.find("```json") {
        let after_json = &input[start_index + 7..]; // 7 is the length of "```json"

        let parts: Vec<&str> = after_json.split("```").collect();
        if !parts.is_empty() {
            return parts[0].trim().to_string();
        }
    }

    // If ```json is not found, return the original string
    input.to_string()
}
