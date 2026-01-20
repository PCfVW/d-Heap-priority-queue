# Experiment Runner

A Rust CLI tool for running reproducible LLM code generation experiments across multiple providers and models.

## Why This Runner?

Running LLM experiments manually is tedious and error-prone:
- Copy-pasting prompts loses formatting
- Forgetting to record token counts
- Inconsistent temperature settings between runs
- No systematic way to compare models

This runner automates the entire workflow: **prompt assembly → API call → response parsing → file output**.

## What It Does

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐     ┌──────────────┐
│   Prompt    │ ──► │  LLM API     │ ──► │   Parse     │ ──► │   Output     │
│  Templates  │     │  (Anthropic, │     │  Response   │     │  - code.ext  │
│  + Corpus   │     │   Mistral,   │     │  + Extract  │     │  - meta.json │
│             │     │   LMStudio)  │     │    Code     │     │  - prompt.md │
└─────────────┘     └──────────────┘     └─────────────┘     └──────────────┘
```

For each experiment, it produces:
- `{condition}_{language}_{model}_code.{ext}` — Extracted implementation
- `{condition}_{language}_{model}_meta.json` — Token counts, timing, model info
- `{condition}_{language}_{model}_prompt.md` — Exact prompt sent (for reproducibility)

## Quick Start

```bash
# Set your API key
export ANTHROPIC_API_KEY="sk-..."

# Run a single experiment
cargo run -- --provider anthropic --condition baseline --language rust

# Dry run (see prompt without calling API)
cargo run -- --provider anthropic --condition test_guided --language zig --dry-run

# Use a specific model
cargo run -- --provider anthropic --condition baseline --language go --model claude-opus-4-5-20251101
```

### Available Options

| Flag | Values | Description |
|------|--------|-------------|
| `--provider` | `anthropic`, `mistral`, `lmstudio` | LLM provider |
| `--condition` | `baseline`, `doc_guided`, `struct_guided`, `test_guided`, `combined` | Experiment condition |
| `--language` | `go`, `rust`, `cpp`, `typescript`, `zig` | Target language |
| `--model` | (provider-specific) | Override default model |
| `--dry-run` | | Show prompt without API call |

## Specialized Experiment Binaries

Beyond the main runner, we created specialized binaries for specific hypotheses:

| Binary | Purpose |
|--------|---------|
| `rust_no_module` | Test if `#[test]` alone (no module wrapper) triggers preservation |
| `rust_mod_only` | Test if `mod tests {}` without `#[cfg(test)]` triggers preservation |
| `inline_test` | Test Zig inline vs `@import` presentation |
| `python_doctest` | Test Python doctest preservation |

```bash
cargo run --bin rust_no_module
cargo run --bin python_doctest
```

## Architecture: What You Can Learn

### 1. Provider Trait Pattern

The `LlmProvider` trait abstracts over different APIs:

```rust
#[async_trait]
pub trait LlmProvider: Send + Sync {
    fn default_model(&self) -> &str;
    async fn complete(&self, prompt: &str, config: &RequestConfig) -> Result<LlmResponse>;
}
```

Each provider (Anthropic, Mistral, LMStudio) implements this trait. Adding a new provider means implementing ~50 lines of HTTP/JSON handling.

### 2. Deterministic Settings

For reproducibility, we use `temperature: 0.0`:

```rust
impl Default for RequestConfig {
    fn default() -> Self {
        Self {
            temperature: 0.0, // Deterministic for reproducibility
            // ...
        }
    }
}
```

### 3. Code Extraction

LLMs return markdown with code blocks. We extract the actual code:

```rust
fn extract_code(response: &str, language: &str) -> String {
    // Find ```language ... ``` blocks
    // Handle multiple blocks (implementation + tests)
    // Strip markdown formatting
}
```

### 4. Structured Output

Every run produces a JSON metadata file for analysis:

```json
{
  "experiment_id": "test_guided_rust_claude-sonnet-4-20250514_20260120_103045",
  "condition": "test_guided",
  "language": "rust",
  "model": "claude-sonnet-4-20250514",
  "input_tokens": 2847,
  "output_tokens": 6370,
  "elapsed_ms": 45230
}
```

## Dependencies

```toml
tokio = { version = "1", features = ["full"] }  # Async runtime
reqwest = { version = "0.12", features = ["json"] }  # HTTP client
clap = { version = "4", features = ["derive"] }  # CLI parsing
serde = { version = "1", features = ["derive"] }  # JSON serialization
anyhow = "1"  # Error handling
```

Minimal dependencies, focused on the task.

## Extending

To add a new provider:

1. Create `src/newprovider.rs` implementing `LlmProvider`
2. Add to `mod` declarations in `main.rs`
3. Add match arm in `get_provider()`

To add a new experiment condition:

1. Add prompt template to `../prompts/`
2. Add condition handling in `build_prompt()`
