//! DeepSeek inference provider implementation.

use super::{InferenceProvider, InferenceRequest, InferenceResponse, ProviderError};
use async_trait::async_trait;
use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

const DEEPSEEK_API_URL: &str = "https://api.deepseek.com/chat/completions";
const DEFAULT_MODEL: &str = "deepseek-chat";
const TIMEOUT_SECS: u64 = 120;

/// DeepSeek inference provider - cost-effective inference
pub struct DeepSeekProvider {
    client: Client,
    api_key: String,
    model: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    stream: bool,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct StreamChunk {
    choices: Vec<StreamChoice>,
    usage: Option<Usage>,
}

#[derive(Deserialize)]
struct StreamChoice {
    delta: Delta,
    #[serde(rename = "finish_reason")]
    _finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct Delta {
    content: Option<String>,
}

#[derive(Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
}

impl DeepSeekProvider {
    /// Create a new DeepSeek provider from environment variables
    pub fn from_env() -> Result<Self, ProviderError> {
        let api_key = std::env::var("DEEPSEEK_API_KEY").map_err(|_| {
            ProviderError::NotConfigured(
                "DEEPSEEK_API_KEY environment variable not set".to_string(),
            )
        })?;

        let client = Client::builder()
            .timeout(Duration::from_secs(TIMEOUT_SECS))
            .build()
            .map_err(|e| ProviderError::Network(e.to_string()))?;

        Ok(Self {
            client,
            api_key,
            model: DEFAULT_MODEL.to_string(),
        })
    }
}

#[async_trait]
impl InferenceProvider for DeepSeekProvider {
    fn name(&self) -> &str {
        "deepseek"
    }

    fn display_name(&self) -> &str {
        "DeepSeek"
    }

    async fn is_available(&self) -> bool {
        true
    }

    async fn infer(&self, request: &InferenceRequest) -> Result<InferenceResponse, ProviderError> {
        let start = Instant::now();

        let model = request.model.clone().unwrap_or_else(|| self.model.clone());

        let chat_request = ChatRequest {
            model,
            messages: vec![Message {
                role: "user".to_string(),
                content: request.prompt.clone(),
            }],
            max_tokens: request.max_tokens,
            stream: true,
        };

        let response = self
            .client
            .post(DEEPSEEK_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&chat_request)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    ProviderError::Timeout(TIMEOUT_SECS * 1000)
                } else if e.is_connect() {
                    ProviderError::Network(e.to_string())
                } else {
                    ProviderError::ApiError(e.to_string())
                }
            })?;

        let time_to_prompt_ms = start.elapsed().as_millis() as u64;

        if response.status() == 429 {
            return Err(ProviderError::RateLimited);
        }

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(ProviderError::ApiError(format!(
                "HTTP {}: {}",
                status, body
            )));
        }

        let mut stream = response.bytes_stream();
        let mut first_token_time: Option<Duration> = None;
        let mut output_text = String::new();
        let mut input_tokens = 0u32;
        let mut output_tokens = 0u32;
        let mut buffer = String::new();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| ProviderError::Network(e.to_string()))?;

            if first_token_time.is_none() && !chunk.is_empty() {
                first_token_time = Some(start.elapsed());
            }

            let chunk_str = String::from_utf8_lossy(&chunk);
            buffer.push_str(&chunk_str);

            while let Some(line_end) = buffer.find('\n') {
                let line = buffer[..line_end].trim().to_string();
                buffer = buffer[line_end + 1..].to_string();

                if let Some(data) = line.strip_prefix("data: ") {
                    if data == "[DONE]" {
                        continue;
                    }

                    if let Ok(chunk_data) = serde_json::from_str::<StreamChunk>(data) {
                        for choice in chunk_data.choices {
                            if let Some(content) = choice.delta.content {
                                output_text.push_str(&content);
                            }
                        }
                        if let Some(usage) = chunk_data.usage {
                            input_tokens = usage.prompt_tokens;
                            output_tokens = usage.completion_tokens;
                        }
                    }
                }
            }
        }

        let total_latency_ms = start.elapsed().as_millis() as u64;
        let ttft_ms = first_token_time
            .map(|t| t.as_millis() as u64)
            .unwrap_or(total_latency_ms);

        let time_to_first_token_ms = ttft_ms.saturating_sub(time_to_prompt_ms);

        Ok(InferenceResponse {
            text: output_text,
            input_tokens,
            output_tokens,
            time_to_prompt_ms,
            time_to_first_token_ms,
            total_latency_ms,
            model_load_time_ms: None,
        })
    }

    fn default_model(&self) -> &str {
        &self.model
    }

    fn pricing_per_million(&self) -> (f64, f64) {
        // DeepSeek pricing as of Jan 2025 - very affordable
        (0.014, 0.028)
    }
}
