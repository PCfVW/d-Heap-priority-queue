const std = @import("std");
const mem = std.mem;
const hash_map = std.hash_map;

/// Error types for the priority queue
const Error = error{
    ItemNotFound,
    EmptyQueue,
    DuplicateItem,
    InvalidArity,
};

/// Item type with identity and priority
pub const Item = struct {
    number: u32,
    cost: u32,

    pub fn init(number: u32, cost: u32) Item {
        return Item{ .number = number, .cost = cost };
    }
};

/// Comparison function type
pub const Comparator = *const fn (a: Item, b: Item) bool;

/// Min-by-cost comparator
pub const MinByCost = &minByCost;
fn minByCost(a: Item, b: Item) bool {
    return a.cost < b.cost;
}

/// D-ary heap priority queue
pub const DHeapItem = struct {
    const Self = @This();

    d: usize,
    comparator: Comparator,
    allocator: std.mem.Allocator,

    heap: std.ArrayList(Item),
    position_map: hash_map.HashMap(u32, usize),

    pub fn init(d: usize, comparator: Comparator, allocator: std.mem.Allocator) !Self {
        if (d < 2) {
            return Error.InvalidArity;
        }

        var position_map = hash_map.HashMap(u32, usize).init(allocator);
        var heap = std.ArrayList(Item).init(allocator);

        return Self{
            .d = d,
            .comparator = comparator,
            .allocator = allocator,
            .heap = heap,
            .position_map = position_map,
        };
    }

    pub fn deinit(self: *Self) void {
        self.heap.deinit();
        self.position_map.deinit();
    }

    /// Get the number of items in the queue
    pub fn len(self: *Self) usize {
        return self.heap.items.len;
    }

    /// Check if the queue is empty
    pub fn isEmpty(self: *Self) bool {
        return self.heap.items.len == 0;
    }

    /// Check if an item with the given identity exists in the queue
    pub fn contains(self: *Self, item: Item) bool {
        return self.position_map.getPtr(item.number) != null;
    }

    /// Get the front item without removing it
    pub fn front(self: *Self) ?Item {
        if (self.heap.items.len == 0) {
            return null;
        }
        return self.heap.items[0];
    }

    /// Insert an item into the queue
    pub fn insert(self: *Self, item: Item) !void {
        if (self.position_map.getPtr(item.number) != null) {
            return Error.DuplicateItem;
        }

        try self.heap.append(item);
        const index = self.heap.items.len - 1;
        try self.position_map.put(item.number, index);
        self.siftUp(index);
    }

    /// Remove and return the item with highest priority
    pub fn pop(self: *Self) !?Item {
        if (self.heap.items.len == 0) {
            return null;
        }

        const front = self.heap.items[0];
        self.position_map.remove(front.number);

        if (self.heap.items.len == 1) {
            try self.heap.pop();
            return front;
        }

        // Swap root with last element
        const last = self.heap.items[self.heap.items.len - 1];
        self.heap.items[0] = last;
        try self.position_map.put(last.number, 0);

        try self.heap.pop();
        self.siftDown(0);

        return front;
    }

    /// Increase priority of an existing item (lower cost in min-heap)
    pub fn increasePriority(self: *Self, item: Item) !void {
        if (self.position_map.getPtr(item.number) == null) {
            return Error.ItemNotFound;
        }

        const index = self.position_map.get(item.number) catch unreachable;
        const old_item = self.heap.items[index];

        // Only update if the new priority is actually higher (lower cost)
        if (self.comparator(item, old_item)) {
            self.heap.items[index] = item;
            self.siftUp(index);
        }
    }

    /// Decrease priority of an existing item (higher cost in min-heap)
    pub fn decreasePriority(self: *Self, item: Item) !void {
        if (self.position_map.getPtr(item.number) == null) {
            return Error.ItemNotFound;
        }

        const index = self.position_map.get(item.number) catch unreachable;
        const old_item = self.heap.items[index];

        // Only update if the new priority is actually lower (higher cost)
        if (!self.comparator(item, old_item)) {
            self.heap.items[index] = item;
            self.siftDown(index);
        }
    }

    /// Move an item up the heap to restore heap property
    fn siftUp(self: *Self, index: usize) void {
        var current = index;
        while (current > 0) {
            const parent = (current - 1) / self.d;
            if (self.comparator(self.heap.items[current], self.heap.items[parent])) {
                self.swap(current, parent);
                current = parent;
            } else {
                break;
            }
        }
    }

    /// Move an item down the heap to restore heap property
    fn siftDown(self: *Self, index: usize) void {
        var current = index;
        loop {
            const first_child = current * self.d + 1;
            if (first_child >= self.heap.items.len) {
                break;
            }

            // Find the child with highest priority (lowest cost in min-heap)
            var min_child = first_child;
            var child = first_child + 1;
            while (child < self.heap.items.len and child <= (current + 1) * self.d) {
                if (self.comparator(self.heap.items[child], self.heap.items[min_child])) {
                    min_child = child;
                }
                child += 1;
            }

            if (self.comparator(self.heap.items[min_child], self.heap.items[current])) {
                self.swap(current, min_child);
                current = min_child;
            } else {
                break;
            }
        }
    }

    /// Swap two items in the heap and update their positions in the map
    fn swap(self: *Self, i: usize, j: usize) void {
        const temp = self.heap.items[i];
        self.heap.items[i] = self.heap.items[j];
        self.heap.items[j] = temp;

        // Update position map
        self.position_map.put(self.heap.items[i].number, i);
        self.position_map.put(self.heap.items[j].number, j);
    }
};