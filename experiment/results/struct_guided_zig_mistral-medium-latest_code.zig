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
            if (d < 2) {
                return error.DHeapInvalidArity;
            }

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
            if (self.positions.getPtr(item) != null) {
                return error.DHeapItemExists;
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

            const root = self.container.items[0];
            try self.positions.remove(root);

            if (self.container.items.len == 1) {
                _ = self.container.pop();
                return root;
            }

            // Move last item to root
            const last = self.container.items[self.container.items.len - 1];
            self.container.items[0] = last;
            try self.positions.put(last, 0);
            _ = self.container.pop();

            try self.heapifyDown(0);
            return root;
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
            if (self.positions.getPtr(item) == null) {
                return error.DHeapItemNotFound;
            }

            const index = self.positions.getPtr(item).*;
            const old_item = self.container.items[index];
            if (item.cost >= old_item.cost) {
                return error.DHeapPriorityNotIncreased;
            }

            self.container.items[index] = item;
            try self.heapifyUp(index);
        }

        /// Updates an item to have lower priority (higher cost).
        /// Returns error if item doesn't exist.
        pub fn decreasePriority(self: *Self, item: Item) !void {
            if (self.positions.getPtr(item) == null) {
                return error.DHeapItemNotFound;
            }

            const index = self.positions.getPtr(item).*;
            const old_item = self.container.items[index];
            if (item.cost <= old_item.cost) {
                return error.DHeapPriorityNotDecreased;
            }

            self.container.items[index] = item;
            try self.heapifyDown(index);
        }

        /// Returns true if an item with the given identity exists.
        pub fn contains(self: *const Self, item: Item) bool {
            return self.positions.getPtr(item) != null;
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

        fn heapifyUp(self: *Self, index: usize) !void {
            var current = index;
            while (current > 0) {
                const parent = self.parent(current);
                if (compareFn(self.container.items[current], self.container.items[parent])) {
                    self.swap(current, parent);
                    current = parent;
                } else {
                    break;
                }
            }
        }

        fn heapifyDown(self: *Self, index: usize) !void {
            var current = index;
            while (true) {
                var smallest = current;
                const start = self.child(current);
                const end = std.math.min(self.child(current) + self.d, self.container.items.len);

                // Find the smallest child
                var i: usize = start;
                while (i < end) : (i += 1) {
                    if (compareFn(self.container.items[i], self.container.items[smallest])) {
                        smallest = i;
                    }
                }

                if (smallest != current) {
                    self.swap(current, smallest);
                    current = smallest;
                } else {
                    break;
                }
            }
        }

        fn swap(self: *Self, a: usize, b: usize) void {
            const temp = self.container.items[a];
            self.container.items[a] = self.container.items[b];
            self.container.items[b] = temp;

            // Update positions
            self.positions.put(self.container.items[a], a) catch unreachable;
            self.positions.put(self.container.items[b], b) catch unreachable;
        }

        fn parent(self: *const Self, index: usize) usize {
            return (index - 1) / self.d;
        }

        fn child(self: *const Self, index: usize) usize {
            return index * self.d + 1;
        }
    };
}

// Error definitions
pub const error = error{
    DHeapInvalidArity,
    DHeapItemExists,
    DHeapItemNotFound,
    DHeapPriorityNotIncreased,
    DHeapPriorityNotDecreased,
};