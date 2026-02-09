mod cli;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, Commands};
use taran_config::Scenario;
use taran_core::runner::TestRunner;
use taran_report::ConsoleReporter;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt().with_env_filter(log_level).init();

    match cli.command {
        Commands::Run { scenario, users, duration } => {
            info!("Loading scenario from: {}", scenario.display());

            let config = Scenario::from_file(&scenario)
                .with_context(|| format!("Failed to load scenario from {}", scenario.display()))?;

            // Override configuration if specified
            if let Some(user_count) = users {
                info!("Overriding VU count to: {}", user_count);
                // TODO: Override users in config
            }

            if let Some(dur) = duration {
                info!("Overriding duration to: {}", dur);
                // TODO: Override duration in config
            }

            config.validate().context("Scenario validation failed")?;

            info!("Running load test: {}", config.scenario.name);

            let runner = TestRunner::new(config);
            let summary = runner.run().await.context("Test execution failed")?;

            let reporter = ConsoleReporter::new();
            reporter.print_summary(&summary).context("Failed to print summary")?;

            // Exit with error code if there were failures
            if summary.failed_requests > 0 {
                std::process::exit(1);
            }
        }

        Commands::Validate { scenario } => {
            info!("Validating scenario: {}", scenario.display());

            let config = Scenario::from_file(&scenario)
                .with_context(|| format!("Failed to load scenario from {}", scenario.display()))?;

            config.validate().context("Scenario validation failed")?;

            println!("✓ Scenario is valid");
        }
    }

    Ok(())
}
