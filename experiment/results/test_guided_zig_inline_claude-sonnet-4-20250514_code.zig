//! Test corpus for d-ary heap priority queue operations.
//!
//! These tests are inline with the implementation (same file).

const std = @import("std");
const testing = std.testing;

// Item struct - implement this
pub const Item = struct {
    number: u32,
    cost: u32,

    pub fn init(number: u32, cost: u32) Item {
        return .{ .number = number, .cost = cost };
    }
};

// Comparator for min-heap by cost
pub fn MinByCost(a: Item, b: Item) bool {
    return a.cost < b.cost;
}

// DHeapItem - d-ary heap priority queue implementation
pub const DHeapItem = struct {
    const Self = @This();
    
    heap: std.ArrayList(Item),
    index_map: std.HashMap(u32, usize, std.hash_map.DefaultContext(u32), std.hash_map.default_max_load_percentage),
    d: u32, // arity
    compareFn: fn (Item, Item) bool,
    allocator: std.mem.Allocator,

    pub fn init(d: u32, compareFn: fn (Item, Item) bool, allocator: std.mem.Allocator) !Self {
        return Self{
            .heap = std.ArrayList(Item).init(allocator),
            .index_map = std.HashMap(u32, usize, std.hash_map.DefaultContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
            .d = d,
            .compareFn = compareFn,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Self) void {
        self.heap.deinit();
        self.index_map.deinit();
    }

    pub fn len(self: *const Self) usize {
        return self.heap.items.len;
    }

    pub fn isEmpty(self: *const Self) bool {
        return self.heap.items.len == 0;
    }

    pub fn contains(self: *const Self, item: Item) bool {
        return self.index_map.contains(item.number);
    }

    pub fn front(self: *const Self) ?Item {
        if (self.isEmpty()) return null;
        return self.heap.items[0];
    }

    pub fn insert(self: *Self, item: Item) !void {
        try self.heap.append(item);
        const index = self.heap.items.len - 1;
        try self.index_map.put(item.number, index);
        self.bubbleUp(index);
    }

    pub fn pop(self: *Self) !?Item {
        if (self.isEmpty()) return null;

        const result = self.heap.items[0];
        _ = self.index_map.remove(result.number);

        if (self.heap.items.len == 1) {
            _ = self.heap.pop();
            return result;
        }

        // Move last element to root
        const last = self.heap.pop();
        self.heap.items[0] = last;
        try self.index_map.put(last.number, 0);
        
        self.bubbleDown(0);
        return result;
    }

    pub fn increasePriority(self: *Self, item: Item) !void {
        const index_ptr = self.index_map.getPtr(item.number) orelse return error.ItemNotFound;
        const index = index_ptr.*;
        
        self.heap.items[index] = item;
        self.bubbleUp(index);
    }

    pub fn decreasePriority(self: *Self, item: Item) !void {
        const index_ptr = self.index_map.getPtr(item.number) orelse return error.ItemNotFound;
        const index = index_ptr.*;
        
        self.heap.items[index] = item;
        self.bubbleDown(index);
    }

    fn parent(self: *const Self, index: usize) usize {
        if (index == 0) return 0;
        return (index - 1) / self.d;
    }

    fn firstChild(self: *const Self, index: usize) usize {
        return self.d * index + 1;
    }

    fn bubbleUp(self: *Self, start_index: usize) void {
        var index = start_index;
        
        while (index > 0) {
            const parent_index = self.parent(index);
            if (!self.compareFn(self.heap.items[index], self.heap.items[parent_index])) {
                break;
            }
            self.swap(index, parent_index);
            index = parent_index;
        }
    }

    fn bubbleDown(self: *Self, start_index: usize) void {
        var index = start_index;
        
        while (true) {
            const first_child = self.firstChild(index);
            if (first_child >= self.heap.items.len) break;

            // Find the child with highest priority (minimum value for min-heap)
            var best_child = first_child;
            var child = first_child + 1;
            const last_child = @min(first_child + self.d, self.heap.items.len);
            
            while (child < last_child) {
                if (self.compareFn(self.heap.items[child], self.heap.items[best_child])) {
                    best_child = child;
                }
                child += 1;
            }

            // If current node has higher priority than best child, stop
            if (self.compareFn(self.heap.items[index], self.heap.items[best_child])) {
                break;
            }

            self.swap(index, best_child);
            index = best_child;
        }
    }

    fn swap(self: *Self, i: usize, j: usize) void {
        const temp = self.heap.items[i];
        self.heap.items[i] = self.heap.items[j];
        self.heap.items[j] = temp;

        // Update index map
        self.index_map.put(self.heap.items[i].number, i) catch unreachable;
        self.index_map.put(self.heap.items[j].number, j) catch unreachable;
    }
};

// =============================================================================
// insert() Tests
// =============================================================================

test "insert_postcondition_item_findable" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    const item = Item.init(50, 50);
    try pq.insert(item);

    try testing.expect(pq.contains(item));
}

test "insert_invariant_heap_property" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

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

    var pq = try DHeapItem.init(4, MinByCost, allocator);
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

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(100, 100));
    try pq.insert(Item.init(50, 50));
    try pq.insert(Item.init(10, 10));

    try testing.expectEqual(@as(u32, 10), pq.front().?.cost);

    try pq.insert(Item.init(1, 1));

    try testing.expectEqual(@as(u32, 1), pq.front().?.cost);
}

// =============================================================================
// pop() Tests
// =============================================================================

test "pop_postcondition_returns_minimum" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(30, 30));
    try pq.insert(Item.init(10, 10));
    try pq.insert(Item.init(20, 20));

    try testing.expectEqual(@as(u32, 10), pq.front().?.cost);

    const popped = try pq.pop();
    try testing.expectEqual(@as(u32, 10), popped.?.cost);

    try testing.expect(!pq.contains(Item.init(10, 10)));
}

test "pop_invariant_maintains_heap_property" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
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

    var pq = try DHeapItem.init(4, MinByCost, allocator);
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

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try testing.expect(pq.isEmpty());
    try testing.expectEqual(@as(?Item, null), pq.front());
}

// =============================================================================
// front() Tests
// =============================================================================

test "front_postcondition_returns_minimum" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
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

    var pq = try DHeapItem.init(4, MinByCost, allocator);
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

    var pq = try DHeapItem.init(4, MinByCost, allocator);
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

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try testing.expectEqual(@as(?Item, null), pq.front());
}

// =============================================================================
// increasePriority() Tests
// =============================================================================

test "increase_priority_postcondition_priority_changed" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(50, 50));
    try pq.insert(Item.init(30, 30));

    try testing.expectEqual(@as(u32, 30), pq.front().?.cost);

    const updated = Item.init(50, 10);
    try pq.increasePriority(updated);

    try testing.expectEqual(@as(u32, 10), pq.front().?.cost);
}

test "increase_priority_invariant_heap_property" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
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

    var pq = try DHeapItem.init(4, MinByCost, allocator);
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

    var pq = try DHeapItem.init(4, MinByCost, allocator);
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

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(50, 50));

    const nonexistent = Item.init(999, 10);
    const result = pq.increasePriority(nonexistent);
    try testing.expectError(error.ItemNotFound, result);
}

// =============================================================================
// decreasePriority() Tests
// =============================================================================

test "decrease_priority_postcondition_priority_changed" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
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

    var pq = try DHeapItem.init(4, MinByCost, allocator);
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

    var pq = try DHeapItem.init(4, MinByCost, allocator);
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

    var pq = try DHeapItem.init(4, MinByCost, allocator);
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

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try pq.insert(Item.init(50, 50));

    const nonexistent = Item.init(999, 100);
    const result = pq.decreasePriority(nonexistent);
    try testing.expectError(error.ItemNotFound, result);
}