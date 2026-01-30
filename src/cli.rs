//! Command-line interface definitions using clap.

use clap::{Parser, Subcommand, ValueEnum};

/// LLM inference benchmarking tool - compare speed, latency, and cost across providers
#[derive(Parser, Debug)]
#[command(name = "speed-kings")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run inference benchmarks across providers
    Benchmark {
        /// Providers to benchmark (comma-separated, or "all")
        #[arg(short, long, default_value = "all")]
        providers: String,

        /// Number of iterations per provider
        #[arg(short, long, default_value = "1")]
        iterations: u32,

        /// Test prompt size
        #[arg(short, long, default_value = "short", value_enum)]
        size: PromptSize,

        /// Output format
        #[arg(short, long, default_value = "table", value_enum)]
        output: OutputFormat,

        /// Skip cost confirmation prompt
        #[arg(long)]
        yes: bool,
    },

    /// List available providers and their status
    List,

    /// Show pricing information for all providers
    Pricing,
}

/// Test prompt size - affects token count and cost
#[derive(ValueEnum, Clone, Debug, Copy, PartialEq, Eq)]
pub enum PromptSize {
    /// ~50 output tokens, minimal cost
    Short,
    /// ~200 output tokens, typical interaction
    Medium,
    /// ~500 output tokens, extended response
    Long,
}

impl PromptSize {
    /// Expected output tokens for this prompt size
    pub fn expected_output_tokens(&self) -> u32 {
        match self {
            PromptSize::Short => 50,
            PromptSize::Medium => 200,
            PromptSize::Long => 500,
        }
    }
}

/// Output format for benchmark results
#[derive(ValueEnum, Clone, Debug, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Terminal table with aligned columns
    Table,
    /// JSON for programmatic consumption
    Json,
    /// Markdown for documentation
    Markdown,
    /// CSV for spreadsheets
    Csv,
}
