//! D-ary heap priority queue implementation with O(1) item lookup.
//!
//! A d-ary heap is a generalization of a binary heap where each node has up to d children.
//! This implementation maintains a position map for O(1) contains() and priority updates.

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

/// Item stored in the priority queue
pub const Item = struct {
    number: u32, // identity
    cost: u32,   // priority

    pub fn init(number: u32, cost: u32) Item {
        return Item{ .number = number, .cost = cost };
    }

    pub fn eql(self: Item, other: Item) bool {
        return self.number == other.number;
    }
};

/// Comparator for min-heap based on cost
pub const MinByCost = struct {
    pub fn lessThan(a: Item, b: Item) bool {
        return a.cost < b.cost;
    }
};

/// Hash context for Item based on identity (number)
const ItemHashContext = struct {
    pub fn hash(self: @This(), item: Item) u64 {
        _ = self;
        return std.hash_map.getAutoHashFn(u32, void)(item.number);
    }

    pub fn eql(self: @This(), a: Item, b: Item) bool {
        _ = self;
        return a.number == b.number;
    }
};

/// D-ary heap priority queue with O(1) item lookup
pub const DHeapItem = struct {
    const Self = @This();
    
    // Core data structures
    items: ArrayList(Item),
    position_map: HashMap(Item, usize, ItemHashContext, std.hash_map.default_max_load_percentage),
    d: usize, // arity
    allocator: Allocator,

    /// Initialize a new d-ary heap with the given arity
    pub fn init(d: usize, comptime comparator: type, allocator: Allocator) !Self {
        _ = comparator; // We'll use MinByCost directly
        if (d < 2) return error.InvalidArity;
        
        return Self{
            .items = ArrayList(Item).init(allocator),
            .position_map = HashMap(Item, usize, ItemHashContext, std.hash_map.default_max_load_percentage).init(allocator),
            .d = d,
            .allocator = allocator,
        };
    }

    /// Clean up allocated memory
    pub fn deinit(self: *Self) void {
        self.items.deinit();
        self.position_map.deinit();
    }

    /// Get the number of items in the queue
    pub fn len(self: *const Self) usize {
        return self.items.items.len;
    }

    /// Check if the queue is empty
    pub fn isEmpty(self: *const Self) bool {
        return self.len() == 0;
    }

    /// Check if an item with the given identity exists in the queue
    pub fn contains(self: *const Self, item: Item) bool {
        return self.position_map.contains(item);
    }

    /// Get the parent index of a node
    fn parentIndex(self: *const Self, index: usize) ?usize {
        if (index == 0) return null;
        return (index - 1) / self.d;
    }

    /// Get the first child index of a node
    fn firstChildIndex(self: *const Self, index: usize) usize {
        return self.d * index + 1;
    }

    /// Get the last child index of a node (may not exist)
    fn lastChildIndex(self: *const Self, index: usize) usize {
        return self.d * index + self.d;
    }

    /// Swap two items in the heap and update position map
    fn swap(self: *Self, i: usize, j: usize) void {
        const temp = self.items.items[i];
        self.items.items[i] = self.items.items[j];
        self.items.items[j] = temp;

        // Update position map
        self.position_map.put(self.items.items[i], i) catch unreachable;
        self.position_map.put(self.items.items[j], j) catch unreachable;
    }

    /// Restore heap property by moving an item up toward the root
    fn siftUp(self: *Self, start_index: usize) void {
        var index = start_index;
        
        while (self.parentIndex(index)) |parent_idx| {
            if (MinByCost.lessThan(self.items.items[index], self.items.items[parent_idx])) {
                self.swap(index, parent_idx);
                index = parent_idx;
            } else {
                break;
            }
        }
    }

    /// Restore heap property by moving an item down toward the leaves
    fn siftDown(self: *Self, start_index: usize) void {
        var index = start_index;
        
        while (true) {
            const first_child = self.firstChildIndex(index);
            if (first_child >= self.items.items.len) break;
            
            // Find the child with the highest priority (lowest cost)
            var min_child_idx = first_child;
            var child_idx = first_child + 1;
            const last_child = @min(self.lastChildIndex(index), self.items.items.len - 1);
            
            while (child_idx <= last_child) : (child_idx += 1) {
                if (MinByCost.lessThan(self.items.items[child_idx], self.items.items[min_child_idx])) {
                    min_child_idx = child_idx;
                }
            }
            
            // If the minimum child has higher priority than current item, swap
            if (MinByCost.lessThan(self.items.items[min_child_idx], self.items.items[index])) {
                self.swap(index, min_child_idx);
                index = min_child_idx;
            } else {
                break;
            }
        }
    }

    /// Add an item to the queue
    pub fn insert(self: *Self, item: Item) !void {
        if (self.contains(item)) return error.ItemAlreadyExists;
        
        // Add to end of array
        try self.items.append(item);
        const new_index = self.items.items.len - 1;
        
        // Update position map
        try self.position_map.put(item, new_index);
        
        // Restore heap property
        self.siftUp(new_index);
    }

    /// Remove and return the item with highest priority
    pub fn pop(self: *Self) !?Item {
        if (self.isEmpty()) return null;
        
        const result = self.items.items[0];
        
        // Remove from position map
        _ = self.position_map.remove(result);
        
        if (self.len() == 1) {
            _ = self.items.pop();
            return result;
        }
        
        // Move last item to root
        const last_item = self.items.pop();
        self.items.items[0] = last_item;
        
        // Update position map for moved item
        try self.position_map.put(last_item, 0);
        
        // Restore heap property
        self.siftDown(0);
        
        return result;
    }

    /// Return the item with highest priority without removing it
    pub fn front(self: *const Self) ?Item {
        if (self.isEmpty()) return null;
        return self.items.items[0];
    }

    /// Update an existing item to have higher priority (lower cost)
    pub fn increasePriority(self: *Self, item: Item) !void {
        const index = self.position_map.get(item) orelse return error.ItemNotFound;
        
        // Update the item in place
        self.items.items[index] = item;
        
        // Update position map (key might have changed, but index is same)
        try self.position_map.put(item, index);
        
        // Since priority increased (cost decreased), item may need to move up
        self.siftUp(index);
    }

    /// Update an existing item to have lower priority (higher cost)
    pub fn decreasePriority(self: *Self, item: Item) !void {
        const index = self.position_map.get(item) orelse return error.ItemNotFound;
        
        // Update the item in place
        self.items.items[index] = item;
        
        // Update position map (key might have changed, but index is same)
        try self.position_map.put(item, index);
        
        // Since priority decreased (cost increased), item may need to move down
        self.siftDown(index);
    }
};

// Tests
test "basic functionality" {
    const testing = std.testing;
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try DHeapItem.init(4, MinByCost, allocator);
    defer pq.deinit();

    try testing.expect(pq.isEmpty());
    try testing.expectEqual(@as(usize, 0), pq.len());

    try pq.insert(Item.init(1, 10));
    try pq.insert(Item.init(2, 5));
    try pq.insert(Item.init(3, 15));

    try testing.expect(!pq.isEmpty());
    try testing.expectEqual(@as(usize, 3), pq.len());
    try testing.expectEqual(@as(u32, 5), pq.front().?.cost);

    const popped = try pq.pop();
    try testing.expectEqual(@as(u32, 5), popped.?.cost);
    try testing.expectEqual(@as(u32, 10), pq.front().?.cost);
}