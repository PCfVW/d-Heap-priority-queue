![Zig 0.15.2](https://img.shields.io/badge/Zig-0.15.2-f7a41d.svg)
![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-green.svg)

# d-Heap Priority Queue (Zig 0.15.2) v2.5.0

A **generic** d-ary heap priority queue supporting both min-queue and max-queue behavior through comparator functions.

## What's New in v2.5.0

- **`updatePriority()`**: New method for when priority change direction is unknown
- **`getPosition()`**: O(1) lookup of item's index in the heap array
- **`*ByIndex` methods**: `increasePriorityByIndex()`, `decreasePriorityByIndex()` for index-based operations
- **Bulk operations**: `insertMany()` with Floyd's heapify, `popMany()` for batch removal
- **`toArray()`**: Get a copy of the internal heap array
- **Snake_case aliases**: `is_empty()`, `increase_priority()`, `decrease_priority()`, `update_priority()`, etc.
- **Fixed `decreasePriority()`**: Now only moves down (was incorrectly calling both directions)
- **Comprehensive tests**: 54 tests covering all functionality
- **Zig 0.15.2 compatibility**: Updated for latest Zig API changes

## Strengths

- **Truly generic**: Define your own item types with custom hash/equality
- **Flexible behavior**: min-heap or max-heap via comparator functions, configurable arity `d`
- **Efficient operations** on n items:
  - O(1): access highest-priority item (`front()`/`peek()`), membership test (`contains()`), position lookup (`getPosition()`)
  - O(log_d n): `insert()` and `increasePriority()`
  - O(d · log_d n): `pop()` and `decreasePriority()`
  - O((d+1) · log_d n): `updatePriority()` (when direction unknown)
  - O(n): `insertMany()` bulk insertion using Floyd's heapify algorithm
- **O(1) item lookup**: internal hash map tracks positions by item identity
- **Unified API**: Cross-language standardized methods matching TypeScript, Go, C++, and Rust implementations
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
    while (try pq.pop()) |item| {
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
        .url = "https://github.com/PCfVW/d-Heap-priority-queue/archive/refs/tags/v2.5.0.tar.gz",
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

### Query Methods

| Method | Complexity | Description |
|--------|------------|-------------|
| `len()` | O(1) | Get number of items |
| `isEmpty()` | O(1) | Check if empty (primary method) |
| `is_empty()` | O(1) | Alias for `isEmpty()` (cross-language compatibility) |
| `d()` | O(1) | Get arity |
| `contains(item)` | O(1) | Check if item exists by identity |
| `getPosition(item)` | O(1) | Get item's index in heap (primary method) |
| `get_position(item)` | O(1) | Alias for `getPosition()` (cross-language compatibility) |
| `front()` | O(1) | Get highest-priority item (null if empty) |
| `peek()` | O(1) | Alias for `front()` (cross-language: TypeScript/Go) |

### Modification Methods

| Method | Complexity | Description |
|--------|------------|-------------|
| `init(d, cmp, alloc)` | O(1) | Create heap with arity d |
| `initCapacity(d, cmp, alloc, cap)` | O(1) | Create with pre-allocated capacity |
| `deinit()` | O(n) | Free all resources |
| `insert(item)` | O(log_d n) | Add new item |
| `insertMany(items)` | O(n) | Insert multiple items (uses Floyd's heapify) |
| `insert_many(items)` | O(n) | Alias for `insertMany()` (cross-language compatibility) |
| `pop()` | O(d·log_d n) | Remove and return highest-priority item |
| `popMany(count)` | O(count·d·log_d n) | Remove and return multiple items |
| `pop_many(count)` | O(count·d·log_d n) | Alias for `popMany()` (cross-language compatibility) |
| `increasePriority(item)` | O(log_d n) | Update item to higher priority (primary method) |
| `increase_priority(item)` | O(log_d n) | Alias for `increasePriority()` (cross-language compatibility) |
| `increasePriorityByIndex(i)` | O(log_d n) | Update by index (primary method) |
| `increase_priority_by_index(i)` | O(log_d n) | Alias for `increasePriorityByIndex()` (cross-language compatibility) |
| `decreasePriority(item)` | O(d·log_d n) | Update item to lower priority (primary method) |
| `decrease_priority(item)` | O(d·log_d n) | Alias for `decreasePriority()` (cross-language compatibility) |
| `decreasePriorityByIndex(i)` | O(d·log_d n) | Update by index (primary method) |
| `decrease_priority_by_index(i)` | O(d·log_d n) | Alias for `decreasePriorityByIndex()` (cross-language compatibility) |
| `updatePriority(item)` | O((d+1)·log_d n) | Update item when direction unknown (primary method) |
| `update_priority(item)` | O((d+1)·log_d n) | Alias for `updatePriority()` (cross-language compatibility) |
| `clear(new_depth?)` | O(1) | Clear all items, optionally change arity |

### Utility Methods

| Method | Complexity | Description |
|--------|------------|-------------|
| `toString()` | O(n) | Get string representation (primary method, caller owns memory) |
| `to_string()` | O(n) | Alias for `toString()` (cross-language compatibility) |
| `toArray()` | O(n) | Get copy of internal array (primary method, caller owns memory) |
| `to_array()` | O(n) | Alias for `toArray()` (cross-language compatibility) |

### Error Types

| Error | Cause |
|-------|-------|
| `DepthMustBePositive` | Arity d must be ≥ 1 |
| `ItemNotFound` | Priority update on non-existent item |
| `IndexOutOfBounds` | Index-based operation with invalid index |

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

## Method Naming Convention

This Zig implementation follows **camelCase** as the primary naming convention (Zig standard), with **snake_case aliases** provided for cross-language compatibility:

- **Primary methods**: `isEmpty()`, `getPosition()`, `insertMany()`, `popMany()`, `increasePriority()`, `increasePriorityByIndex()`, `decreasePriority()`, `decreasePriorityByIndex()`, `updatePriority()`, `toString()`, `toArray()`
- **Compatibility aliases**: `is_empty()`, `get_position()`, `insert_many()`, `pop_many()`, `increase_priority()`, `increase_priority_by_index()`, `decrease_priority()`, `decrease_priority_by_index()`, `update_priority()`, `to_string()`, `to_array()`

Use the primary camelCase methods for Zig projects, and the snake_case aliases when porting code from C++/Rust implementations.

## Priority Update Semantics

This library uses **importance-based** semantics:

- **`increasePriority()`**: Make an item **more important** (moves toward heap root). Only moves up for O(log_d n) performance.
- **`decreasePriority()`**: Make an item **less important** (moves toward leaves). Only moves down for O(d · log_d n) performance.
- **`updatePriority()`**: Update when direction is **unknown**. Checks both directions for O((d+1) · log_d n) performance.

**When to use each:**
- Use `increasePriority()` when you know the item became more important
- Use `decreasePriority()` when you know the item became less important
- Use `updatePriority()` when you don't know which direction the priority changed

**Heap Context:**
- **Min-heap**: Lower priority values = higher importance
- **Max-heap**: Higher priority values = higher importance

## Cross-Language API Parity

| Operation | Zig | TypeScript | Go |
|-----------|-----|------------|-----|
| Create | `init(d, cmp, alloc)` | `new PriorityQueue(options)` | `New(d)` |
| Size | `len()` | `len()` | `Len()` |
| Empty check | `isEmpty()` / `is_empty()` | `isEmpty()` / `is_empty()` | `IsEmpty()` |
| Arity | `d()` | `d()` | `D()` |
| Membership | `contains(item)` | `contains(item)` | `Contains(item)` |
| Position | `getPosition(item)` / `get_position(item)` | `getPosition(item)` | `GetPosition(item)` |
| Top item | `front()` / `peek()` | `front()` / `peek()` | `Front()` / `Peek()` |
| Insert | `insert(item)` | `insert(item)` | `Insert(item)` |
| Insert many | `insertMany(items)` | `insertMany(items)` | `InsertMany(items)` |
| Increase priority | `increasePriority(item)` | `increasePriority(item)` | `IncreasePriority(item)` |
| Increase by index | `increasePriorityByIndex(i)` | `increasePriorityByIndex(i)` | `IncreasePriorityByIndex(i)` |
| Decrease priority | `decreasePriority(item)` | `decreasePriority(item)` | `DecreasePriority(item)` |
| Decrease by index | `decreasePriorityByIndex(i)` | `decreasePriorityByIndex(i)` | `DecreasePriorityByIndex(i)` |
| Update priority | `updatePriority(item)` | `updatePriority(item)` | `UpdatePriority(item)` |
| Remove top | `pop()` | `pop()` | `Pop()` |
| Remove many | `popMany(count)` | `popMany(count)` | `PopMany(count)` |
| Clear | `clear(new_d)` | `clear(new_d)` | `Clear(new_d)` |
| To array | `toArray()` / `to_array()` | `toArray()` | `ToArray()` |
| String | `toString()` / `to_string()` | `toString()` / `to_string()` | `String()` |

**Note**: Zig uses camelCase as primary (Zig convention), TypeScript uses camelCase (JavaScript convention), Go uses PascalCase (Go convention). All implementations provide snake_case aliases for cross-language compatibility.

## What is a d-Heap?

A [d-ary heap](https://en.wikipedia.org/wiki/D-ary_heap) is a generalization of a binary heap where each node has up to d children instead of 2. This provides:

- **Shallower tree**: Height is log_d(n) instead of log_2(n)
- **Faster inserts**: O(log_d n) comparisons
- **Trade-off**: Pop operations examine up to d children per level

Optimal d depends on your workload. For insert-heavy workloads, larger d (3-4) often performs better.

## Reference

Section A.3, [d-Heaps](https://en.wikipedia.org/wiki/D-ary_heap), pp. 773–778 of Ravindra Ahuja, Thomas Magnanti & James Orlin, **Network Flows** (Prentice Hall, 1993).
