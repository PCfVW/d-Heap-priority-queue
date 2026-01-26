// comprehensive.rs
//
// Comprehensive test suite for d-ary heap priority queue in Rust
// Aligned with TypeScript, Go, and Zig test patterns for cross-language consistency
//
// Copyright (c) 2023-2025 Eric Jacopin
//
// Licensed under the Apache License, Version 2.0 (the "License")

use d_ary_heap::{PriorityQueue, MinBy, MaxBy, Error, Position};
use std::hash::{Hash, Hasher};
use std::fmt;

// =============================================================================
// Test Item Type
// =============================================================================

#[derive(Clone, Debug)]
struct Item {
    id: u32,
    cost: u32,
}

impl Item {
    fn new(id: u32, cost: u32) -> Self {
        Self { id, cost }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Item {}

impl Hash for Item {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Item(id: {}, cost: {})", self.id, self.cost)
    }
}

// =============================================================================
// Basic Operations Tests
// =============================================================================

#[test]
fn test_new() {
    let pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    assert_eq!(pq.len(), 0);
    assert!(pq.is_empty());
    assert_eq!(pq.d(), 2);
}

#[test]
fn test_new_default_arity() {
    // Test various arities
    for d in [1, 2, 3, 4, 8, 16] {
        let pq: PriorityQueue<Item, MinBy<_>> =
            PriorityQueue::new(d, MinBy(|x: &Item| x.cost)).unwrap();
        assert_eq!(pq.d(), d);
    }
}

#[test]
fn test_new_invalid_arity() {
    let result: Result<PriorityQueue<Item, MinBy<_>>, Error> =
        PriorityQueue::new(0, MinBy(|x: &Item| x.cost));
    assert!(matches!(result, Err(Error::InvalidArity)));
}

#[test]
fn test_with_first() {
    let pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::with_first(3, MinBy(|x: &Item| x.cost), Item::new(1, 10)).unwrap();
    assert_eq!(pq.len(), 1);
    assert!(!pq.is_empty());
    assert_eq!(pq.front().id, 1);
}

#[test]
fn test_with_first_invalid_arity() {
    let result: Result<PriorityQueue<Item, MinBy<_>>, Error> =
        PriorityQueue::with_first(0, MinBy(|x: &Item| x.cost), Item::new(1, 10));
    assert!(matches!(result, Err(Error::InvalidArity)));
}

#[test]
fn test_len() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    assert_eq!(pq.len(), 0);
    pq.insert(Item::new(1, 10));
    assert_eq!(pq.len(), 1);
    pq.insert(Item::new(2, 20));
    assert_eq!(pq.len(), 2);
}

#[test]
fn test_is_empty() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    assert!(pq.is_empty());
    pq.insert(Item::new(1, 10));
    assert!(!pq.is_empty());
    pq.pop();
    assert!(pq.is_empty());
}

#[test]
fn test_d() {
    for d in [1, 2, 3, 4, 8, 16] {
        let pq: PriorityQueue<Item, MinBy<_>> =
            PriorityQueue::new(d, MinBy(|x: &Item| x.cost)).unwrap();
        assert_eq!(pq.d(), d);
    }
}

// =============================================================================
// Insert and Pop Tests
// =============================================================================

#[test]
fn test_insert() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 10));
    assert_eq!(pq.len(), 1);
    assert_eq!(pq.front().id, 1);
}

#[test]
fn test_insert_many() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();

    let items = vec![
        Item::new(1, 50),
        Item::new(2, 30),
        Item::new(3, 70),
        Item::new(4, 10),
        Item::new(5, 40),
    ];
    pq.insert_many(items);

    assert_eq!(pq.len(), 5);
    assert_eq!(pq.front().id, 4); // Lowest cost = highest priority
}

#[test]
fn test_insert_many_empty() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    let empty: Vec<Item> = vec![];
    pq.insert_many(empty);
    assert!(pq.is_empty());
}

#[test]
fn test_pop() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 30));
    pq.insert(Item::new(2, 10));
    pq.insert(Item::new(3, 20));

    assert_eq!(pq.pop().map(|i| i.id), Some(2));
    assert_eq!(pq.pop().map(|i| i.id), Some(3));
    assert_eq!(pq.pop().map(|i| i.id), Some(1));
    assert_eq!(pq.pop(), None);
}

#[test]
fn test_pop_many() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert_many(vec![
        Item::new(1, 50),
        Item::new(2, 10),
        Item::new(3, 30),
        Item::new(4, 20),
        Item::new(5, 40),
    ]);

    let items = pq.pop_many(3);
    assert_eq!(items.len(), 3);
    assert_eq!(items[0].id, 2); // cost 10
    assert_eq!(items[1].id, 4); // cost 20
    assert_eq!(items[2].id, 3); // cost 30
    assert_eq!(pq.len(), 2);
}

#[test]
fn test_pop_many_more_than_available() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert_many(vec![Item::new(1, 10), Item::new(2, 20)]);

    let items = pq.pop_many(10);
    assert_eq!(items.len(), 2);
    assert!(pq.is_empty());
}

#[test]
fn test_pop_empty() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    assert_eq!(pq.pop(), None);
}

// =============================================================================
// Front/Peek Tests
// =============================================================================

#[test]
fn test_front() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 30));
    pq.insert(Item::new(2, 10));
    assert_eq!(pq.front().id, 2);
}

#[test]
#[should_panic(expected = "front() called on empty priority queue")]
fn test_front_empty() {
    let pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    let _ = pq.front();
}

#[test]
fn test_peek() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    assert_eq!(pq.peek(), None);

    pq.insert(Item::new(1, 10));
    assert_eq!(pq.peek().map(|i| i.id), Some(1));
}

#[test]
fn test_peek_empty() {
    let pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    assert_eq!(pq.peek(), None);
}

// =============================================================================
// Contains and GetPosition Tests
// =============================================================================

#[test]
fn test_contains() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 10));

    assert!(pq.contains(&Item::new(1, 999))); // ID matches, cost doesn't matter
    assert!(!pq.contains(&Item::new(2, 10)));
}

#[test]
fn test_contains_empty() {
    let pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    assert!(!pq.contains(&Item::new(1, 10)));
}

#[test]
fn test_get_position() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 30));
    pq.insert(Item::new(2, 10));
    pq.insert(Item::new(3, 20));

    // Root (highest priority) is at position 0
    assert_eq!(pq.get_position(&Item::new(2, 0)), Some(0));
    assert!(pq.get_position(&Item::new(1, 0)).is_some());
    assert!(pq.get_position(&Item::new(3, 0)).is_some());
    assert_eq!(pq.get_position(&Item::new(99, 0)), None);
}

#[test]
fn test_get_position_missing() {
    let pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    assert_eq!(pq.get_position(&Item::new(1, 10)), None);
}

// =============================================================================
// Priority Update Tests
// =============================================================================

#[test]
fn test_increase_priority() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 30));
    pq.insert(Item::new(2, 10));
    pq.insert(Item::new(3, 20));

    // Item 1 has cost 30, increase priority by lowering cost to 5
    pq.increase_priority(&Item::new(1, 5)).unwrap();
    assert_eq!(pq.front().id, 1);
}

#[test]
fn test_increase_priority_not_found() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 10));

    let result = pq.increase_priority(&Item::new(99, 5));
    assert_eq!(result, Err(Error::ItemNotFound));
}

#[test]
fn test_decrease_priority() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 10));
    pq.insert(Item::new(2, 20));
    pq.insert(Item::new(3, 30));

    // Item 1 has cost 10, decrease priority by raising cost to 50
    pq.decrease_priority(&Item::new(1, 50)).unwrap();
    assert_eq!(pq.front().id, 2); // Now item 2 (cost 20) is front
}

#[test]
fn test_decrease_priority_not_found() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 10));

    let result = pq.decrease_priority(&Item::new(99, 50));
    assert_eq!(result, Err(Error::ItemNotFound));
}

#[test]
fn test_update_priority_moves_up() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 30));
    pq.insert(Item::new(2, 10));
    pq.insert(Item::new(3, 20));

    // Item 3 has cost 20, update to cost 5 (moves up)
    pq.update_priority(&Item::new(3, 5)).unwrap();
    assert_eq!(pq.front().id, 3);
}

#[test]
fn test_update_priority_moves_down() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 10));
    pq.insert(Item::new(2, 20));
    pq.insert(Item::new(3, 30));

    // Item 1 has cost 10, update to cost 100 (moves down)
    pq.update_priority(&Item::new(1, 100)).unwrap();
    assert_eq!(pq.front().id, 2);
}

#[test]
fn test_update_priority_not_found() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 10));

    let result = pq.update_priority(&Item::new(99, 5));
    assert_eq!(result, Err(Error::ItemNotFound));
}

// =============================================================================
// By-Index Priority Update Tests
// =============================================================================

#[test]
fn test_increase_priority_by_index() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 10));
    pq.insert(Item::new(2, 20));

    // Get position of item 2, modify container directly, then increase priority
    let pos: Position = pq.get_position(&Item::new(2, 0)).unwrap();
    pq.increase_priority_by_index(pos).unwrap();
    // Item should maintain heap property
    assert_eq!(pq.len(), 2);
}

#[test]
fn test_increase_priority_by_index_out_of_bounds() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 10));

    let result = pq.increase_priority_by_index(99);
    assert_eq!(result, Err(Error::IndexOutOfBounds));
}

#[test]
fn test_decrease_priority_by_index() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 10));
    pq.insert(Item::new(2, 20));

    pq.decrease_priority_by_index(0).unwrap();
    // Item should maintain heap property
    assert_eq!(pq.len(), 2);
}

#[test]
fn test_decrease_priority_by_index_out_of_bounds() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 10));

    let result = pq.decrease_priority_by_index(99);
    assert_eq!(result, Err(Error::IndexOutOfBounds));
}

#[test]
fn test_update_priority_by_index() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 10));
    pq.insert(Item::new(2, 20));

    pq.update_priority_by_index(0).unwrap();
    assert_eq!(pq.len(), 2);
}

#[test]
fn test_update_priority_by_index_out_of_bounds() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 10));

    let result = pq.update_priority_by_index(99);
    assert_eq!(result, Err(Error::IndexOutOfBounds));
}

// =============================================================================
// Min/Max Heap Tests
// =============================================================================

#[test]
fn test_min_heap() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();

    for cost in [50, 30, 70, 10, 40, 60, 20, 80] {
        pq.insert(Item::new(cost, cost));
    }

    let mut prev = 0;
    while let Some(item) = pq.pop() {
        assert!(item.cost >= prev);
        prev = item.cost;
    }
}

#[test]
fn test_max_heap() {
    let mut pq: PriorityQueue<Item, MaxBy<_>> =
        PriorityQueue::new(2, MaxBy(|x: &Item| x.cost)).unwrap();

    for cost in [50, 30, 70, 10, 40, 60, 20, 80] {
        pq.insert(Item::new(cost, cost));
    }

    let mut prev = u32::MAX;
    while let Some(item) = pq.pop() {
        assert!(item.cost <= prev);
        prev = item.cost;
    }
}

// =============================================================================
// Different Arities Tests
// =============================================================================

#[test]
fn test_arity_1() {
    test_arity_helper(1);
}

#[test]
fn test_arity_2() {
    test_arity_helper(2);
}

#[test]
fn test_arity_3() {
    test_arity_helper(3);
}

#[test]
fn test_arity_4() {
    test_arity_helper(4);
}

#[test]
fn test_arity_8() {
    test_arity_helper(8);
}

#[test]
fn test_arity_16() {
    test_arity_helper(16);
}

fn test_arity_helper(d: usize) {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(d, MinBy(|x: &Item| x.cost)).unwrap();

    let items: Vec<u32> = vec![50, 30, 70, 10, 40, 60, 20, 80, 90, 5];
    for (i, &cost) in items.iter().enumerate() {
        pq.insert(Item::new(i as u32, cost));
    }

    let mut prev = 0;
    while let Some(item) = pq.pop() {
        assert!(item.cost >= prev, "Arity {}: heap property violated", d);
        prev = item.cost;
    }
}

// =============================================================================
// Clear Tests
// =============================================================================

#[test]
fn test_clear() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 10));
    pq.insert(Item::new(2, 20));

    pq.clear(None).unwrap();
    assert!(pq.is_empty());
    assert_eq!(pq.d(), 2); // Arity preserved
}

#[test]
fn test_clear_with_new_arity() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 10));

    pq.clear(Some(4)).unwrap();
    assert!(pq.is_empty());
    assert_eq!(pq.d(), 4); // Arity changed
}

#[test]
fn test_clear_invalid_arity() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();

    let result = pq.clear(Some(0));
    assert_eq!(result, Err(Error::InvalidArity));
}

// =============================================================================
// String Representation Tests
// =============================================================================

#[test]
fn test_to_string() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 10));
    pq.insert(Item::new(2, 5));

    let output = pq.to_string();
    assert!(output.starts_with('{'));
    assert!(output.ends_with('}'));
    assert!(output.contains("Item"));
}

#[test]
fn test_to_string_empty() {
    let pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    assert_eq!(pq.to_string(), "{}");
}

#[test]
fn test_display_trait() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 10));

    let display_output = format!("{}", pq);
    let to_string_output = pq.to_string();
    assert_eq!(display_output, to_string_output);
}

// =============================================================================
// to_array Tests
// =============================================================================

#[test]
fn test_to_array() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 30));
    pq.insert(Item::new(2, 10));
    pq.insert(Item::new(3, 20));

    let arr = pq.to_array();
    assert_eq!(arr.len(), 3);
    assert_eq!(arr[0].id, 2); // Root is highest priority (lowest cost)
}

#[test]
fn test_to_array_empty() {
    let pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    let arr = pq.to_array();
    assert!(arr.is_empty());
}

// =============================================================================
// Heap Property Maintenance Tests
// =============================================================================

#[test]
fn test_heap_property_maintained() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(3, MinBy(|x: &Item| x.cost)).unwrap();

    // Insert many items
    for i in 0..100 {
        pq.insert(Item::new(i, (i * 7 + 13) % 100)); // Pseudo-random costs
    }

    // Verify heap property with sequential pops
    let mut prev = 0;
    while let Some(item) = pq.pop() {
        assert!(item.cost >= prev);
        prev = item.cost;
    }
}

#[test]
fn test_heap_property_after_updates() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();

    // Insert items
    for i in 0..50 {
        pq.insert(Item::new(i, i * 2));
    }

    // Perform random updates
    for i in 0..25 {
        let new_cost = (i * 3 + 7) % 100;
        pq.update_priority(&Item::new(i, new_cost)).unwrap();
    }

    // Pop some items
    for _ in 0..10 {
        pq.pop();
    }

    // Verify remaining items maintain heap property
    let mut prev = 0;
    while let Some(item) = pq.pop() {
        assert!(item.cost >= prev);
        prev = item.cost;
    }
}

// =============================================================================
// Edge Cases Tests
// =============================================================================

#[test]
fn test_single_element() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 10));

    assert_eq!(pq.len(), 1);
    assert_eq!(pq.front().id, 1);
    assert!(pq.contains(&Item::new(1, 0)));

    pq.increase_priority(&Item::new(1, 5)).unwrap();
    assert_eq!(pq.front().cost, 5);

    assert_eq!(pq.pop().map(|i| i.id), Some(1));
    assert!(pq.is_empty());
}

#[test]
fn test_duplicate_priorities() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();

    // All items have same priority
    pq.insert(Item::new(1, 10));
    pq.insert(Item::new(2, 10));
    pq.insert(Item::new(3, 10));

    assert_eq!(pq.len(), 3);

    // All items should be poppable
    let mut ids = Vec::new();
    while let Some(item) = pq.pop() {
        ids.push(item.id);
    }
    assert_eq!(ids.len(), 3);
}

// =============================================================================
// Large Scale Tests
// =============================================================================

#[test]
fn test_large_heap() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(4, MinBy(|x: &Item| x.cost)).unwrap();

    // Insert 10000 items with pseudo-random costs
    for i in 0..10000 {
        let cost = ((i * 31337 + 12345) % 5000) as u32;
        pq.insert(Item::new(i, cost));
    }

    assert_eq!(pq.len(), 10000);

    // Verify sorted output
    let mut prev = 0;
    while let Some(item) = pq.pop() {
        assert!(item.cost >= prev);
        prev = item.cost;
    }
}

#[test]
fn test_large_heap_with_updates() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(4, MinBy(|x: &Item| x.cost)).unwrap();

    // Insert 1000 items
    for i in 0..1000 {
        pq.insert(Item::new(i, i));
    }

    // Perform 500 updates
    for i in 0..500 {
        let new_cost = ((i * 17 + 23) % 1000) as u32;
        pq.update_priority(&Item::new(i, new_cost)).unwrap();
    }

    // Verify sorted output
    let mut prev = 0;
    while let Some(item) = pq.pop() {
        assert!(item.cost >= prev);
        prev = item.cost;
    }
}

// =============================================================================
// Position Type Alias Test
// =============================================================================

#[test]
fn test_position_type_alias() {
    let mut pq: PriorityQueue<Item, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &Item| x.cost)).unwrap();
    pq.insert(Item::new(1, 10));

    let pos: Position = pq.get_position(&Item::new(1, 0)).unwrap();
    assert_eq!(pos, 0);
}

// =============================================================================
// Error Display Test
// =============================================================================

#[test]
fn test_error_display() {
    assert_eq!(format!("{}", Error::InvalidArity), "Heap arity (d) must be >= 1");
    assert_eq!(format!("{}", Error::ItemNotFound), "Item not found");
    assert_eq!(format!("{}", Error::IndexOutOfBounds), "Index out of bounds");
    assert_eq!(format!("{}", Error::EmptyQueue), "Operation called on empty priority queue");
}

// =============================================================================
// Primitive Type Tests (using i32 directly)
// =============================================================================

#[test]
fn test_primitive_min_heap() {
    let mut pq: PriorityQueue<i32, MinBy<_>> =
        PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();

    pq.insert(5);
    pq.insert(3);
    pq.insert(7);
    pq.insert(1);

    assert_eq!(pq.pop(), Some(1));
    assert_eq!(pq.pop(), Some(3));
    assert_eq!(pq.pop(), Some(5));
    assert_eq!(pq.pop(), Some(7));
    assert_eq!(pq.pop(), None);
}

#[test]
fn test_primitive_max_heap() {
    let mut pq: PriorityQueue<i32, MaxBy<_>> =
        PriorityQueue::new(2, MaxBy(|x: &i32| *x)).unwrap();

    pq.insert(5);
    pq.insert(3);
    pq.insert(7);
    pq.insert(1);

    assert_eq!(pq.pop(), Some(7));
    assert_eq!(pq.pop(), Some(5));
    assert_eq!(pq.pop(), Some(3));
    assert_eq!(pq.pop(), Some(1));
    assert_eq!(pq.pop(), None);
}
