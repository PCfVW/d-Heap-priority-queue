# AI Code Generation Experiment: Three-Level Hierarchy Hypothesis

**Research Question:** What forms of structured assistance most improve AI-generated code correctness?

The **Three-Level Hierarchy** proposes that different guidance types constrain LLM output with varying effectiveness: (1) type signatures, (2) documentation, and (3) tests. See [the full findings](results/three_level_hypothesis_findings.md) for complete analysis.

## Key Finding: 100% Test Preservation (Not Amplification)

When Claude Sonnet is given inline tests (Rust, Zig, Python doctests), it **preserves them with 100% fidelity**:

| Language | Tests in Prompt | Tests in Output | Behavior |
|----------|-----------------|-----------------|----------|
| Rust     | 22              | 22              | 100% preservation |
| Zig (inline) | 22          | 22              | 100% preservation |
| Python doctests | 50        | 50              | 100% preservation |

This is **preservation**, not amplification. The model faithfully reproduces the exact tests provided.

## Summary of Findings

1. **Type signatures work** (-23% tokens vs baseline, +23 points API conformance)
2. **Documentation doesn't help** (+1.6% vs baseline — essentially no effect)
3. **"Kitchen sink" prompts hurt** (+5.4% vs baseline — more verbose, not more constrained)
4. **Model tier matters**: Opus suppresses tests, Sonnet/Haiku 4.5 preserves them
5. **Prompt structure signals output**: `@import` triggers suppression; inline presentation triggers preservation

## Directory Structure

```
experiment/
├── README.md                    # This file
├── experimental_protocol.md     # Full experimental design
├── results_template.md          # Template for recording results
├── prompts/                     # Prompt templates for each condition
├── results/                     # All experimental outputs
│   ├── three_level_hypothesis_findings.md  # Main research findings
│   ├── *_code.*                 # Generated implementations
│   ├── *_response.md            # Raw model responses
│   ├── *_prompt.md              # Actual prompts sent
│   ├── *_meta.json              # Token counts, timing
│   └── api_conformance_*.md     # API conformance analysis
└── experiment-runner/           # Rust automation tool
    └── src/bin/                 # Experiment runner binaries
```

## Reproducing the Experiments

### Prerequisites

- Rust toolchain (for experiment runner)
- Anthropic API key (set `ANTHROPIC_API_KEY` environment variable)
- Optional: Mistral API key for comparison

### Running an Experiment

```bash
cd experiment/experiment-runner

# Run baseline condition for all languages with Claude Sonnet
cargo run --bin baseline

# Run test-guided condition
cargo run --bin test_guided

# Run Rust-specific signal strength tests
cargo run --bin rust_no_module
cargo run --bin rust_mod_only

# Run Python doctest experiment
cargo run --bin python_doctest
```

### Output Files

Each experiment produces:
- `{condition}_{language}_{model}_code.{ext}` — Extracted implementation
- `{condition}_{language}_{model}_response.md` — Full model response
- `{condition}_{language}_{model}_prompt.md` — Prompt sent to model
- `{condition}_{language}_{model}_meta.json` — Metadata (tokens, timing)

## The Three-Level Hierarchy

| Level | Guidance Type | Effectiveness |
|-------|---------------|---------------|
| **Level 1** | Type Signatures | **Most effective** (-23% tokens) |
| **Level 2** | Documentation | Ineffective (+1.6% tokens) |
| **Level 3** | Tests | Context-dependent* |

*Tests work as **preservation scaffolding** for inline-test languages (Rust, Zig, Python), but cause elaboration for external-test languages (Go, C++, TypeScript).

## Key Experiments

### 1. Condition Comparison (5 languages × 5 conditions × 2 models)

Tested: Baseline, Doc-guided, Struct-guided, Test-guided, Combined

**Winner:** Struct-guided (type signatures only)

### 2. Test-Mimicking Study (7 Claude models)

Tested: Haiku 3, Sonnet 4, Opus 4, Opus 4.1, Sonnet 4.5, Haiku 4.5, Opus 4.5

**Finding:** Opus models suppress tests; Sonnet/Haiku 4.5 preserve them with 100% fidelity

### 3. Rust Signal Strength (2026-01-20)

Tested whether `#[cfg(test)] mod tests {}` is required for preservation

**Finding:** No — plain `#[test]` achieves 100% preservation (20→20)

### 4. Zig Import Pattern (2026-01-20)

Tested whether `@import("module")` triggers suppression

**Finding:** Yes — inline tests achieve 100% preservation; `@import` causes 0 tests

### 5. Python Doctest (2026-01-20)

Tested whether doctests are treated as documentation or tests

**Finding:** 100% preservation (50→50) — doctests treated as inline tests

## Citation

If you use these findings:

```
Three-Level Hierarchy Hypothesis: AI Code Generation Study
Priority Queues Research Project, 2026
https://github.com/PCfVW/d-Heap-priority-queue
```

## Full Findings

See [results/three_level_hypothesis_findings.md](results/three_level_hypothesis_findings.md) for complete analysis including:

- Detailed token count tables
- API conformance analysis
- Model-specific behavior patterns
- Theoretical grounding (Attention mechanism implications)
- Product opportunities for AI coding tools
