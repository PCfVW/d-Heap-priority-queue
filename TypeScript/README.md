![TypeScript](https://img.shields.io/badge/TypeScript-5.3-blue.svg)
![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-green.svg)

# d-Heap Priority Queue (TypeScript) v2.3.0

A high-performance, generic d-ary heap priority queue with O(1) item lookup, supporting both min-heap and max-heap behavior.

## Strengths

- **Flexible behavior**: min-heap or max-heap via comparator functions, and configurable arity `d` at construction time.
- **Efficient operations** on n items:
  - O(1): access the highest-priority item (`front()`, `peek()`).
  - O(log_d n): `insert()` and upward reheapification.
  - O(d · log_d n): delete-top (`pop()`), with up to d children examined per level.
- **O(1) item lookup**: internal Map tracks positions by item key, enabling efficient priority updates.
- **Practical API**: `insert`, `front`, `peek`, `pop`, `increasePriority`, `decreasePriority`, `isEmpty`, `len`, `contains`.
- **Unified API**: Cross-language standardized methods matching C++, Rust, and Zig implementations.
- **TypeScript-native**: Full type safety, generics, and IDE support.
- **Zero dependencies**: No runtime dependencies.

## Installation

```bash
npm install d-ary-heap
```

## Quick Start

```typescript
import { PriorityQueue, minBy, maxBy } from 'd-ary-heap';

// Define your item type
interface Task {
  id: number;
  priority: number;
  name: string;
}

// Create a min-heap (lower priority value = higher importance)
const pq = new PriorityQueue<Task, number>({
  d: 4,                                    // 4-ary heap
  comparator: minBy(task => task.priority), // Min-heap by priority
  keyExtractor: task => task.id,           // Identity by id
});

// Insert items
pq.insert({ id: 1, priority: 10, name: 'Low priority task' });
pq.insert({ id: 2, priority: 1, name: 'High priority task' });
pq.insert({ id: 3, priority: 5, name: 'Medium priority task' });

// Get highest priority item (lowest priority value in min-heap)
console.log(pq.front()); // { id: 2, priority: 1, name: 'High priority task' }

// Update priority of existing item
pq.increasePriority({ id: 1, priority: 0, name: 'Now urgent!' });
console.log(pq.front()); // { id: 1, priority: 0, name: 'Now urgent!' }

// Remove items in priority order
while (!pq.isEmpty()) {
  console.log(pq.pop());
}
```

## API Reference

### Constructor

```typescript
new PriorityQueue<T, K>(options: PriorityQueueOptions<T, K>)
```

Options:
- `d`: Number of children per node (arity). Default: 2. Must be >= 1.
- `comparator`: Function `(a: T, b: T) => boolean` returning true if `a` has higher priority.
- `keyExtractor`: Function `(item: T) => K` extracting the identity key from an item.
- `initialCapacity`: Optional hint for pre-allocation.

### Static Factory Methods

| Method | Description |
|--------|-------------|
| `PriorityQueue.withFirst(options, item)` | Create queue with initial item |

### Query Methods

| Method | Description | Time |
|--------|-------------|------|
| `len()` | Number of items | O(1) |
| `size` | Property alias for `len()` | O(1) |
| `isEmpty()` | Check if empty (primary method) | O(1) |
| `is_empty()` | Alias for `isEmpty()` (cross-language compatibility) | O(1) |
| `d()` | Get arity | O(1) |
| `contains(item)` | Check if item exists (by key) | O(1) |
| `containsKey(key)` | Check if key exists | O(1) |
| `getPosition(item)` | Get item's index in heap | O(1) |
| `getPositionByKey(key)` | Get index by key | O(1) |
| `front()` | Get highest-priority item (throws if empty) | O(1) |
| `peek()` | Get highest-priority item (returns undefined if empty) | O(1) |

### Modification Methods

| Method | Description | Time |
|--------|-------------|------|
| `insert(item)` | Add new item | O(log_d n) |
| `pop()` | Remove and return highest-priority item | O(d · log_d n) |
| `increasePriority(item)` | Update item to higher priority (primary method) | O(log_d n) |
| `increase_priority(item)` | Alias for `increasePriority()` (cross-language compatibility) | O(log_d n) |
| `increasePriorityByIndex(i)` | Update by index (primary method) | O(log_d n) |
| `increase_priority_by_index(i)` | Alias for `increasePriorityByIndex()` (cross-language compatibility) | O(log_d n) |
| `decreasePriority(item)` | Update item to lower priority (primary method) | O(d · log_d n) |
| `decrease_priority(item)` | Alias for `decreasePriority()` (cross-language compatibility) | O(d · log_d n) |
| `clear(newD?)` | Remove all items, optionally change arity | O(1) |

### Utility Methods

| Method | Description |
|--------|-------------|
| `toString()` | String representation (primary method) |
| `to_string()` | Alias for `toString()` (cross-language compatibility) |
| `toArray()` | Copy of internal array |
| `[Symbol.iterator]()` | Iterate over items in heap order |

### Method Naming Convention

This TypeScript implementation follows **camelCase** as the primary naming convention (TypeScript/JavaScript standard), with **snake_case aliases** provided for cross-language compatibility:

- **Primary methods**: `isEmpty()`, `increasePriority()`, `decreasePriority()`, `toString()`
- **Compatibility aliases**: `is_empty()`, `increase_priority()`, `decrease_priority()`, `to_string()`

Use the primary camelCase methods for TypeScript/JavaScript projects, and the snake_case aliases when porting code from C++/Rust implementations.

## Comparator Helpers

```typescript
import { minBy, maxBy, minNumber, maxNumber, reverse, chain } from 'd-ary-heap';

// Min-heap by extracted key
const minByCost = minBy<Item, number>(item => item.cost);

// Max-heap by extracted key
const maxByCost = maxBy<Item, number>(item => item.cost);

// For primitive numbers
const minHeap = minNumber;  // (a, b) => a < b
const maxHeap = maxNumber;  // (a, b) => a > b

// Reverse any comparator
const reversed = reverse(minByCost);

// Chain comparators (tiebreaker)
const byPriorityThenTime = chain(
  minBy<Task, number>(t => t.priority),
  minBy<Task, number>(t => t.timestamp)
);
```

## Priority Update Semantics

The `increasePriority()` and `decreasePriority()` methods have an intentionally **asymmetric design**:

- **`increasePriority()`**: Make an item **more important** (moves toward heap root). Only moves up for O(log_d n) performance.
- **`decreasePriority()`**: Make an item **less important** (moves toward leaves). Checks both directions for robustness.

**Heap Context:**
- **Min-heap**: Lower priority values = higher importance
- **Max-heap**: Higher priority values = higher importance

## Performance

### Benchmark Results (Node.js v20, typical hardware)

| Operation | d=2 | d=4 | d=8 |
|-----------|-----|-----|-----|
| 100k inserts | ~21ms | ~13ms | ~11ms |
| 100k pops | ~187ms | ~136ms | ~140ms |
| Throughput (insert) | ~2.2M ops/sec | | |
| Throughput (pop+insert) | ~900k ops/sec | | |

### Performance Tips

1. **Choose arity wisely**: d=4 is often optimal, balancing tree height vs comparison work per level.

2. **Use inline comparators**: `(a, b) => a.cost < b.cost` is faster than `minBy(x => x.cost)` in hot paths.

3. **Pre-allocate**: Use `initialCapacity` if you know the approximate size.

4. **Simple key extractors**: Keep `keyExtractor` simple—`(item) => item.id` is ideal.

5. **Avoid unnecessary updates**: Check if priority actually changed before calling update methods.

### Running Benchmarks

```bash
# Full benchmark suite
npx tsx benchmarks/run-benchmarks.ts

# Compare with naive implementation
npx tsx benchmarks/compare-implementations.ts
```

## What is a d-Heap?

A [d-ary heap](https://en.wikipedia.org/wiki/D-ary_heap) is a tree where:
- Each node has up to d children
- The root holds the highest priority
- Children are unordered
- Priorities decrease along any root-to-leaf path

Time complexities over n items:
- O(1): access top
- O(d · log_d n): delete-top
- O(log_d n): insert and upward update

## Development

```bash
# Install dependencies
npm install

# Run tests
npm test

# Build
npm run build

# Type check
npm run typecheck
```

## Reference

Section A.3, [d-Heaps](https://en.wikipedia.org/wiki/D-ary_heap), pp. 773–778 of Ravindra Ahuja, Thomas Magnanti & James Orlin, **Network Flows** (Prentice Hall, 1993).
