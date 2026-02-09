use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("HTTP request failed: {0}")]
    HttpRequestFailed(String),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, ProtocolError>;
