# Implementation Plan

> Phased implementation plan for Speed Kings.

## Overview

This document outlines the implementation phases, priorities, and task breakdowns for building the Speed Kings inference benchmarking tool.

## Implementation Phases

### Phase 1: Core Foundation

**Goal**: Minimal viable benchmarking with 2 providers

**Tasks**:

1. **Project Setup**
   - [x] Initialize Rust project with Cargo.toml
   - [ ] Add core dependencies (clap, tokio, reqwest, serde)
   - [ ] Setup module structure
   - [ ] Configure CI/CD (GitHub Actions)

2. **CLI Framework**
   - [ ] Implement argument parsing with clap
   - [ ] Add benchmark subcommand
   - [ ] Add list subcommand
   - [ ] Implement --help with examples

3. **Provider Trait**
   - [ ] Define InferenceProvider trait
   - [ ] Define InferenceRequest/Response structs
   - [ ] Define ProviderError enum
   - [ ] Create ProviderRegistry

4. **First Provider: Cerebras**
   - [ ] Implement CerebrasProvider
   - [ ] Handle streaming responses
   - [ ] Measure TTFT accurately
   - [ ] Add integration test (requires API key)

5. **Second Provider: Local (Ollama)**
   - [ ] Implement LocalProvider
   - [ ] Auto-detect Ollama installation
   - [ ] List available local models
   - [ ] Handle connection errors gracefully

6. **Basic Benchmark Engine**
   - [ ] Implement BenchmarkRunner
   - [ ] Add warmup iterations
   - [ ] Collect basic metrics (TTFT, throughput, latency)
   - [ ] Run single provider benchmark

7. **Terminal Output**
   - [ ] Implement table formatter
   - [ ] Display results with aligned columns
   - [ ] Show progress during benchmark

**Deliverable**: `speed-kings benchmark --providers cerebras,local` produces results

### Phase 2: Full Provider Support

**Goal**: Support all major cloud providers

**Tasks**:

1. **Provider: Groq**
   - [ ] Implement GroqProvider
   - [ ] Handle rate limiting (free tier)
   - [ ] Test with llama3-70b model

2. **Provider: SambaNova**
   - [ ] Implement SambaNovaProvider
   - [ ] Handle enterprise API format
   - [ ] Add pricing data

3. **Provider: Fireworks**
   - [ ] Implement FireworksProvider
   - [ ] Handle serverless model loading
   - [ ] Test with llama-v3p1-70b

4. **Provider: DeepSeek**
   - [ ] Implement DeepSeekProvider
   - [ ] Handle DeepSeek API format
   - [ ] Add pricing data

5. **Provider: OpenAI-Compatible**
   - [ ] Implement generic OpenAICompatibleProvider
   - [ ] Allow custom endpoint URL
   - [ ] Support local OpenAI-compatible servers

6. **Pricing System**
   - [ ] Create pricing.json data file
   - [ ] Implement pricing lookup
   - [ ] Calculate cost per benchmark run
   - [ ] Add pricing subcommand

7. **Provider Discovery**
   - [ ] Improve list command output
   - [ ] Show availability status
   - [ ] Display required environment variables

**Deliverable**: All 6+ providers working with pricing

### Phase 3: Advanced Features

**Goal**: Production-ready benchmarking

**Tasks**:

1. **Test Prompts**
   - [ ] Define short/medium/long prompts
   - [ ] Add --size CLI flag
   - [ ] Validate consistent output length
   - [ ] Document prompt methodology

2. **Metrics Enhancement**
   - [ ] Add P50/P95 latency percentiles
   - [ ] Calculate standard deviation
   - [ ] Track input/output token counts
   - [ ] Compute tokens per dollar

3. **Retry Logic**
   - [ ] Add exponential backoff
   - [ ] Configure max retries
   - [ ] Handle transient failures
   - [ ] Rate limit detection

4. **Timeout Handling**
   - [ ] Configurable timeout per request
   - [ ] Graceful timeout recovery
   - [ ] Partial results on timeout

5. **Progress Indication**
   - [ ] Add progress bar during benchmark
   - [ ] Show real-time results as they complete
   - [ ] Estimate time remaining

**Deliverable**: Robust benchmarking with reliable metrics

### Phase 4: Output and Polish

**Goal**: Professional output formats and documentation

**Tasks**:

1. **JSON Output**
   - [ ] Implement JSON formatter
   - [ ] Include full metadata
   - [ ] Add --output json flag
   - [ ] Document JSON schema

2. **Markdown Output**
   - [ ] Implement Markdown formatter
   - [ ] Create documentation-ready tables
   - [ ] Add timestamp and version info

3. **CSV Output**
   - [ ] Implement CSV formatter
   - [ ] Header row with column names
   - [ ] Compatible with spreadsheets

4. **README Documentation**
   - [ ] Write comprehensive README
   - [ ] Add installation instructions
   - [ ] Include usage examples
   - [ ] Document all CLI options

5. **Release Preparation**
   - [ ] Create release binaries
   - [ ] Write CHANGELOG
   - [ ] Tag version 0.1.0
   - [ ] Publish to crates.io (optional)

**Deliverable**: Release-ready v0.1.0

## Task Dependencies

```
Phase 1:
  Project Setup
      |
      v
  CLI Framework --> Provider Trait
      |                 |
      v                 v
  First Provider --> Benchmark Engine --> Terminal Output
      |
      v
  Second Provider

Phase 2:
  Phase 1 Complete
      |
      v
  Additional Providers (parallel)
      |
      v
  Pricing System --> Provider Discovery

Phase 3:
  Phase 2 Complete
      |
      v
  Test Prompts --> Metrics Enhancement
                        |
                        v
                  Retry Logic --> Timeout Handling
                                      |
                                      v
                                Progress Indication

Phase 4:
  Phase 3 Complete
      |
      v
  Output Formats (parallel: JSON, Markdown, CSV)
      |
      v
  Documentation --> Release
```

## Environment Variables

| Variable | Provider | Required |
|----------|----------|----------|
| `CEREBRAS_API_KEY` | Cerebras | For cloud |
| `GROQ_API_KEY` | Groq | For cloud |
| `SAMBANOVA_API_KEY` | SambaNova | For cloud |
| `FIREWORKS_API_KEY` | Fireworks | For cloud |
| `DEEPSEEK_API_KEY` | DeepSeek | For cloud |
| `OLLAMA_URL` | Local | No (default: localhost:11434) |

## API Endpoints

| Provider | Endpoint | Auth |
|----------|----------|------|
| Cerebras | `https://api.cerebras.ai/v1/chat/completions` | Bearer token |
| Groq | `https://api.groq.com/openai/v1/chat/completions` | Bearer token |
| SambaNova | `https://api.sambanova.ai/v1/chat/completions` | Bearer token |
| Fireworks | `https://api.fireworks.ai/inference/v1/chat/completions` | Bearer token |
| DeepSeek | `https://api.deepseek.com/chat/completions` | Bearer token |
| Ollama | `http://localhost:11434/api/generate` | None |

## Dependencies

```toml
[package]
name = "speed-kings"
version = "0.1.0"
edition = "2024"

[dependencies]
# CLI
clap = { version = "4", features = ["derive"] }

# Async runtime
tokio = { version = "1", features = ["full"] }

# HTTP client
reqwest = { version = "0.12", features = ["json", "stream"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Async traits
async-trait = "0.1"

# Error handling
thiserror = "2"
anyhow = "1"

# Output formatting
comfy-table = "7"

# Time handling
chrono = { version = "0.4", features = ["serde"] }

# Progress indication
indicatif = "0.17"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

[dev-dependencies]
tokio-test = "0.4"
tempfile = "3"
```

## Testing Strategy

### Unit Tests

- Metrics calculations
- Pricing calculations
- Output formatting
- CLI argument parsing

### Integration Tests

- Mock provider benchmark flow
- Output format verification
- Error handling paths

### Manual Testing

```bash
# Test with local Ollama
ollama serve &
speed-kings benchmark --providers local

# Test with single cloud provider
export CEREBRAS_API_KEY=...
speed-kings benchmark --providers cerebras

# Full benchmark
export GROQ_API_KEY=...
export FIREWORKS_API_KEY=...
speed-kings benchmark --providers all
```

## Success Criteria

### Phase 1 Complete When:

- [ ] `speed-kings benchmark --providers cerebras` returns results
- [ ] `speed-kings benchmark --providers local` returns results
- [ ] Terminal table displays correctly
- [ ] All unit tests pass
- [ ] CI/CD pipeline green

### Phase 2 Complete When:

- [ ] All 6+ providers implemented
- [ ] Pricing data accurate for all providers
- [ ] `speed-kings list` shows all providers with status
- [ ] `speed-kings pricing` displays cost information

### Phase 3 Complete When:

- [ ] Three test prompt sizes available
- [ ] P50/P95 metrics calculated
- [ ] Retry logic handles transient failures
- [ ] Progress bar shows during benchmark

### Phase 4 Complete When:

- [ ] JSON/Markdown/CSV output working
- [ ] README complete with examples
- [ ] CHANGELOG written
- [ ] v0.1.0 tagged and released

## Notes

1. **Start simple**: Get one provider working end-to-end before adding more
2. **Test locally first**: Use Ollama for development to avoid API costs
3. **Measure accurately**: Warmup runs and multiple iterations are essential
4. **Document as you go**: Update these docs as implementation proceeds
5. **Follow TDD**: Write tests before implementation where practical
