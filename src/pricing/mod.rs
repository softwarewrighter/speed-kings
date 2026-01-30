//! Pricing data for inference providers.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pricing information for a provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderPricing {
    pub name: String,
    pub models: HashMap<String, ModelPricing>,
}

/// Pricing for a specific model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPricing {
    /// USD per 1M input tokens
    pub input_per_million: f64,
    /// USD per 1M output tokens
    pub output_per_million: f64,
}

/// Get default pricing data (January 2025)
pub fn default_pricing() -> HashMap<String, ProviderPricing> {
    let mut pricing = HashMap::new();

    // Cerebras pricing
    pricing.insert(
        "cerebras".to_string(),
        ProviderPricing {
            name: "Cerebras".to_string(),
            models: [
                (
                    "llama3.1-70b".to_string(),
                    ModelPricing {
                        input_per_million: 0.10,
                        output_per_million: 0.10,
                    },
                ),
                (
                    "llama3.1-8b".to_string(),
                    ModelPricing {
                        input_per_million: 0.02,
                        output_per_million: 0.02,
                    },
                ),
            ]
            .into_iter()
            .collect(),
        },
    );

    // Groq pricing
    pricing.insert(
        "groq".to_string(),
        ProviderPricing {
            name: "Groq".to_string(),
            models: [
                (
                    "llama3-70b-8192".to_string(),
                    ModelPricing {
                        input_per_million: 0.05,
                        output_per_million: 0.08,
                    },
                ),
                (
                    "mixtral-8x7b-32768".to_string(),
                    ModelPricing {
                        input_per_million: 0.02,
                        output_per_million: 0.02,
                    },
                ),
            ]
            .into_iter()
            .collect(),
        },
    );

    // Fireworks pricing
    pricing.insert(
        "fireworks".to_string(),
        ProviderPricing {
            name: "Fireworks".to_string(),
            models: [(
                "llama-v3p1-70b-instruct".to_string(),
                ModelPricing {
                    input_per_million: 0.20,
                    output_per_million: 0.20,
                },
            )]
            .into_iter()
            .collect(),
        },
    );

    // DeepSeek pricing (very affordable)
    pricing.insert(
        "deepseek".to_string(),
        ProviderPricing {
            name: "DeepSeek".to_string(),
            models: [(
                "deepseek-chat".to_string(),
                ModelPricing {
                    input_per_million: 0.014,
                    output_per_million: 0.028,
                },
            )]
            .into_iter()
            .collect(),
        },
    );

    // Local (Ollama) - free
    pricing.insert(
        "local".to_string(),
        ProviderPricing {
            name: "Local (Ollama)".to_string(),
            models: [(
                "default".to_string(),
                ModelPricing {
                    input_per_million: 0.0,
                    output_per_million: 0.0,
                },
            )]
            .into_iter()
            .collect(),
        },
    );

    pricing
}

/// Format pricing information as a displayable string
pub fn format_pricing_table() -> String {
    let pricing = default_pricing();
    let mut output = String::new();

    output.push_str("Provider Pricing (per 1M tokens)\n");
    output.push_str("================================\n\n");

    for (_, provider) in pricing.iter() {
        output.push_str(&format!("{}:\n", provider.name));
        for (model, prices) in &provider.models {
            output.push_str(&format!(
                "  {}: ${:.3} input / ${:.3} output\n",
                model, prices.input_per_million, prices.output_per_million
            ));
        }
        output.push('\n');
    }

    output
        .push_str("Note: Prices as of January 2025. Check provider websites for current rates.\n");

    output
}
