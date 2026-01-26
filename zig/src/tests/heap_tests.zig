//! Comprehensive test suite for d-ary heap priority queue.
//!
//! Tests cover:
//! - Min-heap and max-heap ordering
//! - Priority update operations (increase/decrease)
//! - Membership testing (contains)
//! - Clear operations
//! - Edge cases (empty heap, single item)
//! - Error conditions
//!
//! Run with: zig build test

const std = @import("std");
const testing = std.testing;

// Import types from d_heap module (via build.zig imports)
const d_heap = @import("d_heap");
const DHeapItem = d_heap.DHeapItem;  // Pre-configured type for default Item
const MinByCost = d_heap.MinByCost;
const MaxByCost = d_heap.MaxByCost;
const Item = d_heap.Item;  // Re-exported from d_heap module

// Alias for convenience in tests
const DHeap = DHeapItem;

// ============================================================================
// Basic Operations Tests
// ============================================================================

test "basic heap operations" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    // Test empty state
    try testing.expect(heap.isEmpty());
    try testing.expectEqual(@as(usize, 0), heap.len());
    try testing.expectEqual(@as(usize, 2), heap.d());

    // Insert items
    try heap.insert(Item.init(5, 5));
    try heap.insert(Item.init(1, 1));

    // Verify state after inserts
    try testing.expect(!heap.isEmpty());
    try testing.expectEqual(@as(usize, 2), heap.len());

    // Front should be lowest cost (min-heap)
    try testing.expectEqual(@as(u32, 1), heap.front().?.cost);

    // Pop and verify
    _ = try heap.pop();
    try testing.expectEqual(@as(u32, 5), heap.front().?.cost);
    try testing.expectEqual(@as(usize, 1), heap.len());
}

// ============================================================================
// Min-Heap Ordering Tests
// ============================================================================

test "min heap ordering" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(3, MinByCost, allocator);
    defer heap.deinit();

    // Insert items in random order
    const values = [_]u32{ 20, 5, 22, 16, 18, 17, 12, 9 };
    for (values) |v| {
        try heap.insert(Item.init(v, v));
    }

    // Pop all items and verify non-decreasing order
    var last_cost: u32 = 0;
    var first = true;
    while (!heap.isEmpty()) {
        const top = heap.front().?;
        if (!first) {
            try testing.expect(top.cost >= last_cost);
        }
        first = false;
        last_cost = top.cost;
        _ = try heap.pop();
    }
}

test "min heap with different arities" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Test with d = 2, 3, 4, 5
    const arities = [_]usize{ 2, 3, 4, 5 };
    for (arities) |arity| {
        var heap = try DHeap.init(arity, MinByCost, allocator);
        defer heap.deinit();

        const values = [_]u32{ 50, 30, 70, 20, 60, 10, 80, 40 };
        for (values) |v| {
            try heap.insert(Item.init(v, v));
        }

        // Verify min is at front
        try testing.expectEqual(@as(u32, 10), heap.front().?.cost);

        // Verify ordering on pop
        var last: u32 = 0;
        var first = true;
        while (!heap.isEmpty()) {
            const top = heap.front().?;
            if (!first) {
                try testing.expect(top.cost >= last);
            }
            first = false;
            last = top.cost;
            _ = try heap.pop();
        }
    }
}

// ============================================================================
// Max-Heap Ordering Tests
// ============================================================================

test "max heap ordering" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(4, MaxByCost, allocator);
    defer heap.deinit();

    // Insert items
    const values = [_]u32{ 20, 5, 22, 16, 18, 17, 12, 9 };
    for (values) |v| {
        try heap.insert(Item.init(v, v));
    }

    // Pop all items and verify non-increasing order
    var last_cost: u32 = std.math.maxInt(u32);
    var first = true;
    while (!heap.isEmpty()) {
        const top = heap.front().?;
        if (!first) {
            try testing.expect(top.cost <= last_cost);
        }
        first = false;
        last_cost = top.cost;
        _ = try heap.pop();
    }
}

// ============================================================================
// Priority Update Tests
// ============================================================================

test "increase priority moves item up in min-heap" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(3, MinByCost, allocator);
    defer heap.deinit();

    // Insert items
    try heap.insert(Item.init(1, 10));
    try heap.insert(Item.init(2, 9));
    try heap.insert(Item.init(3, 8));

    // Item 3 should be at front (cost 8)
    try testing.expectEqual(@as(u32, 3), heap.front().?.number);

    // Increase priority of item 1 (lower cost = higher priority in min-heap)
    try heap.increasePriority(Item.init(1, 1));

    // Now item 1 should be at front
    try testing.expectEqual(@as(u32, 1), heap.front().?.number);
    try testing.expectEqual(@as(u32, 1), heap.front().?.cost);
}

test "increase priority in max-heap" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MaxByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 10));
    try heap.insert(Item.init(2, 20));
    try heap.insert(Item.init(3, 15));

    // Item 2 should be at front (cost 20)
    try testing.expectEqual(@as(u32, 2), heap.front().?.number);

    // Increase priority of item 1 (higher cost = higher priority in max-heap)
    try heap.increasePriority(Item.init(1, 50));

    // Now item 1 should be at front
    try testing.expectEqual(@as(u32, 1), heap.front().?.number);
    try testing.expectEqual(@as(u32, 50), heap.front().?.cost);
}

test "decrease priority moves item down in min-heap" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 5));
    try heap.insert(Item.init(2, 10));
    try heap.insert(Item.init(3, 15));

    // Item 1 should be at front (cost 5)
    try testing.expectEqual(@as(u32, 1), heap.front().?.number);

    // Decrease priority of item 1 (higher cost = lower priority in min-heap)
    try heap.decreasePriority(Item.init(1, 20));

    // Now item 2 should be at front (cost 10)
    try testing.expectEqual(@as(u32, 2), heap.front().?.number);
}

test "decrease priority in max-heap" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MaxByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 50));
    try heap.insert(Item.init(2, 30));
    try heap.insert(Item.init(3, 20));

    // Item 1 should be at front (cost 50)
    try testing.expectEqual(@as(u32, 1), heap.front().?.number);

    // Decrease priority of item 1 (lower cost = lower priority in max-heap)
    try heap.decreasePriority(Item.init(1, 10));

    // Now item 2 should be at front (cost 30)
    try testing.expectEqual(@as(u32, 2), heap.front().?.number);
}

// ============================================================================
// Contains Tests
// ============================================================================

test "contains returns true for existing item" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(42, 100));
    try heap.insert(Item.init(7, 50));

    // Contains should find items by identity (number), not cost
    try testing.expect(heap.contains(Item.init(42, 0)));
    try testing.expect(heap.contains(Item.init(7, 999)));
}

test "contains returns false for missing item" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 10));

    try testing.expect(!heap.contains(Item.init(999, 0)));
    try testing.expect(!heap.contains(Item.init(2, 10)));
}

test "contains on empty heap" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try testing.expect(!heap.contains(Item.init(1, 0)));
}

// ============================================================================
// Clear Tests
// ============================================================================

test "clear removes all items" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(3, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 10));
    try heap.insert(Item.init(2, 20));
    try heap.insert(Item.init(3, 30));

    try testing.expectEqual(@as(usize, 3), heap.len());

    try heap.clear(null);

    try testing.expectEqual(@as(usize, 0), heap.len());
    try testing.expect(heap.isEmpty());
    try testing.expectEqual(heap.front(), null);
}

test "clear with new depth changes arity" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 10));
    try testing.expectEqual(@as(usize, 2), heap.d());

    try heap.clear(5);

    try testing.expectEqual(@as(usize, 5), heap.d());
    try testing.expect(heap.isEmpty());
}

// ============================================================================
// Edge Cases
// ============================================================================

test "empty heap front returns null" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try testing.expectEqual(heap.front(), null);
}

test "empty heap pop returns null" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try testing.expectEqual(try heap.pop(), null);
}

test "single item operations" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(42, 100));

    try testing.expectEqual(@as(usize, 1), heap.len());
    try testing.expectEqual(@as(u32, 42), heap.front().?.number);
    try testing.expect(heap.contains(Item.init(42, 0)));

    // Update priority of single item
    try heap.increasePriority(Item.init(42, 50));
    try testing.expectEqual(@as(u32, 50), heap.front().?.cost);

    // Pop single item
    const popped = try heap.pop();
    try testing.expectEqual(@as(u32, 42), popped.?.number);
    try testing.expect(heap.isEmpty());
}

// ============================================================================
// Error Condition Tests
// ============================================================================

test "increase priority on missing item returns error" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 10));

    // Try to update non-existent item
    const result = heap.increasePriority(Item.init(999, 5));
    try testing.expectError(error.ItemNotFound, result);
}

test "decrease priority on missing item returns error" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 10));

    const result = heap.decreasePriority(Item.init(999, 50));
    try testing.expectError(error.ItemNotFound, result);
}

test "init with depth 0 returns error" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const result = DHeap.init(0, MinByCost, allocator);
    try testing.expectError(error.DepthMustBePositive, result);
}

test "clear with depth 0 returns error" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    const result = heap.clear(0);
    try testing.expectError(error.DepthMustBePositive, result);
}

// ============================================================================
// toString Tests
// ============================================================================

test "toString produces valid output" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    // Empty heap
    const empty_str = try heap.toString();
    defer allocator.free(empty_str);
    try testing.expectEqualStrings("{}", empty_str);

    // With items
    try heap.insert(Item.init(1, 10));
    try heap.insert(Item.init(2, 5));

    const str = try heap.toString();
    defer allocator.free(str);

    // Should start with { and end with }
    try testing.expect(str.len > 2);
    try testing.expectEqual(@as(u8, '{'), str[0]);
    try testing.expectEqual(@as(u8, '}'), str[str.len - 1]);
}

// ============================================================================
// Heap Property Maintenance Tests
// ============================================================================

test "heap property maintained after mixed operations" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(3, MinByCost, allocator);
    defer heap.deinit();

    // Insert items
    try heap.insert(Item.init(1, 50));
    try heap.insert(Item.init(2, 30));
    try heap.insert(Item.init(3, 70));
    try heap.insert(Item.init(4, 20));
    try heap.insert(Item.init(5, 60));

    // Item 4 should be at front (cost 20)
    try testing.expectEqual(@as(u32, 4), heap.front().?.number);

    // Increase priority of item 1 (50 -> 10)
    try heap.increasePriority(Item.init(1, 10));
    try testing.expectEqual(@as(u32, 1), heap.front().?.number);

    // Decrease priority of item 2 (30 -> 40) using updatePriority since we changed semantics
    try heap.updatePriority(Item.init(2, 40));
    try testing.expectEqual(@as(u32, 1), heap.front().?.number); // Still item 1

    // Pop front
    _ = try heap.pop();
    try testing.expectEqual(@as(u32, 4), heap.front().?.number); // Item 4 (cost 20)

    // Decrease priority of item 4 (20 -> 45) using updatePriority
    try heap.updatePriority(Item.init(4, 45));
    try testing.expectEqual(@as(u32, 2), heap.front().?.number); // Item 2 (cost 40)

    // Verify final ordering by popping all
    var last: u32 = 0;
    var first = true;
    while (!heap.isEmpty()) {
        const top = heap.front().?;
        if (!first) {
            try testing.expect(top.cost >= last);
        }
        first = false;
        last = top.cost;
        _ = try heap.pop();
    }
}

// ============================================================================
// updatePriority Tests
// ============================================================================

test "updatePriority moves item up when priority increases" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 10));
    try heap.insert(Item.init(2, 5));
    try heap.insert(Item.init(3, 15));

    // Item 2 is at front (cost 5)
    try testing.expectEqual(@as(u32, 2), heap.front().?.number);

    // Update item 3's priority to make it more important (lower cost)
    try heap.updatePriority(Item.init(3, 1));

    // Now item 3 should be at front
    try testing.expectEqual(@as(u32, 3), heap.front().?.number);
    try testing.expectEqual(@as(u32, 1), heap.front().?.cost);
}

test "updatePriority moves item down when priority decreases" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 5));
    try heap.insert(Item.init(2, 10));
    try heap.insert(Item.init(3, 15));

    // Item 1 is at front (cost 5)
    try testing.expectEqual(@as(u32, 1), heap.front().?.number);

    // Update item 1's priority to make it less important (higher cost)
    try heap.updatePriority(Item.init(1, 100));

    // Now item 2 should be at front
    try testing.expectEqual(@as(u32, 2), heap.front().?.number);
}

test "updatePriority on missing item returns error" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 10));

    const result = heap.updatePriority(Item.init(999, 5));
    try testing.expectError(error.ItemNotFound, result);
}

test "update_priority alias works" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 10));
    try heap.insert(Item.init(2, 5));

    // Use snake_case alias
    try heap.update_priority(Item.init(1, 1));
    try testing.expectEqual(@as(u32, 1), heap.front().?.number);
}

// ============================================================================
// getPosition Tests
// ============================================================================

test "getPosition returns correct position" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 10));
    try heap.insert(Item.init(2, 5));
    try heap.insert(Item.init(3, 15));

    // Item 2 has lowest cost, should be at position 0 (root)
    const pos2 = heap.getPosition(Item.init(2, 999));
    try testing.expectEqual(@as(?usize, 0), pos2);

    // Items 1 and 3 should be at positions 1 and 2 (children)
    const pos1 = heap.getPosition(Item.init(1, 0));
    const pos3 = heap.getPosition(Item.init(3, 0));
    try testing.expect(pos1 != null);
    try testing.expect(pos3 != null);
    try testing.expect(pos1.? > 0);
    try testing.expect(pos3.? > 0);
}

test "getPosition returns null for missing item" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 10));

    try testing.expectEqual(@as(?usize, null), heap.getPosition(Item.init(999, 0)));
}

test "get_position alias works" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(42, 100));

    // Use snake_case alias
    const pos = heap.get_position(Item.init(42, 0));
    try testing.expect(pos != null);
}

// ============================================================================
// *ByIndex Tests
// ============================================================================

test "increasePriorityByIndex works correctly" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 10));
    try heap.insert(Item.init(2, 5));
    try heap.insert(Item.init(3, 15));

    // Get position of item 3
    const pos = heap.getPosition(Item.init(3, 0)).?;
    try testing.expect(pos > 0); // Should not be at root initially

    // Call increasePriorityByIndex - this should not error
    try heap.increasePriorityByIndex(pos);

    // Heap property should still be maintained
    try testing.expectEqual(@as(u32, 2), heap.front().?.number);
}

test "increasePriorityByIndex returns error on invalid index" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 10));

    const result = heap.increasePriorityByIndex(99);
    try testing.expectError(error.IndexOutOfBounds, result);
}

test "increase_priority_by_index alias works" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 10));

    // Use snake_case alias - should not error
    try heap.increase_priority_by_index(0);
}

test "decreasePriorityByIndex works correctly" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 5));
    try heap.insert(Item.init(2, 10));
    try heap.insert(Item.init(3, 15));

    // Item 1 is at root (position 0)
    try testing.expectEqual(@as(u32, 1), heap.front().?.number);

    // Call decreasePriorityByIndex on root - should not error
    try heap.decreasePriorityByIndex(0);

    // Heap property should still be maintained
    try testing.expect(heap.front() != null);
}

test "decreasePriorityByIndex returns error on invalid index" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 10));

    const result = heap.decreasePriorityByIndex(99);
    try testing.expectError(error.IndexOutOfBounds, result);
}

test "decrease_priority_by_index alias works" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 10));

    // Use snake_case alias - should not error
    try heap.decrease_priority_by_index(0);
}

// ============================================================================
// Snake-case Alias Tests
// ============================================================================

test "is_empty alias works" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try testing.expect(heap.is_empty());

    try heap.insert(Item.init(1, 10));
    try testing.expect(!heap.is_empty());
}

test "increase_priority alias works" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 10));
    try heap.insert(Item.init(2, 5));

    // Use snake_case alias
    try heap.increase_priority(Item.init(1, 1));
    try testing.expectEqual(@as(u32, 1), heap.front().?.number);
}

test "decrease_priority alias works" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 5));
    try heap.insert(Item.init(2, 10));

    // Use snake_case alias - item 1 becomes less important
    try heap.decrease_priority(Item.init(1, 100));
    try testing.expectEqual(@as(u32, 2), heap.front().?.number);
}

// ============================================================================
// Bulk Operations Tests
// ============================================================================

test "insertMany inserts multiple items" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(4, MinByCost, allocator);
    defer heap.deinit();

    const items = [_]Item{
        Item.init(1, 50),
        Item.init(2, 30),
        Item.init(3, 70),
        Item.init(4, 20),
        Item.init(5, 60),
    };

    try heap.insertMany(&items);

    try testing.expectEqual(@as(usize, 5), heap.len());
    try testing.expectEqual(@as(u32, 4), heap.front().?.number); // Item 4 has lowest cost (20)

    // Verify ordering
    var last: u32 = 0;
    var first = true;
    while (!heap.isEmpty()) {
        const top = heap.front().?;
        if (!first) {
            try testing.expect(top.cost >= last);
        }
        first = false;
        last = top.cost;
        _ = try heap.pop();
    }
}

test "insertMany with empty slice does nothing" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    const empty_items = [_]Item{};
    try heap.insertMany(&empty_items);

    try testing.expectEqual(@as(usize, 0), heap.len());
    try testing.expect(heap.isEmpty());
}

test "insert_many alias works" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    const items = [_]Item{ Item.init(1, 10), Item.init(2, 5) };
    try heap.insert_many(&items);

    try testing.expectEqual(@as(usize, 2), heap.len());
    try testing.expectEqual(@as(u32, 2), heap.front().?.number);
}

test "popMany returns multiple items in order" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    const items = [_]Item{
        Item.init(1, 50),
        Item.init(2, 30),
        Item.init(3, 10),
        Item.init(4, 40),
        Item.init(5, 20),
    };
    try heap.insertMany(&items);

    const popped = try heap.popMany(3);
    defer allocator.free(popped);

    try testing.expectEqual(@as(usize, 3), popped.len);
    try testing.expectEqual(@as(u32, 3), popped[0].number); // cost 10
    try testing.expectEqual(@as(u32, 5), popped[1].number); // cost 20
    try testing.expectEqual(@as(u32, 2), popped[2].number); // cost 30

    try testing.expectEqual(@as(usize, 2), heap.len());
}

test "popMany with count greater than size returns all items" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 10));
    try heap.insert(Item.init(2, 5));

    const popped = try heap.popMany(10);
    defer allocator.free(popped);

    try testing.expectEqual(@as(usize, 2), popped.len);
    try testing.expect(heap.isEmpty());
}

test "popMany on empty heap returns empty slice" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    const popped = try heap.popMany(5);
    // Empty slice is a comptime literal, don't free it
    try testing.expectEqual(@as(usize, 0), popped.len);
}

test "pop_many alias works" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 10));
    try heap.insert(Item.init(2, 5));

    const popped = try heap.pop_many(1);
    defer allocator.free(popped);

    try testing.expectEqual(@as(usize, 1), popped.len);
    try testing.expectEqual(@as(u32, 2), popped[0].number);
}

// ============================================================================
// toArray Tests
// ============================================================================

test "toArray returns copy of heap contents" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 10));
    try heap.insert(Item.init(2, 5));
    try heap.insert(Item.init(3, 15));

    const arr = try heap.toArray();
    defer allocator.free(arr);

    try testing.expectEqual(@as(usize, 3), arr.len);
    // Root should be min element
    try testing.expectEqual(@as(u32, 2), arr[0].number);
}

test "toArray on empty heap returns empty slice" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    const arr = try heap.toArray();
    // Empty slice is a comptime literal, don't free it
    try testing.expectEqual(@as(usize, 0), arr.len);
}

test "to_array alias works" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(42, 100));

    const arr = try heap.to_array();
    defer allocator.free(arr);

    try testing.expectEqual(@as(usize, 1), arr.len);
    try testing.expectEqual(@as(u32, 42), arr[0].number);
}

// ============================================================================
// Peek Tests
// ============================================================================

test "peek returns same as front" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 10));
    try heap.insert(Item.init(2, 5));

    const peek_result = heap.peek();
    const front_result = heap.front();

    try testing.expectEqual(front_result, peek_result);
    try testing.expectEqual(@as(u32, 2), peek_result.?.number);
}

test "peek on empty heap returns null" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try testing.expectEqual(@as(?Item, null), heap.peek());
}

// ============================================================================
// Large Scale Tests
// ============================================================================

test "large heap maintains ordering" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(4, MinByCost, allocator);
    defer heap.deinit();

    // Insert 1000 items with pseudo-random costs
    var i: u32 = 0;
    while (i < 1000) : (i += 1) {
        const cost = (i * 17 + 31) % 500; // Pseudo-random distribution
        try heap.insert(Item.init(i, cost));
    }

    try testing.expectEqual(@as(usize, 1000), heap.len());

    // Pop all and verify non-decreasing order
    var last: u32 = 0;
    var first = true;
    while (!heap.isEmpty()) {
        const top = heap.front().?;
        if (!first) {
            try testing.expect(top.cost >= last);
        }
        first = false;
        last = top.cost;
        _ = try heap.pop();
    }
}

// ============================================================================
// Duplicate Values Tests
// ============================================================================

test "heap handles duplicate priority values" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    // Insert items with same priority (cost)
    try heap.insert(Item.init(1, 10));
    try heap.insert(Item.init(2, 10));
    try heap.insert(Item.init(3, 10));

    try testing.expectEqual(@as(usize, 3), heap.len());

    // All should be poppable
    _ = try heap.pop();
    _ = try heap.pop();
    _ = try heap.pop();

    try testing.expect(heap.isEmpty());
}

// ============================================================================
// toString Additional Tests
// ============================================================================

test "toString on empty heap returns braces" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    const str = try heap.toString();
    defer allocator.free(str);

    try testing.expectEqualStrings("{}", str);
}

test "to_string alias works" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 10));

    const str = try heap.to_string();
    defer allocator.free(str);

    try testing.expect(str.len > 2);
    try testing.expectEqual(@as(u8, '{'), str[0]);
    try testing.expectEqual(@as(u8, '}'), str[str.len - 1]);
}
