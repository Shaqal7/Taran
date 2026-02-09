use thiserror::Error;

#[derive(Error, Debug)]
pub enum MetricsError {
    #[error("Failed to record metric: {0}")]
    RecordFailed(String),

    #[error("Failed to aggregate metrics: {0}")]
    AggregationFailed(String),

    #[error("Invalid metric value: {0}")]
    InvalidValue(String),

    #[error("Histogram error: {0}")]
    HistogramError(String),
}

pub type Result<T> = std::result::Result<T, MetricsError>;
