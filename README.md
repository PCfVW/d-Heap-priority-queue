# Min/Max d-Heap Priority Queues (C++, Rust, and Zig) v2.0.0

This repository contains generic d-ary heap (d-heap) priority queue implementations with O(1) lookup for item updates and configurable arity d.

- Min-heap or max-heap behavior via comparator
- Efficient operations: O(1) front, O(log_d n) insert/update, O(d · log_d n) pop
- Examples and unit tests included in each language subproject
- All three implementations provide the exact same set of operations (API parity across C++, Rust, and Zig).
- **Unified API**: Cross-language method names standardized for consistent usage across all implementations.
- <u>Provided</u>: access top (front), insert, update priority of existing item, delete-top (pop), size/length, emptiness check, membership test (contains).
- <u>Not provided</u>: erase/remove arbitrary item by identity, meld/merge of heaps, stable ordering for equal priorities, or iterators supporting removal during traversal.

## Unified API Methods

All three implementations (C++, Rust, and Zig) provide these standardized method names for cross-language consistency:

| Method | Description | C++ | Rust | Zig |
|--------|-------------|-----|------|-----|
| `clear()` | Clear all items, optionally reset arity | ✅ | ✅ | ✅ |
| `contains()` | Check if item exists in queue (O(1)) | ✅ | ✅ | ✅ |
| `d()` | Get arity (number of children per node) | ✅ | ✅ | ✅ |
| `decrease_priority()` | Decrease priority of existing item | ✅ | ✅ | ✅ |
| `front()` | Get reference to highest-priority item | ✅ | ✅ | ✅ |
| `increase_priority()` | Increase priority of existing item | ✅ | ✅ | ✅ |
| `insert()` | Add new item to queue | ✅ | ✅ | ✅ |
| `is_empty()` | Check if queue is empty | ✅ | ✅ | ✅ |
| `len()` | Get number of items | ✅ | ✅ | ✅ |
| `pop()` | Remove highest-priority item | ✅ | ✅ | ✅ |
| `to_string()` | String representation of queue contents | ✅ | ✅* | ✅ |
| `Position` | Type alias for position indices | ✅ | ✅ | ✅ |

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

*\* Rust also implements the `Display` trait, allowing `format!("{}", pq)` in addition to `pq.to_string()`.*

## Language Comparison

Why three implementations? Each language brings unique strengths to priority queue usage:

| Aspect | C++ | Rust | Zig |
|--------|-----|------|-----|
| **Best For** | Performance-critical systems, legacy integration | Memory-safe systems, concurrent applications | Compile-time optimization, embedded systems |
| **Memory Safety** | Manual (developer responsibility) | Compile-time guaranteed (borrow checker) | Explicit allocators, clear ownership |
| **Compile-Time Features** | Templates, constexpr | Generics, const fn, macros | comptime (full language at compile-time) |
| **Learning Curve** | Steep (complex syntax, many features) | Moderate-Steep (ownership concepts) | Gentle (simple, explicit) |
| **Build System** | External (CMake, Make, etc.) | Cargo (integrated) | Zig build (integrated, cross-compile) |
| **Zero-Cost Abstractions** | ✅ Yes | ✅ Yes | ✅ Yes |
| **Standard Library** | Extensive, mature | Modern, safe | Minimal, explicit |
| **Cross-Compilation** | Complex | Moderate | Trivial (built-in) |
| **Interop with C** | Native | Via FFI (unsafe blocks) | Seamless (imports C headers directly) |
| **Typical Use Cases** | Game engines, HPC, databases | Web services, CLI tools, OS components | Compilers, drivers, performance-critical tools |

**When to choose each:**
- **C++**: Maximum performance, existing C++ codebase, need STL compatibility
- **Rust**: Memory safety critical, concurrent systems, modern tooling preferred
- **Zig**: Compile-time computation, C interop, explicit control with safety, cross-platform builds

All three implementations provide identical functionality—choose based on your project's ecosystem and requirements.

## Version Information

**Current Version: 2.0.0** - Major Release

**What's New in 2.0.0:**
- 🚀 **Zig 0.15.2**: Updated Zig implementation for latest Zig version with API changes
- ✨ **Generic Zig**: Zig implementation now fully generic (use your own item types)
- 🧪 **Comprehensive Tests**: 20+ tests in Zig matching Rust coverage
- 📦 **Module Export**: Zig can now be used as a dependency in other projects
- 🔧 **Better Error Handling**: Removed `unreachable` from Zig error paths
- ➕ **New Methods**: Added `peek()` alias and `initCapacity()` in Zig

This version represents a feature-complete, production-ready implementation with:
- ✅ **Complete API**: All 12 core methods implemented across all three languages
- ✅ **Comprehensive Testing**: 20+ test functions covering all functionality and edge cases
- ✅ **Cross-Language Parity**: Identical API and behavior across C++, Rust, and Zig
- ✅ **Professional Documentation**: Detailed usage guides and design explanations
- ✅ **Performance Optimized**: O(1) item lookup, template specialization, memory efficiency
- ✅ **Truly Generic**: All implementations support user-defined item types

All three implementations share synchronized version numbers to ensure feature compatibility and consistent user experience.

## Getting Started

Explore the language-specific implementations:

| Language | README |
| --- | --- |
| ![C++17](https://img.shields.io/badge/C%2B%2B-17-blue.svg) | [Cpp/README.md](Cpp/README.md) |
| ![Rust Edition 2021](https://img.shields.io/badge/Rust-Edition_2021-orange.svg) | [Rust/README.md](Rust/README.md) |
| ![Zig 0.15.2](https://img.shields.io/badge/Zig-0.15.2-f7a41d.svg) | [zig/README.md](zig/README.md) |

## License

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-green.svg)](LICENSE)

This project is licensed under the **Apache License 2.0** - see the [LICENSE](LICENSE) file for details.

Copyright © 2023-2025 Eric Jacopin

## References

- Ahuja, Magnanti & Orlin, **Network Flows** (1993), Section A.3 on d-Heaps
