use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_mirostat")]
    pub mirostat: u8,
    #[serde(default = "default_mirostat_eta")]
    pub mirostat_eta: f32,
    #[serde(default = "default_mirostat_tau")]
    pub mirostat_tau: f32,
    #[serde(default = "default_num_ctx")]
    pub num_ctx: u32,
    #[serde(default = "default_repeat_last_n")]
    pub repeat_last_n: i32,
    #[serde(default = "default_repeat_penalty")]
    pub repeat_penalty: f32,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    #[serde(default = "default_seed")]
    pub seed: u32,
    #[serde(default = "default_stop")]
    pub stop: String,
    #[serde(default = "default_num_predict")]
    pub num_predict: i32,
    #[serde(default = "default_top_k")]
    pub top_k: u32,
    #[serde(default = "default_top_p")]
    pub top_p: f32,
    #[serde(default = "default_min_p")]
    pub min_p: f32,
}

// Implement the Default trait for Config
impl Default for Config {
    fn default() -> Self {
        Self {
            mirostat: default_mirostat(),
            mirostat_eta: default_mirostat_eta(),
            mirostat_tau: default_mirostat_tau(),
            num_ctx: default_num_ctx(),
            repeat_last_n: default_repeat_last_n(),
            repeat_penalty: default_repeat_penalty(),
            temperature: default_temperature(),
            seed: default_seed(),
            stop: default_stop(),
            num_predict: default_num_predict(),
            top_k: default_top_k(),
            top_p: default_top_p(),
            min_p: default_min_p(),
        }
    }
}

// Define default functions
fn default_mirostat() -> u8 {
    0
}
fn default_mirostat_eta() -> f32 {
    0.1
}
fn default_mirostat_tau() -> f32 {
    5.0
}
fn default_num_ctx() -> u32 {
    4096
}
fn default_repeat_last_n() -> i32 {
    256
}
fn default_repeat_penalty() -> f32 {
    1.2
}
fn default_temperature() -> f32 {
    0.3
}
fn default_seed() -> u32 {
    42
}
fn default_stop() -> String {
    "\n\n".to_string()
}
fn default_num_predict() -> i32 {
    -1
}
fn default_top_k() -> u32 {
    20
}
fn default_top_p() -> f32 {
    0.7
}
fn default_min_p() -> f32 {
    0.05
}
