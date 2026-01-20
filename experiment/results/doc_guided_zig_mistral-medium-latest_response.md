# D-ary Heap Priority Queue in Zig

Here's a complete implementation of a d-ary heap priority queue in Zig with all the requested functionality:

```zig
const std = @import("std");
const heap = std.heap;
const mem = std.mem;
const math = std.math;

// Define a generic d-ary heap priority queue
pub const DaryHeap = struct {
    const Self = @This();

    // The arity of the heap (number of children per node)
    d: usize,

    // The heap array storing items
    items: std.ArrayList([]const u8),

    // Position map (hash map) for O(1) lookups
    position_map: std.StringHashMap(usize),

    // Allocator for the heap
    allocator: std.mem.Allocator,

    // Comparator function for priorities
    comparator: *const fn (a: []const u8, b: []const u8) bool,

    // Hash function for items
    hash_fn: *const fn (item: []const u8) []const u8,

    // Equality function for items
    eq_fn: *const fn (a: []const u8, b: []const u8) bool,

    // Initialize a new d-ary heap
    pub fn init(
        allocator: std.mem.Allocator,
        d: usize,
        comparator: *const fn (a: []const u8, b: []const u8) bool,
        hash_fn: *const fn (item: []const u8) []const u8,
        eq_fn: *const fn (a: []const u8, b: []const u8) bool,
    ) !Self {
        if (d < 2) {
            return error.DaryHeapInvalidAritry;
        }

        return Self{
            .d = d,
            .items = std.ArrayList([]const u8).init(allocator),
            .position_map = std.StringHashMap(usize).init(allocator),
            .allocator = allocator,
            .comparator = comparator,
            .hash_fn = hash_fn,
            .eq_fn = eq_fn,
        };
    }

    // Deinitialize the heap
    pub fn deinit(self: *Self) void {
        self.items.deinit();
        self.position_map.deinit();
    }

    // Get the number of items in the heap
    pub fn len(self: *const Self) usize {
        return self.items.items.len;
    }

    // Check if the heap is empty
    pub fn isEmpty(self: *const Self) bool {
        return self.len() == 0;
    }

    // Check if an item exists in the heap
    pub fn contains(self: *const Self, item: []const u8) bool {
        return self.position_map.getPtr(self.hash_fn(item)) != null;
    }

    // Get the parent index of a given index
    fn parentIndex(self: *const Self, index: usize) usize {
        return (index - 1) / self.d;
    }

    // Get the first child index of a given index
    fn firstChildIndex(self: *const Self, index: usize) usize {
        return index * self.d + 1;
    }

    // Get the last child index of a given index
    fn lastChildIndex(self: *const Self, index: usize) usize {
        return index * self.d + self.d;
    }

    // Swap two items in the heap and update their positions in the map
    fn swap(self: *Self, i: usize, j: usize) void {
        const temp = self.items.items[i];
        self.items.items[i] = self.items.items[j];
        self.items.items[j] = temp;

        // Update position map
        const item_i = self.items.items[i];
        const item_j = self.items.items[j];
        self.position_map.putPtr(self.hash_fn(item_i), i) orelse unreachable;
        self.position_map.putPtr(self.hash_fn(item_j), j) orelse unreachable;
    }

    // Sift an item up the heap to maintain heap property
    fn siftUp(self: *Self, index: usize) void {
        var current = index;
        while (current > 0) {
            const parent = self.parentIndex(current);
            if (self.comparator(self.items.items[current], self.items.items[parent])) {
                self.swap(current, parent);
                current = parent;
            } else {
                break;
            }
        }
    }

    // Sift an item down the heap to maintain heap property
    fn siftDown(self: *Self, index: usize) void {
        var current = index;
        const length = self.len();

        while (true) {
            const first_child = self.firstChildIndex(current);
            if (first_child >= length) {
                break; // No children
            }

            // Find the child with highest priority (lowest value)
            var min_child = first_child;
            const last_child = math.min(self.lastChildIndex(current), length - 1);

            var i: usize = first_child + 1;
            while (i <= last_child) : (i += 1) {
                if (self.comparator(self.items.items[i], self.items.items[min_child])) {
                    min_child = i;
                }
            }

            // If the current item has lower priority than the min child, swap
            if (self.comparator(self.items.items[min_child], self.items.items[current])) {
                self.swap(current, min_child);
                current = min_child;
            } else {
                break;
            }
        }
    }

    // Insert an item into the heap
    pub fn insert(self: *Self, item: []const u8) !void {
        // Check if item already exists
        if (self.contains(item)) {
            return error.DaryHeapItemExists;
        }

        // Add to the end of the array
        try self.items.appendSlice(item);

        // Add to position map
        try self.position_map.put(self.hash_fn(item), self.len() - 1);

        // Sift up to maintain heap property
        self.siftUp(self.len() - 1);
    }

    // Remove and return the item with highest priority
    pub fn pop(self: *Self) ?[]const u8 {
        if (self.isEmpty()) {
            return null;
        }

        // Get the root item
        const root = self.items.items[0];

        // Remove the last item and put it at root
        if (self.len() > 1) {
            self.items.items[0] = self.items.items[self.len() - 1];
            // Update position map for the moved item
            self.position_map.putPtr(self.hash_fn(self.items.items[0]), 0) orelse unreachable;
        }

        // Remove from position map
        self.position_map.remove(self.hash_fn(root));

        // Remove the last item
        self.items.pop();

        // Sift down to maintain heap property
        if (!self.isEmpty()) {
            self.siftDown(0);
        }

        return root;
    }

    // Get the item with highest priority without removing it
    pub fn front(self: *const Self) ?[]const u8 {
        if (self.isEmpty()) {
            return null;
        }
        return self.items.items[0];
    }

    // Increase priority (make more important) of an existing item
    pub fn increasePriority(self: *Self, item: []const u8) !void {
        const index_ptr = self.position_map.getPtr(self.hash_fn(item));
        if (index_ptr == null) {
            return error.DaryHeapItemNotFound;
        }

        const index = index_ptr.*;
        // Update the item in place (assuming the caller has updated the priority)
        // Since we can't modify the slice directly, we need to replace it
        self.items.items[index] = item;

        // Sift up to maintain heap property
        self.siftUp(index);
    }

    // Decrease priority (make less important) of an existing item
    pub fn decreasePriority(self: *Self, item: []const u8) !void {
        const index_ptr = self.position_map.getPtr(self.hash_fn(item));
        if (index_ptr == null) {
            return error.DaryHeapItemNotFound;
        }

        const index = index_ptr.*;
        // Update the item in place (assuming the caller has updated the priority)
        self.items.items[index] = item;

        // Sift down to maintain heap property
        self.siftDown(index);
    }
};

// Error types
pub const DaryHeapError = error{
    DaryHeapInvalidAritry,
    DaryHeapItemExists,
    DaryHeapItemNotFound,
    OutOfMemory,
};
```

## Example Usage

Here's how you might use this implementation:

```zig
const std = @import("std");

const PriorityQueue = struct {
    const Self = @This();

    // Define a simple item type with identity and priority
    const Item = struct {
        id: u32,
        priority: u32,

        pub fn hash(item: []const u8) []const u8 {
            const self = @ptrCast(@alignCast(@alignOf(Item), *const Item), item);
            return std.mem.asBytes(&self.id);
        }

        pub fn eq(a: []const u8, b: []const u8) bool {
            const a_item = @ptrCast(@alignCast(@alignOf(Item), *const Item), a);
            const b_item = @ptrCast(@alignCast(@alignOf(Item), *const Item), b);
            return a_item.id == b_item.id;
        }

        pub fn compare(a: []const u8, b: []const u8) bool {
            const a_item = @ptrCast(@alignCast(@alignOf(Item), *const Item), a);
            const b_item = @ptrCast(@alignCast(@alignOf(Item), *const Item), b);
            return a_item.priority < b_item.priority;
        }
    };

    heap: DaryHeap,

    pub fn init(allocator: std.mem.Allocator, d: usize) !Self {
        return Self{
            .heap = try DaryHeap.init(allocator, d, Item.compare, Item.hash, Item.eq),
        };
    }

    pub fn deinit(self: *Self) void {
        self.heap.deinit();
    }

    pub fn insert(self: *Self, item: Item) !void {
        try self.heap.insert(std.mem.asBytes(&item));
    }

    pub fn pop(self: *Self) ?Item {
        return self.heap.pop().?;
    }

    pub fn front(self: *Self) ?Item {
        return self.heap.front().?;
    }

    pub fn increasePriority(self: *Self, item: Item) !void {
        try self.heap.increasePriority(std.mem.asBytes(&item));
    }

    pub fn decreasePriority(self: *Self, item: Item) !void {
        try self.heap.decreasePriority(std.mem.asBytes(&item));
    }

    pub fn contains(self: *Self, item: Item) bool {
        return self.heap.contains(std.mem.asBytes(&item));
    }

    pub fn len(self: *Self) usize {
        return self.heap.len();
    }

    pub fn isEmpty(self: *Self) bool {
        return self.heap.isEmpty();
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var pq = try PriorityQueue.init(allocator, 3);
    defer pq.deinit();

    try pq.insert(.{ .id = 1, .priority = 5 });
    try pq.insert(.{ .id = 2, .priority = 3 });
    try pq.insert(.{ .id = 3, .priority = 7 });
    try pq.insert(.{ .id = 4, .priority = 1 });

    while (!pq.isEmpty()) {
        const item = pq.pop() orelse {
            std.debug.print("Queue is empty\n", .{});
            break;
        };
        std.debug.print("Popped: id={d}, priority={d}\n", .{ item.id, item.priority });
    }
}
```

## Key Features

1. **Generic Implementation**: Works with any type that can be compared and hashed
2. **Position Map**: Uses a hash map for O(1) lookups of item positions
3. **Complete API**: Implements all requested operations (insert, pop, front, increase/decrease priority, contains, etc.)
4. **Efficient Operations**:
   - Insert: O(log_d n)
   - Pop: O(d * log_d n)
   - Front: O(1)
   - Priority updates: O(log_d n) or O(d * log_d n)
   - Contains: O(1)
5. **Memory Safety**: Proper initialization and deinitialization
6. **Error Handling**: Returns appropriate errors for invalid operations

The implementation maintains the heap property at all times and keeps the position map synchronized with the heap array.