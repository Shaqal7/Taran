use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "taran")]
#[command(author = "Taran Contributors")]
#[command(version)]
#[command(about = "High-performance load testing tool written in Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a load test scenario
    Run {
        /// Path to the scenario file (TOML)
        #[arg(value_name = "SCENARIO")]
        scenario: PathBuf,

        /// Override number of virtual users
        #[arg(short = 'u', long)]
        users: Option<usize>,

        /// Override test duration (e.g., "60s", "5m")
        #[arg(short = 'd', long)]
        duration: Option<String>,
    },

    /// Validate a scenario file
    Validate {
        /// Path to the scenario file to validate
        #[arg(value_name = "SCENARIO")]
        scenario: PathBuf,
    },
}
