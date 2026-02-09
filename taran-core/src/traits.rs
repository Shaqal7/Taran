use crate::{error::Result, model::StepResult};
use async_trait::async_trait;

/// Trait for protocol client implementations
#[async_trait]
pub trait Protocol: Send + Sync {
    /// Execute a request and return the result
    async fn execute(&self, context: &mut crate::model::VirtualUserContext) -> Result<StepResult>;
}

/// Trait for load profile strategies
pub trait LoadProfile: Send + Sync {
    /// Calculate the number of VUs at a given time offset from test start
    fn virtual_users_at(&self, elapsed: std::time::Duration) -> usize;

    /// Total duration of the test
    fn duration(&self) -> std::time::Duration;
}

/// Trait for metrics collection
pub trait MetricsCollector: Send + Sync {
    /// Record a step result
    fn record(&self, result: &StepResult);

    /// Get a snapshot of current metrics
    fn snapshot(&self) -> MetricsSnapshot;
}

/// Snapshot of metrics at a point in time
#[derive(Debug, Clone)]
pub struct MetricsSnapshot {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub total_duration_ms: u64,
}
