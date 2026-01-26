![Rust Edition 2021](https://img.shields.io/badge/Rust-Edition_2021-orange.svg)
![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-green.svg)
![crates.io](https://img.shields.io/crates/v/d-ary-heap.svg)
![docs.rs](https://docs.rs/d-ary-heap/badge.svg)

# d-ary Heap Priority Queue (Rust) v2.5.0

**Wikipedia-standard d-ary heap implementation** with O(1) item lookup and configurable arity.

## Key Features

- **d-ary heap (not binary)**: Configurable arity `d` (number of children per node)
- **Min/Max flexibility**: Supports both min-heap and max-heap behavior via comparators
- **O(1) item lookup**: Internal hash map enables efficient priority updates
- **Efficient operations**:
  - O(1): `front()`, `peek()`, `len()`, `is_empty()`, `contains()`, `get_position()`
  - O(log_d n): `insert()`, `increase_priority()`
  - O(d × log_d n): `pop()`, `decrease_priority()`
  - O(n): `insert_many()` (Floyd's heapify), `to_array()`
- **Cross-language API**: Unified methods matching C++, Go, Zig, and TypeScript implementations
- **Rust-idiomatic**: Uses `Result<T, Error>` and `Option<T>` return types

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
d-ary-heap = "2.5.0"
```

## Quick Start

```rust
use d_ary_heap::{PriorityQueue, MinBy, MaxBy, Error};

#[derive(Clone, Eq, PartialEq, Hash)]
struct Task {
    id: u32,
    priority: u32,
}

fn main() -> Result<(), Error> {
    // Min-heap by priority (lower value = higher priority)
    let mut heap = PriorityQueue::new(4, MinBy(|t: &Task| t.priority))?;

    // Insert items
    heap.insert(Task { id: 1, priority: 10 });
    heap.insert(Task { id: 2, priority: 5 });

    // Get highest priority item
    assert_eq!(heap.front().priority, 5);

    // Update priority (make more important)
    heap.increase_priority(&Task { id: 1, priority: 1 })?;
    assert_eq!(heap.front().priority, 1);

    // Remove items in priority order
    while let Some(task) = heap.pop() {
        println!("Processing task {} with priority {}", task.id, task.priority);
    }

    Ok(())
}
```

## Usage Examples

### Basic Operations

```rust
use d_ary_heap::{PriorityQueue, MinBy};

let mut heap = PriorityQueue::new(3, MinBy(|x: &i32| *x)).unwrap();

// Insert items
heap.insert(10);
heap.insert(5);
heap.insert(15);

// Check properties
assert_eq!(heap.len(), 3);
assert!(!heap.is_empty());
assert_eq!(heap.d(), 3);

// Access highest priority (front panics if empty, peek returns Option)
assert_eq!(heap.front(), &5);
assert_eq!(heap.peek(), Some(&5));

// Remove items
assert_eq!(heap.pop(), Some(5));
assert_eq!(heap.pop(), Some(10));
assert_eq!(heap.pop(), Some(15));
assert_eq!(heap.pop(), None);
```

### Max-Heap Example

```rust
use d_ary_heap::{PriorityQueue, MaxBy};

let mut heap = PriorityQueue::new(2, MaxBy(|x: &i32| *x)).unwrap();

heap.insert(10);
heap.insert(5);
heap.insert(15);

// Max-heap: highest value has highest priority
assert_eq!(heap.front(), &15);
```

### Bulk Operations

```rust
use d_ary_heap::{PriorityQueue, MinBy};

let mut heap = PriorityQueue::new(3, MinBy(|x: &i32| *x)).unwrap();

// Insert many items at once (O(n) via Floyd's heapify)
heap.insert_many(vec![5, 3, 7, 1, 9]);
assert_eq!(heap.front(), &1);

// Pop multiple items at once
let items = heap.pop_many(3);
assert_eq!(items, vec![1, 3, 5]);

// Get heap contents as array
let remaining = heap.to_array();
assert_eq!(remaining.len(), 2);
```

### Priority Updates

```rust
use d_ary_heap::{PriorityQueue, MinBy, Error};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
struct Item { id: u32, cost: u32 }

// Important: Hash/Eq must be based on identity (id), not priority (cost)
impl PartialEq for Item { fn eq(&self, other: &Self) -> bool { self.id == other.id } }
impl Eq for Item {}
impl Hash for Item { fn hash<H: Hasher>(&self, state: &mut H) { self.id.hash(state) } }

let mut heap = PriorityQueue::new(3, MinBy(|x: &Item| x.cost)).unwrap();
heap.insert(Item { id: 1, cost: 10 });
heap.insert(Item { id: 2, cost: 5 });

// Increase priority (for min-heap: lower cost = higher priority)
heap.increase_priority(&Item { id: 1, cost: 1 }).unwrap();
assert_eq!(heap.front().id, 1);

// Decrease priority (for min-heap: higher cost = lower priority)
heap.decrease_priority(&Item { id: 1, cost: 20 }).unwrap();
assert_eq!(heap.front().id, 2);

// Update priority when direction is unknown
heap.update_priority(&Item { id: 2, cost: 3 }).unwrap();

// Check item position (O(1) lookup)
assert!(heap.get_position(&Item { id: 1, cost: 0 }).is_some());
```

### Error Handling

```rust
use d_ary_heap::{PriorityQueue, MinBy, Error};

// Invalid arity returns error
let result = PriorityQueue::new(0, MinBy(|x: &i32| *x));
assert_eq!(result, Err(Error::InvalidArity));

let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();
heap.insert(5);

// Item not found error
assert_eq!(heap.increase_priority(&99), Err(Error::ItemNotFound));

// Index out of bounds error
assert_eq!(heap.increase_priority_by_index(99), Err(Error::IndexOutOfBounds));

// Clear with invalid arity
assert_eq!(heap.clear(Some(0)), Err(Error::InvalidArity));
```

## API Reference

### Core Types

| Type | Description |
|------|-------------|
| `PriorityQueue<T, C>` | The main heap type |
| `MinBy<F>` | Comparator wrapper for min-heap behavior |
| `MaxBy<F>` | Comparator wrapper for max-heap behavior |
| `Position` | Type alias for position indices (`usize`) |
| `Error` | Error enum for fallible operations |

### Error Variants

| Variant | Description |
|---------|-------------|
| `Error::InvalidArity` | Arity (d) must be >= 1 |
| `Error::ItemNotFound` | Item not found in the priority queue |
| `Error::IndexOutOfBounds` | Index is out of bounds |
| `Error::EmptyQueue` | Operation requires a non-empty queue |

### Methods

| Method | Return | Complexity | Description |
|--------|--------|------------|-------------|
| `new(d, comparator)` | `Result<Self, Error>` | O(1) | Create new heap with arity d |
| `with_first(d, comparator, item)` | `Result<Self, Error>` | O(1) | Create heap with initial item |
| `len()` | `usize` | O(1) | Number of items |
| `is_empty()` | `bool` | O(1) | Check if empty |
| `d()` | `usize` | O(1) | Get arity |
| `contains(item)` | `bool` | O(1) | Check membership |
| `get_position(item)` | `Option<Position>` | O(1) | Get item's position index |
| `front()` | `&T` | O(1) | Highest priority item (panics if empty) |
| `peek()` | `Option<&T>` | O(1) | Highest priority item (safe) |
| `insert(item)` | `()` | O(log_d n) | Add new item |
| `insert_many(items)` | `()` | O(n) | Bulk insert via Floyd's heapify |
| `increase_priority(item)` | `Result<(), Error>` | O(log_d n) | Update to higher priority |
| `decrease_priority(item)` | `Result<(), Error>` | O(d·log_d n) | Update to lower priority |
| `update_priority(item)` | `Result<(), Error>` | O((d+1)·log_d n) | Update priority (any direction) |
| `increase_priority_by_index(i)` | `Result<(), Error>` | O(log_d n) | Increase priority at index |
| `decrease_priority_by_index(i)` | `Result<(), Error>` | O(d·log_d n) | Decrease priority at index |
| `update_priority_by_index(i)` | `Result<(), Error>` | O((d+1)·log_d n) | Update at index (any direction) |
| `pop()` | `Option<T>` | O(d·log_d n) | Remove highest priority item |
| `pop_many(count)` | `Vec<T>` | O(count·d·log_d n) | Remove multiple items |
| `to_array()` | `Vec<T>` | O(n) | Copy heap contents |
| `clear(new_d?)` | `Result<(), Error>` | O(1) | Remove all items |
| `to_string()` | `String` | O(n) | String representation |

### Traits

| Trait | Description |
|-------|-------------|
| `PriorityCompare<T>` | Define custom priority ordering |
| `Display` | String representation (`{item1, item2, ...}`) |

## Performance Considerations

### Choosing Optimal Arity (d)

| Arity | Use Case | Insert | Pop |
|-------|----------|--------|-----|
| d=2 | Mixed workloads | O(log n) | O(log n) |
| d=3-4 | Insert-heavy | O(log₃ n) | O(3·log₃ n) |
| d=8+ | Very insert-heavy | O(log₈ n) | O(8·log₈ n) |

**Recommendation**: Start with d=4 for most workloads.

### Optimization Tips

1. **Use bulk insert**: `insert_many()` is O(n) vs O(n log n) for individual inserts
2. **Choose d wisely**: Benchmark with your workload (d=4 often optimal)
3. **Use simple comparators**: Inline closures are faster than complex functions
4. **Stable identity**: Ensure Hash/Eq are based on stable identity, not priority

## Cross-Language Compatibility

This implementation provides API parity with:
- **C++**: `PriorityQueue<T>` in `Cpp/PriorityQueue.h`
- **Go**: `PriorityQueue[T]` in `Go/src/dheap.go`
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
cargo run
```

## License

Apache License 2.0 - See [LICENSE](../LICENSE) for details.
