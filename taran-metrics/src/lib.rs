pub mod error;
pub mod collector;

pub use error::{MetricsError, Result};
pub use collector::{SimpleCollector, MetricsSummary};
