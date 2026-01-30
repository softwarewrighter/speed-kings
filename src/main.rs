//! Speed Kings - LLM Inference Benchmarking Tool

use anyhow::Result;
use clap::Parser;
use speed_kings::benchmark::{BenchmarkConfig, BenchmarkRunner};
use speed_kings::cli::{Cli, Commands, OutputFormat};
use speed_kings::output::format_results;
use speed_kings::pricing::format_pricing_table;
use speed_kings::providers::ProviderRegistry;
use std::io::{self, Write};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Benchmark {
            providers,
            iterations,
            size,
            output,
            yes,
        } => {
            run_benchmark(&providers, iterations, size, output, yes, cli.verbose).await?;
        }
        Commands::List => {
            list_providers();
        }
        Commands::Pricing => {
            println!("{}", format_pricing_table());
        }
    }

    Ok(())
}

async fn run_benchmark(
    provider_filter: &str,
    iterations: u32,
    size: speed_kings::cli::PromptSize,
    output_format: OutputFormat,
    skip_confirm: bool,
    verbose: bool,
) -> Result<()> {
    let registry = ProviderRegistry::new();

    if registry.is_empty() {
        eprintln!("No providers available.\n");
        eprintln!("Set one or more API keys:");
        eprintln!("  export CEREBRAS_API_KEY=...");
        eprintln!("  export GROQ_API_KEY=...");
        eprintln!("  export FIREWORKS_API_KEY=...");
        eprintln!("  export SAMBANOVA_API_KEY=...");
        eprintln!("  export DEEPSEEK_API_KEY=...");
        eprintln!("\nOr start Ollama for local inference:");
        eprintln!("  ollama serve");
        std::process::exit(1);
    }

    // Filter providers based on input
    let providers: Vec<_> = if provider_filter == "all" {
        registry.all()
    } else {
        let names: Vec<&str> = provider_filter.split(',').map(|s| s.trim()).collect();
        let mut filtered = Vec::new();
        for name in names {
            if let Some(provider) = registry.get(name) {
                filtered.push(provider);
            } else {
                eprintln!(
                    "Warning: Provider '{}' not available. Available: {:?}",
                    name,
                    registry.available()
                );
            }
        }
        filtered
    };

    if providers.is_empty() {
        eprintln!("No matching providers found.");
        std::process::exit(1);
    }

    let config = BenchmarkConfig {
        iterations,
        prompt_size: size,
        timeout_ms: 60_000,
    };

    let runner = BenchmarkRunner::new(providers.clone(), config);

    // Estimate and confirm cost
    let estimated_cost = runner.estimate_cost();

    if !skip_confirm && estimated_cost > 0.0 {
        println!("Benchmark configuration:");
        println!(
            "  Providers: {:?}",
            providers.iter().map(|p| p.name()).collect::<Vec<_>>()
        );
        println!("  Iterations: {}", iterations);
        println!("  Prompt size: {:?}", size);
        println!("  Estimated cost: ${:.4}", estimated_cost);
        println!();

        print!("Proceed? [y/N] ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    if verbose {
        println!("Starting benchmark...\n");
    }

    // Run benchmarks
    let results = runner.run().await;

    // Output results
    println!("{}", format_results(&results, output_format));

    // Summary
    let total_cost: f64 = results.iter().map(|r| r.metrics.total_cost_usd).sum();
    let successful = results.iter().filter(|r| r.is_success()).count();

    if verbose {
        println!("\nSummary:");
        println!("  Providers tested: {}/{}", successful, results.len());
        println!("  Total cost: ${:.4}", total_cost);
    }

    Ok(())
}

fn list_providers() {
    let registry = ProviderRegistry::new();

    println!("Available Providers");
    println!("===================\n");

    if registry.is_empty() {
        println!("No providers configured.\n");
        println!("To enable providers, set environment variables:");
        println!("  CEREBRAS_API_KEY       - Cerebras inference");
        println!("  GROQ_API_KEY           - Groq inference");
        println!("  FIREWORKS_API_KEY      - Fireworks inference");
        println!("  SAMBANOVA_API_KEY      - SambaNova inference");
        println!("  DEEPSEEK_API_KEY       - DeepSeek inference");
        println!("  OPENAI_COMPATIBLE_URL  - Custom OpenAI-compatible endpoint");
        println!("  OLLAMA_URL             - Local Ollama (default: http://localhost:11434)");
        return;
    }

    for provider in registry.all() {
        let (input_price, output_price) = provider.pricing_per_million();
        println!("  {} ({})", provider.display_name(), provider.name());
        println!("    Model: {}", provider.default_model());
        if input_price > 0.0 {
            println!(
                "    Pricing: ${:.3}/${:.3} per 1M tokens (in/out)",
                input_price, output_price
            );
        } else {
            println!("    Pricing: Free (local)");
        }
        println!();
    }
}
