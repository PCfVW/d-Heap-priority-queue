![Rust Edition 2021](https://img.shields.io/badge/Rust-Edition_2021-orange.svg)
![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-green.svg)
![crates.io](https://img.shields.io/crates/v/d-ary-heap.svg)
![docs.rs](https://docs.rs/d-ary-heap/badge.svg)

# d-ary Heap Priority Queue (Rust) v2.3.0

**Wikipedia-standard d-ary heap implementation** with O(1) item lookup and configurable arity.

## Key Features

- **d-ary heap (not binary)**: Configurable arity `d` (number of children per node)
- **Min/Max flexibility**: Supports both min-heap and max-heap behavior via comparators
- **O(1) item lookup**: Internal hash map enables efficient priority updates
- **Efficient operations**:
  - O(1): `front()`, `peek()`, `len()`, `is_empty()`, `contains()`
  - O(log_d n): `insert()`, `increase_priority()`
  - O(d × log_d n): `pop()`, `decrease_priority()`
- **Cross-language API**: Unified methods matching C++, Zig, and TypeScript implementations
- **Rust-idiomatic**: Implements `Display` trait alongside `to_string()` for flexibility

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
d-ary-heap = "2.3.0"
```

## Quick Start

```rust
use d_ary_heap::{DHeap, MinBy, MaxBy};

#[derive(Clone, Eq, PartialEq, Hash)]
struct Task {
    id: u32,
    priority: u32,
}

// Min-heap by priority (lower value = higher priority)
let mut heap = DHeap::new(4, MinBy(|t: &Task| t.priority));

// Insert items
heap.insert(Task { id: 1, priority: 10 });
heap.insert(Task { id: 2, priority: 5 });

// Get highest priority item
let top = heap.front().unwrap();
assert_eq!(top.priority, 5);

// Update priority (make more important)
heap.increase_priority(&Task { id: 1, priority: 1 });
let top = heap.front().unwrap();
assert_eq!(top.priority, 1);

// Remove items in priority order
while let Some(task) = heap.pop() {
    println!("Processing task {} with priority {}", task.id, task.priority);
}
```

## Usage Examples

### Basic Operations

```rust
use d_ary_heap::{DHeap, MinBy};

let mut heap = DHeap::new(3, MinBy(|x: &i32| *x));

// Insert items
heap.insert(10);
heap.insert(5);
heap.insert(15);

// Check properties
assert_eq!(heap.len(), 3);
assert!(!heap.is_empty());
assert_eq!(heap.d(), 3);

// Access highest priority
assert_eq!(heap.front(), Some(&5));
assert_eq!(heap.peek(), Some(&5));

// Remove items
assert_eq!(heap.pop(), Some(5));
assert_eq!(heap.pop(), Some(10));
assert_eq!(heap.pop(), Some(15));
assert_eq!(heap.pop(), None);
```

### Max-Heap Example

```rust
use d_ary_heap::{DHeap, MaxBy};

let mut heap = DHeap::new(2, MaxBy(|x: &i32| *x));

heap.insert(10);
heap.insert(5);
heap.insert(15);

// Max-heap: highest value has highest priority
assert_eq!(heap.front(), Some(&15));
```

### Custom Comparators

```rust
use d_ary_heap::DHeap;

// Custom comparator for complex types
let heap = DHeap::new(4, |a: &str, b: &str| a.len() < b.len());

heap.insert("short");
heap.insert("medium length");
heap.insert("longest string here");

// Shortest string has highest priority
assert_eq!(heap.front(), Some(&"short"));
```

## API Reference

### Core Types

- `DHeap<T, C>`: The main heap type
- `MinBy<F>`: Comparator wrapper for min-heap behavior
- `MaxBy<F>`: Comparator wrapper for max-heap behavior
- `Position`: Type alias for position indices (cross-language consistency)

### Methods

| Method | Complexity | Description |
|--------|------------|-------------|
| `new(d, comparator)` | O(1) | Create new heap with arity d |
| `len()` | O(1) | Number of items |
| `is_empty()` | O(1) | Check if empty |
| `d()` | O(1) | Get arity |
| `contains(item)` | O(1) | Check membership |
| `front()` | O(1) | Highest priority item (None if empty) |
| `peek()` | O(1) | Alias for front() |
| `insert(item)` | O(log_d n) | Add new item |
| `increase_priority(item)` | O(log_d n) | Update to higher priority |
| `decrease_priority(item)` | O(d·log_d n) | Update to lower priority |
| `pop()` | O(d·log_d n) | Remove highest priority item |
| `clear()` | O(1) | Remove all items |
| `to_string()` | O(n) | String representation |

## Performance Considerations

### Choosing Optimal Arity (d)

| Arity | Use Case | Insert | Pop |
|-------|----------|--------|-----|
| d=2 | Mixed workloads | O(log n) | O(log n) |
| d=3-4 | Insert-heavy | O(log₃ n) | O(3·log₃ n) |
| d=8+ | Very insert-heavy | O(log₈ n) | O(8·log₈ n) |

**Recommendation**: Start with d=4 for most workloads.

### Optimization Tips

1. **Pre-size when possible**: If you know approximate size, create with capacity
2. **Choose d wisely**: Benchmark with your workload (d=4 often optimal)
3. **Use simple comparators**: Inline closures are faster than complex functions
4. **Stable identity**: Ensure Hash/Eq are based on stable identity, not priority

## Cross-Language Compatibility

This implementation provides API parity with:
- **C++**: `PriorityQueue<T>` in `Cpp/PriorityQueue.h`
- **Zig**: `DHeap(T)` in `zig/src/d_heap.zig`
- **TypeScript**: `PriorityQueue<T>` in `TypeScript/src/PriorityQueue.ts`

All implementations share:
- Identical time complexities
- Unified method names (with language-appropriate casing)
- Cross-language API consistency

## What is a d-ary Heap?

A [d-ary heap](https://en.wikipedia.org/wiki/D-ary_heap) is a tree structure where:

- Each node has **up to d children** (configurable arity)
- The **root** contains the highest-priority item
- **Children are unordered** (unlike binary heaps)
- **Heap property**: Each parent has higher priority than all children
- **Complete tree**: Filled left-to-right, level by level

### Advantages over Binary Heaps (d=2)

- **Shallower tree**: Height is log_d(n) instead of log₂(n)
- **Faster inserts**: O(log_d n) vs O(log₂ n)
- **Configurable**: Optimize for your specific workload

### Trade-offs

- **Slower pops**: O(d·log_d n) vs O(log₂ n)
- **More comparisons**: Each pop examines up to d children
- **Memory**: Slightly higher overhead for position tracking

## Reference Implementation

This implementation follows the d-ary heap specification from:

- **Wikipedia**: [D-ary heap](https://en.wikipedia.org/wiki/D-ary_heap)
- **Network Flows textbook**: Section A.3, pp. 773–778
  - Ravindra Ahuja, Thomas Magnanti & James Orlin
  - Prentice Hall (1993)
  - [Book information](https://mitmgmtfaculty.mit.edu/jorlin/network-flows/)

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_min_heap_ordering

# Run demo
cargo run --bin demo
```

## License

Apache License 2.0 - See [LICENSE](../LICENSE) for details.
