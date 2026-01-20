Implement a d-ary heap priority queue in Zig based on the following type definitions and signatures.

Requirements:
1. Items have two distinct properties: an identity (for equality) and a priority (for ordering)
2. Two items are equal if they have the same identity, regardless of priority
3. The queue should support O(1) lookup to check if an item exists
4. Implement a min-heap where lower priority values have higher importance

const std = @import("std");

/// Item represents an element in the priority queue.
/// The `number` field determines identity (equality).
/// The `cost` field determines ordering in the heap (lower = higher priority).
pub const Item = struct {
    number: u32,
    cost: u32,

    pub fn hash(self: Item) u64 {
        var hasher = std.hash.Wyhash.init(0);
        std.hash.autoHash(&hasher, self.number);
        return hasher.final();
    }

    pub fn eq(a: Item, b: Item) bool {
        return a.number == b.number;
    }
};

/// Comparator that returns true if a has higher priority (lower cost) than b.
pub fn MinByCost(a: Item, b: Item) bool {
    return a.cost < b.cost;
}

/// DHeap is a d-ary min-heap with O(1) item lookup.
pub fn DHeap(comptime compareFn: fn (Item, Item) bool) type {
    return struct {
        const Self = @This();

        d: usize,                                                    // arity
        container: std.ArrayList(Item),                              // heap array
        positions: std.HashMap(Item, usize, ItemContext, 80),        // item -> index
        allocator: std.mem.Allocator,

        const ItemContext = struct {
            pub fn hash(self: @This(), item: Item) u64 {
                _ = self;
                return item.hash();
            }
            pub fn eql(self: @This(), a: Item, b: Item) bool {
                _ = self;
                return Item.eq(a, b);
            }
        };

        /// Creates a new priority queue with the given arity d.
        /// The comparator is provided via the comptime parameter to DHeap.
        pub fn init(d: usize, allocator: std.mem.Allocator) !Self;

        /// Frees all resources.
        pub fn deinit(self: *Self) void;

        /// Adds an item to the queue.
        /// Returns error if item with same identity already exists.
        pub fn insert(self: *Self, item: Item) !void;

        /// Removes and returns the highest priority item.
        /// Returns null if empty.
        pub fn pop(self: *Self) !?Item;

        /// Returns the highest priority item without removing it.
        /// Returns null if empty.
        pub fn front(self: *const Self) ?Item;

        /// Updates an item to have higher priority (lower cost).
        /// Returns error if item doesn't exist.
        pub fn increasePriority(self: *Self, item: Item) !void;

        /// Updates an item to have lower priority (higher cost).
        /// Returns error if item doesn't exist.
        pub fn decreasePriority(self: *Self, item: Item) !void;

        /// Returns true if an item with the given identity exists.
        pub fn contains(self: *const Self, item: Item) bool;

        /// Returns the number of items in the queue.
        pub fn len(self: *const Self) usize;

        /// Returns true if the queue is empty.
        pub fn isEmpty(self: *const Self) bool;
    };
}

Provide the complete implementation for all methods.