# Condition 3: Struct-Guided Prompt (Type Signature Assistance)

## Purpose

This prompt provides the core requirements plus complete type definitions and function signatures. This tests whether compiler-enforceable structure (Level 1) helps guide correct implementation.

---

## Prompt Template

Each language has its own type stub format below.

---

## Go

```
Implement a d-ary heap priority queue in Go based on the following type definitions and signatures.

Requirements:
1. Items have two distinct properties: an identity (for equality) and a priority (for ordering)
2. Two items are equal if they have the same identity, regardless of priority
3. The queue should support O(1) lookup to check if an item exists
4. Implement a min-heap where lower priority values have higher importance

// Item represents an element in the priority queue.
// The ID field determines identity (equality).
// The Priority field determines ordering in the heap.
type Item[K comparable] struct {
    ID       K
    Priority int
}

// PriorityQueue is a d-ary min-heap with O(1) item lookup.
type PriorityQueue[K comparable] struct {
    d         int                // arity (children per node)
    container []Item[K]          // heap array
    positions map[K]int          // maps item ID to index in container
}

// New creates a new priority queue with the given arity d.
// Panics if d < 2.
func New[K comparable](d int) *PriorityQueue[K]

// Insert adds an item to the queue.
// Returns an error if an item with the same ID already exists.
func (pq *PriorityQueue[K]) Insert(item Item[K]) error

// Pop removes and returns the item with highest priority (lowest value).
// Returns the item and true, or zero value and false if empty.
func (pq *PriorityQueue[K]) Pop() (Item[K], bool)

// Front returns the item with highest priority without removing it.
// Returns the item and true, or zero value and false if empty.
func (pq *PriorityQueue[K]) Front() (Item[K], bool)

// IncreasePriority updates an item to have higher priority (lower value).
// The item parameter contains the ID to find and the new priority.
// Returns an error if the item doesn't exist.
func (pq *PriorityQueue[K]) IncreasePriority(item Item[K]) error

// DecreasePriority updates an item to have lower priority (higher value).
// The item parameter contains the ID to find and the new priority.
// Returns an error if the item doesn't exist.
func (pq *PriorityQueue[K]) DecreasePriority(item Item[K]) error

// Contains returns true if an item with the given ID exists.
func (pq *PriorityQueue[K]) Contains(item Item[K]) bool

// Len returns the number of items in the queue.
func (pq *PriorityQueue[K]) Len() int

// IsEmpty returns true if the queue contains no items.
func (pq *PriorityQueue[K]) IsEmpty() bool

Provide the complete implementation for all methods.
```

---

## Rust

```
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
```

---

## C++

```
Implement a d-ary heap priority queue in C++17 based on the following type definitions and signatures.

Requirements:
1. Items have two distinct properties: an identity (for equality) and a priority (for ordering)
2. Two items are equal if they have the same identity, regardless of priority
3. The queue should support O(1) lookup to check if an item exists
4. Implement a min-heap where lower priority values have higher importance

#include <vector>
#include <unordered_map>
#include <optional>
#include <functional>

/// Item represents an element in the priority queue.
/// The `id` field determines identity (equality via Hash and Equal).
/// The `priority` field determines ordering in the heap.
template<typename K>
struct Item {
    K id;
    int priority;
};

/// PriorityQueue is a d-ary min-heap with O(1) item lookup.
/// Hash: functor to hash the item's id
/// Equal: functor to compare items by id for equality
template<typename K, typename Hash = std::hash<K>, typename Equal = std::equal_to<K>>
class PriorityQueue {
private:
    size_t d_;                                              // arity
    std::vector<Item<K>> container_;                        // heap array
    std::unordered_map<K, size_t, Hash, Equal> positions_;  // id -> index

public:
    /// Creates a new priority queue with the given arity d.
    /// Throws if d < 2.
    explicit PriorityQueue(size_t d);

    /// Adds an item to the queue.
    /// Throws if an item with the same id already exists.
    void insert(const Item<K>& item);

    /// Removes and returns the item with highest priority (lowest value).
    /// Returns std::nullopt if empty.
    std::optional<Item<K>> pop();

    /// Returns the item with highest priority without removing it.
    /// Throws if empty.
    const Item<K>& front() const;

    /// Updates an item to have higher priority (lower value).
    /// The item parameter contains the id to find and the new priority.
    /// Throws if the item doesn't exist.
    void increase_priority(const Item<K>& item);

    /// Updates an item to have lower priority (higher value).
    /// The item parameter contains the id to find and the new priority.
    /// Throws if the item doesn't exist.
    void decrease_priority(const Item<K>& item);

    /// Returns true if an item with the given id exists.
    bool contains(const Item<K>& item) const;

    /// Returns the number of items in the queue.
    size_t len() const;

    /// Returns true if the queue contains no items.
    bool is_empty() const;
};

Provide the complete implementation for all methods as a header-only template.
```

---

## TypeScript

```
Implement a d-ary heap priority queue in TypeScript based on the following type definitions and signatures.

Requirements:
1. Items have two distinct properties: an identity (for equality) and a priority (for ordering)
2. Two items are equal if they have the same identity, regardless of priority
3. The queue should support O(1) lookup to check if an item exists
4. Implement a min-heap where lower priority values have higher importance

/** Item represents an element in the priority queue. */
interface Item<K> {
    /** Identity field - determines equality */
    id: K;
    /** Priority field - determines heap ordering (lower = higher priority) */
    priority: number;
}

/** Function to extract the identity key from an item */
type KeyFn<T, K> = (item: T) => K;

/** Function to extract the priority from an item */
type PriorityFn<T> = (item: T) => number;

/** Comparator function for heap ordering */
type Comparator<T> = (a: T, b: T) => boolean;

/**
 * PriorityQueue is a d-ary min-heap with O(1) item lookup.
 * @typeParam T - The item type stored in the queue
 * @typeParam K - The key type used for identity (must be usable as Map key)
 */
class PriorityQueue<T, K = string | number> {
    private d: number;                    // arity
    private container: T[];               // heap array
    private positions: Map<K, number>;    // id -> index
    private keyFn: KeyFn<T, K>;           // extracts identity from item
    private priorityFn: PriorityFn<T>;    // extracts priority from item
    private comparator: Comparator<T>;    // returns true if a has higher priority than b

    /**
     * Creates a new priority queue.
     * @param d - Arity (children per node), must be >= 2
     * @param keyFn - Function to extract identity key from items
     * @param priorityFn - Function to extract priority from items
     * @param comparator - Returns true if first arg has higher priority
     */
    constructor(
        d: number,
        keyFn: KeyFn<T, K>,
        priorityFn: PriorityFn<T>,
        comparator: Comparator<T>
    );

    /** Adds an item to the queue. Throws if item with same key exists. */
    insert(item: T): void;

    /** Removes and returns the highest priority item. Returns undefined if empty. */
    pop(): T | undefined;

    /** Returns the highest priority item without removing it. Returns undefined if empty. */
    front(): T | undefined;

    /** Updates an item to have higher priority (lower value). Throws if not found. */
    increasePriority(item: T): void;

    /** Updates an item to have lower priority (higher value). Throws if not found. */
    decreasePriority(item: T): void;

    /** Returns true if an item with the given key exists. */
    contains(item: T): boolean;

    /** Returns the number of items in the queue. */
    len(): number;

    /** Returns true if the queue is empty. */
    isEmpty(): boolean;
}

Provide the complete implementation for all methods.
```

---

## Zig

```
Implement a d-ary heap priority queue in Zig based on the following type definitions and signatures.

Requirements:
1. Items have two distinct properties: an identity (for equality) and a priority (for ordering)
2. Two items are equal if they have the same identity, regardless of priority
3. The queue should support O(1) lookup to check if an item exists
4. Implement a min-heap where lower priority values have higher importance

const std = @import("std");

/// Item represents an element in the priority queue.
/// The `number` field determines identity (equality).
/// The `cost` field determines ordering in the heap (lower = higher priority).
pub const Item = struct {
    number: u32,
    cost: u32,

    pub fn hash(self: Item) u64 {
        var hasher = std.hash.Wyhash.init(0);
        std.hash.autoHash(&hasher, self.number);
        return hasher.final();
    }

    pub fn eq(a: Item, b: Item) bool {
        return a.number == b.number;
    }
};

/// Comparator that returns true if a has higher priority (lower cost) than b.
pub fn MinByCost(a: Item, b: Item) bool {
    return a.cost < b.cost;
}

/// DHeap is a d-ary min-heap with O(1) item lookup.
pub fn DHeap(comptime compareFn: fn (Item, Item) bool) type {
    return struct {
        const Self = @This();

        d: usize,                                                    // arity
        container: std.ArrayList(Item),                              // heap array
        positions: std.HashMap(Item, usize, ItemContext, 80),        // item -> index
        allocator: std.mem.Allocator,

        const ItemContext = struct {
            pub fn hash(self: @This(), item: Item) u64 {
                _ = self;
                return item.hash();
            }
            pub fn eql(self: @This(), a: Item, b: Item) bool {
                _ = self;
                return Item.eq(a, b);
            }
        };

        /// Creates a new priority queue with the given arity d.
        /// The comparator is provided via the comptime parameter to DHeap.
        pub fn init(d: usize, allocator: std.mem.Allocator) !Self;

        /// Frees all resources.
        pub fn deinit(self: *Self) void;

        /// Adds an item to the queue.
        /// Returns error if item with same identity already exists.
        pub fn insert(self: *Self, item: Item) !void;

        /// Removes and returns the highest priority item.
        /// Returns null if empty.
        pub fn pop(self: *Self) !?Item;

        /// Returns the highest priority item without removing it.
        /// Returns null if empty.
        pub fn front(self: *const Self) ?Item;

        /// Updates an item to have higher priority (lower cost).
        /// Returns error if item doesn't exist.
        pub fn increasePriority(self: *Self, item: Item) !void;

        /// Updates an item to have lower priority (higher cost).
        /// Returns error if item doesn't exist.
        pub fn decreasePriority(self: *Self, item: Item) !void;

        /// Returns true if an item with the given identity exists.
        pub fn contains(self: *const Self, item: Item) bool;

        /// Returns the number of items in the queue.
        pub fn len(self: *const Self) usize;

        /// Returns true if the queue is empty.
        pub fn isEmpty(self: *const Self) bool;
    };
}

Provide the complete implementation for all methods.
```

---

## Notes

- This prompt adds:
  - Complete type/struct definitions
  - Full function signatures with parameter and return types
  - Field names and their purposes
  - Generic type parameters where applicable
  - Internal data structure hints (container array, positions map)

- This tests whether type structure (Level 1: compiler-enforced constraints) guides the model toward correct implementation, even without behavioral documentation.

## Important: API Design Choice

The type stubs in this file represent a **simplified, idealized API** designed for this experiment. They intentionally differ from the reference implementations' actual APIs, which use more sophisticated patterns (Options structs, comparator types, etc.).

For Condition 3 (Struct-guided alone), the generated code will be tested against a **simplified test harness** that matches these type signatures.

For Condition 5 (Combined), the type stubs must be adapted to match the actual test corpus APIs. See the Assembly Instructions in combined.md.
