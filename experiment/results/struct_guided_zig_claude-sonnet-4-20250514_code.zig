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
        pub fn init(d: usize, allocator: std.mem.Allocator) !Self {
            return Self{
                .d = d,
                .container = std.ArrayList(Item).init(allocator),
                .positions = std.HashMap(Item, usize, ItemContext, 80).init(allocator),
                .allocator = allocator,
            };
        }

        /// Frees all resources.
        pub fn deinit(self: *Self) void {
            self.container.deinit();
            self.positions.deinit();
        }

        /// Adds an item to the queue.
        /// Returns error if item with same identity already exists.
        pub fn insert(self: *Self, item: Item) !void {
            if (self.positions.contains(item)) {
                return error.ItemAlreadyExists;
            }

            try self.container.append(item);
            const index = self.container.items.len - 1;
            try self.positions.put(item, index);
            
            try self.heapifyUp(index);
        }

        /// Removes and returns the highest priority item.
        /// Returns null if empty.
        pub fn pop(self: *Self) !?Item {
            if (self.container.items.len == 0) {
                return null;
            }

            const result = self.container.items[0];
            _ = self.positions.remove(result);

            if (self.container.items.len == 1) {
                _ = self.container.pop();
                return result;
            }

            // Move last element to root
            const last = self.container.pop();
            self.container.items[0] = last;
            try self.positions.put(last, 0);

            try self.heapifyDown(0);
            return result;
        }

        /// Returns the highest priority item without removing it.
        /// Returns null if empty.
        pub fn front(self: *const Self) ?Item {
            if (self.container.items.len == 0) {
                return null;
            }
            return self.container.items[0];
        }

        /// Updates an item to have higher priority (lower cost).
        /// Returns error if item doesn't exist.
        pub fn increasePriority(self: *Self, item: Item) !void {
            const index_opt = self.positions.get(item);
            if (index_opt == null) {
                return error.ItemNotFound;
            }
            const index = index_opt.?;

            // Remove old item from positions map
            _ = self.positions.remove(self.container.items[index]);
            
            // Update the item
            self.container.items[index] = item;
            try self.positions.put(item, index);

            // Since priority increased (cost decreased), bubble up
            try self.heapifyUp(index);
        }

        /// Updates an item to have lower priority (higher cost).
        /// Returns error if item doesn't exist.
        pub fn decreasePriority(self: *Self, item: Item) !void {
            const index_opt = self.positions.get(item);
            if (index_opt == null) {
                return error.ItemNotFound;
            }
            const index = index_opt.?;

            // Remove old item from positions map
            _ = self.positions.remove(self.container.items[index]);
            
            // Update the item
            self.container.items[index] = item;
            try self.positions.put(item, index);

            // Since priority decreased (cost increased), bubble down
            try self.heapifyDown(index);
        }

        /// Returns true if an item with the given identity exists.
        pub fn contains(self: *const Self, item: Item) bool {
            return self.positions.contains(item);
        }

        /// Returns the number of items in the queue.
        pub fn len(self: *const Self) usize {
            return self.container.items.len;
        }

        /// Returns true if the queue is empty.
        pub fn isEmpty(self: *const Self) bool {
            return self.container.items.len == 0;
        }

        // Helper functions

        fn parent(self: *const Self, index: usize) ?usize {
            if (index == 0) return null;
            return (index - 1) / self.d;
        }

        fn firstChild(self: *const Self, index: usize) usize {
            return self.d * index + 1;
        }

        fn heapifyUp(self: *Self, start_index: usize) !void {
            var index = start_index;
            
            while (self.parent(index)) |parent_index| {
                if (!compareFn(self.container.items[index], self.container.items[parent_index])) {
                    break;
                }
                
                try self.swap(index, parent_index);
                index = parent_index;
            }
        }

        fn heapifyDown(self: *Self, start_index: usize) !void {
            var index = start_index;
            
            while (true) {
                var best_index = index;
                const first_child_index = self.firstChild(index);
                
                // Check all children
                var i: usize = 0;
                while (i < self.d) : (i += 1) {
                    const child_index = first_child_index + i;
                    if (child_index >= self.container.items.len) break;
                    
                    if (compareFn(self.container.items[child_index], self.container.items[best_index])) {
                        best_index = child_index;
                    }
                }
                
                if (best_index == index) break;
                
                try self.swap(index, best_index);
                index = best_index;
            }
        }

        fn swap(self: *Self, i: usize, j: usize) !void {
            const temp = self.container.items[i];
            self.container.items[i] = self.container.items[j];
            self.container.items[j] = temp;
            
            // Update positions map
            try self.positions.put(self.container.items[i], i);
            try self.positions.put(self.container.items[j], j);
        }
    };
}

// Example usage and test
test "DHeap basic operations" {
    const allocator = std.testing.allocator;
    var heap = try DHeap(MinByCost).init(3, allocator);
    defer heap.deinit();

    // Test empty heap
    try std.testing.expect(heap.isEmpty());
    try std.testing.expectEqual(@as(usize, 0), heap.len());
    try std.testing.expectEqual(@as(?Item, null), heap.front());
    try std.testing.expectEqual(@as(?Item, null), try heap.pop());

    // Test insert and front
    try heap.insert(Item{ .number = 1, .cost = 10 });
    try heap.insert(Item{ .number = 2, .cost = 5 });
    try heap.insert(Item{ .number = 3, .cost = 15 });

    try std.testing.expect(!heap.isEmpty());
    try std.testing.expectEqual(@as(usize, 3), heap.len());
    
    const front_item = heap.front().?;
    try std.testing.expectEqual(@as(u32, 2), front_item.number);
    try std.testing.expectEqual(@as(u32, 5), front_item.cost);

    // Test contains
    try std.testing.expect(heap.contains(Item{ .number = 1, .cost = 999 })); // cost doesn't matter for contains
    try std.testing.expect(heap.contains(Item{ .number = 2, .cost = 0 }));
    try std.testing.expect(!heap.contains(Item{ .number = 4, .cost = 0 }));

    // Test pop
    const popped = (try heap.pop()).?;
    try std.testing.expectEqual(@as(u32, 2), popped.number);
    try std.testing.expectEqual(@as(u32, 5), popped.cost);
    try std.testing.expectEqual(@as(usize, 2), heap.len());

    // Test priority updates
    try heap.increasePriority(Item{ .number = 3, .cost = 1 }); // decrease cost from 15 to 1
    const new_front = heap.front().?;
    try std.testing.expectEqual(@as(u32, 3), new_front.number);
    try std.testing.expectEqual(@as(u32, 1), new_front.cost);

    try heap.decreasePriority(Item{ .number = 3, .cost = 20 }); // increase cost from 1 to 20
    const updated_front = heap.front().?;
    try std.testing.expectEqual(@as(u32, 1), updated_front.number);
    try std.testing.expectEqual(@as(u32, 10), updated_front.cost);
}

test "DHeap duplicate insert error" {
    const allocator = std.testing.allocator;
    var heap = try DHeap(MinByCost).init(2, allocator);
    defer heap.deinit();

    try heap.insert(Item{ .number = 1, .cost = 10 });
    
    // Should fail because item with number=1 already exists
    try std.testing.expectError(error.ItemAlreadyExists, heap.insert(Item{ .number = 1, .cost = 5 }));
}

test "DHeap priority update errors" {
    const allocator = std.testing.allocator;
    var heap = try DHeap(MinByCost).init(2, allocator);
    defer heap.deinit();

    // Should fail because item doesn't exist
    try std.testing.expectError(error.ItemNotFound, heap.increasePriority(Item{ .number = 1, .cost = 5 }));
    try std.testing.expectError(error.ItemNotFound, heap.decreasePriority(Item{ .number = 1, .cost = 15 }));
}