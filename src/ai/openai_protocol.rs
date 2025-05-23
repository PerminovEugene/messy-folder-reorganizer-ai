use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct OpenAIChatCompletionRequest<'a> {
    pub model: &'a str,
    pub messages: Vec<OpenAIChatMessage<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<OpenAIResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

#[derive(Serialize, Debug, Clone)] // Added Clone
pub struct OpenAIChatMessage<'a> {
    pub role: &'a str, // "system", "user", or "assistant"
    pub content: String,
}

#[derive(Serialize, Debug, Clone)] // Added Clone
pub struct OpenAIResponseFormat {
    #[serde(rename = "type")]
    pub format_type: String, // e.g., "json_object"
}

#[derive(Deserialize, Debug)]
pub struct OpenAIChatCompletionResponse {
    pub model: String,
    pub choices: Vec<OpenAIChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<OpenAIUsage>,
}

#[derive(Deserialize, Debug)]
pub struct OpenAIChoice {
    pub message: OpenAIMessageContent,
}

#[derive(Deserialize, Debug)]
pub struct OpenAIMessageContent {
    pub content: String,
}

#[derive(Serialize, Debug)]
pub struct OpenAIEmbeddingRequest<'a> {
    pub input: &'a Vec<String>,
    pub model: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<String>, // "float" or "base64"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct OpenAIEmbeddingResponse {
    pub data: Vec<OpenAIEmbeddingData>,
    pub model: String,
    pub usage: OpenAIUsage,
}

#[derive(Deserialize, Debug)]
pub struct OpenAIEmbeddingData {
    pub embedding: Vec<f32>,
    pub index: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OpenAIUsage {
    pub prompt_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens: Option<u32>,
    pub total_tokens: u32,
}

#[derive(Deserialize, Debug)]
pub struct OpenAIErrorResponseWrapper {
    pub error: OpenAIErrorDetail,
}

#[derive(Deserialize, Debug)]
pub struct OpenAIErrorDetail {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    pub param: Option<String>,
    pub code: Option<String>,
}
