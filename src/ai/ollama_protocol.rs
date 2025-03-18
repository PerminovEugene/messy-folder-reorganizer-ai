use serde::{Deserialize, Serialize};

// model configuration params https://github.com/ollama/ollama/blob/main/docs/api.md
#[derive(Serialize)]
pub struct OllamaOptions {
    #[serde(skip_serializing_if = "Option::is_none")] // Top-k sampling value
    pub mirostat: Option<u8>, // 0 = disabled, 1 = Mirostat, 2 = Mirostat 2.0

    #[serde(skip_serializing_if = "Option::is_none")] // Top-k sampling value
    pub mirostat_eta: Option<f32>, // Learning rate for Mirostat

    #[serde(skip_serializing_if = "Option::is_none")] // Top-k sampling value
    pub mirostat_tau: Option<f32>, // Balance between coherence and diversity

    #[serde(skip_serializing_if = "Option::is_none")] // Top-k sampling value
    pub num_ctx: Option<u32>, // Context window size

    #[serde(skip_serializing_if = "Option::is_none")] // Top-k sampling value
    pub repeat_last_n: Option<i32>, // Lookback to prevent repetition

    #[serde(skip_serializing_if = "Option::is_none")] // Top-k sampling value
    pub repeat_penalty: Option<f32>, // Repetition penalty strength

    #[serde(skip_serializing_if = "Option::is_none")] // Top-k sampling value
    pub temperature: Option<f32>, // Model temperature

    #[serde(skip_serializing_if = "Option::is_none")] // Top-k sampling value
    pub seed: Option<u32>, // Random seed for generation

    #[serde(skip_serializing_if = "Option::is_none")] // Top-k sampling value
    pub stop: Option<String>, // Stop sequences

    #[serde(skip_serializing_if = "Option::is_none")] // Top-k sampling value
    pub num_predict: Option<i32>, // Maximum number of tokens to predict

    #[serde(skip_serializing_if = "Option::is_none")] // Top-k sampling value
    pub top_k: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")] // Top-k sampling value
    pub top_p: Option<f32>, // Top-p sampling value

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_p: Option<f32>, // Minimum probability threshold
}

#[derive(Serialize)]
pub struct OllamaGenerateRequest<'a> {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    pub options: &'a OllamaOptions,
}

#[derive(Serialize)]
pub struct OllamaEmbedRequest<'a> {
    pub model: String,
    pub input: &'a Vec<String>,
    pub options: &'a OllamaOptions,
    // pub format: String,
}

#[derive(Deserialize)]
pub struct OllamaResponse {
    pub response: String,
    // model: String,
    // created_at: String,
    // done: bool,
}

#[derive(Deserialize)]
pub struct OllamaEmbedResponse {
    pub embeddings: Vec<Vec<f32>>,
    // pub total_duration: u64,
    // pub load_duration: u64,
    // pub prompt_eval_count: u64,
}
