use colored::Colorize;
use reqwest::Client;
use std::error::Error;

use crate::{
    ai::ollama_protocol::{OllamaEmbedRequest, OllamaEmbedResponse, OllamaOptions},
    configuration::config::Config,
    file_info::FileInfo,
};

pub async fn get_embeddings(
    file_names: &Vec<String>,
    model: String,
    ai_server_address: String,
    config: Config,
) -> Result<Vec<Vec<f32>>, Box<dyn Error>> {
    let client = Client::new();

    // Convert file metadata to JSON
    // let file_names = files_data.iter().map(|d| &d.name).collect::<Vec<_>>();

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

    let request_body = OllamaEmbedRequest {
        model,
        input: file_names,
        options: &options,
    };

    let mut endpoint = ai_server_address.clone();
    endpoint.push_str("api/embed");
    println!("{}", endpoint);
    let mut vectors: Vec<Vec<f32>> = vec![];

    // println!("{:?}", request_body.input);

    match client.post(endpoint).json(&request_body).send().await {
        Ok(mut response) => {
            println!("{}", "📁 Processing response from AI:".green());
            let olama_parsed_response: OllamaEmbedResponse = response.json().await?;

            vectors = olama_parsed_response.embeddings.clone();
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            panic!("Error from request to LLM")
        }
    }
    println!("{}", "📁 Cleaning output from extra symbols:".green());
    Ok(vectors)
}
// Compare this snippet from src/ai/prompt.rs:
