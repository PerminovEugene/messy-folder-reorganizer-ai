use reqwest::Client as ReqwestClient;
use crate::ai::{
    ollama_protocol::{OllamaGenerateRequest, OllamaOptions as OllamaModelOptions, OllamaResponse as OllamaLLMResponse},
    openai_client::OpenAIClient, // Added
    prompt::read_prompt,
};
use crate::configuration::config::LLMModelConfig as OllamaLLMConfig;
use crate::errors::app_error::AppError;
use crate::configuration::args::{ProcessArgs, AiProvider}; // Added
use serde_json;

pub async fn get_ai_reordering_plan(
    file_names: Vec<&String>,
    args: &ProcessArgs,
    ollama_config: &OllamaLLMConfig, // Still needed for Ollama path
) -> Result<String, AppError> {
    match args.ai_provider {
        AiProvider::Local => {
            let model_name = args.ollama_llm_model.as_ref().ok_or(AppError::OllamaLlmModelMissing)?.clone();
            ask_ollama_for_reordering_plan(
                file_names,
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
            client.generate_openai_folder_name(file_names).await
        }
    }
}

async fn ask_ollama_for_reordering_plan(
    file_names: Vec<&String>,
    ollama_model_name: String,
    ollama_server_address: String,
    ollama_config: OllamaLLMConfig,
) -> Result<String, AppError> {
    let file_data_json = serde_json::to_string_pretty(&file_names)
        .map_err(|e| AppError::JSONStringify(e.to_string()))?;
    
    let prompt_template = read_prompt(); // This is Ollama's prompt structure
    let prompt_with_input = format!("{}\n{}", prompt_template, file_data_json);
    
    generate_ollama_answer(prompt_with_input, ollama_model_name, ollama_server_address, ollama_config).await
}

async fn generate_ollama_answer(
    prompt: String,
    model: String,
    ollama_server_address: String,
    config: OllamaLLMConfig,
) -> Result<String, AppError> {
    let client = ReqwestClient::new();
    let options = OllamaModelOptions {
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
        prompt,
        stream: true, // Current implementation streams
        options: &options,
    };

    let mut response_text = String::new();
    let mut thinking_is_over = false; 

    let mut endpoint = ollama_server_address.clone();
    endpoint.push_str("/api/generate");

    let mut ollama_response_stream = client
        .post(endpoint)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| AppError::OllamaConnection(e.to_string()))?;

    while let Some(chunk) = ollama_response_stream
        .chunk()
        .await
        .map_err(|e| AppError::OllamaConnection(e.to_string()))?
    {
        let olama_response_token: OllamaLLMResponse = serde_json::from_slice::<OllamaLLMResponse>(&chunk)
            .map_err(|e| AppError::OllamaResponseParse(e.to_string()))?;

        if olama_response_token.response.is_empty() {
            continue;
        }
        if thinking_is_over { 
            response_text.push_str(&olama_response_token.response);
        }
        if olama_response_token.response == "</think>" { // Assuming this tag is significant
            thinking_is_over = true;
        } else if !thinking_is_over && !read_prompt().contains("</think>") { // If no think tag expected, append directly
             response_text.push_str(&olama_response_token.response);
        }
    }
    Ok(clean_json_string(&response_text))
}

fn clean_json_string(input: &str) -> String {
    let trimmed_input = input.trim();
    if let Some(start_index) = trimmed_input.find("```json") {
        if let Some(json_block_content) = trimmed_input.get(start_index + 7..) {
            if let Some(end_index) = json_block_content.rfind("```") {
                 return json_block_content[..end_index].trim().to_string();
            }
            return json_block_content.trim().to_string();
        }
    }
    trimmed_input.to_string()
}
