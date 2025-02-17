use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OllamaRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    // model configuration params https://github.com/ollama/ollama/blob/main/docs/api.md
    pub mirostat: u8,        // 0 = disabled, 1 = Mirostat, 2 = Mirostat 2.0
    pub mirostat_eta: f32,   // Learning rate for Mirostat
    pub mirostat_tau: f32,   // Balance between coherence and diversity
    pub num_ctx: u32,        // Context window size
    pub repeat_last_n: i32,  // Lookback to prevent repetition
    pub repeat_penalty: f32, // Repetition penalty strength
    pub temperature: f32,    // Model temperature
    pub seed: u32,           // Random seed for generation
    pub stop: String,        // Stop sequences
    pub num_predict: i32,    // Maximum number of tokens to predict
    pub top_k: u32,          // Top-k sampling value
    pub top_p: f32,          // Top-p sampling value
    pub min_p: f32,          // Minimum probability threshold
}

#[derive(Deserialize)]
pub struct OllamaResponse {
    pub response: String,
    // model: String,
    // created_at: String,
    // done: bool,
}
