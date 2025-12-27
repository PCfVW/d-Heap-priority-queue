# Min/Max d-Heap Priority Queues (C++, Go, Rust, Zig, and TypeScript) v2.3.0

This repository contains generic d-ary heap (d-heap) priority queue implementations with O(1) lookup for item updates and configurable arity d.

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
| **Decrease Priority** | `decrease_priority()` | `DecreasePriority()` | `decrease_priority()` | `decreasePriority()` | `decreasePriority()` |
| **Front** | `front()` | `Front()` | `front()` | `front()` | `front()` |
| **Increase Priority** | `increase_priority()` | `IncreasePriority()` | `increase_priority()` | `increasePriority()` | `increasePriority()` |
| **Insert** | `insert()` | `Insert()` | `insert()` | `insert()` | `insert()` |
| **Is Empty** | `is_empty()` | `IsEmpty()` | `is_empty()` | `isEmpty()` | `isEmpty()` |
| **Length** | `len()` | `Len()` | `len()` | `len()` | `len()` |
| **Pop** | `pop()` | `Pop()` | `pop()` | `pop()` | `pop()` |
| **String Output** | `to_string()` | `String()` | `to_string()` | `toString()` / `to_string()` | `toString()` / `to_string()` |
| **Position Type** | `Position` | `Position` | `Position` | `Position` | `Position` |

### Method Naming Conventions

**C++ and Rust** follow `snake_case` conventions:
- `is_empty()`, `increase_priority()`, `decrease_priority()`, `to_string()`

**Go** follows `PascalCase` conventions (exported methods):
- `IsEmpty()`, `IncreasePriority()`, `DecreasePriority()`, `String()`

**Zig and TypeScript** follow `camelCase` conventions:
- `isEmpty()`, `increasePriority()`, `decreasePriority()`, `toString()`

**Cross-Language Compatibility**:
- **Go**: Provides snake_case aliases (`Is_empty()`, `Increase_priority()`, `Decrease_priority()`, `To_string()`)
- **Zig**: Provides `to_string()` alias for `toString()`
- **TypeScript**: Provides snake_case aliases for all camelCase methods

### Return Type Variations

Different languages handle safety and error conditions in their idiomatic ways:

| Method | C++ | Go | Rust | Zig | TypeScript |
|--------|-----|-----|------|-----|------------|
| **front()** | `const T&` (UB if empty) | `(T, error)` | `&T` (panics if empty) | `?T` (null if empty) | `T` (throws if empty) |
| **peek()** | *Not available* | `(T, bool)` | `Option<&T>` | `?T` (alias for front) | `T \| undefined` |
| **pop()** | `void` | `(T, bool)` | `()` | `?T` | `T \| undefined` |
| **Error handling** | Assertions | Errors/Panics | Panics | Error unions | Exceptions |

**Safety Recommendations**:
- **C++**: Always check `!is_empty()` before calling `front()`
- **Go**: Use `Peek()` for safe access (returns `(T, bool)`), or check `!IsEmpty()` before `Front()`
- **Rust**: Use `peek()` for safe access or handle panics appropriately
- **Zig**: `front()` is safe by default, returns `null` for empty heaps
- **TypeScript**: Use `peek()` for safe access or wrap `front()` in try-catch

### **Priority Update Method Design**

The priority update methods have an intentionally **asymmetric design** that optimizes for both performance and robustness:

**Method Semantics:**
- **Increase Priority**: Make an item **more important** (moves toward heap root)
- **Decrease Priority**: Make an item **less important** (moves toward heap leaves)

**Heap Context:**
- **Min-heap**: Lower priority values = higher importance (e.g., priority 5 > priority 10)
- **Max-heap**: Higher priority values = higher importance (e.g., priority 10 > priority 5)

**Implementation Strategy:**
- **`increase_priority()`**: Only moves items **up** (O(log_d n)) - assumes correct usage for optimal performance
- **`decrease_priority()`**: Checks **both directions** (O(d × log_d n)) - handles user errors gracefully

This asymmetric design reflects real-world usage patterns: `increase_priority()` is performance-critical in algorithms like Dijkstra's shortest path, while `decrease_priority()` is used less frequently and benefits from defensive programming that prevents heap corruption even when users accidentally call the wrong method.

*Note: Original methods (`size()`, `empty()`, etc.) remain available in C++ for backward compatibility.*

*\* Rust also implements the `Display` trait, allowing `format!("{}", pq)` in addition to `pq.to_string()`.*

## Error Handling by Language

Each implementation follows its language's idiomatic error handling patterns:

| Operation | C++ | Go | Rust | Zig | TypeScript |
|-----------|-----|-----|------|-----|------------|
| **Empty front()** | Undefined behavior | Returns `error` | Panic with message | Returns `null` | Throws `Error` |
| **Invalid arity** | Assert failure | Panics | Panic with message | Returns `error.DepthMustBePositive` | Throws `Error` |
| **Item not found** | Assert failure | Panics | Panic with message | Returns `error.ItemNotFound` | Throws `Error` |
| **Index out of bounds** | Assert failure | Panics | Panic with message | N/A | Throws `Error` |

### Error Handling Best Practices

- **C++**: Check `!empty()` before calling `front()`, validate inputs before operations
- **Go**: Use `Peek()` for safe access (returns `(T, bool)`), check errors from `Front()`, handle panics for invalid operations
- **Rust**: Use `peek()` for safe access, handle panics with `catch_unwind` if needed
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
- **Legacy compatibility**: `size()`, `empty()`, `getd()`, `put()` methods for backward compatibility
- **Position-based operations**: `increase_priority(position)` overload for direct index manipulation
- **Template specialization**: Full STL compatibility with custom hash/equality functors
- **Error handling**: Assertion-based validation with compile-time `INCLUDE_ASSERT` flag

### Go Extensions
- **Functional options**: `Options[T, K]` struct for clean configuration with `Comparator` and `KeyExtractor`
- **Bulk operations**: `InsertMany()`, `PopMany()` for efficient batch processing with Floyd's heapify algorithm
- **Safe access**: `Peek()` returns `(T, bool)` for safe, non-panicking access
- **Array access**: `ToArray()` method for integration with Go slices and standard library
- **Stringer interface**: Implements `fmt.Stringer` for automatic `fmt.Print()` support
- **Cross-language aliases**: Snake_case method aliases (`Is_empty()`, `Increase_priority()`, etc.) for easy porting
- **Error handling**: Idiomatic Go error returns from `Front()`, panics for programmer errors (invalid arity, item not found)

### Rust Extensions  
- **Index-based updates**: `increase_priority_by_index(index)` for position-based priority changes
- **Safe access**: `peek()` method returns `Option<&T>` instead of panicking
- **Display trait**: Automatic `format!("{}", pq)` support alongside explicit `to_string()`
- **Memory safety**: Compile-time guarantees with zero-cost abstractions
- **Error handling**: Panic-based errors with descriptive messages, safe `peek()` alternative

### Zig Extensions
- **Pre-allocation**: `initCapacity()` constructor for performance optimization
- **Generic types**: Full compile-time generics with `DHeap(T, Context, Comparator)`
- **Explicit memory**: Manual allocator management following Zig best practices
- **Compile-time optimization**: `comptime` features for zero-runtime-cost abstractions
- **Error handling**: Error union return types with explicit error handling (`!void`, `error.ItemNotFound`)

### TypeScript Extensions
- **Key-based operations**: `containsKey()`, `getPosition()`, `getPositionByKey()` for advanced lookups
- **Bulk operations**: `insertMany()`, `popMany()` for efficient batch processing
- **Array access**: `toArray()` method and `[Symbol.iterator]()` for integration with JavaScript ecosystem
- **Property access**: `size` property alongside `len()` method
- **Cross-language aliases**: Snake_case method aliases for easy porting from C++/Rust
- **Error handling**: Exception-based errors with try-catch handling, safe `peek()` alternative

Choose extensions based on your specific use case—core functionality remains identical across all implementations.

## Version Information

**Current Version: 2.3.0** — Go Implementation

**What's New in 2.3.0:**
- ✅ **Go Implementation**: Complete d-heap priority queue in Go with full API parity
- ✅ **Go Generics**: Full generic support with `PriorityQueue[T any, K comparable]`
- ✅ **Go Dijkstra Example**: Complete working example demonstrating d-heap usage
- ✅ **Comparator Utilities**: `MinBy()`, `MaxBy()` factory functions and pre-built comparators
- ✅ **Comprehensive Tests**: 47 test cases covering all functionality
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
- 🧪 **Comprehensive Tests**: 20+ tests in Zig matching Rust coverage
- 📦 **Module Export**: Zig can now be used as a dependency in other projects
- 🔧 **Better Error Handling**: Removed `unreachable` from Zig error paths
- ➕ **New Methods**: Added `peek()` alias and `initCapacity()` in Zig
- 🟦 **TypeScript**: New high-performance TypeScript implementation with full API parity

This version represents a feature-complete, production-ready implementation with:
- ✅ **Complete API**: All 12 core methods implemented across all five languages
- ✅ **Comprehensive Testing**: 20+ test functions covering all functionality and edge cases
- ✅ **Cross-Language Parity**: Identical API and behavior across C++, Go, Rust, Zig, and TypeScript
- ✅ **Professional Documentation**: Detailed usage guides and design explanations
- ✅ **Performance Optimized**: O(1) item lookup, template specialization, memory efficiency
- ✅ **Truly Generic**: All implementations support user-defined item types

All five implementations share synchronized version numbers to ensure feature compatibility and consistent user experience.

## Getting Started

**New in v2.3.0**: Check out the [Dijkstra's Algorithm Example](https://github.com/PCfVW/d-Heap-priority-queue/tree/master/examples/dijkstra) to see d-ary heaps in action with a classic shortest path algorithm from the Network Flows textbook (now available in Go, TypeScript, and more).

Explore the language-specific implementations:

| Language | README |
| --- | --- |
| ![C++17](https://img.shields.io/badge/C%2B%2B-17-blue.svg) | [Cpp/README.md](Cpp/README.md) |
| ![Go 1.21](https://img.shields.io/badge/Go-1.21-00ADD8.svg) | [Go/README.md](Go/README.md) |
| ![Rust Edition 2021](https://img.shields.io/badge/Rust-Edition_2021-orange.svg) | [Rust/README.md](Rust/README.md) |
| ![Zig 0.15.2](https://img.shields.io/badge/Zig-0.15.2-f7a41d.svg) | [zig/README.md](zig/README.md) |
| ![TypeScript 5.3](https://img.shields.io/badge/TypeScript-5.3-blue.svg) | [TypeScript/README.md](TypeScript/README.md) |

## License

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-green.svg)](LICENSE)

This project is licensed under the **Apache License 2.0** - see the [LICENSE](https://github.com/PCfVW/d-Heap-priority-queue/blob/master/LICENSE) file for details.

Copyright © 2023-2025 Eric Jacopin

## References

- Ahuja, Magnanti & Orlin, **Network Flows** (1993), Section A.3 on d-Heaps
