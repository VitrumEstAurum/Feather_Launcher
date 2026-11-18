use thiserror::Error;
#[derive(Error, Debug)]
pub enum FeatherError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("Provider error: {0}")]
    Provider(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal error: {0}")]
    Internal(String)
}

pub type FeatherResult<T> = Result<T, FeatherError>;