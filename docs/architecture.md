# Architecture

> Technical architecture for the Speed Kings inference benchmarking tool.

## System Overview

Speed Kings is a Rust CLI application that benchmarks LLM inference across multiple providers. The architecture prioritizes modularity, testability, and extensibility.

```
+------------------+
|   CLI Interface  |  <-- clap argument parsing
+------------------+
         |
         v
+------------------+
|  Benchmark Engine|  <-- Orchestrates test runs
+------------------+
         |
    +----+----+
    |         |
    v         v
+-------+ +--------+
|Metrics| |Provider|  <-- Trait-based provider abstraction
+-------+ |Registry|
    |     +--------+
    |         |
    v         v
+------------------+
|    Providers     |  <-- Cerebras, Groq, Local, etc.
+------------------+
         |
         v
+------------------+
|  Output Formats  |  <-- Table, JSON, Markdown, CSV
+------------------+
```

## Directory Structure

```
speed-kings/
|-- Cargo.toml
|-- src/
|   |-- main.rs              # Entry point, CLI setup
|   |-- lib.rs               # Library exports
|   |-- cli.rs               # Argument parsing (clap)
|   |-- benchmark/
|   |   |-- mod.rs           # Benchmark engine
|   |   |-- runner.rs        # Test execution logic
|   |   |-- metrics.rs       # Metrics collection
|   |   +-- prompts.rs       # Test prompt definitions
|   |-- providers/
|   |   |-- mod.rs           # Provider trait + registry
|   |   |-- cerebras.rs      # Cerebras implementation
|   |   |-- groq.rs          # Groq implementation
|   |   |-- sambanova.rs     # SambaNova implementation
|   |   |-- fireworks.rs     # Fireworks implementation
|   |   |-- deepseek.rs      # DeepSeek implementation
|   |   |-- openai.rs        # OpenAI-compatible implementation
|   |   +-- local.rs         # Ollama/llama.cpp implementation
|   |-- output/
|   |   |-- mod.rs           # Output format selection
|   |   |-- table.rs         # Terminal table formatting
|   |   |-- json.rs          # JSON export
|   |   |-- markdown.rs      # Markdown export
|   |   +-- csv.rs           # CSV export
|   +-- pricing/
|       |-- mod.rs           # Pricing data management
|       +-- data.rs          # Provider pricing definitions
|-- tests/
|   |-- integration/
|   |   +-- benchmark_tests.rs
|   +-- providers/
|       +-- mock_provider.rs
|-- docs/
|   |-- prd.md
|   |-- architecture.md
|   |-- design.md
|   |-- plan.md
|   +-- status.md
+-- data/
    +-- pricing.json         # Updatable pricing data
```

## Core Components

### 1. CLI Interface (cli.rs)

Uses `clap` for argument parsing with derive macros.

```rust
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "speed-kings")]
#[command(about = "LLM inference benchmarking tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run inference benchmarks
    Benchmark {
        /// Providers to benchmark (comma-separated)
        #[arg(short, long, default_value = "all")]
        providers: String,

        /// Number of iterations per provider
        #[arg(short, long, default_value = "3")]
        iterations: u32,

        /// Test prompt size
        #[arg(short, long, default_value = "medium")]
        size: PromptSize,

        /// Output format
        #[arg(short, long, default_value = "table")]
        output: OutputFormat,
    },
    /// List available providers
    List,
    /// Show pricing information
    Pricing,
}

#[derive(ValueEnum, Clone)]
pub enum PromptSize {
    Short,
    Medium,
    Long,
}

#[derive(ValueEnum, Clone)]
pub enum OutputFormat {
    Table,
    Json,
    Markdown,
    Csv,
}
```

### 2. Provider Trait (providers/mod.rs)

Defines the interface all providers must implement.

```rust
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct InferenceRequest {
    pub prompt: String,
    pub max_tokens: u32,
    pub model: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InferenceResponse {
    pub text: String,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub time_to_prompt_ms: u64,       // Time until prompt fully sent
    pub time_to_first_token_ms: u64,  // Time from prompt sent to first token
    pub total_latency_ms: u64,
    pub model_load_time_ms: Option<u64>,  // One-time load (local only)
}

#[async_trait]
pub trait InferenceProvider: Send + Sync {
    /// Provider identifier
    fn name(&self) -> &str;

    /// Check if provider is configured and available
    async fn is_available(&self) -> bool;

    /// Execute inference request
    async fn infer(&self, request: &InferenceRequest) -> Result<InferenceResponse, ProviderError>;

    /// Get default model for this provider
    fn default_model(&self) -> &str;
}

#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("Provider not configured: {0}")]
    NotConfigured(String),
    #[error("API error: {0}")]
    ApiError(String),
    #[error("Timeout after {0}ms")]
    Timeout(u64),
    #[error("Rate limited")]
    RateLimited,
}
```

### 3. Benchmark Engine (benchmark/runner.rs)

Orchestrates benchmark execution with proper timing.

```rust
use std::time::Instant;

pub struct BenchmarkRunner {
    providers: Vec<Box<dyn InferenceProvider>>,
    config: BenchmarkConfig,
}

pub struct BenchmarkConfig {
    pub iterations: u32,
    pub warmup_iterations: u32,
    pub timeout_ms: u64,
    pub prompt_size: PromptSize,
}

pub struct BenchmarkResult {
    pub provider: String,
    pub model: String,
    pub metrics: AggregatedMetrics,
    pub raw_results: Vec<SingleRunResult>,
}

pub struct AggregatedMetrics {
    pub avg_time_to_prompt_ms: f64,
    pub avg_ttft_ms: f64,
    pub avg_tokens_per_sec: f64,
    pub avg_latency_ms: f64,
    pub p50_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub total_cost_usd: f64,
    pub model_load_time_ms: Option<u64>,  // One-time, not averaged
}

impl BenchmarkRunner {
    pub async fn run(&self) -> Vec<BenchmarkResult> {
        let mut results = Vec::new();

        for provider in &self.providers {
            if !provider.is_available().await {
                eprintln!("Skipping {}: not available", provider.name());
                continue;
            }

            let result = self.benchmark_provider(provider).await;
            results.push(result);
        }

        results
    }

    async fn benchmark_provider(
        &self,
        provider: &dyn InferenceProvider
    ) -> BenchmarkResult {
        // Warmup runs (not recorded)
        for _ in 0..self.config.warmup_iterations {
            let _ = provider.infer(&self.create_request()).await;
        }

        // Actual benchmark runs
        let mut raw_results = Vec::new();
        for _ in 0..self.config.iterations {
            let start = Instant::now();
            match provider.infer(&self.create_request()).await {
                Ok(response) => {
                    raw_results.push(SingleRunResult::from_response(response));
                }
                Err(e) => {
                    eprintln!("Error from {}: {}", provider.name(), e);
                }
            }
        }

        BenchmarkResult {
            provider: provider.name().to_string(),
            model: provider.default_model().to_string(),
            metrics: AggregatedMetrics::from_raw(&raw_results),
            raw_results,
        }
    }
}
```

### 4. Metrics Collection (benchmark/metrics.rs)

Calculates aggregated statistics from benchmark runs.

```rust
impl AggregatedMetrics {
    pub fn from_raw(results: &[SingleRunResult]) -> Self {
        let ttfts: Vec<f64> = results.iter().map(|r| r.ttft_ms as f64).collect();
        let latencies: Vec<f64> = results.iter().map(|r| r.latency_ms as f64).collect();
        let throughputs: Vec<f64> = results.iter().map(|r| r.tokens_per_sec()).collect();

        Self {
            avg_ttft_ms: mean(&ttfts),
            avg_tokens_per_sec: mean(&throughputs),
            avg_latency_ms: mean(&latencies),
            p50_latency_ms: percentile(&latencies, 50.0),
            p95_latency_ms: percentile(&latencies, 95.0),
            total_cost_usd: results.iter().map(|r| r.cost_usd).sum(),
        }
    }
}
```

### 5. Pricing Data (pricing/data.rs)

Stores and retrieves provider pricing information.

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderPricing {
    pub name: String,
    pub models: HashMap<String, ModelPricing>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPricing {
    pub input_per_million: f64,   // USD per 1M input tokens
    pub output_per_million: f64,  // USD per 1M output tokens
}

// Default pricing data (January 2025)
pub fn default_pricing() -> HashMap<String, ProviderPricing> {
    let mut pricing = HashMap::new();

    pricing.insert("cerebras".to_string(), ProviderPricing {
        name: "Cerebras".to_string(),
        models: [
            ("llama3.1-70b".to_string(), ModelPricing {
                input_per_million: 0.10,
                output_per_million: 0.10,
            }),
        ].into_iter().collect(),
    });

    pricing.insert("groq".to_string(), ProviderPricing {
        name: "Groq".to_string(),
        models: [
            ("llama3-70b".to_string(), ModelPricing {
                input_per_million: 0.05,
                output_per_million: 0.08,
            }),
        ].into_iter().collect(),
    });

    // ... additional providers

    pricing
}
```

## Data Flow

### Benchmark Execution Flow

```
1. User runs: speed-kings benchmark --providers cerebras,groq

2. CLI parses arguments
   |
   v
3. ProviderRegistry loads requested providers
   |
   v
4. BenchmarkRunner created with providers + config
   |
   v
5. For each provider:
   a. Check availability (API key, connectivity)
   b. Run warmup iterations
   c. Run benchmark iterations
   d. Collect metrics (TTFT, throughput, latency)
   e. Calculate costs from pricing data
   |
   v
6. Aggregate results across providers
   |
   v
7. Format output (table/json/markdown/csv)
   |
   v
8. Display to user
```

### Provider Request Flow

```
1. BenchmarkRunner creates InferenceRequest
   |
   v
2. Provider.infer() called
   |
   v
3. Provider-specific HTTP client makes API call
   - Records start time
   - Sends prompt, records time_to_prompt
   - Streams response (for TTFT measurement)
   - Records first token time (relative to prompt sent)
   - Accumulates tokens
   - Records completion time
   |
   v
4. InferenceResponse returned with timing data

Local Inference Additional Step:
   - Detect if model needs loading (first run)
   - Track model_load_time separately
   - Document one-time overhead in output
```

## Key Design Decisions

### D1: Trait-Based Provider Abstraction

**Decision**: Use Rust traits for provider interface.

**Rationale**:
- Enables easy addition of new providers
- Allows mock providers for testing
- Type-safe at compile time
- Clear contract for implementers

### D2: Async Runtime

**Decision**: Use Tokio as async runtime.

**Rationale**:
- Industry standard for Rust async
- Excellent HTTP client support (reqwest)
- Supports concurrent benchmarking
- Mature and well-documented

### D3: Streaming Response Measurement

**Decision**: Use streaming API responses where available.

**Rationale**:
- Accurate TTFT measurement
- More realistic real-world behavior
- Better progress indication

### D4: External Pricing Data

**Decision**: Store pricing in external JSON file.

**Rationale**:
- Easily updateable without recompilation
- User can override with custom pricing
- Supports multiple pricing tiers

## Error Handling

### Provider Errors

```rust
match provider.infer(&request).await {
    Ok(response) => process_response(response),
    Err(ProviderError::NotConfigured(msg)) => {
        eprintln!("Provider not configured: {}", msg);
        // Skip this provider, continue with others
    }
    Err(ProviderError::Timeout(ms)) => {
        eprintln!("Request timed out after {}ms", ms);
        // Record as failed attempt, retry if configured
    }
    Err(ProviderError::RateLimited) => {
        eprintln!("Rate limited, backing off...");
        // Exponential backoff, then retry
    }
    Err(ProviderError::ApiError(msg)) => {
        eprintln!("API error: {}", msg);
        // Log and continue
    }
}
```

### Configuration Errors

- Missing API keys: Clear message indicating which env var to set
- Invalid arguments: Helpful clap error messages with suggestions
- Network issues: Retry with exponential backoff

## Testing Strategy

### Unit Tests

- Metrics calculations (mean, percentile, aggregation)
- Pricing calculations
- Output formatting
- CLI argument parsing

### Integration Tests

- Mock provider implementation
- Full benchmark flow with mock
- Output format verification

### Manual Testing

- Real provider integration (requires API keys)
- Performance validation
- Edge cases (rate limiting, timeouts)

## Security Considerations

1. **API Keys**: Never logged or included in output
2. **Network**: HTTPS only for all provider APIs
3. **Input Validation**: Sanitize user-provided prompts
4. **Dependencies**: Regular audit with `cargo audit`

## Future Considerations

1. **Custom Models**: Allow user to specify exact model per provider
2. **Historical Tracking**: Store results over time for trend analysis
3. **Plugin System**: Dynamic provider loading

## Design Constraints

1. **Sequential Execution**: Providers run one at a time (no concurrent benchmarking)
2. **Budget-Conscious Defaults**: Target $0.10-0.25 per benchmark run max
3. **Minimal Iterations**: Default to 1-2 iterations for cost control
