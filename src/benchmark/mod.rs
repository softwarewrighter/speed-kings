//! Benchmark engine for running inference tests.

mod metrics;
mod prompts;
mod runner;

pub use metrics::AggregatedMetrics;
pub use prompts::{LONG_PROMPT, MEDIUM_PROMPT, SHORT_PROMPT, TestPrompt};
pub use runner::{BenchmarkConfig, BenchmarkResult, BenchmarkRunner, SingleRunResult};
