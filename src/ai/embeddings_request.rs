use reqwest::Client;

use crate::{
    ai::ollama_protocol::{OllamaEmbedRequest, OllamaEmbedResponse, OllamaOptions},
    configuration::config::EmbeddingModelConfig,
    errors::app_error::AppError,
};

pub async fn get_embeddings(
    file_names: &Vec<String>,
    model: String,
    ai_server_address: String,
    config: EmbeddingModelConfig,
) -> Result<Vec<Vec<f32>>, AppError> {
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

    let result = client.post(endpoint).json(&request_body).send().await;

    match result {
        Ok(response) => {
            let parsed: Result<OllamaEmbedResponse, _> = response.json().await;
            match parsed {
                Ok(olama_parsed_response) => {
                    vectors.extend(olama_parsed_response.embeddings);
                    Ok(vectors)
                }
                Err(e) => Err(AppError::OllamaResponseParse(format!(
                    "JSON parsing error: {}",
                    e
                ))),
            }
        }
        Err(err) => Err(AppError::OllamaConnection(err.to_string())),
    }
}
