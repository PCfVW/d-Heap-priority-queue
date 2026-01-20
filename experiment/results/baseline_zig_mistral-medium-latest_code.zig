const std = @import("std");
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;
const hash_map = std.hash_map;

/// A d-ary heap priority queue implementation
pub const DaryHeap = struct {
    allocator: Allocator,
    d: usize, // arity of the heap
    items: []Item,
    identity_map: HashMap(Identity, usize), // maps identity to index in items array

    const Item = struct {
        identity: Identity,
        priority: Priority,
    };

    const Identity = []const u8; // or any type that implements std.hash.Hash and std.mem.eql
    const Priority = f64; // or any type that implements std.math.Order

    /// Initialize a new d-ary heap
    pub fn init(allocator: Allocator, d: usize) !DaryHeap {
        if (d < 2) {
            return error.DaryHeapInvalidArity;
        }

        return DaryHeap{
            .allocator = allocator,
            .d = d,
            .items = try allocator.alloc(Item, 0),
            .identity_map = HashMap(Identity, usize).init(allocator),
        };
    }

    /// Deinitialize the heap
    pub fn deinit(self: *DaryHeap) void {
        self.allocator.free(self.items);
        self.identity_map.deinit();
    }

    /// Get the number of items in the heap
    pub fn len(self: *DaryHeap) usize {
        return self.items.len;
    }

    /// Check if the heap is empty
    pub fn isEmpty(self: *DaryHeap) bool {
        return self.len() == 0;
    }

    /// Check if an item with the given identity exists in the heap
    pub fn contains(self: *DaryHeap, identity: Identity) bool {
        return self.identity_map.get(identity) != null;
    }

    /// Get the item with the highest priority (lowest value) without removing it
    pub fn front(self: *DaryHeap) ?Item {
        if (self.isEmpty()) {
            return null;
        }
        return self.items[0];
    }

    /// Insert a new item into the heap
    pub fn insert(self: *DaryHeap, identity: Identity, priority: Priority) !void {
        if (self.contains(identity)) {
            return error.DaryHeapDuplicateIdentity;
        }

        // Add to items array
        self.items = try self.allocator.realloc(self.items, self.items.len + 1);
        self.items[self.items.len - 1] = .{
            .identity = identity,
            .priority = priority,
        };

        // Add to identity map
        try self.identity_map.put(identity, self.items.len - 1);

        // Bubble up
        self.bubbleUp(self.items.len - 1);
    }

    /// Remove and return the item with the highest priority (lowest value)
    pub fn pop(self: *DaryHeap) ?Item {
        if (self.isEmpty()) {
            return null;
        }

        const front = self.items[0];

        // Move last item to front
        self.items[0] = self.items[self.items.len - 1];
        self.identity_map.put(self.items[0].identity, 0) catch unreachable;

        // Remove last item
        self.items = self.allocator.shrink(self.items, self.items.len - 1);
        self.identity_map.remove(front.identity) catch unreachable;

        // Bubble down
        if (!self.isEmpty()) {
            self.bubbleDown(0);
        }

        return front;
    }

    /// Update an existing item to have higher priority (lower value)
    pub fn increasePriority(self: *DaryHeap, identity: Identity, new_priority: Priority) !void {
        const index = self.identity_map.get(identity) orelse {
            return error.DaryHeapItemNotFound;
        };

        if (new_priority > self.items[index].priority) {
            return error.DaryHeapInvalidPriorityIncrease;
        }

        self.items[index].priority = new_priority;
        self.bubbleUp(index);
    }

    /// Update an existing item to have lower priority (higher value)
    pub fn decreasePriority(self: *DaryHeap, identity: Identity, new_priority: Priority) !void {
        const index = self.identity_map.get(identity) orelse {
            return error.DaryHeapItemNotFound;
        };

        if (new_priority < self.items[index].priority) {
            return error.DaryHeapInvalidPriorityDecrease;
        }

        self.items[index].priority = new_priority;
        self.bubbleDown(index);
    }

    /// Bubble an item up the heap until heap property is restored
    fn bubbleUp(self: *DaryHeap, index: usize) void {
        var current = index;
        while (current > 0) {
            const parent = self.parent(current);
            if (self.items[current].priority >= self.items[parent].priority) {
                break;
            }
            self.swap(current, parent);
            current = parent;
        }
    }

    /// Bubble an item down the heap until heap property is restored
    fn bubbleDown(self: *DaryHeap, index: usize) void {
        var current = index;
        while (true) {
            const smallest = self.smallestChild(current);
            if (smallest == current) {
                break;
            }
            self.swap(current, smallest);
            current = smallest;
        }
    }

    /// Find the smallest child of a given node (including the node itself)
    fn smallestChild(self: *DaryHeap, index: usize) usize {
        var smallest = index;
        const start = self.firstChild(index);
        const end = std.math.min(start + self.d, self.items.len);

        var i: usize = start;
        while (i < end) : (i += 1) {
            if (self.items[i].priority < self.items[smallest].priority) {
                smallest = i;
            }
        }

        return smallest;
    }

    /// Get the parent index of a given node
    fn parent(self: *DaryHeap, index: usize) usize {
        return (index - 1) / self.d;
    }

    /// Get the first child index of a given node
    fn firstChild(self: *DaryHeap, index: usize) usize {
        return index * self.d + 1;
    }

    /// Swap two items in the heap
    fn swap(self: *DaryHeap, a: usize, b: usize) void {
        const temp = self.items[a];
        self.items[a] = self.items[b];
        self.items[b] = temp;

        // Update identity map
        self.identity_map.put(self.items[a].identity, a) catch unreachable;
        self.identity_map.put(self.items[b].identity, b) catch unreachable;
    }
};

/// Error types for the D-ary heap
pub const error = error{
    DaryHeapInvalidArity,
    DaryHeapDuplicateIdentity,
    DaryHeapItemNotFound,
    DaryHeapInvalidPriorityIncrease,
    DaryHeapInvalidPriorityDecrease,
};