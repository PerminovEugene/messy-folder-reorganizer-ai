// use serde::Deserialize;
// use serde_json;
// use std::{error::Error, str::Bytes};

// #[derive(Deserialize)]
// struct OllamaResponse {
//     response: String,
// }

// pub fn parse_json_response(body: Bytes) -> Result<String, Box<dyn Error>> {
//     // Attempt to deserialize the JSON response
//     let json_data: OllamaResponse = serde_json::from_slice(&body)?;

//     // Extract the `response` field and return it
//     Ok(json_data.response)
// }
