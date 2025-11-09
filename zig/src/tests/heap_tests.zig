const std = @import("std");
const DHeap = @import("../d_heap.zig");
const Item = @import("../types.zig");
const MinByCost = @import("../d_heap.zig").MinByCost;

test "heap operations" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    var heap = try DHeap.init(2, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(5, 5));
    try heap.insert(Item.init(1, 1));
    try std.testing.expectEqual(1, heap.front().?.cost);
    _ = heap.pop();
    try std.testing.expectEqual(5, heap.front().?.cost);
}
