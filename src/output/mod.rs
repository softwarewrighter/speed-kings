//! Output formatting for benchmark results.

mod table;

pub use table::format_table;

use crate::benchmark::BenchmarkResult;
use crate::cli::OutputFormat;
use chrono::Utc;
use serde::Serialize;

/// Format benchmark results according to the specified output format
pub fn format_results(results: &[BenchmarkResult], format: OutputFormat) -> String {
    match format {
        OutputFormat::Table => format_table(results),
        OutputFormat::Json => format_json(results),
        OutputFormat::Markdown => format_markdown(results),
        OutputFormat::Csv => format_csv(results),
    }
}

/// JSON output with full metadata
#[derive(Serialize)]
struct JsonOutput<'a> {
    timestamp: String,
    version: &'static str,
    results: &'a [BenchmarkResult],
}

fn format_json(results: &[BenchmarkResult]) -> String {
    let output = JsonOutput {
        timestamp: Utc::now().to_rfc3339(),
        version: env!("CARGO_PKG_VERSION"),
        results,
    };

    serde_json::to_string_pretty(&output).unwrap_or_else(|e| format!("JSON error: {}", e))
}

fn format_markdown(results: &[BenchmarkResult]) -> String {
    let mut output = String::new();

    output.push_str("# Inference Benchmark Results\n\n");
    output.push_str(&format!(
        "*Generated: {}*\n\n",
        Utc::now().format("%Y-%m-%d %H:%M UTC")
    ));

    output.push_str("| Provider | Model | TTFT | Throughput | Latency | Cost |\n");
    output.push_str("|----------|-------|------|------------|---------|------|\n");

    for result in results {
        if result.is_success() {
            output.push_str(&format!(
                "| {} | {} | {}ms | {:.0} tok/s | {}ms | ${:.4} |\n",
                result.display_name,
                result.model,
                result.metrics.avg_ttft_ms as u64,
                result.metrics.avg_tokens_per_sec,
                result.metrics.avg_latency_ms as u64,
                result.metrics.total_cost_usd,
            ));
        } else {
            output.push_str(&format!(
                "| {} | {} | - | - | - | - |\n",
                result.display_name, result.model,
            ));
        }
    }

    // Add notes section for model load times
    let has_load_times = results
        .iter()
        .any(|r| r.metrics.model_load_time_ms.is_some());

    if has_load_times {
        output.push_str("\n**Notes:**\n");
        for result in results {
            if let Some(load_time) = result.metrics.model_load_time_ms {
                output.push_str(&format!(
                    "- {}: Model load time {}ms (one-time, not included in metrics)\n",
                    result.display_name, load_time
                ));
            }
        }
    }

    output
}

fn format_csv(results: &[BenchmarkResult]) -> String {
    let mut output = String::new();

    // Header
    output.push_str("provider,model,ttft_ms,tokens_per_sec,latency_ms,cost_usd,runs\n");

    // Data rows
    for result in results {
        output.push_str(&format!(
            "{},{},{:.0},{:.1},{:.0},{:.6},{}\n",
            result.provider,
            result.model,
            result.metrics.avg_ttft_ms,
            result.metrics.avg_tokens_per_sec,
            result.metrics.avg_latency_ms,
            result.metrics.total_cost_usd,
            result.metrics.run_count,
        ));
    }

    output
}
