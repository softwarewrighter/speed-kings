//! Speed Kings - LLM Inference Benchmarking Library
//!
//! This library provides the core functionality for benchmarking LLM inference
//! across multiple providers including Cerebras, Groq, Fireworks, and local
//! inference via Ollama.

pub mod benchmark;
pub mod cli;
pub mod output;
pub mod pricing;
pub mod providers;

pub use benchmark::{BenchmarkConfig, BenchmarkResult, BenchmarkRunner};
pub use cli::{Cli, Commands, OutputFormat, PromptSize};
pub use providers::{InferenceProvider, InferenceRequest, InferenceResponse, ProviderError};
