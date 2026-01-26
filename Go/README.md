![Go Version](https://img.shields.io/badge/Go-1.21+-00ADD8.svg)
![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-green.svg)
![pkg.go.dev](https://pkg.go.dev/badge/github.com/PCfVW/d-Heap-priority-queue/Go.svg)

# d-ary Heap Priority Queue (Go) v2.5.0

**Wikipedia-standard d-ary heap implementation** with O(1) item lookup and configurable arity.

## Key Features

- **d-ary heap (not binary)**: Configurable arity `d` (number of children per node)
- **Min/Max flexibility**: Supports both min-heap and max-heap behavior via comparators
- **O(1) item lookup**: Internal map enables efficient priority updates
- **Efficient operations**:
  - O(1): `Front()`, `Peek()`, `Len()`, `IsEmpty()`, `Contains()`
  - O(log_d n): `Insert()`, `IncreasePriority()`
  - O(d × log_d n): `Pop()`, `DecreasePriority()`
- **Cross-language API**: Unified methods matching C++, Rust, Zig, and TypeScript implementations
- **Go-idiomatic**: Implements `fmt.Stringer` interface, uses generics (Go 1.21+)

## Installation

```bash
go get github.com/PCfVW/d-Heap-priority-queue/Go/src
```

## Quick Start

```go
package main

import (
    "fmt"
    dheap "github.com/PCfVW/d-Heap-priority-queue/Go/src"
)

type Task struct {
    ID       string
    Priority int
}

func main() {
    // Min-heap by priority (lower value = higher priority)
    pq := dheap.New(dheap.Options[Task, string]{
        D:            4,
        Comparator:   dheap.MinBy(func(t Task) int { return t.Priority }),
        KeyExtractor: func(t Task) string { return t.ID },
    })

    // Insert items
    pq.Insert(Task{ID: "task1", Priority: 10})
    pq.Insert(Task{ID: "task2", Priority: 5})

    // Get highest priority item
    top, _ := pq.Front()
    fmt.Printf("Top: %s (priority %d)\n", top.ID, top.Priority) // task2, priority 5

    // Update priority (make more important)
    pq.IncreasePriority(Task{ID: "task1", Priority: 1})
    top, _ = pq.Front()
    fmt.Printf("Top: %s (priority %d)\n", top.ID, top.Priority) // task1, priority 1

    // Remove items in priority order
    for !pq.IsEmpty() {
        task, _ := pq.Pop()
        fmt.Printf("Processing %s with priority %d\n", task.ID, task.Priority)
    }
}
```

## Usage Examples

### Basic Operations

```go
import dheap "github.com/PCfVW/d-Heap-priority-queue/Go/src"

pq := dheap.New(dheap.Options[int, int]{
    D:            3,
    Comparator:   dheap.MinNumber,
    KeyExtractor: func(x int) int { return x },
})

// Insert items
pq.Insert(10)
pq.Insert(5)
pq.Insert(15)

// Check properties
fmt.Println(pq.Len())      // 3
fmt.Println(pq.IsEmpty())  // false
fmt.Println(pq.D())        // 3

// Access highest priority
front, _ := pq.Front()     // 5
peek, ok := pq.Peek()      // 5, true

// Remove items
val, ok := pq.Pop()        // 5, true
val, ok = pq.Pop()         // 10, true
val, ok = pq.Pop()         // 15, true
val, ok = pq.Pop()         // 0, false (empty)
```

### Max-Heap Example

```go
pq := dheap.New(dheap.Options[int, int]{
    D:            2,
    Comparator:   dheap.MaxNumber,
    KeyExtractor: func(x int) int { return x },
})

pq.Insert(10)
pq.Insert(5)
pq.Insert(15)

// Max-heap: highest value has highest priority
front, _ := pq.Front() // 15
```

### Custom Comparators

```go
// Using MinBy for custom types
type Task struct {
    ID    string
    Score float64
}

pq := dheap.New(dheap.Options[Task, string]{
    D:            4,
    Comparator:   dheap.MinBy(func(t Task) float64 { return t.Score }),
    KeyExtractor: func(t Task) string { return t.ID },
})

// Using Reverse to flip priority
maxByScore := dheap.Reverse(dheap.MinBy(func(t Task) float64 { return t.Score }))

// Using Chain for multi-key comparison
cmp := dheap.Chain(
    dheap.MinBy(func(t Task) int { return t.Priority }),
    dheap.MinBy(func(t Task) int64 { return t.Timestamp }),
)
```

### Bulk Operations

```go
pq := dheap.New(dheap.Options[int, int]{
    D:            4,
    Comparator:   dheap.MinNumber,
    KeyExtractor: func(x int) int { return x },
})

// InsertMany uses O(n) heapify vs O(n log n) for individual inserts
pq.InsertMany([]int{5, 3, 7, 1, 9, 2})

// PopMany removes multiple items efficiently
items := pq.PopMany(3) // [1, 2, 3]
```

## API Reference

### Core Types

- `PriorityQueue[T, K]`: The main heap type (T = item type, K = key type)
- `Comparator[T]`: Function type `func(a, b T) bool` returning true if a has higher priority
- `KeyExtractor[T, K]`: Function type `func(item T) K` for identity extraction
- `Position`: Type alias for `int` (cross-language consistency)

### Constructor Functions

| Function | Description |
|----------|-------------|
| `New(opts)` | Create new heap with options |
| `WithFirst(opts, item)` | Create heap with initial item |

### Methods

| Method | Complexity | Description |
|--------|------------|-------------|
| `Len()` | O(1) | Number of items |
| `IsEmpty()` | O(1) | Check if empty |
| `D()` | O(1) | Get arity |
| `Contains(item)` | O(1) | Check membership by item |
| `ContainsKey(key)` | O(1) | Check membership by key |
| `Front()` | O(1) | Highest priority item (error if empty) |
| `Peek()` | O(1) | Highest priority item (ok bool) |
| `GetPosition(item)` | O(1) | Get heap index of item |
| `GetPositionByKey(key)` | O(1) | Get heap index by key |
| `Insert(item)` | O(log_d n) | Add new item |
| `InsertMany(items)` | O(n) | Bulk insert with heapify |
| `IncreasePriority(item)` | O(log_d n) | Update to higher priority (moveUp only) |
| `IncreasePriorityByIndex(index)` | O(log_d n) | Update by index (moveUp only) |
| `DecreasePriority(item)` | O(d·log_d n) | Update to lower priority (moveDown only) |
| `DecreasePriorityByIndex(index)` | O(d·log_d n) | Update by index (moveDown only) |
| `UpdatePriority(item)` | O((d+1)·log_d n) | Update when direction unknown (both) |
| `Pop()` | O(d·log_d n) | Remove highest priority item |
| `PopMany(count)` | O(count·d·log_d n) | Remove multiple items |
| `Clear(newD...)` | O(1) | Remove all items |
| `ToArray()` | O(n) | Copy of internal array |
| `String()` | O(n) | String representation |

### Snake_case Aliases

For cross-language consistency, these aliases are provided:
- `Is_empty()` → `IsEmpty()`
- `Increase_priority(item)` → `IncreasePriority(item)`
- `Increase_priority_by_index(index)` → `IncreasePriorityByIndex(index)`
- `Decrease_priority(item)` → `DecreasePriority(item)`
- `Decrease_priority_by_index(index)` → `DecreasePriorityByIndex(index)`
- `Update_priority(item)` → `UpdatePriority(item)`
- `To_string()` → `String()`

### Pre-built Comparators

| Comparator | Description |
|------------|-------------|
| `MinNumber` | Min-heap for `int` |
| `MaxNumber` | Max-heap for `int` |
| `MinFloat` | Min-heap for `float64` |
| `MaxFloat` | Max-heap for `float64` |
| `MinString` | Min-heap for `string` |
| `MaxString` | Max-heap for `string` |

### Comparator Factories

| Function | Description |
|----------|-------------|
| `MinBy(keyFn)` | Create min-heap comparator from key extractor |
| `MaxBy(keyFn)` | Create max-heap comparator from key extractor |
| `Reverse(cmp)` | Reverse a comparator (min↔max) |
| `Chain(cmps...)` | Compare by multiple keys in order |

## Priority Update Semantics

This library uses **importance-based** semantics:

- **`IncreasePriority()`**: Make an item **more important** (moves toward heap root). Only moves up for O(log_d n) performance.
- **`DecreasePriority()`**: Make an item **less important** (moves toward leaves). Only moves down for O(d·log_d n) performance.
- **`UpdatePriority()`**: Update when direction is **unknown**. Checks both directions for O((d+1)·log_d n) performance.

**When to use each:**
- Use `IncreasePriority()` when you know the item became more important
- Use `DecreasePriority()` when you know the item became less important
- Use `UpdatePriority()` when you don't know which direction the priority changed

**Heap Context:**
- **Min-heap**: Lower priority values = higher importance
- **Max-heap**: Higher priority values = higher importance

## Performance Considerations

### Choosing Optimal Arity (d)

| Arity | Use Case | Insert | Pop |
|-------|----------|--------|-----|
| d=2 | Mixed workloads | O(log n) | O(log n) |
| d=3-4 | Insert-heavy | O(log₃ n) | O(3·log₃ n) |
| d=8+ | Very insert-heavy | O(log₈ n) | O(8·log₈ n) |

**Recommendation**: Start with d=4 for most workloads.

## Cross-Language Compatibility

This implementation provides API parity with:
- **C++**: `PriorityQueue<T>` in `Cpp/PriorityQueue.h`
- **Rust**: `d_ary_heap::PriorityQueue` in `Rust/src/lib.rs`
- **Zig**: `DHeap(T)` in `zig/src/dheap.zig`
- **TypeScript**: `PriorityQueue<T>` in `TypeScript/src/PriorityQueue.ts`

All implementations share identical time complexities and method semantics.

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
cd Go && go test ./src/...

# Run with verbose output
cd Go && go test -v ./src/...

# Run specific test
cd Go && go test -v ./src/... -run TestMinHeap

# Run benchmarks
cd Go && go test -bench=. ./src/...
```

## License

Apache License 2.0 - See [LICENSE](https://github.com/PCfVW/d-Heap-priority-queue/blob/master/LICENSE) for details.
