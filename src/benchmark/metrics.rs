//! Metrics aggregation and statistical calculations.

use super::SingleRunResult;
use serde::{Deserialize, Serialize};

/// Aggregated metrics from multiple benchmark runs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedMetrics {
    /// Average time to send prompt (ms)
    pub avg_time_to_prompt_ms: f64,
    /// Average time to first token (ms)
    pub avg_ttft_ms: f64,
    /// Average tokens per second
    pub avg_tokens_per_sec: f64,
    /// Average total latency (ms)
    pub avg_latency_ms: f64,
    /// 50th percentile latency (ms)
    pub p50_latency_ms: f64,
    /// 95th percentile latency (ms)
    pub p95_latency_ms: f64,
    /// Total cost for all runs (USD)
    pub total_cost_usd: f64,
    /// One-time model load time if applicable (ms)
    pub model_load_time_ms: Option<u64>,
    /// Number of successful runs
    pub run_count: usize,
}

impl AggregatedMetrics {
    /// Calculate aggregated metrics from raw run results
    pub fn from_raw(results: &[SingleRunResult]) -> Self {
        if results.is_empty() {
            return Self::empty();
        }

        let time_to_prompts: Vec<f64> =
            results.iter().map(|r| r.time_to_prompt_ms as f64).collect();
        let ttfts: Vec<f64> = results
            .iter()
            .map(|r| r.time_to_first_token_ms as f64)
            .collect();
        let mut latencies: Vec<f64> = results.iter().map(|r| r.total_latency_ms as f64).collect();
        let throughputs: Vec<f64> = results.iter().map(|r| r.tokens_per_sec()).collect();

        // Sort latencies for percentile calculation
        latencies.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Get model load time from first run (it's one-time)
        let model_load_time_ms = results.first().and_then(|r| r.model_load_time_ms);

        Self {
            avg_time_to_prompt_ms: mean(&time_to_prompts),
            avg_ttft_ms: mean(&ttfts),
            avg_tokens_per_sec: mean(&throughputs),
            avg_latency_ms: mean(&latencies),
            p50_latency_ms: percentile(&latencies, 50.0),
            p95_latency_ms: percentile(&latencies, 95.0),
            total_cost_usd: results.iter().map(|r| r.cost_usd).sum(),
            model_load_time_ms,
            run_count: results.len(),
        }
    }

    /// Create empty metrics (no successful runs)
    fn empty() -> Self {
        Self {
            avg_time_to_prompt_ms: 0.0,
            avg_ttft_ms: 0.0,
            avg_tokens_per_sec: 0.0,
            avg_latency_ms: 0.0,
            p50_latency_ms: 0.0,
            p95_latency_ms: 0.0,
            total_cost_usd: 0.0,
            model_load_time_ms: None,
            run_count: 0,
        }
    }
}

/// Calculate mean of a slice of f64 values
fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

/// Calculate percentile of a sorted slice of f64 values
fn percentile(sorted_values: &[f64], pct: f64) -> f64 {
    if sorted_values.is_empty() {
        return 0.0;
    }
    if sorted_values.len() == 1 {
        return sorted_values[0];
    }

    let idx = (pct / 100.0 * (sorted_values.len() - 1) as f64).round() as usize;
    sorted_values[idx.min(sorted_values.len() - 1)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean_empty() {
        assert_eq!(mean(&[]), 0.0);
    }

    #[test]
    fn test_mean_single() {
        assert_eq!(mean(&[5.0]), 5.0);
    }

    #[test]
    fn test_mean_multiple() {
        assert_eq!(mean(&[1.0, 2.0, 3.0, 4.0, 5.0]), 3.0);
    }

    #[test]
    fn test_percentile_empty() {
        assert_eq!(percentile(&[], 50.0), 0.0);
    }

    #[test]
    fn test_percentile_single() {
        assert_eq!(percentile(&[42.0], 50.0), 42.0);
    }

    #[test]
    fn test_percentile_p50() {
        let sorted = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(percentile(&sorted, 50.0), 3.0);
    }
}
