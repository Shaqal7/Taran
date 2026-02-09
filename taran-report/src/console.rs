use crate::error::Result;
use taran_metrics::MetricsSummary;

/// Simple console reporter that prints metrics to stdout
pub struct ConsoleReporter;

impl ConsoleReporter {
    pub const fn new() -> Self {
        Self
    }

    /// Print a summary of test results to the console
    pub fn print_summary(&self, summary: &MetricsSummary) -> Result<()> {
        println!("\n{}", "=".repeat(60));
        println!("  Test Results Summary");
        println!("{}", "=".repeat(60));
        println!();

        println!("Requests:");
        println!("  Total:      {}", summary.total_requests);
        println!("  Successful: {}", summary.successful_requests);
        println!("  Failed:     {}", summary.failed_requests);
        println!("  Success Rate: {:.2}%", summary.success_rate);
        println!();

        println!("Latency (ms):");
        println!("  Average: {:.2}", summary.avg_latency_ms);
        println!("  Min:     {}", summary.min_latency_ms);
        println!("  Max:     {}", summary.max_latency_ms);
        println!("  p50:     {}", summary.p50_latency_ms);
        println!("  p95:     {}", summary.p95_latency_ms);
        println!("  p99:     {}", summary.p99_latency_ms);
        println!();

        println!("Data Transfer:");
        println!("  Sent:     {} bytes", summary.total_bytes_sent);
        println!("  Received: {} bytes", summary.total_bytes_received);
        println!();

        if !summary.errors_by_type.is_empty() {
            println!("Errors:");
            for (error_type, count) in &summary.errors_by_type {
                println!("  {error_type}: {count}");
            }
            println!();
        }

        println!("{}", "=".repeat(60));

        Ok(())
    }
}

impl Default for ConsoleReporter {
    fn default() -> Self {
        Self::new()
    }
}
