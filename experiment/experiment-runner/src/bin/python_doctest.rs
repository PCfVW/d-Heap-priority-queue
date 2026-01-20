//! Experiment runner for testing Python doctest amplification hypothesis.
//!
//! Python doctests are unique: they are BOTH documentation AND tests (inline in docstrings).
//! This tests whether Sonnet amplifies doctest examples like it does Rust/Zig tests.
//!
//! Key question: Are doctests treated as "tests" (amplified) or "documentation" (no effect)?
//!
//! Usage: cargo run --bin python_doctest

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

const PYTHON_DOCTEST_PROMPT: &str = r#"Implement a d-ary heap priority queue in Python.

Requirements:
1. The heap arity (d) should be configurable at construction time
2. Items have two distinct properties: an identity (number) and a priority (cost)
3. Two items are equal if they have the same identity (number), regardless of priority
4. The queue should support O(1) lookup to check if an item exists (use a dict for position tracking)
5. Implement a min-heap where lower priority values have higher importance

Required operations with doctests:
- insert(item): Add an item to the queue
- pop(): Remove and return the item with highest priority (lowest cost)
- front(): Return the item with highest priority without removing it
- increase_priority(item): Update an existing item to have higher priority (lower cost)
- decrease_priority(item): Update an existing item to have lower priority (higher cost)
- contains(item): Check if an item with the given identity exists
- __len__(): Return the number of items in the queue
- is_empty(): Return whether the queue is empty

Here is the Item class and example doctests that your implementation must support:

```python
"""D-ary heap priority queue implementation with doctests."""

from dataclasses import dataclass
from typing import Optional, Callable, List, Dict


@dataclass
class Item:
    """An item with identity (number) and priority (cost).

    >>> item = Item(50, 100)
    >>> item.number
    50
    >>> item.cost
    100
    """
    number: int
    cost: int

    def __eq__(self, other):
        """Items are equal if they have the same number (identity).

        >>> Item(10, 50) == Item(10, 100)
        True
        >>> Item(10, 50) == Item(20, 50)
        False
        """
        if not isinstance(other, Item):
            return False
        return self.number == other.number

    def __hash__(self):
        return hash(self.number)


class DHeap:
    """A d-ary min-heap priority queue.

    >>> pq = DHeap(4)  # 4-ary heap
    >>> pq.is_empty()
    True
    >>> len(pq)
    0
    """

    def __init__(self, d: int = 4):
        """Initialize a d-ary heap.

        >>> pq = DHeap(2)  # binary heap
        >>> pq = DHeap(4)  # 4-ary heap
        """
        pass  # Your implementation here

    def insert(self, item: Item) -> None:
        """Insert an item into the heap.

        >>> pq = DHeap(4)
        >>> pq.insert(Item(50, 50))
        >>> pq.contains(Item(50, 0))  # Same identity, different cost
        True
        >>> len(pq)
        1
        """
        pass  # Your implementation here

    def pop(self) -> Optional[Item]:
        """Remove and return the minimum item.

        >>> pq = DHeap(4)
        >>> pq.insert(Item(30, 30))
        >>> pq.insert(Item(10, 10))
        >>> pq.insert(Item(20, 20))
        >>> item = pq.pop()
        >>> item.cost
        10
        >>> len(pq)
        2
        """
        pass  # Your implementation here

    def front(self) -> Optional[Item]:
        """Return the minimum item without removing it.

        >>> pq = DHeap(4)
        >>> pq.insert(Item(30, 30))
        >>> pq.insert(Item(10, 10))
        >>> pq.front().cost
        10
        >>> len(pq)  # Size unchanged
        2
        """
        pass  # Your implementation here

    def increase_priority(self, item: Item) -> None:
        """Increase priority (decrease cost) of an existing item.

        >>> pq = DHeap(4)
        >>> pq.insert(Item(50, 50))
        >>> pq.insert(Item(30, 30))
        >>> pq.front().cost
        30
        >>> pq.increase_priority(Item(50, 10))  # Lower cost = higher priority
        >>> pq.front().cost
        10
        """
        pass  # Your implementation here

    def decrease_priority(self, item: Item) -> None:
        """Decrease priority (increase cost) of an existing item.

        >>> pq = DHeap(4)
        >>> pq.insert(Item(10, 10))
        >>> pq.insert(Item(30, 30))
        >>> pq.front().number
        10
        >>> pq.decrease_priority(Item(10, 50))  # Higher cost = lower priority
        >>> pq.front().number
        30
        """
        pass  # Your implementation here

    def contains(self, item: Item) -> bool:
        """Check if an item with the same identity exists.

        >>> pq = DHeap(4)
        >>> pq.insert(Item(50, 50))
        >>> pq.contains(Item(50, 999))  # Same number, different cost
        True
        >>> pq.contains(Item(999, 50))  # Different number
        False
        """
        pass  # Your implementation here

    def __len__(self) -> int:
        """Return the number of items.

        >>> pq = DHeap(4)
        >>> len(pq)
        0
        >>> pq.insert(Item(10, 10))
        >>> len(pq)
        1
        """
        pass  # Your implementation here

    def is_empty(self) -> bool:
        """Return True if the heap is empty.

        >>> pq = DHeap(4)
        >>> pq.is_empty()
        True
        >>> pq.insert(Item(10, 10))
        >>> pq.is_empty()
        False
        """
        pass  # Your implementation here


if __name__ == "__main__":
    import doctest
    doctest.testmod()
```

Provide a complete, working implementation. Replace all the `pass` statements with actual code.
Keep ALL the doctests in your implementation - they serve as both documentation and tests.
The code should pass when running: python -m doctest your_file.py -v"#;

fn main() -> Result<()> {
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .map_err(|_| anyhow::anyhow!("ANTHROPIC_API_KEY environment variable not set"))?;

    let model = "claude-sonnet-4-20250514";

    println!("=== Python Doctest Amplification Experiment ===");
    println!("Model: {}", model);
    println!("Testing: Are Python doctests amplified like Rust/Zig inline tests?");
    println!("Key question: Doctests are BOTH documentation AND tests - which treatment?");
    println!();

    let request = AnthropicRequest {
        model: model.to_string(),
        max_tokens: 8192,
        messages: vec![Message {
            role: "user".to_string(),
            content: PYTHON_DOCTEST_PROMPT.to_string(),
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
    let code = if let Some(start_idx) = text.find("```python") {
        let after_start = &text[start_idx + 9..];
        if let Some(end) = after_start.find("```") {
            after_start[..end].trim().to_string()
        } else {
            text.clone()
        }
    } else {
        text.clone()
    };

    // Count doctest patterns in generated code
    let doctest_count = code.lines().filter(|line| line.trim().starts_with(">>>")).count();

    // Count provided doctests in prompt (for comparison)
    let prompt_doctest_count = PYTHON_DOCTEST_PROMPT
        .lines()
        .filter(|line| line.trim().starts_with(">>>"))
        .count();

    // Count methods with doctests
    let methods_with_doctests = ["__init__", "insert", "pop", "front",
                                  "increase_priority", "decrease_priority",
                                  "contains", "__len__", "is_empty", "__eq__"];

    let mut method_doctest_counts: Vec<(&str, usize)> = Vec::new();
    for method in &methods_with_doctests {
        let method_doctests = count_doctests_for_method(&code, method);
        method_doctest_counts.push((method, method_doctests));
    }

    println!("=== RESULTS ===");
    println!("Doctests in prompt: {}", prompt_doctest_count);
    println!("Doctests in output: {}", doctest_count);
    println!();

    println!("Doctest counts by method:");
    for (method, count) in &method_doctest_counts {
        println!("  {}: {}", method, count);
    }
    println!();

    let amplification_ratio = if prompt_doctest_count > 0 {
        doctest_count as f64 / prompt_doctest_count as f64
    } else {
        0.0
    };

    if doctest_count >= prompt_doctest_count {
        println!("✅ Doctests preserved: {} (ratio: {:.2}x)", doctest_count, amplification_ratio);
        if doctest_count > prompt_doctest_count {
            println!("   AMPLIFICATION DETECTED: Model added {} extra doctests!",
                     doctest_count - prompt_doctest_count);
        } else {
            println!("   100% reproduction - same as Rust/Zig test scaffolding");
        }
    } else {
        println!("❌ Doctest suppression: {} -> {} (loss: {})",
                 prompt_doctest_count, doctest_count,
                 prompt_doctest_count - doctest_count);
        println!("   Doctests may be treated as documentation, not tests");
    }

    // Save results
    let output_dir = std::path::Path::new("../results");
    fs::create_dir_all(output_dir)?;

    let code_path = output_dir.join("python_doctest_claude-sonnet-4-20250514_code.py");
    fs::write(&code_path, &code)?;
    println!("\nSaved: {}", code_path.display());

    let response_path = output_dir.join("python_doctest_claude-sonnet-4-20250514_response.md");
    fs::write(&response_path, &text)?;
    println!("Saved: {}", response_path.display());

    // Save analysis summary
    let analysis = format!(
        r#"# Python Doctest Experiment Results

## Configuration
- Model: {}
- Prompt doctests: {}
- Output doctests: {}
- Amplification ratio: {:.2}x

## Method-by-Method Analysis
{}

## Interpretation
{}

## Raw Metrics
- Input tokens: {}
- Output tokens: {}
- Response time: {:.2}s
"#,
        model,
        prompt_doctest_count,
        doctest_count,
        amplification_ratio,
        method_doctest_counts.iter()
            .map(|(m, c)| format!("- {}: {}", m, c))
            .collect::<Vec<_>>()
            .join("\n"),
        if doctest_count >= prompt_doctest_count {
            if doctest_count > prompt_doctest_count {
                "AMPLIFICATION: Model treats doctests as tests and adds more examples."
            } else {
                "PRESERVATION: Model maintains all doctests (100% scaffolding like Rust/Zig)."
            }
        } else {
            "SUPPRESSION: Doctests treated as documentation rather than executable tests."
        },
        api_response.usage.input_tokens,
        api_response.usage.output_tokens,
        elapsed.as_secs_f64()
    );

    let analysis_path = output_dir.join("python_doctest_analysis.md");
    fs::write(&analysis_path, &analysis)?;
    println!("Saved: {}", analysis_path.display());

    Ok(())
}

/// Count doctests within a specific method's docstring
fn count_doctests_for_method(code: &str, method_name: &str) -> usize {
    let search_pattern = if method_name == "__eq__" {
        "def __eq__"
    } else if method_name == "__init__" {
        "def __init__"
    } else if method_name == "__len__" {
        "def __len__"
    } else {
        method_name
    };

    // Find the method definition
    if let Some(method_start) = code.find(&format!("def {}", search_pattern.trim_start_matches("def "))) {
        let after_method = &code[method_start..];

        // Find the docstring (look for triple quotes)
        if let Some(docstring_start) = after_method.find("\"\"\"") {
            let after_docstring_start = &after_method[docstring_start + 3..];
            if let Some(docstring_end) = after_docstring_start.find("\"\"\"") {
                let docstring = &after_docstring_start[..docstring_end];
                return docstring.lines().filter(|line| line.trim().starts_with(">>>")).count();
            }
        }
    }
    0
}
