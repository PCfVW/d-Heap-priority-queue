# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[2.1.1]: https://github.com/your-username/priority-queues/compare/v2.0.0...v2.1.1
[2.0.0]: https://github.com/your-username/priority-queues/compare/v1.1.0...v2.0.0
[1.1.0]: https://github.com/your-username/priority-queues/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/your-username/priority-queues/releases/tag/v1.0.0