use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Simple metrics collector using Mutex<HashMap>
/// This is Phase 0 implementation - will be replaced with lock-free HDR in Phase 3
#[derive(Debug, Clone)]
pub struct SimpleCollector {
    inner: Arc<Mutex<CollectorInner>>,
}

#[derive(Debug)]
struct CollectorInner {
    total_requests: u64,
    successful_requests: u64,
    failed_requests: u64,
    total_bytes_sent: u64,
    total_bytes_received: u64,
    latencies: Vec<Duration>,
    errors: HashMap<String, u64>,
    step_metrics: HashMap<String, StepMetrics>,
}

#[derive(Debug, Clone, Default)]
struct StepMetrics {
    requests: u64,
    successes: u64,
    failures: u64,
    total_latency_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSummary {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub success_rate: f64,
    pub total_bytes_sent: u64,
    pub total_bytes_received: u64,
    pub avg_latency_ms: f64,
    pub min_latency_ms: u64,
    pub max_latency_ms: u64,
    pub p50_latency_ms: u64,
    pub p95_latency_ms: u64,
    pub p99_latency_ms: u64,
    pub errors_by_type: HashMap<String, u64>,
}

impl SimpleCollector {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(CollectorInner {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                total_bytes_sent: 0,
                total_bytes_received: 0,
                latencies: Vec::new(),
                errors: HashMap::new(),
                step_metrics: HashMap::new(),
            })),
        }
    }

    /// Record a successful request
    pub fn record_success(
        &self,
        step_name: &str,
        latency: Duration,
        bytes_sent: u64,
        bytes_received: u64,
    ) {
        let mut inner = self.inner.lock().unwrap();
        inner.total_requests += 1;
        inner.successful_requests += 1;
        inner.total_bytes_sent += bytes_sent;
        inner.total_bytes_received += bytes_received;
        inner.latencies.push(latency);

        let metrics = inner.step_metrics.entry(step_name.to_string()).or_default();
        metrics.requests += 1;
        metrics.successes += 1;
        metrics.total_latency_ms += latency.as_millis() as u64;
    }

    /// Record a failed request
    pub fn record_failure(&self, step_name: &str, error: &str, latency: Duration) {
        let mut inner = self.inner.lock().unwrap();
        inner.total_requests += 1;
        inner.failed_requests += 1;
        inner.latencies.push(latency);

        *inner.errors.entry(error.to_string()).or_insert(0) += 1;

        let metrics = inner.step_metrics.entry(step_name.to_string()).or_default();
        metrics.requests += 1;
        metrics.failures += 1;
        metrics.total_latency_ms += latency.as_millis() as u64;
    }

    /// Get a summary of all collected metrics
    pub fn summary(&self) -> MetricsSummary {
        let inner = self.inner.lock().unwrap();

        let mut sorted_latencies: Vec<u64> =
            inner.latencies.iter().map(|d| d.as_millis() as u64).collect();
        sorted_latencies.sort_unstable();

        let (avg, min, max, p50, p95, p99) = if sorted_latencies.is_empty() {
            (0.0, 0, 0, 0, 0, 0)
        } else {
            let sum: u64 = sorted_latencies.iter().sum();
            let avg = sum as f64 / sorted_latencies.len() as f64;
            let min = sorted_latencies[0];
            let max = sorted_latencies[sorted_latencies.len() - 1];
            let p50 = percentile(&sorted_latencies, 50.0);
            let p95 = percentile(&sorted_latencies, 95.0);
            let p99 = percentile(&sorted_latencies, 99.0);
            (avg, min, max, p50, p95, p99)
        };

        let success_rate = if inner.total_requests > 0 {
            (inner.successful_requests as f64 / inner.total_requests as f64) * 100.0
        } else {
            0.0
        };

        MetricsSummary {
            total_requests: inner.total_requests,
            successful_requests: inner.successful_requests,
            failed_requests: inner.failed_requests,
            success_rate,
            total_bytes_sent: inner.total_bytes_sent,
            total_bytes_received: inner.total_bytes_received,
            avg_latency_ms: avg,
            min_latency_ms: min,
            max_latency_ms: max,
            p50_latency_ms: p50,
            p95_latency_ms: p95,
            p99_latency_ms: p99,
            errors_by_type: inner.errors.clone(),
        }
    }

    /// Reset all metrics
    pub fn reset(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.total_requests = 0;
        inner.successful_requests = 0;
        inner.failed_requests = 0;
        inner.total_bytes_sent = 0;
        inner.total_bytes_received = 0;
        inner.latencies.clear();
        inner.errors.clear();
        inner.step_metrics.clear();
    }
}

impl Default for SimpleCollector {
    fn default() -> Self {
        Self::new()
    }
}

fn percentile(sorted_data: &[u64], p: f64) -> u64 {
    if sorted_data.is_empty() {
        return 0;
    }
    let index = ((p / 100.0) * sorted_data.len() as f64) as usize;
    let index = index.min(sorted_data.len() - 1);
    sorted_data[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collector_basic() {
        let collector = SimpleCollector::new();
        collector.record_success("test_step", Duration::from_millis(100), 1024, 2048);

        let summary = collector.summary();
        assert_eq!(summary.total_requests, 1);
        assert_eq!(summary.successful_requests, 1);
        assert_eq!(summary.failed_requests, 0);
    }

    #[test]
    fn test_collector_percentiles() {
        let collector = SimpleCollector::new();
        for i in 0..100 {
            collector.record_success("test", Duration::from_millis(i), 0, 0);
        }

        let summary = collector.summary();
        assert!(summary.p50_latency_ms >= 45 && summary.p50_latency_ms <= 55);
        assert!(summary.p95_latency_ms >= 90);
    }
}
