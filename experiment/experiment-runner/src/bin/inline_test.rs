//! Quick experiment runner for testing the Zig inline hypothesis (H3).
//!
//! This runs the modified Zig prompt that presents tests as inline (no @import)
//! to test whether the import pattern causes suppression.
//!
//! Usage: cargo run --bin inline_test

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

const INLINE_ZIG_PROMPT: &str = r#"Implement a d-ary heap priority queue in Zig.

Requirements:
1. The heap arity (d) should be configurable at construction time
2. Items have two distinct properties: an identity (for equality) and a priority (for ordering)
3. Two items are equal if they have the same identity, regardless of priority
4. The queue should support O(1) lookup to check if an item exists
5. Implement a min-heap where lower priority values have higher importance

Required operations:
- insert(item): Add an item to the queue
- pop(): Remove and return the item with highest priority (lowest value)
- front(): Return the item with highest priority without removing it
- increase_priority(item): Update an existing item to have higher priority (lower value)
- decrease_priority(item): Update an existing item to have lower priority (higher value)
- contains(item): Check if an item with the given identity exists
- len(): Return the number of items in the queue
- is_empty(): Return whether the queue is empty

Your implementation must pass all of the following tests. Note: these tests are meant to be
in the SAME FILE as the implementation (Zig's standard inline test pattern).

//! Test corpus for d-ary heap priority queue operations.
//!
//! These tests are inline with the implementation (same file).

const std = @import("std");
const testing = std.testing;

// Item struct - implement this
pub const Item = struct {
    number: u32,
    cost: u32,

    pub fn init(number: u32, cost: u32) Item {
        return .{ .number = number, .cost = cost };
    }
};

// Comparator for min-heap by cost
pub fn MinByCost(a: Item, b: Item) bool {
    return a.cost < b.cost;
}

// DHeapItem - your implementation goes here
// pub const DHeapItem = struct { ... };

// =============================================================================
// insert() Tests
// =============================================================================

test "insert_postcondition_item_findable" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    const item = Item.init(50, 50);
    try pq.insert(item);

    try testing.expect(pq.contains(item));
}

test "insert_invariant_heap_property" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    const items = [_]Item{
        Item.init(30, 30),
        Item.init(10, 10),
        Item.init(50, 50),
        Item.init(20, 20),
        Item.init(40, 40),
    };

    for (items) |item| {
        try pq.insert(item);
        try testing.expect(pq.front().?.cost <= 30);
    }

    try testing.expectEqual(@as(u32, 10), pq.front().?.cost);
}

test "insert_size_increments" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    var i: u32 = 0;
    while (i < 5) : (i += 1) {
        const size_before = pq.len();
        try pq.insert(Item.init(i, i * 10));
        try testing.expectEqual(size_before + 1, pq.len());
    }
}

test "insert_edge_becomes_front_if_highest_priority" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(100, 100));
    try pq.insert(Item.init(50, 50));
    try pq.insert(Item.init(10, 10));

    try testing.expectEqual(@as(u32, 10), pq.front().?.cost);

    try pq.insert(Item.init(1, 1));

    try testing.expectEqual(@as(u32, 1), pq.front().?.cost);
}

// =============================================================================
// pop() Tests
// =============================================================================

test "pop_postcondition_returns_minimum" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(30, 30));
    try pq.insert(Item.init(10, 10));
    try pq.insert(Item.init(20, 20));

    try testing.expectEqual(@as(u32, 10), pq.front().?.cost);

    const popped = try pq.pop();
    try testing.expectEqual(@as(u32, 10), popped.?.cost);

    try testing.expect(!pq.contains(Item.init(10, 10)));
}

test "pop_invariant_maintains_heap_property" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    const items = [_]Item{
        Item.init(50, 50),
        Item.init(20, 20),
        Item.init(80, 80),
        Item.init(10, 10),
        Item.init(60, 60),
        Item.init(30, 30),
        Item.init(70, 70),
        Item.init(40, 40),
    };

    for (items) |item| {
        try pq.insert(item);
    }

    const expected_order = [_]u32{ 10, 20, 30, 40 };
    for (expected_order) |expected| {
        try testing.expectEqual(expected, pq.front().?.cost);
        _ = try pq.pop();
    }
}

test "pop_size_decrements" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(10, 10));
    try pq.insert(Item.init(20, 20));
    try pq.insert(Item.init(30, 30));

    var expected_size: usize = 2;
    while (expected_size > 0) : (expected_size -= 1) {
        const size_before = pq.len();
        _ = try pq.pop();
        try testing.expectEqual(size_before - 1, pq.len());
    }
}

test "pop_edge_empty_returns_null" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try testing.expect(pq.isEmpty());
    try testing.expectEqual(@as(?Item, null), pq.front());
}

// =============================================================================
// front() Tests
// =============================================================================

test "front_postcondition_returns_minimum" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(30, 30));
    try pq.insert(Item.init(10, 10));
    try pq.insert(Item.init(20, 20));

    try testing.expectEqual(@as(u32, 10), pq.front().?.cost);
}

test "front_invariant_no_modification" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(30, 30));
    try pq.insert(Item.init(10, 10));
    try pq.insert(Item.init(20, 20));

    const first = pq.front().?;
    const second = pq.front().?;
    const third = pq.front().?;

    try testing.expectEqual(first.cost, second.cost);
    try testing.expectEqual(second.cost, third.cost);
}

test "front_size_unchanged" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(10, 10));
    try pq.insert(Item.init(20, 20));
    try pq.insert(Item.init(30, 30));

    const size_before = pq.len();

    var i: usize = 0;
    while (i < 5) : (i += 1) {
        _ = pq.front();
    }

    try testing.expectEqual(size_before, pq.len());
}

test "front_edge_empty_returns_null" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try testing.expectEqual(@as(?Item, null), pq.front());
}

// =============================================================================
// increasePriority() Tests
// =============================================================================

test "increase_priority_postcondition_priority_changed" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(50, 50));
    try pq.insert(Item.init(30, 30));

    try testing.expectEqual(@as(u32, 30), pq.front().?.cost);

    const updated = Item.init(50, 10);
    try pq.increasePriority(updated);

    try testing.expectEqual(@as(u32, 10), pq.front().?.cost);
}

test "increase_priority_invariant_heap_property" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    const items = [_]Item{
        Item.init(80, 80),
        Item.init(60, 60),
        Item.init(40, 40),
        Item.init(20, 20),
        Item.init(100, 100),
        Item.init(50, 50),
    };

    for (items) |item| {
        try pq.insert(item);
    }

    try testing.expectEqual(@as(u32, 20), pq.front().?.cost);

    const updated = Item.init(80, 5);
    try pq.increasePriority(updated);

    try testing.expectEqual(@as(u32, 5), pq.front().?.cost);
}

test "increase_priority_position_item_moves_up" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(10, 10));
    try pq.insert(Item.init(50, 50));
    try pq.insert(Item.init(100, 100));

    try testing.expect(pq.front().?.number != 100);

    const updated = Item.init(100, 1);
    try pq.increasePriority(updated);

    try testing.expectEqual(@as(u32, 100), pq.front().?.number);
}

test "increase_priority_size_unchanged" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(50, 50));
    try pq.insert(Item.init(30, 30));
    try pq.insert(Item.init(70, 70));

    const size_before = pq.len();

    const updated = Item.init(70, 10);
    try pq.increasePriority(updated);

    try testing.expectEqual(size_before, pq.len());
}

test "increase_priority_edge_not_found_returns_error" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(50, 50));

    const nonexistent = Item.init(999, 10);
    const result = pq.increasePriority(nonexistent);
    try testing.expectError(error.ItemNotFound, result);
}

// =============================================================================
// decreasePriority() Tests
// =============================================================================

test "decrease_priority_postcondition_priority_changed" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(10, 10));
    try pq.insert(Item.init(30, 30));

    try testing.expectEqual(@as(u32, 10), pq.front().?.number);

    const updated = Item.init(10, 50);
    try pq.decreasePriority(updated);

    try testing.expectEqual(@as(u32, 30), pq.front().?.number);

    _ = try pq.pop();
    try testing.expectEqual(@as(u32, 50), pq.front().?.cost);
}

test "decrease_priority_invariant_heap_property" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    const items = [_]Item{
        Item.init(10, 10),
        Item.init(30, 30),
        Item.init(50, 50),
        Item.init(70, 70),
        Item.init(20, 20),
        Item.init(40, 40),
    };

    for (items) |item| {
        try pq.insert(item);
    }

    try testing.expectEqual(@as(u32, 10), pq.front().?.number);

    const updated = Item.init(10, 100);
    try pq.decreasePriority(updated);

    try testing.expectEqual(@as(u32, 20), pq.front().?.cost);
}

test "decrease_priority_position_item_moves_down" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(10, 10));
    try pq.insert(Item.init(50, 50));
    try pq.insert(Item.init(60, 60));
    try pq.insert(Item.init(70, 70));

    try testing.expectEqual(@as(u32, 10), pq.front().?.number);

    const updated = Item.init(10, 100);
    try pq.decreasePriority(updated);

    try testing.expect(pq.front().?.number != 10);
    try testing.expectEqual(@as(u32, 50), pq.front().?.number);
}

test "decrease_priority_size_unchanged" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(10, 10));
    try pq.insert(Item.init(30, 30));
    try pq.insert(Item.init(50, 50));

    const size_before = pq.len();

    const updated = Item.init(10, 100);
    try pq.decreasePriority(updated);

    try testing.expectEqual(size_before, pq.len());
}

test "decrease_priority_edge_not_found_returns_error" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(50, 50));

    const nonexistent = Item.init(999, 100);
    const result = pq.decreasePriority(nonexistent);
    try testing.expectError(error.ItemNotFound, result);
}

Provide a complete, working implementation that passes all tests. Include the tests in your output file."#;

fn main() -> Result<()> {
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .map_err(|_| anyhow::anyhow!("ANTHROPIC_API_KEY environment variable not set"))?;

    let model = "claude-sonnet-4-20250514";

    println!("=== Zig Inline Test Experiment (H3 Hypothesis) ===");
    println!("Model: {}", model);
    println!("Testing: Does removing @import pattern prevent test suppression?");
    println!();

    let request = AnthropicRequest {
        model: model.to_string(),
        max_tokens: 8192,
        messages: vec![Message {
            role: "user".to_string(),
            content: INLINE_ZIG_PROMPT.to_string(),
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
    let code = if let Some(start) = text.find("```zig") {
        let after_start = &text[start + 6..];
        if let Some(end) = after_start.find("```") {
            after_start[..end].trim().to_string()
        } else {
            text.clone()
        }
    } else {
        text.clone()
    };

    // Count tests in generated code
    let test_count = code.lines().filter(|line: &&str| line.trim().starts_with("test ")).count();

    println!("=== RESULTS ===");
    println!("Tests generated: {}", test_count);
    println!();

    if test_count > 0 {
        println!("✅ H3 SUPPORTED: Inline tests (no @import) = {} tests generated", test_count);
        println!("   Compare to standard test_guided: 0 tests (suppression)");
    } else {
        println!("❌ H3 NOT SUPPORTED: Still 0 tests despite inline presentation");
        println!("   Suppression is NOT caused by import pattern");
    }

    // Save results
    let output_dir = std::path::Path::new("../results");
    fs::create_dir_all(output_dir)?;

    let code_path = output_dir.join("test_guided_zig_inline_claude-sonnet-4-20250514_code.zig");
    fs::write(&code_path, &code)?;
    println!("\nSaved: {}", code_path.display());

    let response_path = output_dir.join("test_guided_zig_inline_claude-sonnet-4-20250514_response.md");
    fs::write(&response_path, &text)?;
    println!("Saved: {}", response_path.display());

    Ok(())
}
