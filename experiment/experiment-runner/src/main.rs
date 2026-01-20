//! Experiment Runner for d-ary Heap Code Generation Research
//!
//! Usage:
//!   cargo run -- --provider anthropic --condition baseline --language go
//!   cargo run -- --provider lmstudio --condition baseline --language rust --dry-run

mod anthropic;
mod lmstudio;
mod mistral;
mod provider;

use anyhow::{anyhow, Result};
use chrono::Utc;
use clap::Parser;
use provider::{LlmProvider, LlmResponse, RequestConfig};
use serde::Serialize;
use std::borrow::Cow;
use std::path::Path;
use std::time::Instant;

#[derive(Parser, Debug, Clone)]
#[command(name = "experiment-runner")]
#[command(about = "Run LLM experiments for d-ary heap code generation")]
struct Args {
    /// LLM provider: anthropic, mistral, lmstudio
    #[arg(short, long)]
    provider: String,

    /// Experimental condition: baseline, doc_guided, struct_guided, test_guided, combined
    #[arg(short, long)]
    condition: String,

    /// Target language: go, rust, cpp, typescript, zig
    #[arg(short, long)]
    language: String,

    /// Model override (uses provider default if not specified)
    #[arg(short, long)]
    model: Option<String>,

    /// Maximum tokens for response
    #[arg(long, default_value = "8192")]
    max_tokens: u32,

    /// Dry run - show prompt without calling API
    #[arg(long)]
    dry_run: bool,

    /// Base directory for experiment files (default: current directory's parent)
    #[arg(long)]
    base_dir: Option<String>,

    /// Run test-mimicking study across multiple Claude models
    #[arg(long)]
    test_mimicking_study: bool,
}

/// Models to test for the test-mimicking emergence study
/// Format: (model_id, max_tokens)
const TEST_MIMICKING_MODELS: &[(&str, u32)] = &[
    // Already tested:
    // ("claude-3-haiku-20240307", 4096),      // Haiku 3: 1,899 tokens, 0 tests
    // ("claude-haiku-4-5-20251001", 8192),    // Haiku 4.5: 6,788 tokens, 22 tests
    // ("claude-opus-4-5-20251101", 8192),     // Opus 4.5: 2,233 tokens, 0 tests
    // ("claude-sonnet-4-20250514", 8192),     // Sonnet 4: 6,370 tokens, 22 tests (original)

    // Remaining to test:
    ("claude-opus-4-20250514", 8192),       // Opus 4 (May 2025)
    ("claude-opus-4-1-20250805", 8192),     // Opus 4.1 (Aug 2025)
    ("claude-sonnet-4-5-20250929", 8192),   // Sonnet 4.5 latest (Sep 2025)
];

/// Check if an error indicates credit exhaustion
fn is_credit_error(err: &anyhow::Error) -> bool {
    err.to_string().starts_with("CREDIT_EXHAUSTED")
}

#[derive(Serialize)]
struct ExperimentResult {
    experiment_id: String,
    condition: String,
    language: String,
    model: String,
    provider: String,
    timestamp: String,
    input_tokens: usize,
    output_tokens: usize,
    elapsed_ms: u128,
}

fn get_provider(name: &str) -> Result<Box<dyn LlmProvider>> {
    match name.to_lowercase().as_str() {
        "anthropic" | "claude" => Ok(Box::new(anthropic::AnthropicProvider::new()?)),
        "mistral" => Ok(Box::new(mistral::MistralProvider::new()?)),
        "lmstudio" | "lm-studio" => Ok(Box::new(lmstudio::LmStudioProvider::new())),
        _ => Err(anyhow!(
            "Unknown provider: {}. Valid: anthropic, mistral, lmstudio",
            name
        )),
    }
}

fn get_file_extension(language: &str) -> &'static str {
    match language {
        "go" => "go",
        "rust" => "rs",
        "cpp" => "hpp",
        "typescript" => "ts",
        "zig" => "zig",
        _ => "txt",
    }
}

/// Sanitize model name for use in filenames (replace problematic chars)
fn sanitize_model_name(model: &str) -> Cow<'_, str> {
    if model.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.') {
        Cow::Borrowed(model)
    } else {
        Cow::Owned(
            model
                .chars()
                .map(|c| if c.is_alphanumeric() || c == '-' || c == '.' { c } else { '_' })
                .collect()
        )
    }
}

fn normalize_language(language: &str) -> Result<&'static str> {
    match language.to_lowercase().as_str() {
        "go" => Ok("go"),
        "rust" | "rs" => Ok("rust"),
        "cpp" | "c++" => Ok("cpp"),
        "typescript" | "ts" => Ok("typescript"),
        "zig" => Ok("zig"),
        _ => Err(anyhow!(
            "Unknown language: {}. Valid: go, rust, cpp, typescript, zig",
            language
        )),
    }
}

fn normalize_condition(condition: &str) -> Result<&'static str> {
    match condition.to_lowercase().as_str() {
        "baseline" | "c1" => Ok("baseline"),
        "doc_guided" | "docguided" | "c2" => Ok("doc_guided"),
        "struct_guided" | "structguided" | "c3" => Ok("struct_guided"),
        "test_guided" | "testguided" | "c4" => Ok("test_guided"),
        "combined" | "c5" => Ok("combined"),
        _ => Err(anyhow!(
            "Unknown condition: {}. Valid: baseline, doc_guided, struct_guided, test_guided, combined",
            condition
        )),
    }
}

/// Load test code for a given language from the test-corpus directory
fn load_test_code(base_dir: &Path, language: &str) -> Result<String> {
    // base_dir is the experiment/ directory, test-corpus is a sibling
    // So we need base_dir/../test-corpus
    let test_corpus_dir = base_dir.join("..").join("test-corpus");

    let test_files: Vec<&str> = match language {
        "go" => vec![
            "insert_test.go",
            "pop_test.go",
            "front_test.go",
            "increase_priority_test.go",
            "decrease_priority_test.go",
        ],
        "rust" => vec![
            "src/tests/mod.rs",
            "src/tests/insert.rs",
            "src/tests/pop.rs",
            "src/tests/front.rs",
            "src/tests/increase_priority.rs",
            "src/tests/decrease_priority.rs",
        ],
        "cpp" => vec![
            "insert_test.cpp",
            "pop_test.cpp",
            "front_test.cpp",
            "increase_priority_test.cpp",
            "decrease_priority_test.cpp",
        ],
        "typescript" => vec![
            "insert.test.ts",
            "pop.test.ts",
            "front.test.ts",
            "increase_priority.test.ts",
            "decrease_priority.test.ts",
        ],
        "zig" => vec![
            "src/corpus_tests.zig",
        ],
        _ => return Err(anyhow!("Unknown language for test loading: {}", language)),
    };

    let lang_dir = test_corpus_dir.join(language);
    let mut combined = String::new();

    for file in test_files {
        let file_path = lang_dir.join(file);
        if file_path.exists() {
            let content = std::fs::read_to_string(&file_path).map_err(|e| {
                anyhow!("Failed to read test file {}: {}", file_path.display(), e)
            })?;
            if !combined.is_empty() {
                combined.push_str("\n\n// --- ");
                combined.push_str(file);
                combined.push_str(" ---\n\n");
            }
            combined.push_str(&content);
        }
    }

    if combined.is_empty() {
        return Err(anyhow!(
            "No test files found for {} in {}",
            language,
            lang_dir.display()
        ));
    }

    Ok(combined)
}

fn load_prompt(base_dir: &Path, condition: &str, language: &str) -> Result<String> {
    // The prompt files are in prompts/{condition}.md
    // We need to extract the language-specific section
    let prompt_file = base_dir.join("prompts").join(format!("{}.md", condition));

    let content = std::fs::read_to_string(&prompt_file).map_err(|e| {
        anyhow!(
            "Failed to read prompt file {}: {}",
            prompt_file.display(),
            e
        )
    })?;

    let cap_lang = capitalize(language);

    // For test_guided and combined, use the generic "### Prompt Text" template
    // and inject test code via {TEST_CODE} placeholder
    if condition == "test_guided" || condition == "combined" {
        if let Some(start) = content.find("### Prompt Text") {
            let section = &content[start..];
            if let Some(code_start) = section.find("```") {
                let after_backticks = &section[code_start + 3..];
                if let Some(newline_pos) = after_backticks.find('\n') {
                    let code_content = &after_backticks[newline_pos + 1..];
                    if let Some(code_end) = code_content.find("```") {
                        let mut template = code_content[..code_end].trim_end().to_string();

                        // Replace {LANGUAGE} placeholder
                        template = template.replace("{LANGUAGE}", &cap_lang);

                        // Inject test code
                        if template.contains("{TEST_CODE}") {
                            let test_code = load_test_code(base_dir, language)?;
                            template = template.replace("{TEST_CODE}", &test_code);
                        }

                        return Ok(template);
                    }
                }
            }
        }
        return Err(anyhow!(
            "Could not find ### Prompt Text section for {} in {}",
            condition,
            prompt_file.display()
        ));
    }

    // For baseline, doc_guided, struct_guided: extract the language-specific prompt section
    // The format can be "### {Language}" or "## {Language}" followed by ``` code block
    let headers = [format!("### {}", cap_lang), format!("## {}", cap_lang)];

    for language_header in &headers {
        if let Some(start) = content.find(language_header.as_str()) {
            let section = &content[start..];
            // Find the code block - handle both ```\n and ```\r\n
            if let Some(code_start) = section.find("```") {
                let after_backticks = &section[code_start + 3..];
                // Skip to the newline after ```
                if let Some(newline_pos) = after_backticks.find('\n') {
                    let code_content = &after_backticks[newline_pos + 1..];
                    // Find closing ``` (could be on its own line)
                    if let Some(code_end) = code_content.find("```") {
                        let extracted = code_content[..code_end].trim_end();
                        return Ok(extracted.to_string());
                    }
                }
            }
        }
    }

    Err(anyhow!(
        "Could not extract prompt for {} / {} from {}",
        condition,
        language,
        prompt_file.display()
    ))
}

fn capitalize(s: &str) -> String {
    // Handle special cases for language names in prompt files
    match s {
        "cpp" => "C++".to_string(),
        "typescript" => "TypeScript".to_string(),
        _ => {
            let mut chars = s.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            }
        }
    }
}

fn extract_code<'a>(response: &'a str, language: &str) -> Cow<'a, str> {
    // Try to find code block with language tag - use static arrays to avoid heap allocation
    let lang_tags: &[&str] = match language {
        "go" => &["```go", "```golang"],
        "rust" => &["```rust", "```rs"],
        "cpp" => &["```cpp", "```c++", "```hpp"],
        "typescript" => &["```typescript", "```ts"],
        "zig" => &["```zig"],
        _ => &["```"],
    };

    for tag in lang_tags {
        if let Some(start) = response.find(tag) {
            let code_start = start + tag.len();
            let code_content = &response[code_start..];
            let code_content = code_content.trim_start_matches('\n');
            if let Some(end) = code_content.find("\n```") {
                return Cow::Borrowed(&code_content[..end]);
            }
        }
    }

    // Fallback: try generic code block
    if let Some(start) = response.find("```\n") {
        let code_content = &response[start + 4..];
        if let Some(end) = code_content.find("\n```") {
            return Cow::Borrowed(&code_content[..end]);
        }
    }

    // Last resort: return full response (borrowed, no allocation)
    Cow::Borrowed(response)
}

async fn run_experiment(args: Args) -> Result<()> {
    let condition = normalize_condition(&args.condition)?;
    let language = normalize_language(&args.language)?;

    // Determine base directory
    let base_dir = args
        .base_dir
        .as_deref()
        .map(Path::new)
        .unwrap_or_else(|| Path::new("."));

    println!("=== Experiment: {}_{} ===", condition, language);
    println!("Provider: {}", args.provider);
    println!("Base dir: {}", base_dir.display());

    // Load prompt
    let prompt = load_prompt(base_dir, condition, language)?;
    println!("Prompt loaded ({} chars)", prompt.len());

    if args.dry_run {
        println!("\n--- DRY RUN: Prompt ---\n");
        println!("{}", prompt);
        println!("\n--- End of prompt ---");
        return Ok(());
    }

    // Create provider and send request
    let provider = get_provider(&args.provider)?;
    let model_name = args.model.as_deref().unwrap_or(provider.default_model());
    println!("Using model: {}", model_name);

    let config = RequestConfig {
        model: args.model,
        max_tokens: args.max_tokens,
        temperature: 0.0,
    };

    println!("Sending request...");
    let start_time = Instant::now();
    let response: LlmResponse = provider.complete(&prompt, &config).await?;
    let elapsed = start_time.elapsed();

    println!(
        "Response received: {} chars, {} input tokens, {} output tokens, {:.2}s",
        response.content.len(),
        response.input_tokens,
        response.output_tokens,
        elapsed.as_secs_f64()
    );

    // Create results directory if needed
    let results_dir = base_dir.join("results");
    std::fs::create_dir_all(&results_dir)?;

    // Build file prefix with model name: {condition}_{language}_{model}
    let safe_model = sanitize_model_name(&response.model);
    let file_prefix = format!("{}_{}_{}", condition, language, safe_model);
    let timestamp = Utc::now();

    // Extract code (zero-copy when possible)
    let code = extract_code(&response.content, language);
    let ext = get_file_extension(language);

    // Build metadata
    let result = ExperimentResult {
        experiment_id: format!("{}_{}", condition, language),
        condition: condition.to_string(),
        language: language.to_string(),
        model: response.model.clone(),
        provider: response.provider,
        timestamp: timestamp.to_rfc3339(),
        input_tokens: response.input_tokens,
        output_tokens: response.output_tokens,
        elapsed_ms: elapsed.as_millis(),
    };
    let meta_json = serde_json::to_string_pretty(&result)?;

    // File paths
    let prompt_file = results_dir.join(format!("{}_prompt.md", file_prefix));
    let response_file = results_dir.join(format!("{}_response.md", file_prefix));
    let code_file = results_dir.join(format!("{}_code.{}", file_prefix, ext));
    let meta_file = results_dir.join(format!("{}_meta.json", file_prefix));

    // Write all files in parallel using tokio spawn_blocking
    let (r1, r2, r3, r4) = tokio::join!(
        tokio::task::spawn_blocking({
            let path = prompt_file.clone();
            let data = prompt.clone();
            move || std::fs::write(&path, data)
        }),
        tokio::task::spawn_blocking({
            let path = response_file.clone();
            let data = response.content.clone();
            move || std::fs::write(&path, data)
        }),
        tokio::task::spawn_blocking({
            let path = code_file.clone();
            let data = code.into_owned();
            move || std::fs::write(&path, data)
        }),
        tokio::task::spawn_blocking({
            let path = meta_file.clone();
            move || std::fs::write(&path, meta_json)
        }),
    );

    // Check results
    r1??;
    r2??;
    r3??;
    r4??;

    println!("Saved: {}", prompt_file.display());
    println!("Saved: {}", response_file.display());
    println!("Saved: {}", code_file.display());
    println!("Saved: {}", meta_file.display());

    println!("\n=== Experiment complete ===");

    Ok(())
}

/// Run the test-mimicking emergence study
async fn run_test_mimicking_study(base_args: Args) -> Result<()> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║       TEST-MIMICKING EMERGENCE STUDY                         ║");
    println!("║  Testing: When did Claude start mimicking test patterns?     ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("Models to test: {:?}", TEST_MIMICKING_MODELS);
    println!("Condition: test_guided");
    println!("Language: rust (highest test count difference observed)");
    println!();

    let base_dir = base_args
        .base_dir
        .as_deref()
        .map(Path::new)
        .unwrap_or_else(|| Path::new("."));

    let mut completed = 0;
    let mut results_summary: Vec<(String, usize)> = Vec::new();

    for (model, max_tokens) in TEST_MIMICKING_MODELS {
        println!("────────────────────────────────────────────────────────────────");
        println!("Testing model: {} (max_tokens: {})", model, max_tokens);
        println!("────────────────────────────────────────────────────────────────");

        let args = Args {
            provider: "anthropic".to_string(),
            condition: "test_guided".to_string(),
            language: "rust".to_string(),
            model: Some(model.to_string()),
            max_tokens: *max_tokens,
            dry_run: base_args.dry_run,
            base_dir: base_args.base_dir.clone(),
            test_mimicking_study: false,
        };

        match run_experiment(args).await {
            Ok(()) => {
                completed += 1;
                // Try to read the output tokens from the meta file
                let safe_model = sanitize_model_name(model);
                let meta_path = base_dir.join("results").join(
                    format!("test_guided_rust_{}_meta.json", safe_model)
                );
                if let Ok(content) = std::fs::read_to_string(&meta_path) {
                    if let Ok(meta) = serde_json::from_str::<serde_json::Value>(&content) {
                        if let Some(tokens) = meta.get("output_tokens").and_then(|v| v.as_u64()) {
                            results_summary.push((model.to_string(), tokens as usize));
                        }
                    }
                }
                println!("✓ {} completed successfully\n", model);
            }
            Err(e) => {
                if is_credit_error(&e) {
                    println!();
                    println!("╔══════════════════════════════════════════════════════════════╗");
                    println!("║  ⚠️  CREDIT EXHAUSTED - STOPPING GRACEFULLY                  ║");
                    println!("╚══════════════════════════════════════════════════════════════╝");
                    println!();
                    println!("Completed {}/{} models before running out of credits.",
                             completed, TEST_MIMICKING_MODELS.len());
                    break;
                } else {
                    println!("✗ {} failed: {}\n", model, e);
                    // Continue with next model on non-credit errors
                }
            }
        }
    }

    // Print summary
    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    STUDY RESULTS SUMMARY                     ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("Completed: {}/{} models", completed, TEST_MIMICKING_MODELS.len());
    println!();

    if !results_summary.is_empty() {
        println!("Output tokens by model (test_guided rust):");
        println!("┌─────────────────────────────────────┬──────────────┐");
        println!("│ Model                               │ Output Tokens│");
        println!("├─────────────────────────────────────┼──────────────┤");
        for (model, tokens) in &results_summary {
            println!("│ {:35} │ {:>12} │", model, tokens);
        }
        println!("└─────────────────────────────────────┴──────────────┘");
        println!();

        // Reference: Claude Sonnet 4 produced 6,370 tokens with 22 tests
        println!("Reference: claude-sonnet-4-20250514 produced 6,370 tokens (22 tests)");
        println!("Reference: mistral-medium-latest produced 1,950 tokens (0 tests)");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.test_mimicking_study {
        run_test_mimicking_study(args).await
    } else {
        run_experiment(args).await
    }
}
