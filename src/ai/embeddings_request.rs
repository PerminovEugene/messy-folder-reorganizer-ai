use reqwest::Client;
use std::error::Error;

use crate::{
    ai::ollama_protocol::{OllamaEmbedRequest, OllamaEmbedResponse, OllamaOptions},
    configuration::config::EmbeddingModelConfig,
};

pub async fn get_embeddings(
    file_names: &Vec<String>,
    model: String,
    ai_server_address: String,
    config: EmbeddingModelConfig,
) -> Result<Vec<Vec<f32>>, Box<dyn Error>> {
    let client = Client::new();

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

    let mut vectors: Vec<Vec<f32>> = vec![];

    match client.post(endpoint).json(&request_body).send().await {
        Ok(response) => {
            let olama_parsed_response: OllamaEmbedResponse = response.json().await?;

            vectors.extend(olama_parsed_response.embeddings);
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            panic!("Error from request to LLM")
        }
    }
    Ok(vectors)
}
