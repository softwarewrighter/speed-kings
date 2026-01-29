# Product Requirements Document (PRD)

> Speed Kings: LLM Inference Benchmarking Tool

## Overview

Speed Kings (inference-showdown) is a CLI tool that benchmarks LLM inference performance across multiple cloud providers and local hardware. It measures and compares speed (tokens/second), latency (time to first token), and cost efficiency to help developers choose the optimal inference platform for their use case.

## Problem Statement

Developers and organizations face challenges when selecting LLM inference providers:

1. **Performance varies widely** - Different providers offer dramatically different inference speeds (40 tok/s local vs 1800 tok/s Cerebras)
2. **Pricing is opaque** - Cost structures differ (per-token, per-request, tiered)
3. **No standardized benchmarks** - Hard to make apples-to-apples comparisons
4. **Hardware requirements unclear** - Local vs cloud trade-offs not well documented

## Target Users

1. **AI/ML Engineers** - Choosing inference infrastructure for production
2. **Startup CTOs** - Optimizing cost vs performance for limited budgets
3. **Content Creators** - Demonstrating LLM performance comparisons (video series)
4. **Hobbyists** - Understanding local hardware capabilities vs cloud options

## Goals

### Primary Goals

1. **Accurate Performance Measurement**
   - Measure tokens per second (throughput)
   - Measure time to first token (TTFT/latency)
   - Measure total request latency
   - Support multiple test prompt sizes

2. **Multi-Provider Support**
   - Cerebras (ultra-fast inference)
   - Groq (optimized LPU inference)
   - SambaNova (enterprise inference)
   - Fireworks (serverless inference)
   - DeepSeek (cost-effective)
   - OpenAI-compatible APIs
   - Local inference (Ollama, llama.cpp)

3. **Clear Cost Analysis**
   - Track cost per 1M input tokens
   - Track cost per 1M output tokens
   - Calculate cost per benchmark run
   - Support tiered pricing models

4. **Professional Output**
   - Terminal tables with clear formatting
   - JSON export for data analysis
   - Markdown export for documentation
   - CSV export for spreadsheets

### Secondary Goals

1. **Reproducible Benchmarks** - Consistent test prompts and methodology
2. **Extensibility** - Easy to add new providers
3. **Video-Ready Output** - Clean visualizations for content creation
4. **Budget-Conscious** - Default settings keep costs under $0.25 per run

## Non-Goals

1. Model accuracy/quality comparisons (focus is on speed/cost)
2. Fine-tuning or training capabilities
3. Production inference serving
4. GUI interface (CLI only)

## Functional Requirements

### FR1: Provider Configuration

- Support API key configuration via environment variables
- Support endpoint URL customization for OpenAI-compatible APIs
- Validate provider credentials before benchmarking
- Graceful handling of unavailable providers

### FR2: Benchmark Execution

- Run benchmarks with configurable parameters:
  - Number of iterations (default: 1-2 for cost control)
  - Warmup iterations (default: 0-1, skip for costly providers)
  - Test prompt size (short/medium/long)
  - Specific providers or all available
- Sequential execution (not concurrent)
- Display real-time progress during benchmark
- Handle provider errors gracefully (timeout, rate limits)
- Show estimated cost before running, allow user to confirm

### FR3: Metrics Collection

| Metric | Description | Unit |
|--------|-------------|------|
| Time to Prompt | Time from request start to prompt sent | milliseconds |
| TTFT | Time to first token (after prompt sent) | milliseconds |
| Throughput | Output generation speed | tokens/second |
| Total Latency | Full request time | milliseconds |
| Input Tokens | Tokens in prompt | count |
| Output Tokens | Tokens generated | count |
| Cost | Estimated cost | USD |
| Model Load Time | One-time model download/load (local only) | seconds |

**Note on Local Inference**: First run may include one-time model download overhead. This is tracked separately from inference metrics and documented clearly in output.

### FR4: Output Formats

**Terminal Table (default)**:
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

**JSON Export**: Machine-readable results with full metadata
**Markdown Export**: Documentation-ready tables
**CSV Export**: Spreadsheet-compatible format

### FR5: Test Prompts

Three standardized prompts for consistent benchmarking:

1. **Short (~50 tokens)**: Quick response test
2. **Medium (~200 tokens)**: Typical interaction
3. **Long (~500 tokens)**: Extended context handling

### FR6: Local Inference Support

- Detect and connect to Ollama installation
- Support llama.cpp server endpoints
- Auto-detect available local models
- No cost tracking for local inference

## Non-Functional Requirements

### NFR1: Performance

- CLI startup time < 500ms
- Minimal overhead on benchmark measurements
- Support concurrent provider testing

### NFR2: Reliability

- Graceful timeout handling (30s default)
- Automatic retry on transient failures (3 attempts)
- Clear error messages with troubleshooting hints

### NFR3: Usability

- Comprehensive --help documentation
- Sensible defaults for all options
- Progress indicators for long-running benchmarks

### NFR4: Portability

- Cross-platform: macOS, Linux, Windows
- Single binary distribution
- No runtime dependencies (except for local inference)

## User Stories

### US1: Quick Comparison

> As an ML engineer, I want to quickly compare inference speeds across providers so I can choose the fastest option for my real-time application.

**Acceptance Criteria**:
- Run `speed-kings benchmark --providers cerebras,groq,fireworks`
- See results in < 2 minutes
- Clear ranking by tokens/second

### US2: Cost Analysis

> As a startup CTO, I want to understand the cost implications of different providers so I can optimize my inference budget.

**Acceptance Criteria**:
- Run `speed-kings benchmark --output json`
- Export includes cost per 1M tokens for each provider
- Calculate estimated monthly cost at given volume

### US3: Local vs Cloud

> As a hobbyist, I want to compare my local M3 Mac against cloud providers so I can decide if cloud inference is worth the cost.

**Acceptance Criteria**:
- Run `speed-kings benchmark --providers local,cerebras`
- See side-by-side comparison
- Clear latency and throughput differences

### US4: Video Content

> As a content creator, I want clean, visual benchmark results so I can include them in my Speed Kings video series.

**Acceptance Criteria**:
- Run `speed-kings benchmark --output markdown`
- Results formatted for video overlay
- Include timestamp and test metadata

## Success Metrics

1. **Accuracy**: Benchmark variance < 10% across runs
2. **Coverage**: Support 6+ major inference providers
3. **Adoption**: Used in Speed Kings video series (Part 1/7)
4. **Usability**: New user can run first benchmark in < 5 minutes

## Timeline

| Phase | Scope | Status |
|-------|-------|--------|
| Phase 1 | Core benchmarking + 2 providers | Not Started |
| Phase 2 | All cloud providers | Not Started |
| Phase 3 | Local inference support | Not Started |
| Phase 4 | Output formats + polish | Not Started |

## Resolved Questions

1. ~~Should we support concurrent benchmarking across providers?~~ **No, sequential only**
2. ~~What's the right default for benchmark iterations?~~ **Budget-conscious defaults, target $0.10-0.25 per benchmark run max**
3. ~~Should we include model download/loading time for local inference?~~ **Yes, include but document the one-time download overhead separately. Track time-to-prompt and time-to-first-token as distinct metrics.**
4. ~~How to handle rate-limited providers (Groq free tier)?~~ **Document provider upgrade options (credit card pre-load, monthly plans). Research optimal benchmark timing to avoid contention.**

## Budget Constraints

- Target cost per benchmark run: $0.10-0.25 max
- Overall budget: Keep total spending low (avoid multi-dollar runs)
- Default iterations should be minimal for costly providers (1-2)
- Consider auto-limiting based on provider pricing

## Provider Account Considerations

### Rate Limiting and Throttling

Free tiers and basic accounts may experience:
- Request rate limits (429 errors)
- Throttling during peak hours
- De-prioritization vs paid users

### Upgrade Options by Provider

| Provider | Free Tier | Paid Options | Notes |
|----------|-----------|--------------|-------|
| Cerebras | Limited, may throttle | Credit pre-load | May need upgrade to avoid de-prioritization |
| Groq | Strict rate limits | Monthly plan, credit card | Free tier very limited for benchmarking |
| Fireworks | Limited | Pay-as-you-go | Credit card pre-load |
| SambaNova | Enterprise focus | Contact sales | May require enterprise account |
| DeepSeek | Generous free tier | Pay-as-you-go | Good for budget testing |

### Timing Considerations

- Peak hours (US business hours) may have more contention
- Off-peak benchmarking may yield better/more consistent results
- Document time-of-day in benchmark metadata for reproducibility
- Consider researching optimal benchmark windows per provider

## References

- [Cerebras Inference API](https://cerebras.ai/)
- [Groq API Documentation](https://groq.com/)
- [SambaNova API](https://sambanova.ai/)
- [Fireworks AI](https://fireworks.ai/)
- [Ollama](https://ollama.ai/)
- [llama.cpp](https://github.com/ggerganov/llama.cpp)
