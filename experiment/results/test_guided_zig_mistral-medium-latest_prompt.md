Implement a d-ary heap priority queue in Zig.

Requirements:
1. The heap arity (d) should be configurable at construction time
2. Items have two distinct properties: an identity (for equality) and a priority (for ordering)
3. Two items are equal if they have the same identity, regardless of priority
4. The queue should support O(1) lookup to check if an item exists
5. Implement a min-heap where lower priority values have higher importance

Required operations:
- insert(item): Add an item to the queue
- pop(): Remove and return the item with highest priority (lowest value)
- front(): Return the item with highest priority without removing it
- increase_priority(item): Update an existing item to have higher priority (lower value)
- decrease_priority(item): Update an existing item to have lower priority (higher value)
- contains(item): Check if an item with the given identity exists
- len(): Return the number of items in the queue
- is_empty(): Return whether the queue is empty

Your implementation must pass all of the following tests:

//! Test corpus for d-ary heap priority queue operations.
//!
//! Part of: Amphigraphic-Strict × d-ary Heap Priority Queue research
//!
//! Run with: zig build test

const std = @import("std");
const testing = std.testing;

const d_heap = @import("d_heap");
const DHeapItem = d_heap.DHeapItem;
const MinByCost = d_heap.MinByCost;
const Item = d_heap.Item;

const DHeap = DHeapItem;

// =============================================================================
// insert() Tests - Spec: specifications/insert.md
// =============================================================================

test "insert_postcondition_item_findable" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeap.init(4, MinByCost, allocator);
    defer pq.deinit();

    const item = Item.init(50, 50);
    try pq.insert(item);

    try testing.expect(pq.contains(item));
}

test "insert_invariant_heap_property" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeap.init(4, MinByCost, allocator);
    defer pq.deinit();

    // Insert items in arbitrary order
    const items = [_]Item{
        Item.init(30, 30),
        Item.init(10, 10),
        Item.init(50, 50),
        Item.init(20, 20),
        Item.init(40, 40),
    };

    for (items) |item| {
        try pq.insert(item);
        try testing.expect(pq.front().?.cost <= 30);
    }

    try testing.expectEqual(@as(u32, 10), pq.front().?.cost);
}

test "insert_size_increments" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeap.init(4, MinByCost, allocator);
    defer pq.deinit();

    var i: u32 = 0;
    while (i < 5) : (i += 1) {
        const size_before = pq.len();
        try pq.insert(Item.init(i, i * 10));
        try testing.expectEqual(size_before + 1, pq.len());
    }
}

test "insert_edge_becomes_front_if_highest_priority" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeap.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(100, 100));
    try pq.insert(Item.init(50, 50));
    try pq.insert(Item.init(10, 10));

    try testing.expectEqual(@as(u32, 10), pq.front().?.cost);

    try pq.insert(Item.init(1, 1));

    try testing.expectEqual(@as(u32, 1), pq.front().?.cost);
}

// =============================================================================
// pop() Tests - Spec: specifications/pop.md
// =============================================================================

test "pop_postcondition_returns_minimum" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeap.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(30, 30));
    try pq.insert(Item.init(10, 10));
    try pq.insert(Item.init(20, 20));

    try testing.expectEqual(@as(u32, 10), pq.front().?.cost);

    const popped = try pq.pop();
    try testing.expectEqual(@as(u32, 10), popped.?.cost);

    // After pop, item should not be in heap
    try testing.expect(!pq.contains(Item.init(10, 10)));
}

test "pop_invariant_maintains_heap_property" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeap.init(4, MinByCost, allocator);
    defer pq.deinit();

    const items = [_]Item{
        Item.init(50, 50),
        Item.init(20, 20),
        Item.init(80, 80),
        Item.init(10, 10),
        Item.init(60, 60),
        Item.init(30, 30),
        Item.init(70, 70),
        Item.init(40, 40),
    };

    for (items) |item| {
        try pq.insert(item);
    }

    const expected_order = [_]u32{ 10, 20, 30, 40 };
    for (expected_order) |expected| {
        try testing.expectEqual(expected, pq.front().?.cost);
        _ = try pq.pop();
    }
}

test "pop_size_decrements" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeap.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(10, 10));
    try pq.insert(Item.init(20, 20));
    try pq.insert(Item.init(30, 30));

    var expected_size: usize = 2;
    while (expected_size > 0) : (expected_size -= 1) {
        const size_before = pq.len();
        _ = try pq.pop();
        try testing.expectEqual(size_before - 1, pq.len());
    }
}

test "pop_edge_empty_returns_null" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeap.init(4, MinByCost, allocator);
    defer pq.deinit();

    try testing.expect(pq.isEmpty());
    try testing.expectEqual(@as(?Item, null), pq.front());
}

// =============================================================================
// front() Tests - Spec: specifications/front.md
// =============================================================================

test "front_postcondition_returns_minimum" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeap.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(30, 30));
    try pq.insert(Item.init(10, 10));
    try pq.insert(Item.init(20, 20));

    try testing.expectEqual(@as(u32, 10), pq.front().?.cost);
}

test "front_invariant_no_modification" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeap.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(30, 30));
    try pq.insert(Item.init(10, 10));
    try pq.insert(Item.init(20, 20));

    const first = pq.front().?;
    const second = pq.front().?;
    const third = pq.front().?;

    try testing.expectEqual(first.cost, second.cost);
    try testing.expectEqual(second.cost, third.cost);
}

test "front_size_unchanged" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeap.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(10, 10));
    try pq.insert(Item.init(20, 20));
    try pq.insert(Item.init(30, 30));

    const size_before = pq.len();

    var i: usize = 0;
    while (i < 5) : (i += 1) {
        _ = pq.front();
    }

    try testing.expectEqual(size_before, pq.len());
}

test "front_edge_empty_returns_null" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeap.init(4, MinByCost, allocator);
    defer pq.deinit();

    try testing.expectEqual(@as(?Item, null), pq.front());
}

// =============================================================================
// increasePriority() Tests - Spec: specifications/increase_priority.md
// =============================================================================

test "increase_priority_postcondition_priority_changed" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeap.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(50, 50));
    try pq.insert(Item.init(30, 30));

    try testing.expectEqual(@as(u32, 30), pq.front().?.cost);

    // Increase priority of item 50 (lower cost = higher priority in min-heap)
    const updated = Item.init(50, 10);
    try pq.increasePriority(updated);

    try testing.expectEqual(@as(u32, 10), pq.front().?.cost);
}

test "increase_priority_invariant_heap_property" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeap.init(4, MinByCost, allocator);
    defer pq.deinit();

    const items = [_]Item{
        Item.init(80, 80),
        Item.init(60, 60),
        Item.init(40, 40),
        Item.init(20, 20),
        Item.init(100, 100),
        Item.init(50, 50),
    };

    for (items) |item| {
        try pq.insert(item);
    }

    try testing.expectEqual(@as(u32, 20), pq.front().?.cost);

    const updated = Item.init(80, 5);
    try pq.increasePriority(updated);

    try testing.expectEqual(@as(u32, 5), pq.front().?.cost);
}

test "increase_priority_position_item_moves_up" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeap.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(10, 10));
    try pq.insert(Item.init(50, 50));
    try pq.insert(Item.init(100, 100));

    try testing.expect(pq.front().?.number != 100);

    const updated = Item.init(100, 1);
    try pq.increasePriority(updated);

    try testing.expectEqual(@as(u32, 100), pq.front().?.number);
}

test "increase_priority_size_unchanged" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeap.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(50, 50));
    try pq.insert(Item.init(30, 30));
    try pq.insert(Item.init(70, 70));

    const size_before = pq.len();

    const updated = Item.init(70, 10);
    try pq.increasePriority(updated);

    try testing.expectEqual(size_before, pq.len());
}

test "increase_priority_edge_not_found_returns_error" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeap.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(50, 50));

    const nonexistent = Item.init(999, 10);
    const result = pq.increasePriority(nonexistent);
    try testing.expectError(error.ItemNotFound, result);
}

// =============================================================================
// decreasePriority() Tests - Spec: specifications/decrease_priority.md
// =============================================================================

test "decrease_priority_postcondition_priority_changed" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeap.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(10, 10));
    try pq.insert(Item.init(30, 30));

    try testing.expectEqual(@as(u32, 10), pq.front().?.number);

    const updated = Item.init(10, 50);
    try pq.decreasePriority(updated);

    try testing.expectEqual(@as(u32, 30), pq.front().?.number);

    _ = try pq.pop();
    try testing.expectEqual(@as(u32, 50), pq.front().?.cost);
}

test "decrease_priority_invariant_heap_property" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeap.init(4, MinByCost, allocator);
    defer pq.deinit();

    const items = [_]Item{
        Item.init(10, 10),
        Item.init(30, 30),
        Item.init(50, 50),
        Item.init(70, 70),
        Item.init(20, 20),
        Item.init(40, 40),
    };

    for (items) |item| {
        try pq.insert(item);
    }

    try testing.expectEqual(@as(u32, 10), pq.front().?.number);

    const updated = Item.init(10, 100);
    try pq.decreasePriority(updated);

    try testing.expectEqual(@as(u32, 20), pq.front().?.cost);
}

test "decrease_priority_position_item_moves_down" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeap.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(10, 10));
    try pq.insert(Item.init(50, 50));
    try pq.insert(Item.init(60, 60));
    try pq.insert(Item.init(70, 70));

    try testing.expectEqual(@as(u32, 10), pq.front().?.number);

    const updated = Item.init(10, 100);
    try pq.decreasePriority(updated);

    try testing.expect(pq.front().?.number != 10);
    try testing.expectEqual(@as(u32, 50), pq.front().?.number);
}

test "decrease_priority_size_unchanged" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeap.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(10, 10));
    try pq.insert(Item.init(30, 30));
    try pq.insert(Item.init(50, 50));

    const size_before = pq.len();

    const updated = Item.init(10, 100);
    try pq.decreasePriority(updated);

    try testing.expectEqual(size_before, pq.len());
}

test "decrease_priority_edge_not_found_returns_error" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeap.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(50, 50));

    const nonexistent = Item.init(999, 100);
    const result = pq.decreasePriority(nonexistent);
    try testing.expectError(error.ItemNotFound, result);
}


Provide a complete, working implementation that passes all tests.