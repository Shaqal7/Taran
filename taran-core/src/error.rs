use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Test execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Virtual user error: {0}")]
    VirtualUserError(String),

    #[error("Protocol error: {0}")]
    ProtocolError(#[from] crate::protocols::ProtocolError),

    #[error("Configuration error: {0}")]
    ConfigError(#[from] taran_config::ConfigError),

    #[error("Metrics error: {0}")]
    MetricsError(#[from] taran_metrics::MetricsError),

    #[error("Tokio runtime error: {0}")]
    RuntimeError(String),
}

pub type Result<T> = std::result::Result<T, CoreError>;
