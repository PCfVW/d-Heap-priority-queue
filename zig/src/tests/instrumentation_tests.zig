//! Phase 2 comparison-count instrumentation tests.
//!
//! Each test mirrors the equivalent test in the C++/Go/Rust suites so the
//! cross-language contract stays observable from the test code: bucket
//! isolation per operation, lockstep pop order between instrumented and
//! un-instrumented heaps, and a comptime size check that confirms the
//! `void` `stats` field is zero-cost.

const std = @import("std");
const testing = std.testing;

const d_heap = @import("d_heap");
const DHeapItem = d_heap.DHeapItem;
const InstrumentedDHeap = d_heap.InstrumentedDHeap;
const Item = d_heap.Item;
const ItemContext = d_heap.ItemContext;
const ItemComparator = d_heap.ItemComparator;
const MinByCost = d_heap.MinByCost;

const InstrumentedItemHeap = InstrumentedDHeap(Item, ItemContext, ItemComparator);

// ============================================================================
// Initial state
// ============================================================================

test "stats initial state" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try InstrumentedItemHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try testing.expectEqual(@as(u64, 0), heap.stats.insert_count);
    try testing.expectEqual(@as(u64, 0), heap.stats.pop_count);
    try testing.expectEqual(@as(u64, 0), heap.stats.decrease_priority_count);
    try testing.expectEqual(@as(u64, 0), heap.stats.increase_priority_count);
    try testing.expectEqual(@as(u64, 0), heap.stats.update_priority_count);
    try testing.expectEqual(@as(u64, 0), heap.stats.total());
}

// ============================================================================
// Per-bucket isolation
// ============================================================================

test "insert bucket isolation" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try InstrumentedItemHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    const values = [_]u32{ 5, 3, 8, 1, 9, 2, 7, 4 };
    for (values) |v| try heap.insert(Item.init(v, v));

    try testing.expect(heap.stats.insert_count > 0);
    try testing.expectEqual(@as(u64, 0), heap.stats.pop_count);
    try testing.expectEqual(@as(u64, 0), heap.stats.decrease_priority_count);
    try testing.expectEqual(@as(u64, 0), heap.stats.increase_priority_count);
    try testing.expectEqual(@as(u64, 0), heap.stats.update_priority_count);
}

test "pop bucket isolation" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try InstrumentedItemHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    const values = [_]u32{ 5, 3, 8, 1, 9, 2, 7, 4 };
    for (values) |v| try heap.insert(Item.init(v, v));
    const inserts_before_pop = heap.stats.insert_count;

    var i: usize = 0;
    while (i < values.len) : (i += 1) {
        _ = try heap.pop();
    }

    // pop comparisons accumulated; insert bucket frozen at its prior value.
    try testing.expect(heap.stats.pop_count > 0);
    try testing.expectEqual(inserts_before_pop, heap.stats.insert_count);
    try testing.expectEqual(@as(u64, 0), heap.stats.decrease_priority_count);
    try testing.expectEqual(@as(u64, 0), heap.stats.increase_priority_count);
    try testing.expectEqual(@as(u64, 0), heap.stats.update_priority_count);
}

test "increase_priority bucket isolation" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try InstrumentedItemHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    // Insert in *increasing* cost order so item #3 lands as a leaf (not at
    // the root). Otherwise increasePriority(3) would call moveUp(0), which
    // exits without comparing — and the bucket would stay at zero.
    try heap.insert(Item.init(1, 10));
    try heap.insert(Item.init(2, 20));
    try heap.insert(Item.init(3, 30));
    heap.stats.reset();

    // Lower the cost of #3 — for a min-heap that's an increase in priority.
    try heap.increasePriority(Item.init(3, 1));

    try testing.expect(heap.stats.increase_priority_count > 0);
    try testing.expectEqual(@as(u64, 0), heap.stats.insert_count);
    try testing.expectEqual(@as(u64, 0), heap.stats.pop_count);
    try testing.expectEqual(@as(u64, 0), heap.stats.decrease_priority_count);
    try testing.expectEqual(@as(u64, 0), heap.stats.update_priority_count);
}

test "decrease_priority bucket isolation" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try InstrumentedItemHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 5));
    try heap.insert(Item.init(2, 40));
    try heap.insert(Item.init(3, 30));
    heap.stats.reset();

    // Raise the cost of #1 — for a min-heap that's a decrease in priority.
    try heap.decreasePriority(Item.init(1, 100));

    try testing.expect(heap.stats.decrease_priority_count > 0);
    try testing.expectEqual(@as(u64, 0), heap.stats.insert_count);
    try testing.expectEqual(@as(u64, 0), heap.stats.pop_count);
    try testing.expectEqual(@as(u64, 0), heap.stats.increase_priority_count);
    try testing.expectEqual(@as(u64, 0), heap.stats.update_priority_count);
}

test "update_priority bucket isolation" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try InstrumentedItemHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 5));
    try heap.insert(Item.init(2, 40));
    try heap.insert(Item.init(3, 30));
    heap.stats.reset();

    try heap.updatePriority(Item.init(2, 10));

    try testing.expect(heap.stats.update_priority_count > 0);
    try testing.expectEqual(@as(u64, 0), heap.stats.insert_count);
    try testing.expectEqual(@as(u64, 0), heap.stats.pop_count);
    try testing.expectEqual(@as(u64, 0), heap.stats.increase_priority_count);
    try testing.expectEqual(@as(u64, 0), heap.stats.decrease_priority_count);
}

// ============================================================================
// Reset & total
// ============================================================================

test "reset zeros counters but preserves heap" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try InstrumentedItemHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    const values = [_]u32{ 5, 3, 8, 1, 9 };
    for (values) |v| try heap.insert(Item.init(v, v));

    try testing.expect(heap.stats.total() > 0);
    const len_before = heap.len();
    const front_before = heap.front().?;

    heap.stats.reset();

    try testing.expectEqual(@as(u64, 0), heap.stats.total());
    try testing.expectEqual(len_before, heap.len());
    try testing.expectEqual(front_before.cost, heap.front().?.cost);
}

test "total equals sum of buckets" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try InstrumentedItemHeap.init(3, MinByCost, allocator);
    defer heap.deinit();

    const values = [_]u32{ 20, 5, 22, 16, 18, 17, 12, 9 };
    for (values) |v| try heap.insert(Item.init(v, v));
    try heap.increasePriority(Item.init(20, 1));
    try heap.decreasePriority(Item.init(5, 99));
    try heap.updatePriority(Item.init(16, 50));
    _ = try heap.pop();
    _ = try heap.pop();

    const sum =
        heap.stats.insert_count +
        heap.stats.pop_count +
        heap.stats.decrease_priority_count +
        heap.stats.increase_priority_count +
        heap.stats.update_priority_count;
    try testing.expectEqual(sum, heap.stats.total());
}

// ============================================================================
// Lockstep pop order: instrumented vs un-instrumented produce identical results
// ============================================================================

test "lockstep pop order" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var plain = try DHeapItem.init(4, MinByCost, allocator);
    defer plain.deinit();
    var instrumented = try InstrumentedItemHeap.init(4, MinByCost, allocator);
    defer instrumented.deinit();

    const values = [_]u32{ 20, 5, 22, 16, 18, 17, 12, 9, 42, 27, 48, 36, 32, 13, 14, 28 };
    for (values) |v| {
        try plain.insert(Item.init(v, v));
        try instrumented.insert(Item.init(v, v));
    }

    var i: usize = 0;
    while (i < values.len) : (i += 1) {
        const a = (try plain.pop()).?;
        const b = (try instrumented.pop()).?;
        try testing.expectEqual(a.number, b.number);
        try testing.expectEqual(a.cost, b.cost);
    }
}

// ============================================================================
// Default DHeap's stats field is zero-sized
// ============================================================================

test "default heap stats field is zero-cost" {
    // `DHeapItem` (no stats) must be strictly smaller than its instrumented
    // sibling — proves the `void` field carries no payload, mirroring
    // Rust's `default_stats_member_is_zero_sized`.
    try testing.expect(@sizeOf(DHeapItem) < @sizeOf(InstrumentedItemHeap));
}
