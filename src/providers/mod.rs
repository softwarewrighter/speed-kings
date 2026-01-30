//! Provider trait and implementations for LLM inference APIs.

mod cerebras;
mod deepseek;
mod fireworks;
mod groq;
mod local;
mod openai_compatible;
mod sambanova;

pub use cerebras::CerebrasProvider;
pub use deepseek::DeepSeekProvider;
pub use fireworks::FireworksProvider;
pub use groq::GroqProvider;
pub use local::LocalProvider;
pub use openai_compatible::OpenAICompatibleProvider;
pub use sambanova::SambaNovaProvider;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Request to an inference provider
#[derive(Debug, Clone)]
pub struct InferenceRequest {
    /// The prompt to send
    pub prompt: String,
    /// Maximum tokens to generate
    pub max_tokens: u32,
    /// Specific model to use (provider default if None)
    pub model: Option<String>,
}

/// Response from an inference provider with timing metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResponse {
    /// Generated text
    pub text: String,
    /// Number of input tokens
    pub input_tokens: u32,
    /// Number of output tokens generated
    pub output_tokens: u32,
    /// Time until prompt was fully sent (ms)
    pub time_to_prompt_ms: u64,
    /// Time from prompt sent to first token received (ms)
    pub time_to_first_token_ms: u64,
    /// Total request latency (ms)
    pub total_latency_ms: u64,
    /// One-time model load time, if applicable (ms)
    pub model_load_time_ms: Option<u64>,
}

/// Errors that can occur during inference
#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("Provider not configured: {0}")]
    NotConfigured(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Request timeout after {0}ms")]
    Timeout(u64),

    #[error("Rate limited by provider")]
    RateLimited,

    #[error("Network error: {0}")]
    Network(String),

    #[error("Failed to parse response: {0}")]
    ParseError(String),
}

/// Trait that all inference providers must implement
#[async_trait]
pub trait InferenceProvider: Send + Sync {
    /// Unique identifier for this provider
    fn name(&self) -> &str;

    /// Human-readable display name
    fn display_name(&self) -> &str;

    /// Check if the provider is configured and available
    async fn is_available(&self) -> bool;

    /// Execute an inference request
    async fn infer(&self, request: &InferenceRequest) -> Result<InferenceResponse, ProviderError>;

    /// Get the default model for this provider
    fn default_model(&self) -> &str;

    /// Get pricing per million tokens (input, output)
    fn pricing_per_million(&self) -> (f64, f64);
}

/// Registry of all available providers
pub struct ProviderRegistry {
    providers: HashMap<String, Box<dyn InferenceProvider>>,
}

impl ProviderRegistry {
    /// Create a new registry, attempting to initialize all known providers
    pub fn new() -> Self {
        let mut registry = Self {
            providers: HashMap::new(),
        };

        // Try to register each provider - failures are silent (provider not available)

        // Cloud providers (require API keys)
        if let Ok(provider) = CerebrasProvider::from_env() {
            registry
                .providers
                .insert("cerebras".to_string(), Box::new(provider));
        }

        if let Ok(provider) = GroqProvider::from_env() {
            registry
                .providers
                .insert("groq".to_string(), Box::new(provider));
        }

        if let Ok(provider) = FireworksProvider::from_env() {
            registry
                .providers
                .insert("fireworks".to_string(), Box::new(provider));
        }

        if let Ok(provider) = SambaNovaProvider::from_env() {
            registry
                .providers
                .insert("sambanova".to_string(), Box::new(provider));
        }

        if let Ok(provider) = DeepSeekProvider::from_env() {
            registry
                .providers
                .insert("deepseek".to_string(), Box::new(provider));
        }

        // OpenAI-compatible custom endpoint
        if let Ok(provider) = OpenAICompatibleProvider::from_env() {
            registry
                .providers
                .insert("openai-compatible".to_string(), Box::new(provider));
        }

        // Local provider (Ollama) - always try to register
        if let Ok(provider) = LocalProvider::detect() {
            registry
                .providers
                .insert("local".to_string(), Box::new(provider));
        }

        registry
    }

    /// Get a provider by name
    pub fn get(&self, name: &str) -> Option<&dyn InferenceProvider> {
        self.providers.get(name).map(|p| p.as_ref())
    }

    /// List all available provider names
    pub fn available(&self) -> Vec<&str> {
        self.providers.keys().map(|s| s.as_str()).collect()
    }

    /// Get all providers
    pub fn all(&self) -> Vec<&dyn InferenceProvider> {
        self.providers.values().map(|p| p.as_ref()).collect()
    }

    /// Check if any providers are available
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }

    /// Get count of registered providers
    pub fn len(&self) -> usize {
        self.providers.len()
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}
