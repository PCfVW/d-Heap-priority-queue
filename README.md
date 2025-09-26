# Min/Max d-Heap Priority Queues (C++ and Rust) v1.0.0

This repository contains generic d-ary heap (d-heap) priority queue implementations with O(1) lookup for item updates and configurable arity d.

- Min-heap or max-heap behavior via comparator
- Efficient operations: O(1) front, O(log_d n) insert/update, O(d · log_d n) pop
- Examples and unit tests included in each language subproject
- Both implementations provide the exact same set of operations (API parity between C++ and Rust).
- **Unified API**: Cross-language method names standardized for consistent usage across C++ and Rust implementations.
- <u>Provided</u>: access top (front), insert, update priority of existing item, delete-top (pop), size/length, emptiness check.
- <u>Not provided</u>: erase/remove arbitrary item by identity, meld/merge of heaps, stable ordering for equal priorities, or iterators supporting removal during traversal.

## Unified API Methods

Both C++ and Rust implementations now provide these standardized method names for cross-language consistency:

| Method | Description | C++ | Rust |
|--------|-------------|-----|------|
| `clear()` | Clear all items, optionally reset arity | ✅ | ✅ |
| `d()` | Get arity (number of children per node) | ✅ | ✅ |
| `decrease_priority()` | Decrease priority of existing item | ✅ | ✅ |
| `front()` | Get reference to highest-priority item | ✅ | ✅ |
| `increase_priority()` | Increase priority of existing item | ✅ | ✅ |
| `insert()` | Add new item to queue | ✅ | ✅ |
| `is_empty()` | Check if queue is empty | ✅ | ✅ |
| `len()` | Get number of items | ✅ | ✅ |
| `pop()` | Remove highest-priority item | ✅ | ✅ |
| `to_string()` | String representation of queue contents | ✅ | ✅ |
| `Position` | Type alias for position indices | ✅ | ✅ |

### **Priority Update Method Design**

The `increase_priority()` and `decrease_priority()` methods have an intentionally **asymmetric design** that optimizes for both performance and robustness:

**Method Semantics:**
- **`increase_priority()`**: Make an item **more important** (moves toward heap root)
- **`decrease_priority()`**: Make an item **less important** (moves toward heap leaves)

**Heap Context:**
- **Min-heap**: Lower priority values = higher importance (e.g., priority 5 > priority 10)
- **Max-heap**: Higher priority values = higher importance (e.g., priority 10 > priority 5)

**Implementation Strategy:**
- **`increase_priority()`**: Only moves items **up** (O(log_d n)) - assumes correct usage for optimal performance
- **`decrease_priority()`**: Checks **both directions** (O(d × log_d n)) - handles user errors gracefully

This asymmetric design reflects real-world usage patterns: `increase_priority()` is performance-critical in algorithms like Dijkstra's shortest path, while `decrease_priority()` is used less frequently and benefits from defensive programming that prevents heap corruption even when users accidentally call the wrong method.

*Note: Original methods (`size()`, `empty()`, etc.) remain available in C++ for backward compatibility.*

## Version Information

**Current Version: 1.0.0** - Stable Release

This version represents a feature-complete, production-ready implementation with:
- ✅ **Complete API**: All 11 core methods implemented in both languages
- ✅ **Comprehensive Testing**: 14 test functions covering all functionality and edge cases
- ✅ **Cross-Language Parity**: Identical API and behavior between C++ and Rust
- ✅ **Professional Documentation**: Detailed usage guides and design explanations
- ✅ **Performance Optimized**: O(1) item lookup, template specialization, memory efficiency

Both C++ and Rust implementations share synchronized version numbers to ensure feature compatibility and consistent user experience.

## Getting Started

Explore the language-specific implementations:

| Language | README |
| --- | --- |
| ![C++17](https://img.shields.io/badge/C%2B%2B-17-blue.svg) | [Cpp/README.md](Cpp/README.md) |
| ![Rust Edition 2021](https://img.shields.io/badge/Rust-Edition_2021-orange.svg) | [Rust/README.md](Rust/README.md) |

References:
- Ahuja, Magnanti & Orlin, **Network Flows** (1993), Section A.3 on d-Heaps
