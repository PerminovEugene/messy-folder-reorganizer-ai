use crate::console::errors::print_app_error;

use super::app_error::AppError;

pub fn handle(e: AppError) {
    match e {
        AppError::OllamaConnection(_) => {
            print_app_error("Ollama Error", e);
            std::process::exit(1);
        }
        AppError::OllamaResponseParse(_) => {
            print_app_error("Ollama Response Error", e);
            std::process::exit(1);
        }
        AppError::QdrantClient(_) => {
            print_app_error("Qdrant Error", e);
            std::process::exit(1);
        }
        AppError::JSONStringify(_) => {
            print_app_error("JSON stringify error", e);
            std::process::exit(1);
        }
        AppError::FileError(_) => {
            print_app_error("File processing error", e);
            std::process::exit(1);
        }
        _ => {
            print_app_error("Panic", e);
            panic!("Unhandled error.\nPlease report this on GitHub:\nhttps://github.com/PerminovEugene/messy-folder-reorganizer-ai/issues");
        }
    }
}
