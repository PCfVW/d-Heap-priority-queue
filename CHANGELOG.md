# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.4.0] - 2026-01-07

### Added
- **Interactive React Flow Demo**: Live visualization of d-ary heap priority queues with Dijkstra's algorithm
  - Dual-panel layout: heap tree (left) and graph (right)
  - Real-time vertex state coloring (unvisited/in-queue/processed)
  - Arity toggle (d=2, d=4, d=8) for performance comparison
  - Race Mode: Compare all three arities simultaneously
  - Timeline scrubber and playback controls
  - Live at: https://eric-jacopin.github.io/Priority-Queues/
- **TypeScript Instrumentation**: Opt-in comparison counting for performance analysis
  - `instrumentComparator()` for tracking comparisons per operation
  - Theoretical complexity functions for Big-O reference
  - Zero overhead when not used
- **GitHub Actions Deployment**: Automatic demo deployment to GitHub Pages
- **Zig Bulk Operations**: Added `insertMany()` with Floyd's O(n) heapify algorithm for efficient batch insertion
- **Zig Pop Many**: Added `popMany(count)` method for removing multiple highest-priority items at once
- **Zig Array Access**: Added `toArray()` method for integration with Zig slices
- **Zig Snake_case Aliases**: Added `insert_many()`, `pop_many()`, `to_array()` aliases for cross-language consistency
- **Go Snake_case Alias**: Added `Increase_priority_by_index()` alias for consistency with other Go aliases

### Changed
- **Go Internal Consistency**: Now uses `Position` type alias internally instead of raw `int` for documentation clarity
- **Go Documentation**: Renamed `right` to `rightBound` in `bestChildPosition()` with clarifying comments
- **Zig Error Handling**: `swapItems()` now properly propagates errors instead of silently ignoring allocation failures
- **Zig pop() Signature**: Changed from `?T` to `!?T` to properly propagate potential errors from internal operations
- **README Zig Extensions**: Updated to document new bulk operations and array access features

### Technical Details (Demo)
- React 18 + TypeScript + Vite 7
- React Flow (@xyflow/react) with dagre layout
- Uses d-ary-heap NPM package with instrumentation hooks
- Keyboard shortcuts: Space (play/pause), arrows (step), R (reset), 1/2/3 (arity)

### Technical Details (Zig)
- `insertMany()` uses Floyd's heapify algorithm when starting from empty heap (O(n) vs O(n log n))
- `popMany()` returns caller-owned allocated slice that must be freed
- `toArray()` returns a copy of all items in heap order (not priority order)
- Error propagation ensures heap invariants are never silently corrupted

### Technical Details (Go)
- `Position` type alias now used in `swap()`, `bestChildPosition()`, `moveUp()`, `moveDown()`, and `IncreasePriorityByIndex()`
- Variable naming improved for clarity in child position calculations

## [2.3.0] - 2025-12-27

### Added
- **Go Implementation**: Complete d-heap priority queue library in Go with full API parity
- **Go Generics**: Full generic support with `PriorityQueue[T any, K comparable]`
- **Go Dijkstra Example**: Complete working example demonstrating d-heap usage in `examples/dijkstra/Go/`
- **Comparator Utilities**: `MinBy()`, `MaxBy()` factory functions and pre-built comparators for common types
- **Bulk Operations**: `InsertMany()`, `PopMany()` for efficient batch processing with Floyd's heapify algorithm
- **Safe Access**: `Peek()` method returns `(T, bool)` for safe, non-panicking access
- **Go Stringer Interface**: Implements `fmt.Stringer` for automatic `fmt.Print()` support
- **Cross-Language Aliases**: Snake_case method aliases (`Is_empty()`, `Increase_priority()`, etc.) for easy porting
- **Go Workspace**: Added `go.work` file for multi-module development
- **Rust Documentation**: Created comprehensive doctests for all public methods and types
- **Rust Package Metadata**: Enhanced Cargo.toml with complete publication-ready metadata

### Changed
- **Five Language Support**: Project now supports C++, Go, Rust, Zig, and TypeScript
- **Updated Documentation**: Root README.md updated to include Go in all API tables and comparisons
- **Version Information**: Updated to reflect Go implementation as primary feature of v2.3.0
- **Rust Package Naming**: Updated package name to `d-ary-heap` for clarity and consistency
- **Rust Library Naming**: Standardized library name to `d_ary_heap` throughout the codebase

### Technical Details (Go)
- Go module path: `github.com/PCfVW/d-Heap-priority-queue/Go`
- Package name: `dheap`
- Requires Go 1.21+ for generics support
- 47 test cases covering all functionality
- Idiomatic Go error handling: errors for recoverable conditions, panics for programmer errors

## [2.2.0] - 2025-12-26

### Added
- **Examples Infrastructure**: Complete `examples/dijkstra/` directory structure
- **TypeScript Dijkstra Implementation**: Working example with path reconstruction
- **Shared Test Graph**: JSON format graph from Network Flows textbook (Figure 4.7, page 110)
- **Performance Comparisons**: Demonstrates d-ary heap advantages across different arities
- **Algorithm Documentation**: Comprehensive README with complexity analysis

### Enhanced
- **Documentation**: Added detailed algorithm explanation and expected results
- **Visual Diagrams**: Mermaid graph visualization with red edge weights
- **Cross-Language Foundation**: Established pattern for future language implementations

## [2.1.2] - 2025-12-25

### Added
- **Cross-Language Error Handling Documentation**: Comprehensive table showing error handling patterns for each language
- **Return Type Variations Guide**: Detailed documentation of return type differences and safety recommendations
- **Cross-Language Compatibility**: Added `to_string()` alias to Zig implementation for consistency with C++/Rust
- **Language-Specific Error Handling**: Enhanced language extensions documentation with error handling approaches

### Fixed
- **Misleading API Documentation**: Replaced false "unified API" claims with accurate cross-language reference table
- **Method Naming Confusion**: Clarified actual method names per language (camelCase vs snake_case)
- **TypeScript Alias Documentation**: Clear distinction between primary methods and cross-language compatibility aliases
- **Missing Zig Method**: Added documented `to_string()` method alongside existing `toString()`

### Changed
- **Documentation Structure**: Reorganized API documentation to accurately reflect language-specific conventions
- **Error Handling Clarity**: Added best practices and safety recommendations for each language's error handling approach
- **API Reference Format**: Updated from misleading unified table to accurate cross-language mapping

### Technical Details
- Updated main README.md with comprehensive error handling and return type documentation
- Enhanced TypeScript README with clear primary vs alias method documentation
- Added Zig `to_string()` alias method with proper documentation
- Resolved all critical issues identified in comprehensive API audit
- Maintained full backward compatibility across all implementations

## [2.1.1] - 2025-12-18

### Added
- **TypeScript Tooling**: Complete ESLint configuration with TypeScript support
- **Module Type**: Added `"type": "module"` to package.json for better Node.js compatibility
- **Publishing Pipeline**: Complete NPM publishing setup with automated build process

### Fixed
- **ESLint Configuration**: Updated to ESLint v9 flat config format
- **Build Warnings**: Eliminated module type warnings during build process

### Technical Details
- Added ESLint v9 with `@typescript-eslint/eslint-plugin` and `@typescript-eslint/parser`
- Created `eslint.config.js` with modern flat configuration
- Updated package.json with proper module type declaration
- All linting, type checking, and build processes now pass cleanly

## [2.0.0] - 2025-12-18

### Added
- **Zig 0.15.2 Support**: Complete compatibility with latest Zig version
- **Generic Implementation**: Zig implementation now fully generic - use your own item types
- **Comprehensive Test Suite**: 20+ tests covering all functionality and edge cases
- **Module Export**: Zig can now be used as a dependency in other projects via `build.zig.zon`
- **New Methods**: Added `peek()` alias for `front()` and `initCapacity()` for pre-allocation
- **Better Error Handling**: Removed `unreachable` from error paths, proper error propagation
- **Cross-Language API Parity**: All three implementations (C++, Rust, Zig) now have identical APIs

### Changed
- **BREAKING**: Zig API updated for 0.15.2 compatibility (ArrayList, HashMap, format function changes)
- **BREAKING**: Generic type construction now required for custom item types in Zig
- **Improved Documentation**: Updated all READMEs with generic API examples and usage patterns
- **Performance**: Optimized `toString()` method to use writer interface directly

### Technical Details
- Updated ArrayList API for Zig 0.15.2 (now unmanaged, allocator passed to methods)
- Updated HashMap API (managed version with `init(allocator)`)
- Updated build system for new `root_module` and `createModule()` API
- Updated format function signature (`format(self, writer)` instead of 4-argument version)
- Added proper module imports system in `build.zig`

### Migration Guide
For existing Zig users upgrading from v1.x:
- Update to Zig 0.15.2
- Use `DHeapItem` for the built-in Item type (backward compatible)
- For custom types, use the new generic API: `DHeap(T, HashContext(T), Comparator(T))`

## [1.1.0] - 2025-12-17

### Added
- Complete Zig implementation with full API parity
- Unified API method names across all three languages
- `contains()` method for O(1) membership testing
- Comprehensive documentation and examples

### Changed
- Standardized method names for cross-language consistency
- Improved error handling in Rust implementation
- Enhanced documentation across all implementations

## [1.0.0] - 2025-12-16

### Added
- Initial release with C++ and Rust implementations
- Generic d-ary heap with configurable arity
- O(1) item lookup using HashMap/unordered_map
- Min-heap and max-heap support via comparators
- Priority update operations (increase/decrease)
- Comprehensive test suites
- Professional documentation

[2.4.0]: https://github.com/PCfVW/d-Heap-priority-queue/compare/v2.3.0...v2.4.0
[2.3.0]: https://github.com/PCfVW/d-Heap-priority-queue/compare/v2.2.0...v2.3.0
[2.2.0]: https://github.com/PCfVW/d-Heap-priority-queue/compare/v2.1.2...v2.2.0
[2.1.2]: https://github.com/PCfVW/d-Heap-priority-queue/compare/v2.1.1...v2.1.2
[2.1.1]: https://github.com/your-username/priority-queues/compare/v2.0.0...v2.1.1
[2.0.0]: https://github.com/your-username/priority-queues/compare/v1.1.0...v2.0.0
[1.1.0]: https://github.com/your-username/priority-queues/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/your-username/priority-queues/releases/tag/v1.0.0