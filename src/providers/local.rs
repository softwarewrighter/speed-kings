//! Local inference provider (Ollama) implementation.

use super::{InferenceProvider, InferenceRequest, InferenceResponse, ProviderError};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

const DEFAULT_OLLAMA_URL: &str = "http://localhost:11434";
const DEFAULT_MODEL: &str = "llama3.2:3b";
const TIMEOUT_SECS: u64 = 300; // Local inference can be slow

/// Local inference provider using Ollama
pub struct LocalProvider {
    client: Client,
    base_url: String,
    model: String,
}

#[derive(Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct GenerateResponse {
    response: String,
    #[serde(rename = "done")]
    _done: bool,
    #[serde(default)]
    prompt_eval_count: u32,
    #[serde(default)]
    eval_count: u32,
    #[serde(default)]
    load_duration: u64, // nanoseconds
}

#[derive(Deserialize)]
struct TagsResponse {
    models: Vec<ModelInfo>,
}

#[derive(Deserialize)]
struct ModelInfo {
    name: String,
}

impl LocalProvider {
    /// Detect and create a local Ollama provider
    pub fn detect() -> Result<Self, ProviderError> {
        let base_url =
            std::env::var("OLLAMA_URL").unwrap_or_else(|_| DEFAULT_OLLAMA_URL.to_string());

        let client = Client::builder()
            .timeout(Duration::from_secs(TIMEOUT_SECS))
            .build()
            .map_err(|e| ProviderError::Network(e.to_string()))?;

        Ok(Self {
            client,
            base_url,
            model: DEFAULT_MODEL.to_string(),
        })
    }

    /// Check if Ollama is running
    async fn check_ollama(&self) -> bool {
        let url = format!("{}/api/tags", self.base_url);
        self.client.get(&url).send().await.is_ok()
    }

    /// List available models
    #[allow(dead_code)]
    async fn list_models(&self) -> Result<Vec<String>, ProviderError> {
        let url = format!("{}/api/tags", self.base_url);
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| ProviderError::Network(e.to_string()))?;

        let tags: TagsResponse = response
            .json()
            .await
            .map_err(|e| ProviderError::ParseError(e.to_string()))?;

        Ok(tags.models.into_iter().map(|m| m.name).collect())
    }
}

#[async_trait]
impl InferenceProvider for LocalProvider {
    fn name(&self) -> &str {
        "local"
    }

    fn display_name(&self) -> &str {
        "Local (Ollama)"
    }

    async fn is_available(&self) -> bool {
        self.check_ollama().await
    }

    async fn infer(&self, request: &InferenceRequest) -> Result<InferenceResponse, ProviderError> {
        let start = Instant::now();

        let model = request.model.clone().unwrap_or_else(|| self.model.clone());

        let generate_request = GenerateRequest {
            model,
            prompt: request.prompt.clone(),
            stream: false, // Non-streaming for simplicity; can add streaming later
        };

        let url = format!("{}/api/generate", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(&generate_request)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    ProviderError::Timeout(TIMEOUT_SECS * 1000)
                } else if e.is_connect() {
                    ProviderError::Network(format!(
                        "Cannot connect to Ollama at {}. Is it running? (ollama serve)",
                        self.base_url
                    ))
                } else {
                    ProviderError::ApiError(e.to_string())
                }
            })?;

        let time_to_prompt_ms = start.elapsed().as_millis() as u64;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(ProviderError::ApiError(format!(
                "HTTP {}: {}",
                status, body
            )));
        }

        let result: GenerateResponse = response
            .json()
            .await
            .map_err(|e| ProviderError::ParseError(e.to_string()))?;

        let total_latency_ms = start.elapsed().as_millis() as u64;

        // Ollama provides load_duration in nanoseconds
        let model_load_time_ms = if result.load_duration > 0 {
            Some(result.load_duration / 1_000_000)
        } else {
            None
        };

        // For non-streaming, TTFT is approximately the full latency minus output generation
        // This is an approximation; streaming would give more accurate TTFT
        let time_to_first_token_ms = time_to_prompt_ms;

        Ok(InferenceResponse {
            text: result.response,
            input_tokens: result.prompt_eval_count,
            output_tokens: result.eval_count,
            time_to_prompt_ms,
            time_to_first_token_ms,
            total_latency_ms,
            model_load_time_ms,
        })
    }

    fn default_model(&self) -> &str {
        &self.model
    }

    fn pricing_per_million(&self) -> (f64, f64) {
        // Local inference is free
        (0.0, 0.0)
    }
}
