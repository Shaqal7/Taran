pub mod collector;
pub mod error;

pub use collector::{MetricsSummary, SimpleCollector};
pub use error::{MetricsError, Result};
