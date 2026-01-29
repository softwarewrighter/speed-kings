# Design Document

> Detailed design decisions and patterns for Speed Kings.

## Overview

This document captures the design rationale, patterns, and technical decisions for the Speed Kings inference benchmarking tool. It complements the architecture document by explaining *why* certain approaches were chosen.

## Design Principles

### 1. Accuracy Over Speed

Benchmark results must be trustworthy. We prioritize measurement accuracy even if it means slightly slower benchmark execution.

**Implications**:
- Warmup runs before recording measurements (when budget allows)
- Multiple iterations with statistical aggregation
- Streaming response handling for precise TTFT
- Clear reporting of variance/confidence
- Separate tracking of time-to-prompt vs time-to-first-token

### 2. Budget Awareness

Keep benchmark costs under control ($0.10-0.25 per run target).

**Implications**:
- Minimal default iterations (1-2)
- Skip warmup for costly providers
- Show estimated cost before execution
- Sequential execution (no concurrent API calls)

### 3. Modularity

Each component should be independently testable and replaceable.

**Implications**:
- Trait-based provider abstraction
- Separate output formatters
- Configurable pricing data
- Clear module boundaries

### 4. User Experience

The tool should be approachable for new users while powerful for advanced use cases.

**Implications**:
- Sensible defaults for all options
- Comprehensive --help text
- Clear error messages with remediation steps
- Progressive disclosure of complexity

### 5. Reproducibility

Benchmark results should be reproducible across runs and machines.

**Implications**:
- Standardized test prompts
- Documented methodology
- Metadata in output (timestamp, versions, config)
- Deterministic ordering

## Provider Implementation Pattern

### Base Structure

Each provider follows a consistent implementation pattern:

```rust
pub struct CerebrasProvider {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
    default_model: String,
}

impl CerebrasProvider {
    pub fn from_env() -> Result<Self, ProviderError> {
        let api_key = std::env::var("CEREBRAS_API_KEY")
            .map_err(|_| ProviderError::NotConfigured(
                "CEREBRAS_API_KEY environment variable not set".to_string()
            ))?;

        Ok(Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(60))
                .build()?,
            api_key,
            base_url: "https://api.cerebras.ai/v1".to_string(),
            default_model: "llama3.1-70b".to_string(),
        })
    }
}
```

### Streaming Implementation

For accurate TTFT measurement, providers use streaming responses:

```rust
#[async_trait]
impl InferenceProvider for CerebrasProvider {
    async fn infer(&self, request: &InferenceRequest) -> Result<InferenceResponse, ProviderError> {
        let start = Instant::now();
        let mut first_token_time: Option<Duration> = None;
        let mut output_text = String::new();
        let mut output_tokens = 0u32;

        let response = self.client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&self.build_request_body(request))
            .send()
            .await?;

        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;

            // Record time to first token
            if first_token_time.is_none() && !chunk.is_empty() {
                first_token_time = Some(start.elapsed());
            }

            // Parse and accumulate response
            if let Some(text) = self.parse_chunk(&chunk) {
                output_text.push_str(&text);
                output_tokens += 1; // Simplified; real impl counts properly
            }
        }

        let total_time = start.elapsed();

        Ok(InferenceResponse {
            text: output_text,
            input_tokens: self.count_tokens(&request.prompt),
            output_tokens,
            time_to_first_token_ms: first_token_time
                .unwrap_or(total_time)
                .as_millis() as u64,
            total_latency_ms: total_time.as_millis() as u64,
        })
    }
}
```

### Provider Registry

The registry manages provider discovery and instantiation:

```rust
pub struct ProviderRegistry {
    providers: HashMap<String, Box<dyn InferenceProvider>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            providers: HashMap::new(),
        };

        // Attempt to register all known providers
        // Failures are logged but don't prevent other providers
        registry.try_register("cerebras", || CerebrasProvider::from_env());
        registry.try_register("groq", || GroqProvider::from_env());
        registry.try_register("sambanova", || SambaNovaProvider::from_env());
        registry.try_register("fireworks", || FireworksProvider::from_env());
        registry.try_register("deepseek", || DeepSeekProvider::from_env());
        registry.try_register("local", || LocalProvider::detect());

        registry
    }

    fn try_register<F, P>(&mut self, name: &str, factory: F)
    where
        F: FnOnce() -> Result<P, ProviderError>,
        P: InferenceProvider + 'static,
    {
        match factory() {
            Ok(provider) => {
                self.providers.insert(name.to_string(), Box::new(provider));
            }
            Err(e) => {
                // Debug-level logging; provider simply not available
                tracing::debug!("Provider {} not available: {}", name, e);
            }
        }
    }

    pub fn get(&self, name: &str) -> Option<&dyn InferenceProvider> {
        self.providers.get(name).map(|p| p.as_ref())
    }

    pub fn available_providers(&self) -> Vec<&str> {
        self.providers.keys().map(|s| s.as_str()).collect()
    }
}
```

## Local Inference Model Loading

### Problem

When using Ollama or llama.cpp, the first request may trigger:
1. Model download (if not cached) - can take minutes
2. Model loading into memory - can take seconds

These are one-time costs that shouldn't be conflated with inference performance.

### Solution

Track model loading separately and document clearly:

```rust
pub struct LocalInferenceMetrics {
    /// One-time model download (None if already cached)
    pub download_time_ms: Option<u64>,

    /// Time to load model into memory
    pub model_load_time_ms: u64,

    /// Standard inference metrics (measured after model loaded)
    pub inference: InferenceMetrics,
}

impl LocalProvider {
    async fn ensure_model_loaded(&self, model: &str) -> Result<u64, ProviderError> {
        let start = Instant::now();

        // Check if model is loaded
        if !self.is_model_loaded(model).await? {
            // This may download + load
            self.load_model(model).await?;
        }

        Ok(start.elapsed().as_millis() as u64)
    }

    async fn infer(&self, request: &InferenceRequest) -> Result<InferenceResponse, ProviderError> {
        // Track model load time separately
        let model_load_time = self.ensure_model_loaded(&request.model).await?;

        // Now run actual inference benchmark
        let inference_start = Instant::now();
        // ... standard inference timing ...

        Ok(InferenceResponse {
            model_load_time_ms: Some(model_load_time),
            // ... other fields ...
        })
    }
}
```

### Output Documentation

When displaying local inference results:

```
Local Inference (Ollama)
========================
Model: llama3.2:3b
Model Load Time: 2.3s (one-time, not included in metrics below)

Time to Prompt:     12ms
Time to First Token: 89ms
Throughput:         42 tok/s
Total Latency:      2.4s
Cost:               $0.00

Note: First run may include model download. Subsequent runs
will skip model loading if kept in memory.
```

## Test Prompt Design

### Prompt Criteria

Test prompts are designed to:

1. **Produce consistent output length** - Similar token counts across runs
2. **Avoid model-specific quirks** - Work across different LLM architectures
3. **Be representative** - Reflect real-world usage patterns
4. **Not trigger safety filters** - Avoid content that might cause refusals

### Prompt Definitions

```rust
pub struct TestPrompt {
    pub name: &'static str,
    pub text: &'static str,
    pub expected_input_tokens: u32,
    pub expected_output_tokens: u32,
}

pub const SHORT_PROMPT: TestPrompt = TestPrompt {
    name: "short",
    text: "Explain what a binary search tree is in exactly three sentences.",
    expected_input_tokens: 15,
    expected_output_tokens: 50,
};

pub const MEDIUM_PROMPT: TestPrompt = TestPrompt {
    name: "medium",
    text: r#"Write a Python function that implements merge sort. Include:
1. The main merge_sort function
2. A helper merge function
3. Brief comments explaining each step
4. An example of calling the function with a sample list"#,
    expected_input_tokens: 50,
    expected_output_tokens: 200,
};

pub const LONG_PROMPT: TestPrompt = TestPrompt {
    name: "long",
    text: r#"You are a technical writer. Write a comprehensive guide about REST API design best practices. The guide should cover:

1. Resource naming conventions
2. HTTP method usage (GET, POST, PUT, PATCH, DELETE)
3. Status code selection
4. Error response formatting
5. Pagination strategies
6. Versioning approaches
7. Authentication considerations

For each topic, provide a brief explanation and a concrete example. The guide should be suitable for intermediate developers who understand HTTP but are new to API design."#,
    expected_input_tokens: 100,
    expected_output_tokens: 500,
};
```

## Output Format Design

### Terminal Table

Optimized for readability in terminal environments:

```rust
use comfy_table::{Table, ContentArrangement, presets::UTF8_FULL};

pub fn format_table(results: &[BenchmarkResult]) -> String {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            "Provider",
            "Model",
            "TTFT (ms)",
            "Tok/sec",
            "Latency (ms)",
            "Cost/1M",
        ]);

    for result in results {
        table.add_row(vec![
            &result.provider,
            &result.model,
            &format!("{:.0}", result.metrics.avg_ttft_ms),
            &format!("{:.0}", result.metrics.avg_tokens_per_sec),
            &format!("{:.0}", result.metrics.avg_latency_ms),
            &format!("${:.2}", result.pricing_per_million()),
        ]);
    }

    table.to_string()
}
```

### JSON Output

Complete data for programmatic consumption:

```rust
#[derive(Serialize)]
pub struct JsonOutput {
    pub metadata: BenchmarkMetadata,
    pub results: Vec<BenchmarkResult>,
    pub summary: BenchmarkSummary,
}

#[derive(Serialize)]
pub struct BenchmarkMetadata {
    pub timestamp: DateTime<Utc>,
    pub version: String,
    pub config: BenchmarkConfig,
    pub system_info: SystemInfo,
}

pub fn format_json(results: &[BenchmarkResult], config: &BenchmarkConfig) -> String {
    let output = JsonOutput {
        metadata: BenchmarkMetadata {
            timestamp: Utc::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config: config.clone(),
            system_info: SystemInfo::collect(),
        },
        results: results.to_vec(),
        summary: BenchmarkSummary::from_results(results),
    };

    serde_json::to_string_pretty(&output).unwrap()
}
```

### Markdown Output

Ready for documentation or video overlays:

```rust
pub fn format_markdown(results: &[BenchmarkResult]) -> String {
    let mut output = String::new();

    output.push_str("# Inference Benchmark Results\n\n");
    output.push_str(&format!("*Generated: {}*\n\n", Utc::now().format("%Y-%m-%d %H:%M UTC")));

    output.push_str("| Provider | Model | TTFT | Throughput | Latency | Cost/1M |\n");
    output.push_str("|----------|-------|------|------------|---------|--------|\n");

    for result in results {
        output.push_str(&format!(
            "| {} | {} | {}ms | {} tok/s | {}ms | ${:.2} |\n",
            result.provider,
            result.model,
            result.metrics.avg_ttft_ms as u64,
            result.metrics.avg_tokens_per_sec as u64,
            result.metrics.avg_latency_ms as u64,
            result.pricing_per_million(),
        ));
    }

    output
}
```

## Rate Limiting and Throttling

### Detection

```rust
#[derive(Debug)]
pub enum RateLimitInfo {
    /// Got 429, retry after N seconds
    RetryAfter(Duration),

    /// Got 429, no retry-after header
    Unknown,

    /// Request succeeded but was slow (possible throttling)
    PossibleThrottling {
        expected_ttft_ms: u64,
        actual_ttft_ms: u64,
    },
}
```

### Handling Strategy

1. **429 Rate Limited**: Log warning, skip provider for this run
2. **Possible Throttling**: Note in output, suggest upgrade or off-peak timing
3. **Consistent Slowness**: Document in results, recommend account review

### Metadata for Reproducibility

```rust
pub struct BenchmarkMetadata {
    pub timestamp: DateTime<Utc>,
    pub time_of_day: String,  // "off-peak", "peak-us", "peak-eu"
    pub provider_tier: String,  // "free", "paid", "enterprise"
    // ...
}
```

### Timing Recommendations

Document in README:
- US peak hours: 9am-5pm PT (high contention)
- Off-peak windows: evenings, weekends, early morning
- Provider-specific patterns may vary

## Error Handling Strategy

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum SpeedKingsError {
    #[error("No providers available. Set API keys for at least one provider.")]
    NoProviders,

    #[error("Provider '{0}' not found. Available: {1}")]
    ProviderNotFound(String, String),

    #[error("Benchmark failed for all providers")]
    AllProvidersFailed,

    #[error("Configuration error: {0}")]
    Config(String),

    #[error(transparent)]
    Provider(#[from] ProviderError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
```

### User-Friendly Messages

```rust
impl SpeedKingsError {
    pub fn user_message(&self) -> String {
        match self {
            Self::NoProviders => {
                let mut msg = String::from("No providers available.\n\n");
                msg.push_str("Set one or more API keys:\n");
                msg.push_str("  export CEREBRAS_API_KEY=...\n");
                msg.push_str("  export GROQ_API_KEY=...\n");
                msg.push_str("  export FIREWORKS_API_KEY=...\n");
                msg.push_str("\nOr start Ollama for local inference:\n");
                msg.push_str("  ollama serve\n");
                msg
            }
            Self::ProviderNotFound(name, available) => {
                format!(
                    "Provider '{}' not found.\n\nAvailable providers: {}",
                    name, available
                )
            }
            _ => self.to_string(),
        }
    }
}
```

## Configuration Management

### Environment Variables

```rust
pub struct Config {
    // Provider API keys
    pub cerebras_api_key: Option<String>,
    pub groq_api_key: Option<String>,
    pub sambanova_api_key: Option<String>,
    pub fireworks_api_key: Option<String>,
    pub deepseek_api_key: Option<String>,

    // Local inference
    pub ollama_url: String,

    // Benchmark settings
    pub default_iterations: u32,
    pub default_timeout_ms: u64,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            cerebras_api_key: std::env::var("CEREBRAS_API_KEY").ok(),
            groq_api_key: std::env::var("GROQ_API_KEY").ok(),
            sambanova_api_key: std::env::var("SAMBANOVA_API_KEY").ok(),
            fireworks_api_key: std::env::var("FIREWORKS_API_KEY").ok(),
            deepseek_api_key: std::env::var("DEEPSEEK_API_KEY").ok(),
            ollama_url: std::env::var("OLLAMA_URL")
                .unwrap_or_else(|_| "http://localhost:11434".to_string()),
            default_iterations: std::env::var("SPEED_KINGS_ITERATIONS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3),
            default_timeout_ms: std::env::var("SPEED_KINGS_TIMEOUT_MS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(60000),
        }
    }
}
```

### Pricing Data File

```json
{
  "version": "2025-01",
  "providers": {
    "cerebras": {
      "name": "Cerebras",
      "models": {
        "llama3.1-70b": {
          "input_per_million": 0.10,
          "output_per_million": 0.10
        },
        "llama3.1-8b": {
          "input_per_million": 0.02,
          "output_per_million": 0.02
        }
      }
    },
    "groq": {
      "name": "Groq",
      "models": {
        "llama3-70b-8192": {
          "input_per_million": 0.05,
          "output_per_million": 0.08
        },
        "mixtral-8x7b-32768": {
          "input_per_million": 0.02,
          "output_per_million": 0.02
        }
      }
    },
    "fireworks": {
      "name": "Fireworks",
      "models": {
        "llama-v3p1-70b-instruct": {
          "input_per_million": 0.20,
          "output_per_million": 0.20
        }
      }
    },
    "local": {
      "name": "Local (Ollama)",
      "models": {
        "default": {
          "input_per_million": 0.00,
          "output_per_million": 0.00
        }
      }
    }
  }
}
```

## Performance Considerations

### Connection Pooling

```rust
// Reuse HTTP client across requests
lazy_static! {
    static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::builder()
        .pool_max_idle_per_host(10)
        .timeout(Duration::from_secs(60))
        .build()
        .expect("Failed to create HTTP client");
}
```

### Budget-Aware Iteration Control

```rust
/// Calculate safe iteration count based on estimated cost
pub fn safe_iterations(
    provider: &dyn InferenceProvider,
    prompt_size: PromptSize,
    max_cost: f64,  // e.g., 0.25 for $0.25 budget
) -> u32 {
    let pricing = get_pricing(provider.name());
    let estimated_tokens = prompt_size.expected_output_tokens();
    let cost_per_run = (estimated_tokens as f64 / 1_000_000.0) * pricing.output_per_million;

    let max_iterations = (max_cost / cost_per_run).floor() as u32;
    max_iterations.clamp(1, 5)  // At least 1, at most 5
}
```

## Testing Approach

### Mock Provider

```rust
#[cfg(test)]
pub struct MockProvider {
    pub name: String,
    pub response: InferenceResponse,
    pub delay: Duration,
    pub should_fail: bool,
}

#[cfg(test)]
#[async_trait]
impl InferenceProvider for MockProvider {
    fn name(&self) -> &str {
        &self.name
    }

    async fn is_available(&self) -> bool {
        !self.should_fail
    }

    async fn infer(&self, _request: &InferenceRequest) -> Result<InferenceResponse, ProviderError> {
        tokio::time::sleep(self.delay).await;

        if self.should_fail {
            Err(ProviderError::ApiError("Mock failure".to_string()))
        } else {
            Ok(self.response.clone())
        }
    }

    fn default_model(&self) -> &str {
        "mock-model"
    }
}
```

### Property-Based Testing

```rust
#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn metrics_aggregation_is_consistent(
            ttfts in prop::collection::vec(1u64..10000, 1..100),
            latencies in prop::collection::vec(1u64..100000, 1..100),
        ) {
            let results: Vec<SingleRunResult> = ttfts.iter()
                .zip(latencies.iter())
                .map(|(t, l)| SingleRunResult {
                    ttft_ms: *t,
                    latency_ms: *l,
                    output_tokens: 100,
                    ..Default::default()
                })
                .collect();

            let metrics = AggregatedMetrics::from_raw(&results);

            // Verify statistical invariants
            assert!(metrics.avg_ttft_ms >= 0.0);
            assert!(metrics.p50_latency_ms <= metrics.p95_latency_ms);
            assert!(metrics.avg_tokens_per_sec >= 0.0);
        }
    }
}
```
