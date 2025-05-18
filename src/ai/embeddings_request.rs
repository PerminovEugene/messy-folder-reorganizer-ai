use reqwest::Client as ReqwestClient;
use crate::{
    ai::ollama_protocol::{OllamaEmbedRequest, OllamaEmbedResponse, OllamaOptions as OllamaModelOptions},
    ai::openai_client::OpenAIClient, // Added
    configuration::config::EmbeddingModelConfig as OllamaEmbeddingConfig,
    errors::app_error::AppError,
    configuration::args::{ProcessArgs, AiProvider}, // Added
};

pub async fn get_ai_embeddings(
    input_strings: &Vec<String>,
    args: &ProcessArgs,
    ollama_config: &OllamaEmbeddingConfig, // Still needed for Ollama path
) -> Result<Vec<Vec<f32>>, AppError> {
    match args.ai_provider {
        AiProvider::Local => {
            let model_name = args.ollama_embedding_model.as_ref().ok_or(AppError::OllamaEmbeddingModelMissing)?.clone();
            get_ollama_embeddings(
                input_strings,
                model_name,
                args.ollama_server_address.clone(),
                ollama_config.clone(),
            ).await
        }
        AiProvider::OpenAI => {
             let api_key = args.openai_api_key.clone()
                .or_else(|| std::env::var("OPENAI_API_KEY").ok())
                .ok_or_else(|| AppError::Configuration("OpenAI API key not provided. Set --openai-api-key or OPENAI_API_KEY env var.".to_string()))?;
            let client = OpenAIClient::new(args, api_key);
            client.generate_openai_embeddings(input_strings).await
        }
    }
}

async fn get_ollama_embeddings(
    input_strings: &Vec<String>,
    ollama_model_name: String,
    ollama_server_address: String,
    ollama_config: OllamaEmbeddingConfig,
) -> Result<Vec<Vec<f32>>, AppError> {
    let client = ReqwestClient::new();
    let options: OllamaModelOptions = OllamaModelOptions {
        mirostat: ollama_config.mirostat,
        mirostat_tau: ollama_config.mirostat_tau,
        mirostat_eta: ollama_config.mirostat_eta,
        num_ctx: ollama_config.num_ctx,
        repeat_last_n: ollama_config.repeat_last_n,
        repeat_penalty: ollama_config.repeat_penalty,
        temperature: ollama_config.temperature,
        seed: ollama_config.seed,
        stop: ollama_config.stop,
        num_predict: ollama_config.num_predict,
        top_k: ollama_config.top_k,
        top_p: ollama_config.top_p,
        min_p: ollama_config.min_p,
    };

    let request_body = OllamaEmbedRequest {
        model: ollama_model_name,
        input: input_strings,
        options: &options,
    };

    let mut endpoint = ollama_server_address.clone();
    endpoint.push_str("/api/embed"); // Ollama's endpoint might be /api/embeddings

    let result = client.post(endpoint).json(&request_body).send().await;
    match result {
        Ok(response) => {
            let parsed: Result<OllamaEmbedResponse, _> = response.json().await;
            match parsed {
                Ok(olama_parsed_response) => Ok(olama_parsed_response.embeddings),
                Err(e) => Err(AppError::OllamaResponseParse(format!(
                    "JSON parsing error for Ollama embeddings: {}", e
                ))),
            }
        }
        Err(err) => Err(AppError::OllamaConnection(err.to_string())),
    }
}
