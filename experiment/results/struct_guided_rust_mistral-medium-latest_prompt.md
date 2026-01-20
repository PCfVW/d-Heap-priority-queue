Implement a d-ary heap priority queue in Rust based on the following type definitions and signatures.

Requirements:
1. Items have two distinct properties: an identity (for equality) and a priority (for ordering)
2. Two items are equal if they have the same identity, regardless of priority
3. The queue should support O(1) lookup to check if an item exists
4. Implement a min-heap where lower priority values have higher importance

use std::collections::HashMap;
use std::hash::Hash;

/// Item represents an element in the priority queue.
/// The `id` field determines identity (equality via Hash + Eq).
/// The `priority` field determines ordering in the heap.
#[derive(Clone)]
pub struct Item<K> {
    pub id: K,
    pub priority: i32,
}

/// PriorityQueue is a d-ary min-heap with O(1) item lookup.
pub struct PriorityQueue<K: Hash + Eq + Clone> {
    d: usize,                          // arity (children per node)
    container: Vec<Item<K>>,           // heap array
    positions: HashMap<K, usize>,      // maps item id to index in container
}

impl<K: Hash + Eq + Clone> PriorityQueue<K> {
    /// Creates a new priority queue with the given arity d.
    /// Panics if d < 2.
    pub fn new(d: usize) -> Self;

    /// Adds an item to the queue.
    /// Returns Err if an item with the same id already exists.
    pub fn insert(&mut self, item: Item<K>) -> Result<(), &'static str>;

    /// Removes and returns the item with highest priority (lowest value).
    /// Returns None if empty.
    pub fn pop(&mut self) -> Option<Item<K>>;

    /// Returns a reference to the item with highest priority without removing it.
    /// Returns None if empty.
    pub fn front(&self) -> Option<&Item<K>>;

    /// Updates an item to have higher priority (lower value).
    /// The item parameter contains the id to find and the new priority.
    /// Returns Err if the item doesn't exist.
    pub fn increase_priority(&mut self, item: Item<K>) -> Result<(), &'static str>;

    /// Updates an item to have lower priority (higher value).
    /// The item parameter contains the id to find and the new priority.
    /// Returns Err if the item doesn't exist.
    pub fn decrease_priority(&mut self, item: Item<K>) -> Result<(), &'static str>;

    /// Returns true if an item with the given id exists.
    pub fn contains(&self, item: &Item<K>) -> bool;

    /// Returns the number of items in the queue.
    pub fn len(&self) -> usize;

    /// Returns true if the queue contains no items.
    pub fn is_empty(&self) -> bool;
}

Provide the complete implementation for all methods.