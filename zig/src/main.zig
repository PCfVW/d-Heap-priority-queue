//! Demonstration program for d-ary heap priority queue.
//!
//! This program demonstrates both min-heap and max-heap behavior,
//! including dynamic priority updates and order verification.

const std = @import("std");
const d_heap = @import("d_heap.zig");

// Use the pre-configured types from d_heap module
const DHeapItem = d_heap.DHeapItem;
const MinByCost = d_heap.MinByCost;
const MaxByCost = d_heap.MaxByCost;
const Item = d_heap.Item;

/// Helper function to print the current state of a priority queue.
fn printPQ(pq: *DHeapItem, allocator: std.mem.Allocator) !void {
    const str = try pq.toString();
    defer allocator.free(str);
    std.debug.print("{s}\n", .{str});
}

/// Main demonstration program.
pub fn main() !void {
    // Setup allocator
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    std.debug.print("=== Min-Heap Test (by cost) ===\n", .{});

    // Create min-heap using the pre-configured DHeapItem type
    var pq_less = try DHeapItem.init(3, MinByCost, allocator);
    defer pq_less.deinit();

    const input = [_]u32{ 20, 5, 22, 16, 18, 17, 12, 9, 42, 27, 48, 36, 32, 13, 14, 28, 52, 10, 21, 8, 39, 29, 15, 38, 31, 41 };

    // Insert items and print queue state
    for (input) |n| {
        try pq_less.insert(Item{ .number = n, .cost = n });
        try printPQ(&pq_less, allocator);
    }

    // Test dynamic update - insert new item
    const item1 = Item{ .number = 19, .cost = 19 };
    try pq_less.insert(item1);
    std.debug.print("After inserting (19, 19):\n", .{});
    try printPQ(&pq_less, allocator);

    // Get front element
    if (pq_less.front()) |front| {
        std.debug.print("front: {any}\n", .{front});
    }

    // Increase priority (lower cost for min-heap)
    const item1_new = Item{ .number = 19, .cost = 3 };
    try pq_less.increasePriority(item1_new);
    std.debug.print("After increasing priority of 19 to cost 3:\n", .{});
    try printPQ(&pq_less, allocator);

    if (pq_less.front()) |front| {
        std.debug.print("new front: {any}\n", .{front});
    }

    // Verify non-decreasing order on pops
    std.debug.print("\nPopping all elements (should be in non-decreasing order):\n", .{});
    var first_min = true;
    var last_cost_min: u32 = 0;
    while (!pq_less.isEmpty()) {
        if (pq_less.front()) |top| {
            if (!first_min) {
                if (top.cost < last_cost_min) {
                    std.debug.print("ERROR: Order violation! {any} < {any}\n", .{ top.cost, last_cost_min });
                    return error.OrderViolation;
                }
            } else {
                first_min = false;
            }
            last_cost_min = top.cost;
        }
        _ = pq_less.pop();
        try printPQ(&pq_less, allocator);
    }

    std.debug.print("\n=== Max-Heap Test (by cost) ===\n", .{});

    // Create max-heap using the pre-configured DHeapItem type
    var pq_greater = try DHeapItem.init(3, MaxByCost, allocator);
    defer pq_greater.deinit();

    // Insert items
    for (input) |n| {
        try pq_greater.insert(Item{ .number = n, .cost = n });
        try printPQ(&pq_greater, allocator);
    }

    // Test dynamic update
    const item2 = Item{ .number = 40, .cost = 40 };
    try pq_greater.insert(item2);
    std.debug.print("After inserting (40, 40):\n", .{});
    try printPQ(&pq_greater, allocator);

    const item2_new = Item{ .number = 40, .cost = 50 };
    try pq_greater.increasePriority(item2_new);
    std.debug.print("After increasing priority of 40 to cost 50:\n", .{});
    try printPQ(&pq_greater, allocator);

    // Verify non-increasing order on pops
    std.debug.print("\nPopping all elements (should be in non-increasing order):\n", .{});
    var first_max = true;
    var last_cost_max: u32 = 0;
    while (!pq_greater.isEmpty()) {
        if (pq_greater.front()) |top| {
            if (!first_max) {
                if (top.cost > last_cost_max) {
                    std.debug.print("ERROR: Order violation! {any} > {any}\n", .{ top.cost, last_cost_max });
                    return error.OrderViolation;
                }
            } else {
                first_max = false;
            }
            last_cost_max = top.cost;
        }
        _ = pq_greater.pop();
        try printPQ(&pq_greater, allocator);
    }

    std.debug.print("\n=== All tests passed! ===\n", .{});
}
