//! Benchmark runner - orchestrates benchmark execution.

use super::metrics::AggregatedMetrics;
use super::prompts::{LONG_PROMPT, MEDIUM_PROMPT, SHORT_PROMPT, TestPrompt};
use crate::cli::PromptSize;
use crate::providers::{InferenceProvider, InferenceRequest, InferenceResponse, ProviderError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Configuration for a benchmark run
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// Number of iterations per provider
    pub iterations: u32,
    /// Test prompt size
    pub prompt_size: PromptSize,
    /// Request timeout in milliseconds
    pub timeout_ms: u64,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            iterations: 1,
            prompt_size: PromptSize::Short,
            timeout_ms: 60_000,
        }
    }
}

/// Result from a single benchmark iteration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleRunResult {
    pub time_to_prompt_ms: u64,
    pub time_to_first_token_ms: u64,
    pub total_latency_ms: u64,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub cost_usd: f64,
    pub model_load_time_ms: Option<u64>,
}

impl SingleRunResult {
    /// Calculate tokens per second (output throughput)
    pub fn tokens_per_sec(&self) -> f64 {
        if self.total_latency_ms == 0 {
            return 0.0;
        }
        self.output_tokens as f64 / (self.total_latency_ms as f64 / 1000.0)
    }

    /// Create from inference response with pricing
    pub fn from_response(
        response: &InferenceResponse,
        input_price: f64,
        output_price: f64,
    ) -> Self {
        let input_cost = (response.input_tokens as f64 / 1_000_000.0) * input_price;
        let output_cost = (response.output_tokens as f64 / 1_000_000.0) * output_price;

        Self {
            time_to_prompt_ms: response.time_to_prompt_ms,
            time_to_first_token_ms: response.time_to_first_token_ms,
            total_latency_ms: response.total_latency_ms,
            input_tokens: response.input_tokens,
            output_tokens: response.output_tokens,
            cost_usd: input_cost + output_cost,
            model_load_time_ms: response.model_load_time_ms,
        }
    }
}

/// Complete benchmark result for a single provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Provider identifier
    pub provider: String,
    /// Provider display name
    pub display_name: String,
    /// Model used
    pub model: String,
    /// Aggregated metrics
    pub metrics: AggregatedMetrics,
    /// Raw results from each iteration
    pub raw_results: Vec<SingleRunResult>,
    /// Errors encountered
    pub errors: Vec<String>,
    /// Timestamp of benchmark
    pub timestamp: DateTime<Utc>,
}

impl BenchmarkResult {
    /// Check if benchmark was successful (at least one good run)
    pub fn is_success(&self) -> bool {
        !self.raw_results.is_empty()
    }
}

/// Benchmark runner - executes benchmarks across providers
pub struct BenchmarkRunner<'a> {
    providers: Vec<&'a dyn InferenceProvider>,
    config: BenchmarkConfig,
}

impl<'a> BenchmarkRunner<'a> {
    /// Create a new benchmark runner
    pub fn new(providers: Vec<&'a dyn InferenceProvider>, config: BenchmarkConfig) -> Self {
        Self { providers, config }
    }

    /// Run benchmarks across all providers sequentially
    pub async fn run(&self) -> Vec<BenchmarkResult> {
        let mut results = Vec::new();

        for provider in &self.providers {
            let result = self.benchmark_provider(*provider).await;
            results.push(result);
        }

        results
    }

    /// Benchmark a single provider
    async fn benchmark_provider(&self, provider: &dyn InferenceProvider) -> BenchmarkResult {
        let prompt = self.get_prompt();
        let (input_price, output_price) = provider.pricing_per_million();

        let mut raw_results = Vec::new();
        let mut errors = Vec::new();

        // Check availability first
        if !provider.is_available().await {
            errors.push("Provider not available".to_string());
            return BenchmarkResult {
                provider: provider.name().to_string(),
                display_name: provider.display_name().to_string(),
                model: provider.default_model().to_string(),
                metrics: AggregatedMetrics::from_raw(&[]),
                raw_results,
                errors,
                timestamp: Utc::now(),
            };
        }

        // Run benchmark iterations
        for i in 0..self.config.iterations {
            let request = InferenceRequest {
                prompt: prompt.text.to_string(),
                max_tokens: prompt.expected_output_tokens + 50, // Some buffer
                model: None,
            };

            match provider.infer(&request).await {
                Ok(response) => {
                    let result =
                        SingleRunResult::from_response(&response, input_price, output_price);
                    raw_results.push(result);
                }
                Err(e) => {
                    errors.push(format!("Iteration {}: {}", i + 1, e));
                    // For rate limiting, stop trying
                    if matches!(e, ProviderError::RateLimited) {
                        errors.push("Stopping due to rate limiting".to_string());
                        break;
                    }
                }
            }
        }

        BenchmarkResult {
            provider: provider.name().to_string(),
            display_name: provider.display_name().to_string(),
            model: provider.default_model().to_string(),
            metrics: AggregatedMetrics::from_raw(&raw_results),
            raw_results,
            errors,
            timestamp: Utc::now(),
        }
    }

    /// Get the test prompt based on configuration
    fn get_prompt(&self) -> &'static TestPrompt {
        match self.config.prompt_size {
            PromptSize::Short => &SHORT_PROMPT,
            PromptSize::Medium => &MEDIUM_PROMPT,
            PromptSize::Long => &LONG_PROMPT,
        }
    }

    /// Estimate total cost for the benchmark run
    pub fn estimate_cost(&self) -> f64 {
        let prompt = self.get_prompt();
        let mut total = 0.0;

        for provider in &self.providers {
            let (input_price, output_price) = provider.pricing_per_million();
            let per_run = prompt.estimate_cost(input_price, output_price);
            total += per_run * self.config.iterations as f64;
        }

        total
    }
}
