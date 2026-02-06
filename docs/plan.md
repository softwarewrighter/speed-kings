# Implementation Plan

> Multi-phase 2-way comparison benchmark strategy for Speed Kings.

## Overview

This plan organizes benchmarks as **pairwise comparisons** - each phase compares exactly 2 providers using the **same model**. This isolates the infrastructure difference and allows us to infer patterns across many comparisons.

## Hardware & Infrastructure Types

| Type | Provider | Technology | Speed Class |
|------|----------|------------|-------------|
| **WSE** | Cerebras | Wafer-Scale Engine | Ultra-fast (~1800 tok/s) |
| **LPU** | Groq | Language Processing Unit | Very fast (~750 tok/s) |
| **RDU** | SambaNova | Reconfigurable Dataflow Unit | Fast |
| **GPU Cloud** | Fireworks, Together, OpenRouter | NVIDIA GPUs | Moderate (~400 tok/s) |
| **Apple Silicon** | Local (Ollama M3 Pro) | Neural Engine + CPU | Slow (~50 tok/s) |
| **Consumer GPU** | Local (Ollama RTX 5060) | NVIDIA CUDA | Moderate (~100 tok/s) |
| **Native** | DeepSeek, Z.ai, Moonshot | Proprietary | Varies |

## Model Availability Matrix

### Llama 3.1 8B (Small Model)

| Provider | Model ID | Available |
|----------|----------|-----------|
| Cerebras | `llama3.1-8b` | Yes |
| Groq | `llama-3.1-8b-instant` | Yes |
| Fireworks | `accounts/fireworks/models/llama-v3p1-8b-instruct` | Yes |
| Together | `meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo` | Yes |
| OpenRouter | `meta-llama/llama-3.1-8b-instruct` | Yes |
| Local (Ollama) | `llama3.1:8b` | Yes |

### Llama 3.1/3.3 70B (Large Model)

| Provider | Model ID | Available |
|----------|----------|-----------|
| Cerebras | `llama-3.3-70b` | Yes |
| Groq | `llama-3.3-70b-versatile` | Yes |
| SambaNova | `Meta-Llama-3.1-70B-Instruct` | Yes |
| Fireworks | `accounts/fireworks/models/llama-v3p1-70b-instruct` | Yes |
| Together | `meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo` | Yes |

### GLM-4.7 (Z.ai Model)

| Provider | Model ID | Available |
|----------|----------|-----------|
| Cerebras | `zai-glm-4.7` | Yes |
| Z.ai | `glm-4.7` | Yes (native) |
| Fireworks | `glm-4.7` | TBD |

### DeepSeek-V3

| Provider | Model ID | Available |
|----------|----------|-----------|
| DeepSeek | `deepseek-chat` | Yes (native) |
| Together | `deepseek-ai/DeepSeek-V3` | Yes |
| Fireworks | `deepseek-v3` | TBD |

### Kimi K2.5 (Moonshot Model)

| Provider | Model ID | Available |
|----------|----------|-----------|
| Moonshot | `kimi-k2.5` | Yes (native) |
| Together | `moonshotai/kimi-k2.5` | Yes |
| OpenRouter | `moonshotai/kimi-k2.5` | Yes |

---

## Benchmark Phases

### Group A: Small Model Infrastructure Comparison (Llama 3.1 8B)

These phases establish baseline infrastructure differences using a small, fast model.

| Phase | Provider 1 | Provider 2 | Model | Hardware Comparison |
|-------|------------|------------|-------|---------------------|
| **A1** | Cerebras | Groq | llama3.1-8b | WSE vs LPU (specialized chips) |
| **A2** | Cerebras | Fireworks | llama3.1-8b | WSE vs NVIDIA GPU cloud |
| **A3** | Cerebras | Together | llama3.1-8b | WSE vs NVIDIA GPU cloud |
| **A4** | Groq | Fireworks | llama3.1-8b | LPU vs NVIDIA GPU cloud |
| **A5** | Groq | Together | llama3.1-8b | LPU vs NVIDIA GPU cloud |
| **A6** | Fireworks | Together | llama3.1-8b | GPU cloud vs GPU cloud |
| **A7** | Local M3 | Cerebras | llama3.1-8b | Apple Silicon vs WSE |
| **A8** | Local M3 | Groq | llama3.1-8b | Apple Silicon vs LPU |

**Key Questions:**
- How much faster is WSE vs LPU?
- How much faster is specialized hardware vs NVIDIA GPUs?
- What's the local vs cloud speed gap?

---

### Group B: Large Model Scaling (Llama 3.1/3.3 70B)

These phases test if infrastructure advantages scale with model size.

| Phase | Provider 1 | Provider 2 | Model | Hardware Comparison |
|-------|------------|------------|-------|---------------------|
| **B1** | Cerebras | Groq | llama-3.3-70b | WSE vs LPU at scale |
| **B2** | Cerebras | SambaNova | llama3.1-70b | WSE vs RDU |
| **B3** | Groq | SambaNova | llama3.1-70b | LPU vs RDU |
| **B4** | Cerebras | Fireworks | llama3.1-70b | WSE vs GPU at scale |
| **B5** | Cerebras | Together | llama3.1-70b | WSE vs GPU at scale |
| **B6** | SambaNova | Fireworks | llama3.1-70b | RDU vs GPU |

**Key Questions:**
- Does WSE advantage grow or shrink at 70B?
- How does SambaNova RDU compare?
- Is the GPU gap larger at scale?

---

### Group C: GLM-4.7 Comparison (Z.ai Model)

These phases compare infrastructure using Z.ai's native model.

| Phase | Provider 1 | Provider 2 | Model | Hardware Comparison |
|-------|------------|------------|-------|---------------------|
| **C1** | Cerebras | Z.ai | glm-4.7 | WSE vs Z.ai native |
| **C2** | Z.ai | Fireworks | glm-4.7 | Native vs GPU cloud |

**Key Questions:**
- Does Cerebras maintain speed advantage on non-Llama models?
- How fast is Z.ai's native infrastructure?

---

### Group D: DeepSeek-V3 Comparison

These phases compare DeepSeek's native API vs third-party hosting.

| Phase | Provider 1 | Provider 2 | Model | Hardware Comparison |
|-------|------------|------------|-------|---------------------|
| **D1** | DeepSeek | Together | deepseek-v3 | Native vs GPU cloud |
| **D2** | DeepSeek | Fireworks | deepseek-v3 | Native vs GPU cloud |
| **D3** | Together | Fireworks | deepseek-v3 | GPU vs GPU (same model) |

**Key Questions:**
- Is DeepSeek's native API faster?
- What's the cost/speed tradeoff?

---

### Group E: Kimi K2.5 Comparison (Moonshot Model)

These phases compare Moonshot's native API vs aggregators.

| Phase | Provider 1 | Provider 2 | Model | Hardware Comparison |
|-------|------------|------------|-------|---------------------|
| **E1** | Moonshot | Together | kimi-k2.5 | Native vs GPU cloud |
| **E2** | Moonshot | OpenRouter | kimi-k2.5 | Native vs aggregator |
| **E3** | Together | OpenRouter | kimi-k2.5 | GPU vs aggregator |

**Key Questions:**
- Aggregator overhead (OpenRouter)?
- Native API advantage?

---

### Group F: Local Hardware Comparison

These phases compare local hardware options.

| Phase | Provider 1 | Provider 2 | Model | Hardware Comparison |
|-------|------------|------------|-------|---------------------|
| **F1** | Local M3 Pro | Local RTX 5060 | llama3.1-8b | Apple vs NVIDIA local |
| **F2** | Local RTX 5060 | Fireworks | llama3.1-8b | Local NVIDIA vs cloud NVIDIA |

**Key Questions:**
- Apple Silicon vs consumer NVIDIA?
- Local GPU vs cloud GPU cost/speed tradeoff?

---

## Recommended Execution Order

### Priority 1: Establish Baseline (Low Cost)

1. **A1**: Cerebras vs Groq (llama3.1-8b) - Compare specialized chips
2. **A2**: Cerebras vs Fireworks (llama3.1-8b) - WSE vs GPU baseline
3. **A7**: Local M3 vs Cerebras (llama3.1-8b) - Local vs cloud baseline

### Priority 2: Validate at Scale

4. **B1**: Cerebras vs Groq (llama-3.3-70b) - Scale comparison
5. **B2**: Cerebras vs SambaNova (llama3.1-70b) - Add RDU data point

### Priority 3: Model-Specific Comparisons

6. **C1**: Cerebras vs Z.ai (glm-4.7) - Non-Llama model
7. **D1**: DeepSeek vs Together (deepseek-v3) - Native vs hosted
8. **E1**: Moonshot vs Together (kimi-k2.5) - Native vs hosted

### Priority 4: Fill in the Matrix

9. **A4**, **A5**, **A6**: Complete GPU cloud comparisons
10. **B3**, **B4**, **B5**, **B6**: Complete 70B comparisons
11. **E2**, **E3**, **D2**, **D3**: Complete aggregator comparisons

### Priority 5: Local Hardware

12. **F1**, **F2**: Local hardware comparisons (if RTX 5060 available)

---

## Environment Variables Reference

```bash
# Specialized AI chips
export CEREBRAS_API_KEY="..."
export GROQ_API_KEY="..."
export SAMBANOVA_API_KEY="..."

# NVIDIA GPU clouds
export FIREWORKS_API_KEY="..."
export TOGETHER_API_KEY="..."

# Chinese AI providers
export DEEPSEEK_API_KEY="..."
export ZAI_API_KEY="..."
export MOONSHOT_API_KEY="..."

# Aggregators
export OPENROUTER_API_KEY="..."

# Model overrides (optional)
export TOGETHER_MODEL="meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo"
export ZAI_MODEL="glm-4.7"
export MOONSHOT_MODEL="kimi-k2.5"
export OPENROUTER_MODEL="meta-llama/llama-3.1-8b-instruct"

# Local
export OLLAMA_URL="http://localhost:11434"
```

---

## Expected Patterns to Discover

After running multiple 2-way comparisons, we expect to find:

1. **Chip Technology Rankings**: WSE > LPU > RDU > GPU (for throughput)
2. **Scale Effects**: Specialized hardware advantage grows with model size
3. **Native vs Hosted**: Native APIs may be faster for their own models
4. **Aggregator Overhead**: OpenRouter adds latency vs direct APIs
5. **Local vs Cloud**: Cloud is 10-50x faster, but local is free

---

## Cost Estimation

| Phase Group | Phases | Est. Cost per Phase | Total |
|-------------|--------|---------------------|-------|
| Group A (8B) | 8 | ~$0.05 | ~$0.40 |
| Group B (70B) | 6 | ~$0.15 | ~$0.90 |
| Group C (GLM) | 2 | ~$0.10 | ~$0.20 |
| Group D (DeepSeek) | 3 | ~$0.05 | ~$0.15 |
| Group E (Kimi) | 3 | ~$0.10 | ~$0.30 |
| Group F (Local) | 2 | $0.00 | $0.00 |
| **Total** | **24** | | **~$1.95** |

*Note: Costs are estimates. Use `--size short` and `--iterations 1` to minimize.*

---

## CLI Usage Examples

```bash
# Phase A1: Cerebras vs Groq (8B)
speed-kings benchmark --providers cerebras,groq -s short -i 1 --yes

# Phase B1: Cerebras vs Groq (70B) - need model override
CEREBRAS_MODEL=llama-3.3-70b GROQ_MODEL=llama-3.3-70b-versatile \
  speed-kings benchmark --providers cerebras,groq -s short -i 1 --yes

# Phase C1: Cerebras vs Z.ai (GLM-4.7)
CEREBRAS_MODEL=zai-glm-4.7 ZAI_MODEL=glm-4.7 \
  speed-kings benchmark --providers cerebras,zai -s short -i 1 --yes

# Output as JSON for analysis
speed-kings benchmark --providers cerebras,groq -o json > results/a1.json
```

---

## Success Criteria

Each phase is complete when:
- [ ] Both providers return valid results
- [ ] Metrics are comparable (same prompt, similar output)
- [ ] Results saved in `results/` directory
- [ ] Observations documented in `docs/findings.md`

Overall success:
- [ ] 10+ phases completed
- [ ] Clear pattern emerges for infrastructure rankings
- [ ] Data supports video series narrative
