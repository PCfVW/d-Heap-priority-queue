# Min/Max d-Heap Priority Queues (C++, Go, Rust, Zig, and TypeScript) v2.5.0

This repository contains generic d-ary heap (d-heap) priority queue implementations with O(1) lookup for item updates and configurable arity d.

## Table of Contents

- [Live Demo](#live-demo)
- [Features](#features)
- [Cross-Language API Reference](#cross-language-api-reference)
- [Error Handling by Language](#error-handling-by-language)
- [Language Comparison](#language-comparison)
- [Language-Specific Extensions](#language-specific-extensions)
- [Version Information](#version-information)
- [Getting Started](#getting-started)
- [License](#license)
- [AI Code Generation Research](#ai-code-generation-research)
- [References](#references)

## Live Demo

**[Interactive Visualization](https://pcfvw.github.io/d-Heap-priority-queue/)** — Watch d-ary heaps in action with Dijkstra's algorithm.

- Dual-panel layout: heap tree structure + graph visualization
- Compare arities (d=2, d=4, d=8) side-by-side with Race Mode
- Timeline scrubber and playback controls for step-by-step analysis
- Keyboard shortcuts for efficient navigation

## Features

- Min-heap or max-heap behavior via comparator
- Efficient operations: O(1) front, O(log_d n) insert/update, O(d · log_d n) pop
- Examples and unit tests included in each language subproject
- All five implementations provide the exact same set of operations (API parity across C++, Go, Rust, Zig, and TypeScript).
- **Unified API**: Cross-language method names standardized for consistent usage across all implementations.
- <u>Provided</u>: access top (front), insert, update priority of existing item, delete-top (pop), size/length, emptiness check, membership test (contains).
- <u>Not provided</u>: erase/remove arbitrary item by identity, meld/merge of heaps, stable ordering for equal priorities, or iterators supporting removal during traversal.

## Cross-Language API Reference

All five implementations provide equivalent functionality with method names following their respective language conventions:

### Core Functionality Matrix

| Function | C++ | Go | Rust | Zig | TypeScript |
|----------|-----|-----|------|-----|------------|
| **Clear** | `clear(opt_d)` | `Clear(newD...)` | `clear(opt_d)` | `clear(new_depth?)` | `clear(newD?)` |
| **Contains** | `contains()` | `Contains()` | `contains()` | `contains()` | `contains()` |
| **Arity** | `d()` | `D()` | `d()` | `d()` | `d()` |
| **Get Position** | `get_position()` | `GetPosition()` | `get_position()` | `getPosition()` | `getPosition()` |
| **Front** | `front()` | `Front()` | `front()` | `front()` | `front()` |
| **Peek** | `peek()` | `Peek()` | `peek()` | `peek()` | `peek()` |
| **Insert** | `insert()` | `Insert()` | `insert()` | `insert()` | `insert()` |
| **Insert Many** | `insert_many()` | `InsertMany()` | `insert_many()` | `insertMany()` | `insertMany()` |
| **Is Empty** | `is_empty()` | `IsEmpty()` | `is_empty()` | `isEmpty()` | `isEmpty()` |
| **Length** | `len()` | `Len()` | `len()` | `len()` | `len()` |
| **Pop** | `pop()` | `Pop()` | `pop()` | `pop()` | `pop()` |
| **Pop Front** | `pop_front()` | — | — | — | — |
| **Pop Many** | `pop_many()` | `PopMany()` | `pop_many()` | `popMany()` | `popMany()` |
| **Increase Priority** | `increase_priority()` | `IncreasePriority()` | `increase_priority()` | `increasePriority()` | `increasePriority()` |
| **Increase Priority By Index** | `increase_priority_by_index()` | `IncreasePriorityByIndex()` | `increase_priority_by_index()` | `increasePriorityByIndex()` | `increasePriorityByIndex()` |
| **Decrease Priority** | `decrease_priority()` | `DecreasePriority()` | `decrease_priority()` | `decreasePriority()` | `decreasePriority()` |
| **Decrease Priority By Index** | `decrease_priority_by_index()` | `DecreasePriorityByIndex()` | `decrease_priority_by_index()` | `decreasePriorityByIndex()` | `decreasePriorityByIndex()` |
| **Update Priority** | `update_priority()` | `UpdatePriority()` | `update_priority()` | `updatePriority()` | `updatePriority()` |
| **Update Priority By Index** | `update_priority_by_index()` | — | `update_priority_by_index()` | — | — |
| **To Array** | `to_array()` | `ToArray()` | `to_array()` | `toArray()` | `toArray()` |
| **String Output** | `to_string()` | `String()` | `to_string()` | `toString()` | `toString()` |
| **Position Type** | `Position` | `Position` | `Position` | `Position` | `Position` |

### Method Naming Conventions

**C++ and Rust** follow `snake_case` conventions:
- `is_empty()`, `increase_priority()`, `decrease_priority()`, `to_string()`

**Go** follows `PascalCase` conventions (exported methods):
- `IsEmpty()`, `IncreasePriority()`, `DecreasePriority()`, `String()`

**Zig and TypeScript** follow `camelCase` conventions:
- `isEmpty()`, `increasePriority()`, `decreasePriority()`, `toString()`

**Cross-Language Compatibility**:
- **Go**: Provides snake_case aliases (`Is_empty()`, `Increase_priority()`, `Decrease_priority()`, `To_string()`, etc.)
- **Zig**: Provides snake_case aliases for all camelCase methods (`is_empty()`, `increase_priority()`, `decrease_priority()`, `update_priority()`, `get_position()`, `to_string()`, `to_array()`, etc.)
- **TypeScript**: Provides snake_case aliases for all camelCase methods

### Return Type Variations

Different languages handle safety and error conditions in their idiomatic ways:

| Method | C++ | Go | Rust | Zig | TypeScript |
|--------|-----|-----|------|-----|------------|
| **front()** | `const T&` (UB if empty) | `(T, error)` | `&T` (panics if empty) | `?T` (null if empty) | `T` (throws if empty) |
| **peek()** | `std::optional<T>` | `(T, bool)` | `Option<&T>` | `?T` (alias for front) | `T \| undefined` |
| **pop()** | `void` | `(T, bool)` | `Option<T>` | `?T` | `T \| undefined` |
| **pop_front()** | `std::optional<T>` | — | — | — | — |
| **Error handling** | `std::expected` | Errors/Panics | `Result<T, Error>` | Error unions | Exceptions |

**Safety Recommendations**:
- **C++**: Use `peek()` for safe access (returns `std::optional<T>`), or check `!is_empty()` before `front()`
- **Go**: Use `Peek()` for safe access (returns `(T, bool)`), or check `!IsEmpty()` before `Front()`
- **Rust**: Use `peek()` for safe access or handle panics appropriately
- **Zig**: `front()` is safe by default, returns `null` for empty heaps
- **TypeScript**: Use `peek()` for safe access or wrap `front()` in try-catch

### Priority Update Method Design

The priority update methods use **importance-based semantics** with directional optimization:

**Method Semantics:**
- **`increase_priority()`**: Make an item **more important** (moves toward heap root). Only moves up for O(log_d n) performance.
- **`decrease_priority()`**: Make an item **less important** (moves toward heap leaves). Only moves down for O(d × log_d n) performance.
- **`update_priority()`**: Update when direction is **unknown**. Checks both directions for O((d+1) × log_d n) performance.

**Heap Context:**
- **Min-heap**: Lower priority values = higher importance (e.g., priority 5 > priority 10)
- **Max-heap**: Higher priority values = higher importance (e.g., priority 10 > priority 5)

**When to use each:**
- Use `increase_priority()` when you know the item became more important (e.g., Dijkstra's algorithm)
- Use `decrease_priority()` when you know the item became less important
- Use `update_priority()` when you don't know which direction the priority changed

**Note**: `update_priority()` is now available in all five implementations (C++, Go, Rust, Zig, and TypeScript). Use it when you don't know the direction of priority change.

*Note: Original methods (`size()`, `empty()`, etc.) remain available in C++ for backward compatibility.*

*\* Rust also implements the `Display` trait, allowing `format!("{}", pq)` in addition to `pq.to_string()`.*

## Error Handling by Language

Each implementation follows its language's idiomatic error handling patterns:

| Operation | C++ | Go | Rust | Zig | TypeScript |
|-----------|-----|-----|------|-----|------------|
| **Empty front()** | Undefined behavior | Returns `error` | Panic with message | Returns `null` | Throws `Error` |
| **Invalid arity** | `Error::InvalidArity` | Panics | `Error::InvalidArity` | `error.DepthMustBePositive` | Throws `Error` |
| **Item not found** | `Error::ItemNotFound` | Returns `error` | `Error::ItemNotFound` | `error.ItemNotFound` | Throws `Error` |
| **Index out of bounds** | `Error::IndexOutOfBounds` | Panics | `Error::IndexOutOfBounds` | `error.IndexOutOfBounds` | Throws `Error` |
| **Empty queue op** | `Error::EmptyQueue` | Returns `error` | `Error::EmptyQueue` | Returns `null` | Throws `Error` |

### Error Types by Language

| Language | Error Mechanism | Error Types |
|----------|-----------------|-------------|
| **C++** | `std::expected<T, Error>` | `InvalidArity`, `ItemNotFound`, `IndexOutOfBounds`, `EmptyQueue` |
| **Go** | `(T, error)` / `(T, bool)` | `ErrEmptyQueue`, `ErrItemNotFound`, `ErrInvalidArity` |
| **Rust** | `Result<T, Error>` | `InvalidArity`, `ItemNotFound`, `IndexOutOfBounds`, `EmptyQueue` |
| **Zig** | `!T` (error union) | `DepthMustBePositive`, `ItemNotFound`, `IndexOutOfBounds` |
| **TypeScript** | `throws Error` | Error messages |

### Error Handling Best Practices

- **C++**: Use `std::expected`-returning methods (`try_*`, `*_by_index`) for safe operations, or use `peek()` for safe front access
- **Go**: Use `Peek()` for safe access (returns `(T, bool)`), check errors from `Front()`, handle panics for invalid operations
- **Rust**: Use `peek()` for safe access, use `Result`-returning methods for fallible operations
- **Zig**: Handle error unions with `try` or explicit error checking (`if (result) |value| { ... }`)
- **TypeScript**: Use try-catch blocks or `peek()` for safe access

## Language Comparison

Why five implementations? Each language brings unique strengths to priority queue usage:

| Aspect | C++ | Go | Rust | Zig | TypeScript |
|--------|-----|-----|------|-----|------------|
| **Best For** | Performance-critical systems, legacy integration | Cloud services, microservices, concurrent systems | Memory-safe systems, concurrent applications | Compile-time optimization, embedded systems | Web apps, Node.js services, rapid development |
| **Memory Safety** | Manual (developer responsibility) | Garbage collected with escape analysis | Compile-time guaranteed (borrow checker) | Explicit allocators, clear ownership | Garbage collected |
| **Compile-Time Features** | Templates, constexpr | Generics (1.18+), type inference | Generics, const fn, macros | comptime (full language at compile-time) | Generics, type inference |
| **Learning Curve** | Steep (complex syntax, many features) | Gentle (simple, minimal syntax) | Moderate-Steep (ownership concepts) | Gentle (simple, explicit) | Gentle (familiar JS syntax) |
| **Build System** | External (CMake, Make, etc.) | go build (integrated) | Cargo (integrated) | Zig build (integrated, cross-compile) | npm/yarn (integrated) |
| **Zero-Cost Abstractions** | ✅ Yes | ❌ Runtime overhead (GC) | ✅ Yes | ✅ Yes | ❌ Runtime overhead |
| **Standard Library** | Extensive, mature | Extensive, batteries-included | Modern, safe | Minimal, explicit | Extensive (npm ecosystem) |
| **Cross-Compilation** | Complex | Simple (built-in) | Moderate | Trivial (built-in) | N/A (interpreted) |
| **Interop with C** | Native | Via cgo | Via FFI (unsafe blocks) | Seamless (imports C headers directly) | Via native addons |
| **Typical Use Cases** | Game engines, HPC, databases | Cloud infrastructure, APIs, CLI tools | Web services, CLI tools, OS components | Compilers, drivers, performance-critical tools | Web apps, APIs, tooling |

**When to choose each:**
- **C++**: Maximum performance, existing C++ codebase, need STL compatibility
- **Go**: Cloud services, microservices, simple concurrency with goroutines, readable team codebases
- **Rust**: Memory safety critical, concurrent systems, modern tooling preferred
- **Zig**: Compile-time computation, C interop, explicit control with safety, cross-platform builds
- **TypeScript**: Web/Node.js projects, rapid prototyping, full-stack JavaScript applications

All five implementations provide identical functionality—choose based on your project's ecosystem and requirements.

## Language-Specific Extensions

While all implementations provide the core d-heap functionality, each offers additional features that leverage their language's strengths:

### C++ Extensions
- **C++23 error handling**: `std::expected<T, Error>` for safe, expressive error propagation
- **Safe accessors**: `peek()` returns `std::optional<T>`, `get_position()` returns `std::optional<Position>`
- **Bulk operations**: `insert_many()` with Floyd's O(n) heapify, `pop_many()` for batch extraction
- **Complete priority API**: `update_priority()`, `decrease_priority_by_index()`, `update_priority_by_index()`
- **Safe variants**: `try_increase_priority()`, `try_decrease_priority()`, `try_update_priority()` returning `std::expected`
- **Factory functions**: `create()`, `create_with_first()` returning `std::expected<PriorityQueue, Error>`
- **Legacy compatibility**: `size()`, `empty()`, `getd()`, `put()` methods for backward compatibility
- **Position-based operations**: `increase_priority_by_index()`, `decrease_priority_by_index()` for direct index manipulation
- **Template specialization**: Full STL compatibility with custom hash/equality functors

### Go Extensions
- **Functional options**: `Options[T, K]` struct for clean configuration with `Comparator` and `KeyExtractor`
- **Bulk operations**: `InsertMany()`, `PopMany()` for efficient batch processing with Floyd's heapify algorithm
- **Position lookup**: `GetPosition()` for O(1) index lookup by item identity
- **Index-based updates**: `IncreasePriorityByIndex()`, `DecreasePriorityByIndex()` for direct index manipulation
- **Bidirectional update**: `UpdatePriority()` for when priority change direction is unknown
- **Safe access**: `Peek()` returns `(T, bool)` for safe, non-panicking access
- **Array access**: `ToArray()` method for integration with Go slices and standard library
- **Stringer interface**: Implements `fmt.Stringer` for automatic `fmt.Print()` support
- **Cross-language aliases**: Snake_case method aliases (`Is_empty()`, `Increase_priority()`, etc.) for easy porting
- **Error handling**: Idiomatic Go error returns from `Front()`, panics for programmer errors (invalid arity, item not found)

### Rust Extensions
- **Result-based error handling**: `Result<T, Error>` for all fallible operations
- **Index-based updates**: `increase_priority_by_index()`, `decrease_priority_by_index()`, `update_priority_by_index()`
- **Bulk operations**: `insert_many()` with Floyd's O(n) heapify, `pop_many()` for batch extraction
- **Safe access**: `peek()` method returns `Option<&T>`, `get_position()` returns `Option<Position>`
- **Bidirectional update**: `update_priority()` for when priority change direction is unknown
- **Display trait**: Automatic `format!("{}", pq)` support alongside explicit `to_string()`
- **Array access**: `to_array()` method for integration with Vec and standard library
- **Memory safety**: Compile-time guarantees with zero-cost abstractions

### Zig Extensions
- **Pre-allocation**: `initCapacity()` constructor for performance optimization
- **Bulk operations**: `insertMany()`, `popMany()` for efficient batch processing with Floyd's heapify algorithm
- **Position lookup**: `getPosition()` for O(1) index lookup by item identity
- **Index-based updates**: `increasePriorityByIndex()`, `decreasePriorityByIndex()` for direct index manipulation
- **Bidirectional update**: `updatePriority()` for when priority change direction is unknown
- **Array access**: `toArray()` method for integration with Zig slices
- **Generic types**: Full compile-time generics with `DHeap(T, Context, Comparator)`
- **Explicit memory**: Manual allocator management following Zig best practices
- **Compile-time optimization**: `comptime` features for zero-runtime-cost abstractions
- **Cross-language aliases**: Full snake_case aliases for all camelCase methods
- **Error handling**: Error union return types with explicit error handling (`!void`, `error.ItemNotFound`, `error.IndexOutOfBounds`)

### TypeScript Extensions
- **Key-based operations**: `containsKey()`, `getPosition()`, `getPositionByKey()` for advanced lookups
- **Index-based updates**: `increasePriorityByIndex()`, `decreasePriorityByIndex()` for direct index manipulation
- **Bidirectional update**: `updatePriority()` for when priority change direction is unknown
- **Bulk operations**: `insertMany()`, `popMany()` for efficient batch processing
- **Array access**: `toArray()` method and `[Symbol.iterator]()` for integration with JavaScript ecosystem
- **Property access**: `size` property alongside `len()` method
- **Instrumentation**: Opt-in comparison counting for performance analysis and visualization
- **Cross-language aliases**: Snake_case method aliases for easy porting from C++/Rust
- **Error handling**: Exception-based errors with try-catch handling, safe `peek()` alternative

Choose extensions based on your specific use case—core functionality remains identical across all implementations.

## Version Information

**Current Version: 2.5.0** — C++ API Completeness & Cross-Language Alignment

**What's New in 2.5.0:**
- 🎯 **Complete Dijkstra Examples**: All 5 languages now have working Dijkstra implementations in `examples/dijkstra/`
- 🎯 **C++23 Modernization**: `std::expected<T, Error>` for safe, expressive error handling
- ✅ **C++ API Completeness**: Added `peek()`, `get_position()`, `insert_many()`, `pop_many()`, `pop_front()`, `to_array()`
- ✅ **C++ Priority Updates**: `update_priority()`, `decrease_priority_by_index()`, `update_priority_by_index()`
- ✅ **C++ Safe Variants**: `try_increase_priority()`, `try_decrease_priority()`, `try_update_priority()` with `std::expected`
- ✅ **Rust API Completeness**: Added `insert_many()`, `pop_many()`, `to_array()`, `update_priority()`, `update_priority_by_index()`
- ✅ **Cross-Language Alignment**: All 5 implementations now have identical core APIs
- ✅ **C++ Test Suite**: 61 comprehensive tests aligned with Rust test patterns
- ✅ **Rust Test Suite**: 97 tests (62 comprehensive + 27 doc tests + 8 decrease_priority tests)
- 📚 **Updated Documentation**: Synchronized cross-language comments and API references

**Previous in 2.4.0:**
- 🎯 **[Interactive Demo](https://pcfvw.github.io/d-Heap-priority-queue/)**: React Flow visualization of d-ary heaps with Dijkstra's algorithm
  - Dual-panel layout with heap tree and graph visualization
  - Arity comparison (d=2, d=4, d=8) and Race Mode
  - Timeline scrubber, playback controls, keyboard shortcuts
- ✅ **TypeScript Instrumentation**: Opt-in comparison counting for performance analysis
- ✅ **GitHub Actions**: Automatic demo deployment to GitHub Pages
- ✅ **Unified Priority Update API**: New `updatePriority()` method across Go, Zig, and TypeScript for when direction is unknown
- ✅ **Position Lookup**: `getPosition()` method for O(1) index lookup in Go, Zig, and TypeScript
- ✅ **Index-Based Updates**: `increasePriorityByIndex()`, `decreasePriorityByIndex()` across Go, Zig, and TypeScript
- ✅ **Zig Bulk Operations**: `insertMany()` with Floyd's O(n) heapify, `popMany()`, `toArray()`
- ✅ **Zig API Alignment**: Full snake_case aliases, fixed `decreasePriority()` semantics, 54 tests
- ✅ **Go Refinements**: `Position` type used internally, full API parity, 57 tests
- ✅ **TypeScript Refinements**: Full API parity with Go/Zig, 57 tests

**Previous in 2.3.0:**
- ✅ **Go Implementation**: Complete d-heap priority queue in Go with full API parity
- ✅ **Go Generics**: Full generic support with `PriorityQueue[T any, K comparable]`
- ✅ **Go Dijkstra Example**: Complete working example demonstrating d-heap usage
- ✅ **Comparator Utilities**: `MinBy()`, `MaxBy()` factory functions and pre-built comparators
- ✅ **Comprehensive Tests**: 57 test cases covering all functionality
- ✅ **Cross-Language Aliases**: Snake_case aliases for easy porting from other implementations

**Previous in 2.2.0:**
- ✅ **Examples Infrastructure**: Added `examples/dijkstra/` with Network Flows textbook example
- ✅ **TypeScript Dijkstra Implementation**: Complete working example with path reconstruction
- ✅ **Shared Test Graph**: JSON format graph from Ahuja, Magnanti, Orlin Figure 4.7
- ✅ **Performance Comparisons**: Demonstrates d-ary heap advantages (d=2 vs d=4 vs d=8)
- ✅ **Documentation**: Comprehensive README with algorithm explanation and complexity analysis

**Previous in 2.1.2:**
- 📚 **Documentation Overhaul**: Fixed misleading unified API claims, now accurately documents per-language method names
- 🛡️ **Error Handling Guide**: Added comprehensive error handling documentation with best practices for each language
- 📦 **Publishing Ready**: Complete NPM publishing setup with proper build pipeline
- 🚀 **Zig 0.15.2**: Updated Zig implementation for latest Zig version with API changes
- ✨ **Generic Zig**: Zig implementation now fully generic (use your own item types)
- 🧪 **Comprehensive Tests**: 54 tests in Zig matching Go/TypeScript coverage
- 📦 **Module Export**: Zig can now be used as a dependency in other projects
- 🔧 **Better Error Handling**: Removed `unreachable` from Zig error paths
- ➕ **New Methods**: Added `peek()` alias and `initCapacity()` in Zig
- 🟦 **TypeScript**: New high-performance TypeScript implementation with full API parity

This version represents a feature-complete, production-ready implementation with:
- ✅ **Complete API**: 23+ operations with full API parity across all five languages
- ✅ **Comprehensive Testing**: 50-97 tests per language covering all functionality and edge cases
  - C++: 61 tests | Rust: 97 tests | Go: 57 tests | Zig: 54 tests | TypeScript: 57 tests
- ✅ **Cross-Language Parity**: Identical API and behavior across C++, Go, Rust, Zig, and TypeScript
- ✅ **Modern Error Handling**: `std::expected` (C++), `Result<T, Error>` (Rust), error unions (Zig)
- ✅ **Professional Documentation**: Detailed usage guides and design explanations
- ✅ **Performance Optimized**: O(1) item lookup, template specialization, memory efficiency
- ✅ **Truly Generic**: All implementations support user-defined item types

All five implementations share synchronized version numbers to ensure feature compatibility and consistent user experience.

## Getting Started

Try the [Interactive Demo](https://pcfvw.github.io/d-Heap-priority-queue/) to visualize d-ary heaps with Dijkstra's algorithm. Also check out the [Dijkstra examples](https://github.com/PCfVW/d-Heap-priority-queue/tree/master/examples/dijkstra) available in all five languages (TypeScript, Go, Rust, Zig, C++).

Explore the language-specific implementations:

| Language | README |
| --- | --- |
| ![C++23](https://img.shields.io/badge/C%2B%2B-23-blue.svg) | [Cpp/README.md](Cpp/README.md) |
| ![Go 1.21](https://img.shields.io/badge/Go-1.21-00ADD8.svg) | [Go/README.md](Go/README.md) |
| ![Rust Edition 2021](https://img.shields.io/badge/Rust-Edition_2021-orange.svg) | [Rust/README.md](Rust/README.md) |
| ![Zig 0.15.2](https://img.shields.io/badge/Zig-0.15.2-f7a41d.svg) | [zig/README.md](zig/README.md) |
| ![TypeScript 5.3](https://img.shields.io/badge/TypeScript-5.3-blue.svg) | [TypeScript/README.md](TypeScript/README.md) |

## License

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-green.svg)](LICENSE)

This project is licensed under the **Apache License 2.0** - see the [LICENSE](https://github.com/PCfVW/d-Heap-priority-queue/blob/master/LICENSE) file for details.

Copyright © 2023-2026 Eric Jacopin

## AI Code Generation Research

This repository includes an experiment studying how AI models generate d-ary heap implementations. Key findings:

- **Type signatures are 23% more effective** than documentation for constraining AI output
- **100% test preservation** for Rust, Zig, and Python doctests when using Claude Sonnet
- **Model tier matters**: Opus interprets tests as specs; Sonnet reproduces them verbatim

See [experiment/README.md](experiment/README.md) for the full study with reproducible experiments.

## References

- Ahuja, Magnanti & Orlin, **Network Flows** (1993), Section A.3 on d-Heaps
