use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub mirostat: Option<u8>,
    pub mirostat_eta: Option<f32>,
    pub mirostat_tau: Option<f32>,
    pub num_ctx: Option<u32>,
    pub repeat_last_n: Option<i32>,
    pub repeat_penalty: Option<f32>,
    pub temperature: Option<f32>,
    pub seed: Option<u32>,
    pub stop: Option<String>,
    pub num_predict: Option<i32>,
    pub top_k: Option<u32>,
    pub top_p: Option<f32>,
    pub min_p: Option<f32>,
}
