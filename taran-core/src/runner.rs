use crate::error::Result;
use crate::model::{StepResult, VirtualUserContext};
use std::sync::Arc;
use std::time::Duration;
use taran_config::{Assertions, Scenario};
use taran_metrics::{MetricsSummary, SimpleCollector};
use taran_protocols::{HttpClient, HttpRequest};
use tracing::{debug, info, warn};

/// Test runner - Phase 0 implementation with 1 VU, sequential execution
pub struct TestRunner {
    scenario: Scenario,
    collector: Arc<SimpleCollector>,
}

impl TestRunner {
    pub fn new(scenario: Scenario) -> Self {
        Self { scenario, collector: Arc::new(SimpleCollector::new()) }
    }

    /// Run the load test
    pub async fn run(&self) -> Result<MetricsSummary> {
        info!("Starting test: {}", self.scenario.scenario.name);

        // Phase 0: Simple implementation with 1 VU
        // TODO: In Phase 1, this will spawn multiple VUs based on load_profile

        let iterations = 10; // Hard-coded for Phase 0
        let mut context = VirtualUserContext::new(0);

        for i in 0..iterations {
            context.iteration = i;
            info!("VU 0, iteration {i}");

            for step in &self.scenario.steps {
                let result = self.execute_step(step, &mut context).await;
                self.record_result(&result);
            }

            // Small delay between iterations
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        info!("Test completed");
        Ok(self.collector.summary())
    }

    async fn execute_step(
        &self,
        step: &taran_config::Step,
        _context: &mut VirtualUserContext,
    ) -> StepResult {
        debug!("Executing step: {}", step.name);

        let start = std::time::Instant::now();

        // Only HTTP is supported in Phase 0
        if step.protocol.to_lowercase() != "http" {
            warn!("Unsupported protocol: {}", step.protocol);
            return StepResult {
                step_name: step.name.clone(),
                success: false,
                duration: start.elapsed(),
                error: Some(format!("Unsupported protocol: {}", step.protocol)),
                status_code: None,
                bytes_sent: 0,
                bytes_received: 0,
            };
        }

        let client = match HttpClient::new() {
            Ok(c) => c,
            Err(e) => {
                return StepResult {
                    step_name: step.name.clone(),
                    success: false,
                    duration: start.elapsed(),
                    error: Some(format!("Failed to create HTTP client: {e}")),
                    status_code: None,
                    bytes_sent: 0,
                    bytes_received: 0,
                };
            }
        };

        let request = HttpRequest {
            method: step.method.clone(),
            url: step.url.clone(),
            headers: step.headers.clone(),
            body: step.body.clone(),
            timeout: None,
        };

        match client.execute(request).await {
            Ok(response) => {
                let success = response.is_success();
                let duration = response.duration;

                // Check assertions if present
                let assertion_error = step
                    .assertions
                    .as_ref()
                    .and_then(|assertions| check_assertions(assertions, &response));

                // TODO: Extract variables if extractors are defined
                // This will be implemented in Phase 1

                StepResult {
                    step_name: step.name.clone(),
                    success: success && assertion_error.is_none(),
                    duration,
                    error: assertion_error,
                    status_code: Some(response.status),
                    bytes_sent: response.bytes_sent,
                    bytes_received: response.bytes_received,
                }
            }
            Err(e) => StepResult {
                step_name: step.name.clone(),
                success: false,
                duration: start.elapsed(),
                error: Some(format!("Request failed: {e}")),
                status_code: None,
                bytes_sent: 0,
                bytes_received: 0,
            },
        }
    }

    fn record_result(&self, result: &StepResult) {
        if result.success {
            self.collector.record_success(
                &result.step_name,
                result.duration,
                result.bytes_sent,
                result.bytes_received,
            );
        } else {
            self.collector.record_failure(
                &result.step_name,
                result.error.as_deref().unwrap_or("Unknown error"),
                result.duration,
            );
        }
    }
}

fn check_assertions(
    assertions: &Assertions,
    response: &taran_protocols::HttpResponse,
) -> Option<String> {
    // Check status code
    if let Some(expected_status) = assertions.status {
        if response.status != expected_status {
            return Some(format!("Expected status {}, got {}", expected_status, response.status));
        }
    }

    // Check max response time
    if let Some(ref max_time) = assertions.max_response_time {
        if response.duration > max_time.as_duration() {
            return Some(format!(
                "Response time {}ms exceeded max {}ms",
                response.duration.as_millis(),
                max_time.as_duration().as_millis()
            ));
        }
    }

    // Check body contains
    if let Some(expected_text) = &assertions.body_contains {
        if !response.body.contains(expected_text) {
            return Some(format!("Response body does not contain '{expected_text}'"));
        }
    }

    None
}
