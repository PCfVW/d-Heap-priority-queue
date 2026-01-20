//! Experiment runner for testing Rust amplification signal strength.
//!
//! Tests whether `mod tests { }` alone (without #[cfg(test)]) triggers amplification.
//!
//! Hypothesis: The `mod tests { use super::*; }` pattern is the amplification trigger.
//!
//! Usage: cargo run --bin rust_mod_only

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::Instant;

const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<ContentBlock>,
    usage: Usage,
}

#[derive(Deserialize)]
struct ContentBlock {
    text: Option<String>,
}

#[derive(Deserialize)]
struct Usage {
    input_tokens: usize,
    output_tokens: usize,
}

// Tests presented with `mod tests { }` wrapper but WITHOUT #[cfg(test)]
const RUST_MOD_ONLY_PROMPT: &str = r#"Implement a d-ary heap priority queue in Rust.

Requirements:
1. The heap arity (d) should be configurable at construction time
2. Items have two distinct properties: an identity (for equality) and a priority (for ordering)
3. Two items are equal if they have the same identity, regardless of priority
4. The queue should support O(1) lookup to check if an item exists (use a HashMap for position tracking)
5. Implement a min-heap where lower priority values have higher importance

Required operations:
- insert(item): Add an item to the queue
- pop(): Remove and return the item with highest priority (lowest value)
- front(): Return a reference to the item with highest priority without removing it
- increase_priority(item): Update an existing item to have higher priority (lower value)
- decrease_priority(item): Update an existing item to have lower priority (higher value)
- contains(item): Check if an item with the given identity exists
- len(): Return the number of items in the queue
- is_empty(): Return whether the queue is empty

Your implementation must pass all of the following tests. Note: the tests use a
`mod tests { use super::*; ... }` wrapper but WITHOUT the #[cfg(test)] attribute.

```rust
use std::collections::HashMap;
use std::hash::Hash;

// Item type with separate identity and priority
#[derive(Debug, Clone)]
pub struct Item {
    pub number: u32,
    pub cost: u32,
}

impl Item {
    pub fn new(number: u32, cost: u32) -> Self {
        Self { number, cost }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl Eq for Item {}

impl Hash for Item {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.number.hash(state);
    }
}

// Your DHeap implementation goes here

// Tests in mod wrapper (but NO #[cfg(test)])
mod tests {
    use super::*;

    // =============================================================================
    // insert() Tests
    // =============================================================================

    #[test]
    fn insert_postcondition_item_findable() {
        let mut pq = DHeap::new(4);
        let item = Item::new(50, 50);
        pq.insert(item.clone());
        assert!(pq.contains(&Item::new(50, 999)));
    }

    #[test]
    fn insert_invariant_heap_property() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(30, 30));
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(50, 50));
        pq.insert(Item::new(20, 20));
        pq.insert(Item::new(40, 40));
        assert_eq!(pq.front().unwrap().cost, 10);
    }

    #[test]
    fn insert_size_increments() {
        let mut pq = DHeap::new(4);
        for i in 0..5 {
            let size_before = pq.len();
            pq.insert(Item::new(i, i * 10));
            assert_eq!(pq.len(), size_before + 1);
        }
    }

    #[test]
    fn insert_edge_becomes_front_if_highest_priority() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(100, 100));
        pq.insert(Item::new(50, 50));
        pq.insert(Item::new(10, 10));
        assert_eq!(pq.front().unwrap().cost, 10);
        pq.insert(Item::new(1, 1));
        assert_eq!(pq.front().unwrap().cost, 1);
    }

    // =============================================================================
    // pop() Tests
    // =============================================================================

    #[test]
    fn pop_postcondition_returns_minimum() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(30, 30));
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(20, 20));
        let popped = pq.pop().unwrap();
        assert_eq!(popped.cost, 10);
        assert!(!pq.contains(&Item::new(10, 0)));
    }

    #[test]
    fn pop_invariant_maintains_heap_property() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(50, 50));
        pq.insert(Item::new(20, 20));
        pq.insert(Item::new(80, 80));
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(60, 60));
        pq.insert(Item::new(30, 30));
        pq.insert(Item::new(70, 70));
        pq.insert(Item::new(40, 40));

        let expected = [10, 20, 30, 40];
        for exp in expected {
            assert_eq!(pq.front().unwrap().cost, exp);
            pq.pop();
        }
    }

    #[test]
    fn pop_size_decrements() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(20, 20));
        pq.insert(Item::new(30, 30));
        for _ in 0..3 {
            let size_before = pq.len();
            pq.pop();
            assert_eq!(pq.len(), size_before - 1);
        }
    }

    #[test]
    fn pop_edge_empty_returns_none() {
        let mut pq: DHeap = DHeap::new(4);
        assert!(pq.pop().is_none());
    }

    // =============================================================================
    // front() Tests
    // =============================================================================

    #[test]
    fn front_postcondition_returns_minimum() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(30, 30));
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(20, 20));
        assert_eq!(pq.front().unwrap().cost, 10);
    }

    #[test]
    fn front_invariant_no_modification() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(30, 30));
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(20, 20));
        let first = pq.front().unwrap().cost;
        let second = pq.front().unwrap().cost;
        let third = pq.front().unwrap().cost;
        assert_eq!(first, second);
        assert_eq!(second, third);
    }

    #[test]
    fn front_size_unchanged() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(20, 20));
        pq.insert(Item::new(30, 30));
        let size_before = pq.len();
        for _ in 0..5 {
            let _ = pq.front();
        }
        assert_eq!(pq.len(), size_before);
    }

    #[test]
    fn front_edge_empty_returns_none() {
        let pq: DHeap = DHeap::new(4);
        assert!(pq.front().is_none());
    }

    // =============================================================================
    // increase_priority() Tests
    // =============================================================================

    #[test]
    fn increase_priority_postcondition_priority_changed() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(50, 50));
        pq.insert(Item::new(30, 30));
        assert_eq!(pq.front().unwrap().cost, 30);
        pq.increase_priority(Item::new(50, 10));
        assert_eq!(pq.front().unwrap().cost, 10);
    }

    #[test]
    fn increase_priority_invariant_heap_property() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(80, 80));
        pq.insert(Item::new(60, 60));
        pq.insert(Item::new(40, 40));
        pq.insert(Item::new(20, 20));
        pq.insert(Item::new(100, 100));
        pq.insert(Item::new(50, 50));
        assert_eq!(pq.front().unwrap().cost, 20);
        pq.increase_priority(Item::new(80, 5));
        assert_eq!(pq.front().unwrap().cost, 5);
    }

    #[test]
    fn increase_priority_position_item_moves_up() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(50, 50));
        pq.insert(Item::new(100, 100));
        assert_ne!(pq.front().unwrap().number, 100);
        pq.increase_priority(Item::new(100, 1));
        assert_eq!(pq.front().unwrap().number, 100);
    }

    #[test]
    fn increase_priority_size_unchanged() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(50, 50));
        pq.insert(Item::new(30, 30));
        pq.insert(Item::new(70, 70));
        let size_before = pq.len();
        pq.increase_priority(Item::new(70, 10));
        assert_eq!(pq.len(), size_before);
    }

    // =============================================================================
    // decrease_priority() Tests
    // =============================================================================

    #[test]
    fn decrease_priority_postcondition_priority_changed() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(30, 30));
        assert_eq!(pq.front().unwrap().number, 10);
        pq.decrease_priority(Item::new(10, 50));
        assert_eq!(pq.front().unwrap().number, 30);
    }

    #[test]
    fn decrease_priority_invariant_heap_property() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(30, 30));
        pq.insert(Item::new(50, 50));
        pq.insert(Item::new(70, 70));
        pq.insert(Item::new(20, 20));
        pq.insert(Item::new(40, 40));
        assert_eq!(pq.front().unwrap().number, 10);
        pq.decrease_priority(Item::new(10, 100));
        assert_eq!(pq.front().unwrap().cost, 20);
    }

    #[test]
    fn decrease_priority_position_item_moves_down() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(50, 50));
        pq.insert(Item::new(60, 60));
        pq.insert(Item::new(70, 70));
        assert_eq!(pq.front().unwrap().number, 10);
        pq.decrease_priority(Item::new(10, 100));
        assert_eq!(pq.front().unwrap().number, 50);
    }

    #[test]
    fn decrease_priority_size_unchanged() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(30, 30));
        pq.insert(Item::new(50, 50));
        let size_before = pq.len();
        pq.decrease_priority(Item::new(10, 100));
        assert_eq!(pq.len(), size_before);
    }
}
```

Provide a complete, working implementation. Include all the tests in your output file.
Keep the `mod tests { use super::*; ... }` structure (but no #[cfg(test)])."#;

fn main() -> Result<()> {
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .map_err(|_| anyhow::anyhow!("ANTHROPIC_API_KEY environment variable not set"))?;

    let model = "claude-sonnet-4-20250514";

    println!("=== Rust Signal Strength Experiment (Mod Only, No cfg) ===");
    println!("Model: {}", model);
    println!("Testing: Does mod tests {{ }} alone (without #[cfg(test)]) trigger amplification?");
    println!();

    // Count tests in prompt
    let prompt_test_count = RUST_MOD_ONLY_PROMPT
        .lines()
        .filter(|line| line.trim() == "#[test]")
        .count();
    println!("Tests in prompt: {}", prompt_test_count);

    let request = AnthropicRequest {
        model: model.to_string(),
        max_tokens: 8192,
        messages: vec![Message {
            role: "user".to_string(),
            content: RUST_MOD_ONLY_PROMPT.to_string(),
        }],
    };

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()?;
    let start = Instant::now();

    println!("Sending request...");
    let response = client
        .post(ANTHROPIC_API_URL)
        .header("x-api-key", &api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request)
        .send()?;

    let elapsed = start.elapsed();

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text()?;
        anyhow::bail!("API error {}: {}", status, text);
    }

    let api_response: AnthropicResponse = response.json()?;

    let text: String = api_response
        .content
        .iter()
        .filter_map(|c| c.text.as_ref())
        .cloned()
        .collect::<Vec<String>>()
        .join("");

    println!("Response received in {:.2}s", elapsed.as_secs_f64());
    println!("Input tokens: {}", api_response.usage.input_tokens);
    println!("Output tokens: {}", api_response.usage.output_tokens);
    println!();

    // Extract code from response
    let code = if let Some(start_idx) = text.find("```rust") {
        let after_start = &text[start_idx + 7..];
        if let Some(end) = after_start.find("```") {
            after_start[..end].trim().to_string()
        } else {
            text.clone()
        }
    } else {
        text.clone()
    };

    // Count tests in generated code
    let output_test_count = code
        .lines()
        .filter(|line| line.trim() == "#[test]")
        .count();

    // Check for mod tests wrapper and cfg(test)
    let has_mod_wrapper = code.contains("mod tests") || code.contains("mod test");
    let has_cfg_test = code.contains("#[cfg(test)]");

    println!("=== RESULTS ===");
    println!("Tests in prompt: {}", prompt_test_count);
    println!("Tests in output: {}", output_test_count);
    println!("Has mod tests wrapper: {}", has_mod_wrapper);
    println!("Has #[cfg(test)]: {}", has_cfg_test);
    println!();

    let ratio = output_test_count as f64 / prompt_test_count as f64;

    if output_test_count > prompt_test_count {
        println!("AMPLIFICATION: {} -> {} tests (ratio: {:.2}x)",
                 prompt_test_count, output_test_count, ratio);
        println!("=> mod tests {{ }} wrapper ALONE triggers amplification");
    } else if output_test_count == prompt_test_count {
        println!("PRESERVATION: {} -> {} tests (ratio: {:.2}x)",
                 prompt_test_count, output_test_count, ratio);
        println!("=> mod tests {{ }} alone triggers PRESERVATION (not amplification)");
        println!("=> #[cfg(test)] may be required for amplification");
    } else {
        println!("SUPPRESSION: {} -> {} tests (ratio: {:.2}x)",
                 prompt_test_count, output_test_count, ratio);
        println!("=> Tests were suppressed");
    }

    // Note if model added #[cfg(test)] on its own
    if has_cfg_test && !RUST_MOD_ONLY_PROMPT.contains("#[cfg(test)]") {
        println!("\n⚠️  Model ADDED #[cfg(test)] even though it wasn't in the prompt!");
        println!("   This suggests Sonnet 'corrects' to idiomatic Rust test structure.");
    }

    // Save results
    let output_dir = std::path::Path::new("../results");
    fs::create_dir_all(output_dir)?;

    let code_path = output_dir.join("rust_mod_only_claude-sonnet-4-20250514_code.rs");
    fs::write(&code_path, &code)?;
    println!("\nSaved: {}", code_path.display());

    let response_path = output_dir.join("rust_mod_only_claude-sonnet-4-20250514_response.md");
    fs::write(&response_path, &text)?;
    println!("Saved: {}", response_path.display());

    // Save analysis
    let analysis = format!(
        r#"# Rust Signal Strength Experiment: Mod Only (No cfg)

## Configuration
- Model: {}
- Prompt structure: mod tests {{ use super::*; ... }} WITHOUT #[cfg(test)]
- Tests in prompt: {}

## Results
- Tests in output: {}
- Ratio: {:.2}x
- Has mod tests wrapper in output: {}
- Has #[cfg(test)] in output: {}

## Interpretation
{}

## Comparison
| Experiment | Prompt Structure | Tests In | Tests Out | Ratio | Result |
|------------|------------------|----------|-----------|-------|--------|
| Original | #[cfg(test)] mod tests | 6 | 22 | 3.67x | AMPLIFICATION |
| No Module | Top-level #[test] | 20 | 20 | 1.00x | PRESERVATION |
| Mod Only | mod tests (no cfg) | {} | {} | {:.2}x | {} |

## Raw Metrics
- Input tokens: {}
- Output tokens: {}
- Response time: {:.2}s
"#,
        model,
        prompt_test_count,
        output_test_count,
        ratio,
        has_mod_wrapper,
        has_cfg_test,
        if output_test_count > prompt_test_count {
            "AMPLIFICATION detected - mod tests wrapper alone is sufficient"
        } else if output_test_count == prompt_test_count {
            "PRESERVATION detected - mod tests without #[cfg(test)] does NOT amplify"
        } else {
            "SUPPRESSION detected - unexpected behavior"
        },
        prompt_test_count,
        output_test_count,
        ratio,
        if output_test_count > prompt_test_count {
            "AMPLIFICATION"
        } else if output_test_count == prompt_test_count {
            "PRESERVATION"
        } else {
            "SUPPRESSION"
        },
        api_response.usage.input_tokens,
        api_response.usage.output_tokens,
        elapsed.as_secs_f64()
    );

    let analysis_path = output_dir.join("rust_mod_only_analysis.md");
    fs::write(&analysis_path, &analysis)?;
    println!("Saved: {}", analysis_path.display());

    Ok(())
}
