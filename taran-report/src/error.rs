use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReportError {
    #[error("Failed to generate report: {0}")]
    GenerationFailed(String),

    #[error("Failed to write report: {0}")]
    WriteFailed(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, ReportError>;
