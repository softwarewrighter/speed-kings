# Project Status

> Current development status for Speed Kings.

## Overview

| Aspect | Status |
|--------|--------|
| **Project** | Speed Kings (inference-showdown) |
| **Version** | 0.1.0-dev |
| **Phase** | Phase 1: Core Foundation |
| **Last Updated** | 2026-01-29 |

## Current State

The project is in its initial setup phase. Basic Rust project structure exists with a placeholder main.rs.

### What Exists

- [x] Git repository initialized
- [x] Basic Cargo.toml with project metadata
- [x] Placeholder main.rs
- [x] Documentation structure (docs/)
- [x] AI agent instructions (docs/ai_agent_instructions.md)
- [x] Process documentation (docs/process.md)
- [x] Tools documentation (docs/tools.md)
- [x] PRD (docs/prd.md)
- [x] Architecture doc (docs/architecture.md)
- [x] Design doc (docs/design.md)
- [x] Implementation plan (docs/plan.md)
- [x] LICENSE and COPYRIGHT files

### What's Missing

- [ ] Core dependencies in Cargo.toml
- [ ] CLI argument parsing (clap)
- [ ] Provider trait and implementations
- [ ] Benchmark engine
- [ ] Output formatters
- [ ] Tests
- [ ] CI/CD configuration
- [ ] README.md

## Phase Progress

### Phase 1: Core Foundation

| Task | Status | Notes |
|------|--------|-------|
| Project Setup | In Progress | Basic structure exists |
| CLI Framework | Not Started | Need clap setup |
| Provider Trait | Not Started | Core abstraction |
| Cerebras Provider | Not Started | First cloud provider |
| Local Provider | Not Started | Ollama integration |
| Benchmark Engine | Not Started | Core logic |
| Terminal Output | Not Started | Table formatting |

### Phase 2: Full Provider Support

| Task | Status | Notes |
|------|--------|-------|
| Groq Provider | Not Started | - |
| SambaNova Provider | Not Started | - |
| Fireworks Provider | Not Started | - |
| DeepSeek Provider | Not Started | - |
| OpenAI-Compatible | Not Started | - |
| Pricing System | Not Started | - |

### Phase 3: Advanced Features

| Task | Status | Notes |
|------|--------|-------|
| Test Prompts | Not Started | - |
| Metrics Enhancement | Not Started | - |
| Retry Logic | Not Started | - |
| Progress Indication | Not Started | - |

### Phase 4: Output and Polish

| Task | Status | Notes |
|------|--------|-------|
| JSON Output | Not Started | - |
| Markdown Output | Not Started | - |
| CSV Output | Not Started | - |
| README | Not Started | - |
| Release | Not Started | - |

## Blockers

None currently. Ready to begin Phase 1 implementation.

## Key Decisions Made

1. **No concurrent benchmarking** - Sequential execution only
2. **Budget target** - $0.10-0.25 per benchmark run max
3. **Local model loading** - Track separately, document one-time overhead
4. **Rate limiting** - Document provider upgrade options, research optimal timing
5. **Metrics** - Track time-to-prompt and time-to-first-token as distinct measurements

## Next Steps

1. Add dependencies to Cargo.toml
2. Implement CLI argument parsing with clap
3. Define Provider trait and core types
4. Implement first provider (Cerebras or Local)
5. Create basic benchmark runner

## Milestones

| Milestone | Target | Status |
|-----------|--------|--------|
| First benchmark run | TBD | Not Started |
| All providers working | TBD | Not Started |
| v0.1.0 release | TBD | Not Started |

## Recent Changes

### 2026-01-29

- Initial project setup
- Created documentation structure
- Added PRD, architecture, design, plan, and status docs
- Resolved all open questions:
  - No concurrent benchmarking
  - Budget-conscious defaults ($0.10-0.25/run)
  - Local model loading tracked separately with documentation
  - Rate limiting handled via provider upgrade documentation

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
