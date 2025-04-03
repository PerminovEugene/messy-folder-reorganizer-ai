use crate::errors::app_error::AppError;

pub fn get_dimension_size_by_vectors(vectors: &[Vec<f32>]) -> Result<u64, AppError> {
    let dimension = vectors
        .first()
        .ok_or_else(|| AppError::QdrantCustom("Vectors are empty, nothing to insert".to_string()))?
        .len();

    Ok(dimension as u64)
}
