![Zig 0.15.2](https://img.shields.io/badge/Zig-0.15.2-f7a41d.svg)
![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-green.svg)

# d-Heap Priority Queue (Zig 0.15.2) v1.1.0

This is a generic d-ary heap priority queue supporting both min-queue and max-queue behavior through a comparator.

## Strengths

- **Flexible behavior**: min-heap or max-heap via comparator functions, and configurable arity `d` at construction time.
- **Efficient operations** on n items (see reference below):
  - O(1): access the highest-priority item (`front()`).
  - O(log_d n): `insert()` and upward reheapification.
  - O(d · log_d n): delete-top (`pop()`), with up to d children examined per level.
- **O(1) item lookup**: internal hash map tracks positions by item identity, enabling efficient priority updates for existing items.
- **Practical API**: `insert`, `front`, `pop`, `increasePriority`, `decreasePriority`, `isEmpty`, `len`, `clear`, `d`, `toString`.
- **Unified API**: Cross-language standardized methods matching C++ and Rust implementations for consistent usage patterns.
- **Memory safety**: Explicit allocator management following Zig best practices.

## How to use (basic example)

Define your item type and provide:
- **Hash** for identity (e.g., by stable id/number),
- **Equality** for identity (same identity as hash),
- **Comparator** on priority (e.g., by `cost`) determining min- or max-queue.

```zig
const std = @import("std");
const DHeap = @import("d_heap.zig").DHeap;
const MinByCost = @import("d_heap.zig").MinByCost;
const MaxByCost = @import("d_heap.zig").MaxByCost;
const Item = @import("types.zig").Item;

pub fn main() !void {
    // Setup allocator
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Create min-queue by cost (lower cost = higher priority)
    var pq = try DHeap.init(3, MinByCost, allocator);
    defer pq.deinit();

    // Insert items
    try pq.insert(Item{ .number = 1, .cost = 10 });
    try pq.insert(Item{ .number = 2, .cost = 5 });

    // Get highest priority item (lowest cost here)
    if (pq.front()) |top| {
        std.debug.print("Top: {any}\n", .{top});
    }

    // Increase priority of an existing item (e.g., decrease cost in a min-queue)
    const updated = Item{ .number = 1, .cost = 3 }; // same identity, new priority
    try pq.increasePriority(updated);               // repositions item upward

    // Decrease priority of an existing item (e.g., increase cost in a min-queue)
    const decreased = Item{ .number = 2, .cost = 8 }; // same identity, new priority
    try pq.decreasePriority(decreased);               // repositions item downward

    // Remove current highest-priority item
    _ = pq.pop();

    // Clear the queue (optionally reset arity)
    try pq.clear(null); // or pq.clear(4) to change d to 4

    // Unified API methods (cross-language consistency)
    std.debug.print("Size: {}\n", .{pq.len()});       // Get number of items
    std.debug.print("Empty: {}\n", .{pq.isEmpty()});  // Check if empty
    std.debug.print("Arity: {}\n", .{pq.d()});        // Get d value
    const str = try pq.toString();                    // String output
    defer allocator.free(str);
    std.debug.print("Contents: {s}\n", .{str});
}
```

## Compilation

```bash
# Compile and run the demo
zig build-exe src/main.zig
./main

# Or use the build system
zig build run

# Run tests (if available)
zig build test

# Check formatting
zig fmt --check src/
```

## Requirements

- **Zig 0.15.2** or compatible version
- No external dependencies (uses only Zig standard library)

Notes:
- Identity (hash/equality) should be stable; priority values may change over time.
- Only the highest-priority item can be removed directly (`pop()`).
- Memory management uses explicit allocators - always call `deinit()` to free resources.

## What is a d-Heap?

- A [d-ary heap](https://en.wikipedia.org/wiki/D-ary_heap) is a tree where each node has up to d children, the root holds the highest priority, children are unordered, and priorities decrease along any root-to-leaf path.
- Time complexities over n items (cf. reference):
  - O(1): access top
  - O(d · log_d n): delete-top
  - O(log_d n): insert and upward update

## API Reference

### Types

- **`DHeap`**: Main priority queue structure
- **`Comparator`**: Struct containing a `higher_priority` function pointer
- **`MinByCost`**: Pre-defined comparator for min-heap by cost
- **`MaxByCost`**: Pre-defined comparator for max-heap by cost
- **`Item`**: Example item type with `number` (identity) and `cost` (priority)

### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `init()` | `fn init(depth: usize, comparator: Comparator, allocator: Allocator) !DHeap` | Create new heap with arity d |
| `deinit()` | `fn deinit(self: *Self) void` | Free all resources |
| `len()` | `fn len(self: Self) usize` | Get number of items |
| `isEmpty()` | `fn isEmpty(self: Self) bool` | Check if queue is empty |
| `d()` | `fn d(self: Self) usize` | Get arity (children per node) |
| `front()` | `fn front(self: *Self) ?Item` | Get highest-priority item |
| `insert()` | `fn insert(self: *Self, item: Item) !void` | Add new item to queue |
| `increasePriority()` | `fn increasePriority(self: *Self, updated_item: Item) !void` | Make item more important |
| `decreasePriority()` | `fn decreasePriority(self: *Self, updated_item: Item) !void` | Make item less important |
| `pop()` | `fn pop(self: *Self) ?Item` | Remove highest-priority item |
| `clear()` | `fn clear(self: *Self, new_depth: ?usize) !void` | Clear all items, optionally reset arity |
| `toString()` | `fn toString(self: Self) ![]const u8` | Get string representation (caller owns memory) |

### Error Types

- `DepthMustBePositive`: Arity d must be at least 1
- `ItemNotFound`: Item not found in queue during priority update
- `AllocatorError`: Memory allocation failed

## Zig-Specific Implementation Details

### Memory Management
- Uses explicit allocators passed at initialization
- All allocations go through the provided allocator
- `toString()` returns owned memory that must be freed by caller
- Always call `deinit()` to prevent memory leaks

### Naming Conventions
- Functions use camelCase: `isEmpty()`, `increasePriority()`
- Types use PascalCase: `DHeap`, `Comparator`
- Constants use PascalCase: `MinByCost`, `MaxByCost`

**Note**: Zig uses camelCase for functions (e.g., `isEmpty`, `increasePriority`) following Zig conventions, while C++ and Rust use snake_case (e.g., `is_empty`, `increase_priority`). The functionality is identical across all implementations.

### Error Handling
- Functions that can fail return error unions (`!Type`)
- Use `try` to propagate errors or `catch` to handle them
- `front()` and `pop()` return optional types (`?Item`) for empty queue case

## Reference

Section A.3, [d-Heaps](https://en.wikipedia.org/wiki/D-ary_heap), pp. 773–778 of Ravindra Ahuja, Thomas Magnanti & James Orlin, **Network Flows** (Prentice Hall, 1993). Book info: https://mitmgmtfaculty.mit.edu/jorlin/network-flows/