![Zig 0.15.2](https://img.shields.io/badge/Zig-0.15.2-f7a41d.svg)
![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-green.svg)

# d-Heap Priority Queue (Zig 0.15.2) v2.0.0

A **generic** d-ary heap priority queue supporting both min-queue and max-queue behavior through comparator functions.

## What's New in v2.0.0

- **Generic implementation**: Use your own item types (not limited to built-in `Item`)
- **Improved API**: Added `peek()` alias, `initCapacity()` for pre-allocation
- **Better error handling**: Removed `unreachable` from error paths
- **Comprehensive tests**: 20+ tests covering all functionality
- **Module export**: Can be used as a dependency in other Zig projects
- **Zig 0.15.2 compatibility**: Updated for latest Zig API changes

## Strengths

- **Truly generic**: Define your own item types with custom hash/equality
- **Flexible behavior**: min-heap or max-heap via comparator functions, configurable arity `d`
- **Efficient operations** on n items:
  - O(1): access highest-priority item (`front()`/`peek()`), membership test (`contains()`)
  - O(log_d n): `insert()` and `increasePriority()`
  - O(d · log_d n): `pop()` and `decreasePriority()`
- **O(1) item lookup**: internal hash map tracks positions by item identity
- **Unified API**: Cross-language standardized methods matching C++ and Rust implementations
- **Memory safety**: Explicit allocator management following Zig best practices

## Quick Start (Using Built-in Item Type)

```zig
const std = @import("std");
const d_heap = @import("d_heap.zig");

// Use pre-configured type for the built-in Item type
const DHeapItem = d_heap.DHeapItem;
const MinByCost = d_heap.MinByCost;
const Item = d_heap.Item;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Create min-heap (lower cost = higher priority)
    var pq = try DHeapItem.init(3, MinByCost, allocator);
    defer pq.deinit();

    // Insert items
    try pq.insert(Item.init(1, 10));
    try pq.insert(Item.init(2, 5));

    // Get highest priority item
    if (pq.front()) |top| {
        std.debug.print("Top: {any}\n", .{top});  // (2, 5)
    }

    // Update priority
    try pq.increasePriority(Item.init(1, 3));  // Item 1 now has cost 3

    // Check membership
    if (pq.contains(Item.init(1, 0))) {  // cost doesn't matter for lookup
        std.debug.print("Item 1 exists\n", .{});
    }

    // Pop items in priority order
    while (pq.pop()) |item| {
        std.debug.print("Popped: {any}\n", .{item});
    }
}
```

## Using Custom Item Types (Generic API)

```zig
const std = @import("std");
const d_heap = @import("d_heap.zig");

// Define your custom item type
const Task = struct {
    id: u64,
    name: []const u8,
    priority: i32,

    // Required: hash function for identity
    pub fn hash(self: Task) u64 {
        var hasher = std.hash.Wyhash.init(0);
        std.hash.autoHash(&hasher, self.id);
        return hasher.final();
    }

    // Required: equality function for identity
    pub fn eql(a: Task, b: Task) bool {
        return a.id == b.id;
    }
};

// Create comparator for min-heap by priority
fn taskLessThan(a: Task, b: Task) bool {
    return a.priority < b.priority;
}

// Create the heap type
const TaskHeap = d_heap.DHeap(
    Task,
    d_heap.HashContext(Task),
    d_heap.Comparator(Task),
);

const TaskComparator = d_heap.Comparator(Task){ .cmp = taskLessThan };

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try TaskHeap.init(4, TaskComparator, allocator);
    defer heap.deinit();

    try heap.insert(.{ .id = 1, .name = "Build", .priority = 10 });
    try heap.insert(.{ .id = 2, .name = "Test", .priority = 5 });

    if (heap.front()) |task| {
        std.debug.print("Next task: {s}\n", .{task.name});  // "Test"
    }
}
```

## Building and Testing

```bash
# Build the demo executable
zig build

# Run the demo
zig build run

# Run all tests
zig build test

# Check formatting
zig fmt --check src/
```

## Using as a Dependency

Add to your `build.zig.zon`:

```zig
.dependencies = .{
    .d_heap = .{
        .url = "https://github.com/your-repo/priority-queues/archive/refs/heads/main.tar.gz",
        .hash = "...",
    },
},
```

Then in your `build.zig`:

```zig
const d_heap = b.dependency("d_heap", .{});
exe.root_module.addImport("d-heap", d_heap.module("d-heap"));
```

## API Reference

### Generic Type Construction

```zig
// Create a heap type for your item type T
const MyHeap = d_heap.DHeap(
    T,                        // Your item type
    d_heap.HashContext(T),    // Hash context (uses T.hash() and T.eql())
    d_heap.Comparator(T),     // Comparator wrapper
);
```

### Pre-configured Types

| Type | Description |
|------|-------------|
| `DHeapItem` | Pre-configured heap for built-in `Item` type |
| `Item` | Built-in item with `number` (identity) and `cost` (priority) |
| `MinByCost` | Comparator for min-heap by cost |
| `MaxByCost` | Comparator for max-heap by cost |
| `HashContext(T)` | Generic hash context for types with `hash()`/`eql()` methods |
| `Comparator(T)` | Generic comparator wrapper |

### Methods

| Method | Complexity | Description |
|--------|------------|-------------|
| `init(d, cmp, alloc)` | O(1) | Create heap with arity d |
| `initCapacity(d, cmp, alloc, cap)` | O(1) | Create with pre-allocated capacity |
| `deinit()` | O(n) | Free all resources |
| `len()` | O(1) | Get number of items |
| `isEmpty()` | O(1) | Check if empty |
| `d()` | O(1) | Get arity |
| `contains(item)` | O(1) | Check if item exists by identity |
| `front()` / `peek()` | O(1) | Get highest-priority item (null if empty) |
| `insert(item)` | O(log_d n) | Add new item |
| `increasePriority(item)` | O(log_d n) | Update item to higher priority |
| `decreasePriority(item)` | O(d·log_d n) | Update item to lower priority |
| `pop()` | O(d·log_d n) | Remove and return highest-priority item |
| `clear(new_d)` | O(1) | Clear all items, optionally change arity |
| `toString()` | O(n) | Get string representation (caller owns memory) |

### Error Types

| Error | Cause |
|-------|-------|
| `DepthMustBePositive` | Arity d must be ≥ 1 |
| `ItemNotFound` | Priority update on non-existent item |

## Item Type Requirements

Your custom item type must provide:

1. **Hash function**: `pub fn hash(self: T) u64`
   - Returns hash based on item identity (not priority)

2. **Equality function**: `pub fn eql(a: T, b: T) bool` or `pub fn eq(a: T, b: T) bool`
   - Compares items by identity (not priority)

3. **Comparator function**: `fn(a: T, b: T) bool`
   - Returns true if `a` has higher priority than `b`
   - For min-heap: `a.priority < b.priority`
   - For max-heap: `a.priority > b.priority`

## Cross-Language API Parity

| Operation | Zig | C++ | Rust |
|-----------|-----|-----|------|
| Create | `init(d, cmp, alloc)` | `PriorityQueue(d)` | `PriorityQueue::new(d)` |
| Size | `len()` | `len()` | `len()` |
| Empty check | `isEmpty()` | `is_empty()` | `is_empty()` |
| Arity | `d()` | `d()` | `d()` |
| Membership | `contains(item)` | `contains(item)` | `contains(&item)` |
| Top item | `front()` / `peek()` | `front()` | `front()` |
| Insert | `insert(item)` | `insert(item)` | `insert(item)` |
| Increase priority | `increasePriority(item)` | `increase_priority(item)` | `increase_priority(item)` |
| Decrease priority | `decreasePriority(item)` | `decrease_priority(item)` | `decrease_priority(item)` |
| Remove top | `pop()` | `pop()` | `pop()` |
| Clear | `clear(new_d)` | `clear(new_d)` | `clear(new_d)` |
| String | `toString()` | `to_string()` | `to_string()` |

**Note**: Zig uses camelCase for functions following Zig conventions, while C++ and Rust use snake_case.

## What is a d-Heap?

A [d-ary heap](https://en.wikipedia.org/wiki/D-ary_heap) is a generalization of a binary heap where each node has up to d children instead of 2. This provides:

- **Shallower tree**: Height is log_d(n) instead of log_2(n)
- **Faster inserts**: O(log_d n) comparisons
- **Trade-off**: Pop operations examine up to d children per level

Optimal d depends on your workload. For insert-heavy workloads, larger d (3-4) often performs better.

## Reference

Section A.3, [d-Heaps](https://en.wikipedia.org/wiki/D-ary_heap), pp. 773–778 of Ravindra Ahuja, Thomas Magnanti & James Orlin, **Network Flows** (Prentice Hall, 1993).
