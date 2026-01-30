//! Terminal table formatting for benchmark results.

use crate::benchmark::BenchmarkResult;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table, presets::UTF8_FULL};

/// Format benchmark results as a terminal table
pub fn format_table(results: &[BenchmarkResult]) -> String {
    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Provider").add_attribute(Attribute::Bold),
            Cell::new("Model").add_attribute(Attribute::Bold),
            Cell::new("TTFT").add_attribute(Attribute::Bold),
            Cell::new("Tok/sec").add_attribute(Attribute::Bold),
            Cell::new("Latency").add_attribute(Attribute::Bold),
            Cell::new("Cost").add_attribute(Attribute::Bold),
        ]);

    for result in results {
        if result.is_success() {
            table.add_row(vec![
                Cell::new(&result.display_name),
                Cell::new(&result.model),
                Cell::new(format!("{}ms", result.metrics.avg_ttft_ms as u64)),
                Cell::new(format!("{:.0}", result.metrics.avg_tokens_per_sec)),
                Cell::new(format!("{}ms", result.metrics.avg_latency_ms as u64)),
                Cell::new(format!("${:.4}", result.metrics.total_cost_usd)),
            ]);
        } else {
            // Show failed providers with error indication
            table.add_row(vec![
                Cell::new(&result.display_name),
                Cell::new(&result.model),
                Cell::new("-").fg(Color::Red),
                Cell::new("-").fg(Color::Red),
                Cell::new("-").fg(Color::Red),
                Cell::new("-").fg(Color::Red),
            ]);
        }
    }

    let mut output = table.to_string();

    // Add notes for model load times and errors
    let mut notes = Vec::new();

    for result in results {
        if let Some(load_time) = result.metrics.model_load_time_ms {
            notes.push(format!(
                "{}: Model load time {}ms (one-time overhead)",
                result.display_name, load_time
            ));
        }

        if !result.errors.is_empty() {
            for error in &result.errors {
                notes.push(format!("{}: {}", result.display_name, error));
            }
        }
    }

    if !notes.is_empty() {
        output.push_str("\n\nNotes:\n");
        for note in notes {
            output.push_str(&format!("  - {}\n", note));
        }
    }

    output
}
