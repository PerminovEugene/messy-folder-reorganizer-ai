use reqwest::{Client, header::{AUTHORIZATION, CONTENT_TYPE}};
use crate::ai::openai_protocol::*;
use crate::errors::app_error::AppError;
use crate::configuration::args::ProcessArgs; // For accessing OpenAI specific args

const OPENAI_SYSTEM_PROMPT_FOLDER_NAMING: &str = "You are an expert file organization assistant. Your task is to analyze the provided list of file names and extensions and determine the single most appropriate folder name that best categorizes all these files. The folder name should be broad enough to encompass all files while maintaining logical organization. Respond ONLY with a valid JSON object containing a single key \"folder_name\" and its string value. Do not include any other text, explanations, or markdown formatting around the JSON object.";

pub struct OpenAIClient {
    http_client: Client,
    api_key: String,
    api_base_url: String,
    llm_model: String,
    embedding_model: String,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    embedding_dimensions: Option<u32>,
}

impl OpenAIClient {
    pub fn new(args: &ProcessArgs, api_key: String) -> Self {
        Self {
            http_client: Client::new(),
            api_key,
            api_base_url: args.openai_api_base.clone(),
            llm_model: args.openai_llm_model.clone(),
            embedding_model: args.openai_embedding_model.clone(),
            temperature: args.openai_temperature,
            max_tokens: args.openai_max_tokens,
            embedding_dimensions: args.openai_embedding_dimensions,
        }
    }

    async fn handle_error_response(response: reqwest::Response) -> AppError {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());

        if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
            return AppError::OpenAIAuthError(format!("Status: {}, Body: {}", status, error_text));
        }

        if let Ok(api_error_wrapper) = serde_json::from_str::<OpenAIErrorResponseWrapper>(&error_text) {
            AppError::OpenAIServiceError {
                message: api_error_wrapper.error.message,
                error_type: api_error_wrapper.error.error_type,
                code: api_error_wrapper.error.code,
            }
        } else {
            AppError::OpenAIAPIError(format!("API request failed with status {}: {}", status, error_text))
        }
    }

    pub async fn generate_openai_embeddings(&self, texts: &Vec<String>) -> Result<Vec<Vec<f32>>, AppError> {
        let request_payload = OpenAIEmbeddingRequest {
            input: texts,
            model: &self.embedding_model,
            encoding_format: Some("float".to_string()),
            dimensions: self.embedding_dimensions,
        };

        let endpoint = format!("{}/embeddings", self.api_base_url);
        
        let response = self.http_client
            .post(&endpoint)
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(CONTENT_TYPE, "application/json")
            .json(&request_payload)
            .send()
            .await
            .map_err(|e| AppError::OpenAIAPIError(format!("Network error during OpenAI embedding: {}", e)))?;

        if !response.status().is_success() {
            return Err(Self::handle_error_response(response).await);
        }

        let parsed_response: OpenAIEmbeddingResponse = response.json().await
            .map_err(|e| AppError::OpenAIResponseParseError(format!("Parsing embedding response: {}", e)))?;
        
        Ok(parsed_response.data.into_iter().map(|d| d.embedding).collect())
    }

    pub async fn generate_openai_folder_name(&self, file_names: Vec<&String>) -> Result<String, AppError> {
        let file_list_json_array = serde_json::to_string(&file_names)
            .map_err(|e| AppError::JSONStringify(format!("Failed to serialize file names for OpenAI: {}", e)))?;

        let user_content = format!("Analyze these file names: {}", file_list_json_array);

        let messages = vec![
            OpenAIChatMessage {
                role: "system",
                content: OPENAI_SYSTEM_PROMPT_FOLDER_NAMING.to_string(),
            },
            OpenAIChatMessage {
                role: "user",
                content: user_content,
            },
        ];

        let request_payload = OpenAIChatCompletionRequest {
            model: &self.llm_model,
            messages,
            response_format: Some(OpenAIResponseFormat {
                format_type: "json_object".to_string(),
            }),
            temperature: self.temperature.or(Some(0.7)), // Default if not provided by user
            max_tokens: self.max_tokens.or(Some(150)), // Default if not provided by user
        };
        
        let endpoint = format!("{}/chat/completions", self.api_base_url);

        let response = self.http_client
            .post(&endpoint)
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(CONTENT_TYPE, "application/json")
            .json(&request_payload)
            .send()
            .await
            .map_err(|e| AppError::OpenAIAPIError(format!("Network error during OpenAI chat completion: {}", e)))?;

        if !response.status().is_success() {
            return Err(Self::handle_error_response(response).await);
        }

        let parsed_response: OpenAIChatCompletionResponse = response.json().await
            .map_err(|e| AppError::OpenAIResponseParseError(format!("Parsing chat completion response: {}", e)))?;

        parsed_response.choices.get(0)
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| AppError::OpenAIResponseParseError("No content in OpenAI chat completion response".to_string()))
    }
}
