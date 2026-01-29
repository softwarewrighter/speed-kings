# Speed Kings

> LLM Inference Benchmarking Tool - Compare speed, latency, and cost across providers

## Overview

Speed Kings is a CLI tool that benchmarks LLM inference performance across multiple cloud providers and local hardware. It measures and compares:

- **Throughput** - Tokens per second
- **Latency** - Time to first token (TTFT) and total response time
- **Cost** - Estimated cost per benchmark run

Supports Cerebras, Groq, SambaNova, Fireworks, DeepSeek, OpenAI-compatible APIs, and local inference (Ollama/llama.cpp).

## Purpose

Developers face challenges when selecting LLM inference providers:

- Performance varies dramatically (40 tok/s local vs 1800+ tok/s Cerebras)
- Pricing structures differ across providers
- No standardized way to make apples-to-apples comparisons

Speed Kings provides reproducible, budget-conscious benchmarks to help you choose the right provider for your use case.

## Usage

```bash
# Benchmark all available providers
speed-kings benchmark

# Benchmark specific providers
speed-kings benchmark --providers cerebras,groq,local

# Use short prompts (lower cost)
speed-kings benchmark --size short

# Output as JSON for analysis
speed-kings benchmark --output json

# List available providers and their status
speed-kings list

# Show pricing information
speed-kings pricing
```

### Environment Variables

Set API keys for the providers you want to benchmark:

```bash
export CEREBRAS_API_KEY="..."
export GROQ_API_KEY="..."
export FIREWORKS_API_KEY="..."
export SAMBANOVA_API_KEY="..."
export DEEPSEEK_API_KEY="..."
```

For local inference, start Ollama:

```bash
ollama serve
```

## Example Output

```
+------------+--------+----------+---------+----------+
| Provider   | TTFT   | Tok/sec  | Latency | Cost/1M  |
+------------+--------+----------+---------+----------+
| Cerebras   | 45ms   | 1823     | 1.2s    | $0.10    |
| Groq       | 89ms   | 756      | 2.8s    | $0.05    |
| Fireworks  | 120ms  | 412      | 5.1s    | $0.20    |
| Local M3   | 15ms   | 42       | 48.2s   | $0.00    |
+------------+--------+----------+---------+----------+
```

## Budget

Default settings target $0.10-0.25 per benchmark run. Use `--size short` and `--iterations 1` for minimal cost during exploration.

## Installation

```bash
# From source
git clone https://github.com/softwarewrighter/speed-kings.git
cd speed-kings
cargo build --release
./target/release/speed-kings --help
```

## Documentation

- [Product Requirements](docs/prd.md) - Goals, user stories, requirements
- [Architecture](docs/architecture.md) - System design and components
- [Design](docs/design.md) - Design decisions and patterns
- [Implementation Plan](docs/plan.md) - Phased development roadmap
- [Status](docs/status.md) - Current progress and decisions
- [Development Process](docs/process.md) - TDD workflow and quality gates
- [Tools](docs/tools.md) - Recommended development tools

## License

See [LICENSE](LICENSE) and [COPYRIGHT](COPYRIGHT) files.

## Status

**Phase 1: Core Foundation** - In Progress

Part of the Speed Kings video series (Part 1/7 of "Speed & Cost Showdown").
