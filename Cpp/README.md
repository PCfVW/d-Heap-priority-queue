![C++23](https://img.shields.io/badge/C%2B%2B-23-blue.svg)
![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-green.svg)

# d-Heap Priority Queue (C++23) v2.6.0

**Wikipedia-standard d-ary heap (C++23)** with configurable arity, O(1) item lookup, modern `std::expected` error handling, and zero-cost comparison-count instrumentation. Cross-language API parity with Go, Rust, TypeScript, and Zig.

## 🎬 See it in action

**[Interactive demo →](https://pcfvw.github.io/d-Heap-priority-queue/)**

Watch the heap tree update as items are inserted, popped, and re-prioritized. Toggle arity (d=2 / d=4 / d=8) to see how tree depth changes. Step through Dijkstra's algorithm on a weighted graph, or run **race mode** to compare all three arities side-by-side. Built with React Flow; runs in the browser, no install.

## Why this crate?

If you already know you want a priority queue, here's what this header-only library gives you over `std::priority_queue` and Boost.Heap:

- **Configurable arity `d`** (not just d=2). Pick `d=4` for cache-friendlier inserts, `d=8` for very insert-heavy workloads.
- **O(1) item lookup + priority updates.** `increase_priority(item)`, `decrease_priority(item)`, `update_priority(item)` — the operations Dijkstra and A\* actually need. `std::priority_queue` exposes none of these (you'd build a parallel index manually).
- **Zero-cost comparison-count instrumentation** (new in v2.6.0). Template-policy class with `[[no_unique_address]]` zero-bytes elision; `sizeof(PriorityQueue<int>)` unchanged when the default `NoOpStats` is selected, `static_assert`-checked.
- **C++23 `std::expected<T, Error>`** for fallible operations alongside assertion-based variants for hot paths.
- **Cross-language API parity** with Rust, Go, TypeScript, and Zig — byte-for-byte identical comparison counts on shared benchmarks (verified across 24 cells × 5 languages).
- **Published numbers**: see [`benchmarks/`](https://github.com/PCfVW/d-Heap-priority-queue/tree/master/benchmarks). C++ runs `huge_dense × d=8` at **15.1 ms / ~199 ns/cmp** on AMD Ryzen 9 5950X (MSVC 19.44, Release `/O2 /utf-8`). Full [methodology](https://github.com/PCfVW/d-Heap-priority-queue/blob/master/benchmarks/methodology.md).

## What's New in v2.6.0

- **Comparison-count instrumentation**: 5th defaulted template parameter `TStats = NoOpStats` on `PriorityQueue<T, …>`. New `InstrumentedPriorityQueue<T, …>` alias for the `ComparisonStats` variant. Zero-bytes elision via `[[no_unique_address]]` / `[[msvc::no_unique_address]]`; `sizeof(InstrumentedPriorityQueue<int>) - sizeof(PriorityQueue<int>) == sizeof(ComparisonStats)` is `static_assert`-checked.
- **Cross-language parity for instrumentation**: byte-for-byte identical comparison counts with the Rust, Go, TypeScript, and Zig implementations (verified across 24 (graph, arity) cells).
- **Phase 3 benchmark harness**: `examples/dijkstra/Cpp/` runs the cross-language Dijkstra sweep; see [`benchmarks/methodology.md`](https://github.com/PCfVW/d-Heap-priority-queue/blob/master/benchmarks/methodology.md).

### What's New in v2.5.0

- **C++23 `std::expected` error handling**: Safe, expressive error propagation for all fallible operations
- **New Error enum**: `InvalidArity`, `ItemNotFound`, `IndexOutOfBounds`, `EmptyQueue`
- **Safe accessors**: `peek()` returns `std::optional<T>`, `get_position()` returns `std::optional<Position>`
- **Bulk operations**: `insert_many()` with Floyd's O(n) heapify, `pop_many()` for batch extraction
- **Complete priority API**: `update_priority()`, `decrease_priority_by_index()`, `update_priority_by_index()`
- **Cross-language parity**: `to_array()`, `pop_front()` returning `std::optional<T>`

## API Reference

### Construction

```cpp
// Assert-based (panics on invalid arity)
PriorityQueue<T> pq(3);                          // Create with arity d=3
PriorityQueue<T> pq(3, first_item);              // Create with first item

// Safe factory functions returning std::expected
auto result = PriorityQueue<T>::create(3);       // Returns std::expected<PriorityQueue, Error>
if (result) { auto pq = std::move(*result); }
```

### Query Operations

| Method | Return Type | Description |
|--------|-------------|-------------|
| `len()` / `size()` | `size_t` | Number of items in the queue |
| `is_empty()` / `empty()` | `bool` | Whether the queue is empty |
| `d()` / `getd()` | `size_t` | Arity (number of children per node) |
| `front()` | `const T&` | Highest-priority item (UB if empty) |
| `peek()` | `std::optional<T>` | Safe alternative to `front()` |
| `contains(item)` | `bool` | O(1) membership test |
| `get_position(item)` | `std::optional<Position>` | O(1) position lookup |
| `to_array()` | `std::vector<T>` | Copy of heap contents |
| `to_string()` | `std::string` | String representation |

### Modification Operations

| Method | Return Type | Description |
|--------|-------------|-------------|
| `insert(item)` | `void` | Insert item O(log_d n) |
| `insert_many(items)` | `void` | Bulk insert O(n) using Floyd's heapify |
| `pop()` | `void` | Remove highest-priority item |
| `pop_front()` | `std::optional<T>` | Remove and return highest-priority item |
| `pop_many(count)` | `std::vector<T>` | Remove and return multiple items |
| `clear(optional_d)` | `void` | Clear all items, optionally change arity |

### Priority Update Operations

| Method | Return Type | Direction | Description |
|--------|-------------|-----------|-------------|
| `increase_priority(item)` | `void` | Up only | Move item toward root |
| `increase_priority(index)` | `void` | Up only | By index |
| `increase_priority_by_index(index)` | `std::expected<void, Error>` | Up only | Safe version |
| `try_increase_priority(item)` | `std::expected<void, Error>` | Up only | Safe version |
| `decrease_priority(item)` | `void` | Down only | Move item toward leaves |
| `decrease_priority_by_index(index)` | `std::expected<void, Error>` | Down only | Safe version |
| `try_decrease_priority(item)` | `std::expected<void, Error>` | Down only | Safe version |
| `update_priority(item)` | `void` | Both | When direction unknown |
| `update_priority_by_index(index)` | `std::expected<void, Error>` | Both | Safe version |
| `try_update_priority(item)` | `std::expected<void, Error>` | Both | Safe version |

### Error Handling

```cpp
enum class Error {
    InvalidArity,     // Arity (d) must be >= 1
    ItemNotFound,     // Item not found in the priority queue
    IndexOutOfBounds, // Index is out of bounds
    EmptyQueue        // Operation requires a non-empty queue
};
```

## Usage Examples

### Basic Usage

```cpp
#include "PriorityQueue.h"
using namespace TOOLS;

// Min-heap of integers with arity 3
PriorityQueue<int> pq(3);

pq.insert(10);
pq.insert(5);
pq.insert(15);

int top = pq.front();           // 5 (smallest)
auto maybe = pq.peek();         // std::optional<int>(5)
pq.pop();                       // Removes 5
```

### Safe Error Handling with std::expected

```cpp
#include "PriorityQueue.h"
using namespace TOOLS;

// Safe construction
auto result = PriorityQueue<int>::create(0);  // Invalid arity
if (!result) {
    std::cerr << "Error: " << result.error() << std::endl;
    // Prints: "Heap arity (d) must be >= 1"
}

// Safe priority updates
auto pq = PriorityQueue<int>::create(2).value();
pq.insert(10);

auto update_result = pq.increase_priority_by_index(99);  // Out of bounds
if (!update_result) {
    switch (update_result.error()) {
        case Error::IndexOutOfBounds:
            std::cerr << "Invalid index" << std::endl;
            break;
        case Error::ItemNotFound:
            std::cerr << "Item not in queue" << std::endl;
            break;
        default:
            break;
    }
}
```

### Bulk Operations

```cpp
PriorityQueue<int> pq(4);

// Insert many items efficiently using Floyd's heapify - O(n)
pq.insert_many({50, 30, 70, 20, 60, 10, 80, 40});

// Pop multiple items in priority order
auto top3 = pq.pop_many(3);  // Returns {10, 20, 30}
```

### Custom Item Types

```cpp
struct Item {
    uint32_t id;
    uint32_t cost;
};

struct ItemHash {
    size_t operator()(const Item& x) const {
        return std::hash<uint32_t>()(x.id);
    }
};
struct ItemEq {
    bool operator()(const Item& a, const Item& b) const {
        return a.id == b.id;
    }
};
struct LessCost {
    bool operator()(const Item& a, const Item& b) const {
        return a.cost < b.cost;
    }
};

TOOLS::PriorityQueue<Item, ItemHash, LessCost, ItemEq> pq(3);

pq.insert({.id=1, .cost=10});
pq.insert({.id=2, .cost=5});

// Increase priority (decrease cost in min-heap)
Item updated{.id=1, .cost=3};
pq.increase_priority(updated);

// Safe version
auto result = pq.try_increase_priority(updated);
if (result) {
    std::cout << "Priority updated successfully" << std::endl;
}
```

### Comparison-count instrumentation (v2.6.0)

Opt in by constructing `InstrumentedPriorityQueue<T, …>` instead of `PriorityQueue<T, …>`. The default queue type uses `NoOpStats` — a ZST that monomorphizes away under `[[no_unique_address]]`, so the un-instrumented heap has the same layout and machine code as before v2.6.0.

```cpp
#include "PriorityQueue.h"
using namespace TOOLS;

InstrumentedPriorityQueue<int> pq(4);   // d = 4

pq.insert(5);
pq.insert(3);
pq.insert(8);
pq.pop();

const auto& s = pq.stats();
std::cout << std::format(
    "insert={}, pop={}, total={}\n",
    s.insert, s.pop, s.total()
);
```

`ComparisonStats` exposes one accessor per operation (`insert`, `pop`, `decrease_priority`, `increase_priority`, `update_priority`) plus `total()` and `reset()`.

## Compilation

Requires C++23 compiler with `<expected>` support.

```bash
# Microsoft Visual C++ (from Developer Command Prompt)
cl /std:c++latest /EHsc /O2 your_program.cpp

# GCC 13+
g++ -std=c++23 -O2 your_program.cpp -o your_program

# Clang 16+
clang++ -std=c++23 -O2 your_program.cpp -o your_program
```

### Running Tests

Using CMake (recommended):

```bash
# Configure and build
cmake -B build -S .
cmake --build build --config Release

# Run tests
ctest --test-dir build --output-on-failure
```

Or from a Visual Studio Developer Command Prompt:

```bash
cl /std:c++latest /EHsc /O2 test_comprehensive.cpp /Fe:test_comprehensive.exe
test_comprehensive.exe
```

## Cross-Language Compatibility

This implementation maintains API parity with:
- **Rust**: `d-ary-heap` crate
- **TypeScript**: `d-ary-heap` npm package
- **Go**: `dheap` module
- **Zig**: `d_heap` library

| C++ | Rust | TypeScript | Go | Zig |
|-----|------|------------|-----|-----|
| `peek()` | `peek()` | `peek()` | `Peek()` | `front()` |
| `get_position()` | `get_position()` | `getPosition()` | `GetPosition()` | `getPosition()` |
| `insert_many()` | `insert_many()` | `insertMany()` | `InsertMany()` | `insertMany()` |
| `pop_many()` | `pop_many()` | `popMany()` | `PopMany()` | `popMany()` |
| `to_array()` | `to_array()` | `toArray()` | `ToArray()` | `toArray()` |
| `std::expected` | `Result<T, Error>` | `throws Error` | `(T, error)` | `!T` |

## What is a d-Heap?

A [d-ary heap](https://en.wikipedia.org/wiki/D-ary_heap) is a tree structure where:
- Each node has at most d children
- The root holds the highest-priority item
- Each parent has higher priority than all its children
- The tree is complete (filled left-to-right, level by level)

Time complexities over n items:
- O(1): access top, peek, contains, get_position
- O(d · log_d n): delete-top
- O(log_d n): insert, increase_priority
- O(d · log_d n): decrease_priority
- O((d+1) · log_d n): update_priority

## Reference

Section A.3, [d-Heaps](https://en.wikipedia.org/wiki/D-ary_heap), pp. 773–778 of Ravindra Ahuja, Thomas Magnanti & James Orlin, **Network Flows** (Prentice Hall, 1993). Book info: https://mitmgmtfaculty.mit.edu/jorlin/network-flows/

## License

Apache License 2.0 — See [LICENSE](https://github.com/PCfVW/d-Heap-priority-queue/blob/master/LICENSE) for details.
