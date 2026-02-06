# Project Status

> Current development status for Speed Kings.

## Overview

| Aspect | Status |
|--------|--------|
| **Project** | Speed Kings (inference-showdown) |
| **Version** | 0.1.0-dev |
| **Phase** | Phase 2: Full Provider Support (Complete) |
| **Last Updated** | 2026-02-06 |

## Current State

Core benchmarking tool is complete with 11 providers. Ready to begin pairwise comparison benchmarks.

### What Exists

- [x] Git repository initialized
- [x] Cargo.toml with all dependencies
- [x] Full CLI implementation (benchmark, list, pricing)
- [x] Documentation structure (docs/)
- [x] PRD, architecture, design, plan, status docs
- [x] README.md
- [x] Provider trait and 11 implementations
- [x] Benchmark engine with metrics
- [x] Output formatters (table, JSON, Markdown, CSV)
- [x] 6 unit tests for metrics calculations

### Providers Implemented (11 total)

| Provider | Hardware Type | Status |
|----------|--------------|--------|
| Cerebras | WSE (Wafer-Scale Engine) | Complete |
| Groq | LPU (Language Processing Unit) | Complete |
| SambaNova | RDU (Reconfigurable Dataflow) | Complete |
| Fireworks | NVIDIA GPU Cloud | Complete |
| Together AI | NVIDIA GPU Cloud | Complete |
| DeepSeek | Native API | Complete |
| Z.ai (Zhipu) | Native API | Complete |
| Moonshot (Kimi) | Native API | Complete |
| OpenRouter | Aggregator | Complete |
| OpenAI-Compatible | Custom Endpoint | Complete |
| Local (Ollama) | Apple Silicon / Consumer GPU | Complete |

### What's Missing

- [ ] CI/CD configuration
- [ ] API key testing with real credentials
- [ ] Benchmark execution and results collection

## Phase Progress

### Phase 1: Core Foundation (Complete)

| Task | Status | Notes |
|------|--------|-------|
| Project Setup | Complete | Dependencies, module structure |
| CLI Framework | Complete | clap with subcommands |
| Provider Trait | Complete | InferenceProvider trait |
| Cerebras Provider | Complete | Streaming SSE support |
| Local Provider | Complete | Ollama integration |
| Benchmark Engine | Complete | Sequential runner with metrics |
| Terminal Output | Complete | Table, JSON, Markdown, CSV |

### Phase 2: Full Provider Support (Complete)

| Task | Status | Notes |
|------|--------|-------|
| Groq Provider | Complete | LPU inference, streaming SSE |
| SambaNova Provider | Complete | RDU inference, streaming SSE |
| Fireworks Provider | Complete | GPU cloud, streaming SSE |
| Together AI Provider | Complete | GPU cloud, streaming SSE |
| DeepSeek Provider | Complete | Native API, streaming SSE |
| Z.ai Provider | Complete | GLM models, streaming SSE |
| Moonshot Provider | Complete | Kimi models, streaming SSE |
| OpenRouter Provider | Complete | Aggregator, streaming SSE |
| OpenAI-Compatible | Complete | Generic endpoint support |
| Pricing System | Complete | All providers have pricing |

### Phase 3: Benchmark Execution (Not Started)

See [plan.md](./plan.md) for the 24-phase pairwise comparison strategy.

| Group | Phases | Model | Status |
|-------|--------|-------|--------|
| A: Small Model (8B) | A1-A8 | Llama 3.1 8B | Not Started |
| B: Large Model (70B) | B1-B6 | Llama 3.1/3.3 70B | Not Started |
| C: GLM-4.7 | C1-C2 | GLM-4.7 | Not Started |
| D: DeepSeek-V3 | D1-D3 | DeepSeek-V3 | Not Started |
| E: Kimi K2.5 | E1-E3 | Kimi K2.5 | Not Started |
| F: Local Hardware | F1-F2 | Llama 3.1 8B | Not Started |

## Blockers

None currently. Ready to begin benchmark execution with real API keys.

## Key Decisions Made

1. **No concurrent benchmarking** - Sequential execution only
2. **Budget target** - $0.10-0.25 per benchmark run max
3. **Local model loading** - Track separately, document one-time overhead
4. **Rate limiting** - Document provider upgrade options, research optimal timing
5. **Metrics** - Track time-to-prompt and time-to-first-token as distinct measurements
6. **Pairwise comparisons** - Each benchmark phase compares exactly 2 providers using the same model
7. **Hardware isolation** - Compare WSE vs LPU vs RDU vs GPU vs local to isolate infrastructure differences

## Next Steps

1. Obtain API keys for all cloud providers
2. Run Priority 1 benchmarks (A1, A2, A7)
3. Document findings in docs/findings.md
4. Run remaining benchmark phases

## Milestones

| Milestone | Target | Status |
|-----------|--------|--------|
| All providers implemented | 2026-02-06 | Complete |
| First benchmark run | TBD | Not Started |
| 10+ phases completed | TBD | Not Started |
| v0.1.0 release | TBD | Not Started |

## Recent Changes

### 2026-02-06

- **Added 4 new providers**:
  - Together AI provider (NVIDIA GPU cloud)
  - Z.ai provider (GLM-4.7 native)
  - Moonshot provider (Kimi K2.5 native)
  - OpenRouter provider (aggregator)
- **Total providers: 11** (up from 7)
- **Rewrote plan.md** with 24-phase pairwise comparison strategy:
  - Group A: Small model infrastructure comparison (8 phases)
  - Group B: Large model scaling (6 phases)
  - Group C: GLM-4.7 comparison (2 phases)
  - Group D: DeepSeek-V3 comparison (3 phases)
  - Group E: Kimi K2.5 comparison (3 phases)
  - Group F: Local hardware comparison (2 phases)
- Added model availability matrix to plan.md
- Added environment variables reference to plan.md

### 2026-01-29

- Initial project setup
- Created documentation structure
- Added PRD, architecture, design, plan, and status docs
- **Phase 1 Implementation Complete**:
  - CLI with benchmark, list, pricing subcommands
  - InferenceProvider trait with Cerebras and Local (Ollama) providers
  - BenchmarkRunner with sequential execution
  - Cost estimation and confirmation prompt
  - Output formats: table, JSON, Markdown, CSV
  - Metrics: TTFT, throughput, latency, cost
  - 6 unit tests for metrics calculations
- **Phase 2 Implementation Complete**:
  - Added Groq, SambaNova, Fireworks, DeepSeek providers
  - Added OpenAI-compatible provider (custom endpoints)
  - All API endpoints verified (401 unauthorized with test keys)

## Known Issues

None currently.

## Technical Debt

None currently (greenfield project).

## Metrics

Not yet tracking metrics. Will add after Phase 1 completion:

- Build time
- Test coverage
- Binary size
- Benchmark accuracy

## Resources

- [PRD](./prd.md) - Product requirements
- [Architecture](./architecture.md) - System architecture
- [Design](./design.md) - Design decisions
- [Plan](./plan.md) - Implementation plan
- [Process](./process.md) - Development process
- [Tools](./tools.md) - Development tools
