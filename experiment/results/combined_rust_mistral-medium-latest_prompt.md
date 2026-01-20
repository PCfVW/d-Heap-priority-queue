Implement a d-ary heap priority queue in Rust.

## Overview

A d-ary heap is a generalization of a binary heap where each node has up to d children instead of 2. This implementation requires O(1) item lookup via a position map (hash map tracking each item's index in the heap array).

## Core Concepts

### Identity vs Priority
- **Identity**: Determines equality between items. Used as the key in the position map.
- **Priority**: Determines ordering in the heap. Lower values = higher priority (min-heap).
- Two items with the same identity but different priorities are considered equal.

### Heap Structure
- Array-based complete tree representation
- For a node at index i:
  - Parent index: (i - 1) / d
  - First child index: d * i + 1
  - Last child index: d * i + d (if it exists)
- Root is at index 0

### Position Map
- Hash map from item identity to array index
- Enables O(1) contains() and O(1) lookup for priority updates
- Must be kept synchronized with heap array at all times

## API Documentation

### Constructor
Create a new priority queue with the specified arity.
- **Parameter**: d (arity) - number of children per node, must be >= 2
- **Returns**: New empty priority queue

### insert(item)
Add an item to the queue.
- **Precondition**: Item with same identity must not already exist
- **Postcondition**: Item is in the queue and findable via contains()
- **Postcondition**: Heap property is maintained
- **Postcondition**: Size increases by 1
- **Algorithm**: Add to end of array, then sift up to restore heap property
- **Time complexity**: O(log_d n)

### pop()
Remove and return the item with highest priority (lowest priority value).
- **Precondition**: Queue is not empty
- **Postcondition**: Returned item is no longer in the queue
- **Postcondition**: Heap property is maintained
- **Postcondition**: Size decreases by 1
- **Algorithm**: Swap root with last element, remove last, sift down from root
- **Time complexity**: O(d * log_d n)
- **Edge case**: Return null/None/error if queue is empty

### front()
Return the item with highest priority without removing it.
- **Precondition**: Queue is not empty
- **Postcondition**: Queue is unchanged (same size, same items)
- **Returns**: Item at root (index 0)
- **Time complexity**: O(1)
- **Edge case**: Return null/None/error if queue is empty

### increase_priority(item)
Update an existing item to have higher priority (lower priority value).
- **Precondition**: Item with same identity must exist in queue
- **Input**: Item with the identity to find and the new (lower) priority value
- **Postcondition**: Item's priority is updated to the new value
- **Postcondition**: Heap property is maintained (item may move up)
- **Postcondition**: Size is unchanged
- **Algorithm**: Update priority at current position, then sift up
- **Time complexity**: O(log_d n)
- **Note**: "Increase priority" means making it MORE important (lower value in min-heap)

### decrease_priority(item)
Update an existing item to have lower priority (higher priority value).
- **Precondition**: Item with same identity must exist in queue
- **Input**: Item with the identity to find and the new (higher) priority value
- **Postcondition**: Item's priority is updated to the new value
- **Postcondition**: Heap property is maintained (item may move down)
- **Postcondition**: Size is unchanged
- **Algorithm**: Update priority at current position, then sift down
- **Time complexity**: O(d * log_d n)
- **Note**: "Decrease priority" means making it LESS important (higher value in min-heap)

### contains(item)
Check if an item with the given identity exists in the queue.
- **Returns**: true if item with same identity exists, false otherwise
- **Note**: Compares by identity only, not priority
- **Time complexity**: O(1) via position map lookup

### len()
Return the number of items in the queue.
- **Returns**: Non-negative integer count
- **Time complexity**: O(1)

### is_empty()
Return whether the queue contains no items.
- **Returns**: true if len() == 0, false otherwise
- **Time complexity**: O(1)

## Sift Operations

### sift_up(index)
Restore heap property by moving an item up toward the root.
- Compare item at index with its parent
- If item has higher priority (lower value) than parent, swap them
- Repeat until item is at root or parent has higher/equal priority
- Update position map after each swap

### sift_down(index)
Restore heap property by moving an item down toward the leaves.
- Find the child with highest priority (lowest value) among all children
- If that child has higher priority than the item, swap them
- Repeat until item has no children or no child has higher priority
- Update position map after each swap

## Type Definitions

{TYPE_STUBS}

## Test Corpus

Your implementation must pass all of the following tests:

// Test corpus common definitions
// Part of: Amphigraphic-Strict × d-ary Heap Priority Queue research

use d_ary_heap::{MinBy, PriorityQueue};
use std::hash::{Hash, Hasher};

/// Test item type with separate ID (identity) and priority
#[derive(Debug, Clone)]
pub struct Item {
    pub id: String,
    pub priority: i32,
}

impl Item {
    pub fn new(id: &str, priority: i32) -> Self {
        Self { id: id.to_string(), priority }
    }
}

// Item equality is based on ID only (not priority)
impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Item {}

// Hash is based on ID only (not priority)
impl Hash for Item {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// Helper: create min-heap of Items (lower priority value = higher importance)
pub fn new_item_min_heap(d: usize) -> PriorityQueue<Item, MinBy<impl Fn(&Item) -> i32>> {
    PriorityQueue::new(d, MinBy(|i: &Item| i.priority))
}

/// Helper: verify heap invariant for min-heap
pub fn verify_heap_invariant(pq: &PriorityQueue<Item, impl d_ary_heap::PriorityCompare<Item>>) -> bool {
    // We can't directly access container, so we verify via pop order
    // For test purposes, we verify that we can call front() without panic if non-empty
    if pq.is_empty() {
        return true;
    }
    // Basic sanity check: front() should not panic
    let _ = pq.front();
    true
}

pub mod insert;
pub mod pop;
pub mod front;
pub mod increase_priority;
pub mod decrease_priority;


// --- src/tests/insert.rs ---

// Test corpus for insert() operation
// Spec: specifications/insert.md
// Part of: Amphigraphic-Strict × d-ary Heap Priority Queue research

use super::{new_item_min_heap, Item};

// =============================================================================
// insert() Tests
// =============================================================================

/// Test: insert_postcondition_item_findable
/// Spec: specifications/insert.md
/// Property: inserted item can be found via contains() after insertion
#[test]
fn insert_postcondition_item_findable() {
    let mut pq = new_item_min_heap(4);

    let item = Item::new("test-item", 50);
    pq.insert(item.clone());

    assert!(pq.contains(&item), "inserted item should be findable via contains()");
    assert!(pq.contains(&Item::new("test-item", 999)), "item with same ID should be found regardless of priority value");
}

/// Test: insert_invariant_heap_property
/// Spec: specifications/insert.md
/// Property: heap invariant holds after insertion (front() returns minimum)
#[test]
fn insert_invariant_heap_property() {
    let mut pq = new_item_min_heap(4);

    // Insert items in arbitrary order
    let items = vec![
        Item::new("a", 30),
        Item::new("b", 10),
        Item::new("c", 50),
        Item::new("d", 20),
        Item::new("e", 40),
    ];

    for item in items {
        pq.insert(item);
        // After each insert, front() should be the item with lowest priority
        let front = pq.front();
        // Verify front is the minimum so far
        assert!(front.priority <= 30, "front should have lowest priority value");
    }

    // Final front should be item with priority 10
    assert_eq!(pq.front().priority, 10);
}

/// Test: insert_size_increments
/// Spec: specifications/insert.md
/// Property: heap size increases by 1 after each insertion
#[test]
fn insert_size_increments() {
    let mut pq = new_item_min_heap(4);

    for i in 0..5 {
        let size_before = pq.len();
        pq.insert(Item::new(&format!("item{}", i), i * 10));
        let size_after = pq.len();

        assert_eq!(size_after, size_before + 1, "size should increment by 1 after insert");
    }
}

/// Test: insert_edge_becomes_front_if_highest_priority
/// Spec: specifications/insert.md
/// Property: if inserted item has highest priority, it becomes front()
#[test]
fn insert_edge_becomes_front_if_highest_priority() {
    let mut pq = new_item_min_heap(4);

    // Insert items with decreasing priority values (increasing importance in min-heap)
    pq.insert(Item::new("low", 100));
    pq.insert(Item::new("medium", 50));
    pq.insert(Item::new("high", 10));

    assert_eq!(pq.front().id, "high", "highest priority item should be at front");

    // Insert new highest priority item
    pq.insert(Item::new("urgent", 1));

    assert_eq!(pq.front().id, "urgent", "new highest priority item should become front");
}


// --- src/tests/pop.rs ---

// Test corpus for pop() operation
// Spec: specifications/pop.md
// Part of: Amphigraphic-Strict × d-ary Heap Priority Queue research

use super::{new_item_min_heap, Item};

// =============================================================================
// pop() Tests
// =============================================================================

/// Test: pop_postcondition_returns_minimum
/// Spec: specifications/pop.md
/// Property: pop() removes the item with lowest priority value (min-heap)
/// Note: Rust's pop() doesn't return; use front() before pop() to get value
#[test]
fn pop_postcondition_returns_minimum() {
    let mut pq = new_item_min_heap(4);

    pq.insert(Item::new("a", 30));
    pq.insert(Item::new("b", 10));
    pq.insert(Item::new("c", 20));

    // Get front before popping
    let front = pq.front().clone();
    assert_eq!(front.priority, 10, "front should be minimum priority");
    assert_eq!(front.id, "b");

    pq.pop();

    // After pop, the removed item should not be in heap
    assert!(!pq.contains(&Item::new("b", 0)), "popped item should not be in heap");
}

/// Test: pop_invariant_maintains_heap_property
/// Spec: specifications/pop.md
/// Property: after pop(), heap invariant holds (front() is minimum of remaining)
#[test]
fn pop_invariant_maintains_heap_property() {
    let mut pq = new_item_min_heap(4);

    // Insert multiple items
    let items = vec![
        Item::new("a", 50),
        Item::new("b", 20),
        Item::new("c", 80),
        Item::new("d", 10),
        Item::new("e", 60),
        Item::new("f", 30),
        Item::new("g", 70),
        Item::new("h", 40),
    ];

    for item in items {
        pq.insert(item);
    }

    // Pop half and verify front() is always minimum of remaining
    let expected_order = [10, 20, 30, 40];
    for expected in expected_order {
        assert_eq!(pq.front().priority, expected, "front should be minimum");
        pq.pop();
    }
}

/// Test: pop_size_decrements
/// Spec: specifications/pop.md
/// Property: size() decreases by 1 after successful pop()
#[test]
fn pop_size_decrements() {
    let mut pq = new_item_min_heap(4);

    pq.insert(Item::new("a", 10));
    pq.insert(Item::new("b", 20));
    pq.insert(Item::new("c", 30));

    for expected_size in (0..3).rev() {
        let size_before = pq.len();
        pq.pop();
        let size_after = pq.len();

        assert_eq!(size_after, size_before - 1, "size should decrement by 1 after pop");
        assert_eq!(size_after, expected_size, "size should match expected");
    }
}

/// Test: pop_edge_empty_no_panic
/// Spec: specifications/pop.md
/// Property: pop() on empty heap does not panic (no-op behavior in Rust impl)
#[test]
fn pop_edge_empty_no_panic() {
    let mut pq = new_item_min_heap(4);

    // This should not panic
    pq.pop();

    // Heap should remain empty
    assert!(pq.is_empty(), "heap should remain empty after failed pop");
}


// --- src/tests/front.rs ---

// Test corpus for front() operation
// Spec: specifications/front.md
// Part of: Amphigraphic-Strict × d-ary Heap Priority Queue research

use super::{new_item_min_heap, Item};

// =============================================================================
// front() Tests
// =============================================================================

/// Test: front_postcondition_returns_minimum
/// Spec: specifications/front.md
/// Property: front() returns the item with lowest priority value (min-heap) without removal
#[test]
fn front_postcondition_returns_minimum() {
    let mut pq = new_item_min_heap(4);

    pq.insert(Item::new("a", 30));
    pq.insert(Item::new("b", 10));
    pq.insert(Item::new("c", 20));

    let front = pq.front();
    assert_eq!(front.priority, 10, "front should return minimum priority");
    assert_eq!(front.id, "b", "front should return item with id 'b'");
}

/// Test: front_invariant_no_modification
/// Spec: specifications/front.md
/// Property: front() does not modify the heap (calling multiple times returns same result)
#[test]
fn front_invariant_no_modification() {
    let mut pq = new_item_min_heap(4);

    pq.insert(Item::new("a", 30));
    pq.insert(Item::new("b", 10));
    pq.insert(Item::new("c", 20));

    // Call front() multiple times
    let first = pq.front().clone();
    let second = pq.front().clone();
    let third = pq.front().clone();

    // All should return the same item
    assert_eq!(first.id, second.id, "front() should return same item");
    assert_eq!(second.id, third.id, "front() should return same item");
}

/// Test: front_size_unchanged
/// Spec: specifications/front.md
/// Property: size() remains the same after front()
#[test]
fn front_size_unchanged() {
    let mut pq = new_item_min_heap(4);

    pq.insert(Item::new("a", 10));
    pq.insert(Item::new("b", 20));
    pq.insert(Item::new("c", 30));

    let size_before = pq.len();

    // Call front() multiple times
    for _ in 0..5 {
        let _ = pq.front();
    }

    let size_after = pq.len();

    assert_eq!(size_after, size_before, "size should be unchanged after front()");
}

/// Test: front_edge_empty_peek_returns_none
/// Spec: specifications/front.md
/// Property: peek() on empty heap returns None (front() would panic)
#[test]
fn front_edge_empty_peek_returns_none() {
    let pq = new_item_min_heap(4);

    let result = pq.peek();
    assert!(result.is_none(), "peek() on empty heap should return None");
}


// --- src/tests/increase_priority.rs ---

// Test corpus for increase_priority() operation
// Spec: specifications/increase_priority.md
// Part of: Amphigraphic-Strict × d-ary Heap Priority Queue research

use super::{new_item_min_heap, Item};

// =============================================================================
// increase_priority() Tests
// =============================================================================

/// Test: increase_priority_postcondition_priority_changed
/// Spec: specifications/increase_priority.md
/// Property: item's priority is updated to the new value
#[test]
fn increase_priority_postcondition_priority_changed() {
    let mut pq = new_item_min_heap(4);

    pq.insert(Item::new("target", 50));
    pq.insert(Item::new("other", 30));

    // "other" starts at front with priority 30
    assert_eq!(pq.front().id, "other");

    // Increase priority of "target" (in min-heap: lower value = higher priority)
    // Create updated item with same ID but new priority
    let updated = Item::new("target", 10);
    pq.increase_priority(&updated);

    // "target" should now be at front (highest priority)
    assert_eq!(pq.front().id, "target", "target should be at front after priority increase");
    assert_eq!(pq.front().priority, 10, "priority should be updated to 10");
}

/// Test: increase_priority_invariant_heap_property
/// Spec: specifications/increase_priority.md
/// Property: heap invariant holds after priority increase
#[test]
fn increase_priority_invariant_heap_property() {
    let mut pq = new_item_min_heap(4);

    // Create a larger heap
    let items = vec![
        Item::new("a", 80),
        Item::new("b", 60),
        Item::new("c", 40),
        Item::new("d", 20),
        Item::new("e", 100),
        Item::new("f", 50),
    ];

    for item in items {
        pq.insert(item);
    }

    // "d" with priority 20 should be at front
    assert_eq!(pq.front().priority, 20);

    // Increase priority of "a" (80 -> 5)
    let updated = Item::new("a", 5);
    pq.increase_priority(&updated);

    // Now "a" should be at front
    assert_eq!(pq.front().id, "a", "item with increased priority should be at front");
    assert_eq!(pq.front().priority, 5);
}

/// Test: increase_priority_position_item_moves_up
/// Spec: specifications/increase_priority.md
/// Property: item moves toward root after priority increase (becomes front if highest)
#[test]
fn increase_priority_position_item_moves_up() {
    let mut pq = new_item_min_heap(4);

    pq.insert(Item::new("root", 10));
    pq.insert(Item::new("middle", 50));
    pq.insert(Item::new("leaf", 100));

    // "leaf" should not be at front initially
    assert_ne!(pq.front().id, "leaf");

    // Increase priority of "leaf" to become highest priority
    let updated = Item::new("leaf", 1);
    pq.increase_priority(&updated);

    // "leaf" should now be at front (position 0)
    assert_eq!(pq.front().id, "leaf", "item should move to front after priority increase");
}

/// Test: increase_priority_size_unchanged
/// Spec: specifications/increase_priority.md
/// Property: size() remains unchanged after priority update
#[test]
fn increase_priority_size_unchanged() {
    let mut pq = new_item_min_heap(4);

    pq.insert(Item::new("a", 50));
    pq.insert(Item::new("b", 30));
    pq.insert(Item::new("c", 70));

    let size_before = pq.len();

    let updated = Item::new("c", 10);
    pq.increase_priority(&updated);

    let size_after = pq.len();

    assert_eq!(size_after, size_before, "size should be unchanged after priority update");
}

/// Test: increase_priority_edge_not_found_panics
/// Spec: specifications/increase_priority.md
/// Property: panics if item not in heap (Rust implementation behavior)
#[test]
#[should_panic(expected = "item must exist")]
fn increase_priority_edge_not_found_panics() {
    let mut pq = new_item_min_heap(4);

    pq.insert(Item::new("existing", 50));

    // This should panic - item not found
    let nonexistent = Item::new("nonexistent", 10);
    pq.increase_priority(&nonexistent);
}


// --- src/tests/decrease_priority.rs ---

// Test corpus for decrease_priority() operation
// Spec: specifications/decrease_priority.md
// Part of: Amphigraphic-Strict × d-ary Heap Priority Queue research

use super::{new_item_min_heap, Item};

// =============================================================================
// decrease_priority() Tests
// =============================================================================

/// Test: decrease_priority_postcondition_priority_changed
/// Spec: specifications/decrease_priority.md
/// Property: item's priority is updated to the new value
#[test]
fn decrease_priority_postcondition_priority_changed() {
    let mut pq = new_item_min_heap(4);

    pq.insert(Item::new("target", 10));
    pq.insert(Item::new("other", 30));

    // "target" starts at front with priority 10
    assert_eq!(pq.front().id, "target");

    // Decrease priority of "target" (in min-heap: higher value = lower priority)
    let updated = Item::new("target", 50);
    pq.decrease_priority(&updated);

    // "other" should now be at front (it has higher priority now)
    assert_eq!(pq.front().id, "other", "other should be at front after target's priority decrease");

    // Pop "other" and verify "target" has updated priority
    pq.pop();
    assert_eq!(pq.front().priority, 50, "target's priority should be updated to 50");
}

/// Test: decrease_priority_invariant_heap_property
/// Spec: specifications/decrease_priority.md
/// Property: heap invariant holds after priority decrease
#[test]
fn decrease_priority_invariant_heap_property() {
    let mut pq = new_item_min_heap(4);

    // Create a larger heap
    let items = vec![
        Item::new("a", 10),
        Item::new("b", 30),
        Item::new("c", 50),
        Item::new("d", 70),
        Item::new("e", 20),
        Item::new("f", 40),
    ];

    for item in items {
        pq.insert(item);
    }

    // "a" with priority 10 should be at front
    assert_eq!(pq.front().id, "a");

    // Decrease priority of "a" (10 -> 100)
    let updated = Item::new("a", 100);
    pq.decrease_priority(&updated);

    // "e" with priority 20 should now be at front
    assert_eq!(pq.front().priority, 20, "new minimum should be at front");
}

/// Test: decrease_priority_position_item_moves_down
/// Spec: specifications/decrease_priority.md
/// Property: item moves toward leaves after priority decrease (no longer front if was)
#[test]
fn decrease_priority_position_item_moves_down() {
    let mut pq = new_item_min_heap(4);

    pq.insert(Item::new("root", 10));
    pq.insert(Item::new("child1", 50));
    pq.insert(Item::new("child2", 60));
    pq.insert(Item::new("child3", 70));

    // "root" is at front
    assert_eq!(pq.front().id, "root");

    // Decrease priority of "root" to become lowest priority
    let updated = Item::new("root", 100);
    pq.decrease_priority(&updated);

    // "root" should no longer be at front
    assert_ne!(pq.front().id, "root", "item should move down after priority decrease");
    assert_eq!(pq.front().id, "child1", "child1 should become new front");
}

/// Test: decrease_priority_size_unchanged
/// Spec: specifications/decrease_priority.md
/// Property: size() remains unchanged after priority update
#[test]
fn decrease_priority_size_unchanged() {
    let mut pq = new_item_min_heap(4);

    pq.insert(Item::new("a", 10));
    pq.insert(Item::new("b", 30));
    pq.insert(Item::new("c", 50));

    let size_before = pq.len();

    let updated = Item::new("a", 100);
    pq.decrease_priority(&updated);

    let size_after = pq.len();

    assert_eq!(size_after, size_before, "size should be unchanged after priority update");
}

/// Test: decrease_priority_edge_not_found_panics
/// Spec: specifications/decrease_priority.md
/// Property: panics if item not in heap (Rust implementation behavior)
#[test]
#[should_panic(expected = "item must exist")]
fn decrease_priority_edge_not_found_panics() {
    let mut pq = new_item_min_heap(4);

    pq.insert(Item::new("existing", 50));

    // This should panic - item not found
    let nonexistent = Item::new("nonexistent", 100);
    pq.decrease_priority(&nonexistent);
}


Provide a complete, working implementation that satisfies the documentation, matches the type signatures, and passes all tests.