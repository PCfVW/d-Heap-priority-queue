//! D-ary heap priority queue implementation.
//!
//! This module provides a generic d-ary heap (d-heap) priority queue with:
//! - Configurable arity (d): number of children per node
//! - Min-heap or max-heap behavior via comparator functions
//! - O(1) item lookup using HashMap for efficient priority updates
//! - O(1) access to highest-priority item
//! - O(log_d n) insert and priority increase operations
//! - O(d · log_d n) pop and priority decrease operations
//!
//! The implementation maintains heap invariants where each parent has higher
//! priority than all its children, enabling efficient priority queue operations.

const std = @import("std");
const Item = @import("types.zig").Item;

/// Priority comparator function type.
///
/// Defines how to compare two items to determine which has higher priority.
/// Used to configure min-heap vs max-heap behavior.
///
/// Example for min-heap (lower cost = higher priority):
/// ```zig
/// fn minByCost(a: Item, b: Item) bool {
///     return a.cost < b.cost;
/// }
/// const comparator = Comparator{ .higher_priority = minByCost };
/// ```
pub const Comparator = struct {
    /// Function pointer that returns true if `a` has higher priority than `b`
    higher_priority: *const fn (a: Item, b: Item) bool,
};

/// Context for HashMap operations.
///
/// Provides hash and equality functions required by std.HashMap.
/// This is an internal implementation detail.
const ItemContext = struct {
    /// Compute hash for an item (delegates to Item.hash)
    pub fn hash(_: ItemContext, key: Item) u64 {
        return Item.hash(key);
    }

    /// Check equality between items (delegates to Item.eq)
    pub fn eql(_: ItemContext, a: Item, b: Item) bool {
        return Item.eq(a, b);
    }
};

/// D-ary heap priority queue.
///
/// A d-ary heap is a tree structure where:
/// - Each node has at most d children
/// - The root contains the highest-priority item
/// - Each parent has higher priority than all its children
/// - The tree is complete (filled left-to-right, level by level)
///
/// This implementation uses an array-based representation with O(1) item lookup
/// via a HashMap that tracks each item's position in the heap.
///
/// Time complexities:
/// - front(): O(1)
/// - insert(): O(log_d n)
/// - pop(): O(d · log_d n)
/// - increasePriority(): O(log_d n)
/// - decreasePriority(): O(d · log_d n)
/// - len(), isEmpty(), d(): O(1)
///
/// Memory: O(n) for n items
pub const DHeap = struct {
    const Self = @This();
    const ArrayList = std.ArrayList(Item);
    const HashMap = std.HashMap(Item, usize, ItemContext, std.hash_map.default_max_load_percentage);

    /// Number of children per node (arity of the heap)
    depth: usize,

    /// Array-based heap storage (complete tree representation)
    container: ArrayList,

    /// Maps each item to its position in the container for O(1) lookup
    positions: HashMap,

    /// Comparator function determining heap order (min vs max)
    comparator: Comparator,

    /// Allocator used for all dynamic memory allocations
    allocator: std.mem.Allocator,

    /// Initialize a new d-ary heap.
    ///
    /// Creates an empty priority queue with the specified arity and comparator.
    ///
    /// Parameters:
    /// - `depth`: Number of children per node (must be >= 1)
    /// - `comparator`: Function determining priority order
    /// - `allocator`: Memory allocator for heap operations
    ///
    /// Returns: Initialized heap or error if depth is 0
    ///
    /// Example:
    /// ```zig
    /// var heap = try DHeap.init(3, MinByCost, allocator);
    /// defer heap.deinit();
    /// ```
    pub fn init(
        depth: usize,
        comparator: Comparator,
        allocator: std.mem.Allocator,
    ) !Self {
        if (depth == 0) return error.DepthMustBePositive;
        return Self{
            .depth = depth,
            .container = .empty,
            .positions = HashMap.init(allocator),
            .comparator = comparator,
            .allocator = allocator,
        };
    }

    /// Free all resources used by the heap.
    ///
    /// Must be called to prevent memory leaks. After calling deinit(),
    /// the heap should not be used again.
    ///
    /// Example:
    /// ```zig
    /// var heap = try DHeap.init(3, MinByCost, allocator);
    /// defer heap.deinit(); // Ensures cleanup even on error
    /// ```
    pub fn deinit(self: *Self) void {
        self.container.deinit(self.allocator);
        self.positions.deinit();
    }

    // --- Public API ---

    /// Get the number of items in the heap.
    ///
    /// Returns: Count of items currently in the heap
    ///
    /// Time complexity: O(1)
    pub fn len(self: Self) usize {
        return self.container.items.len;
    }

    /// Check if the heap is empty.
    ///
    /// Returns: true if heap contains no items, false otherwise
    ///
    /// Time complexity: O(1)
    pub fn isEmpty(self: Self) bool {
        return self.container.items.len == 0;
    }

    /// Get the arity (number of children per node) of the heap.
    ///
    /// Returns: The d value specified at initialization
    ///
    /// Time complexity: O(1)
    pub fn d(self: Self) usize {
        return self.depth;
    }

    /// Get the highest-priority item without removing it.
    ///
    /// Returns: The item at the root of the heap, or null if empty
    ///
    /// Time complexity: O(1)
    ///
    /// Example:
    /// ```zig
    /// if (heap.front()) |item| {
    ///     std.debug.print("Top priority: {any}\n", .{item});
    /// }
    /// ```
    pub fn front(self: *Self) ?Item {
        if (self.container.items.len == 0) return null;
        return self.container.items[0];
    }

    /// Insert a new item into the heap.
    ///
    /// The item is added to the heap and repositioned to maintain heap invariants.
    /// If an item with the same identity already exists, behavior is undefined.
    ///
    /// Parameters:
    /// - `item`: The item to insert
    ///
    /// Returns: Error if allocation fails
    ///
    /// Time complexity: O(log_d n)
    ///
    /// Example:
    /// ```zig
    /// try heap.insert(Item{ .number = 42, .cost = 100 });
    /// ```
    pub fn insert(self: *Self, item: Item) !void {
        try self.container.append(self.allocator, item);
        const index = self.container.items.len - 1;
        try self.positions.put(item, index);
        self.moveUp(index);
    }

    /// Increase the priority of an existing item (move toward root).
    ///
    /// Updates an item's priority and repositions it upward in the heap.
    /// The item is identified by its identity (number field), and its priority
    /// (cost field) is updated to the new value.
    ///
    /// For a min-heap: decreasing cost increases priority
    /// For a max-heap: increasing cost increases priority
    ///
    /// This method only moves items upward for performance. If the new priority
    /// is actually lower, use decreasePriority() instead.
    ///
    /// Parameters:
    /// - `updated_item`: Item with same identity but new priority
    ///
    /// Returns: Error if item not found or allocation fails
    ///
    /// Time complexity: O(log_d n)
    ///
    /// Example:
    /// ```zig
    /// // For min-heap: lower cost = higher priority
    /// try heap.increasePriority(Item{ .number = 42, .cost = 50 });
    /// ```
    pub fn increasePriority(self: *Self, updated_item: Item) !void {
        const index = self.positions.getPtr(updated_item) orelse {
            return error.ItemNotFound;
        };
        self.container.items[index.*] = updated_item;
        self.moveUp(index.*);
    }

    /// Decrease the priority of an existing item (move toward leaves).
    ///
    /// Updates an item's priority and repositions it in the heap, checking both
    /// upward and downward directions. This is more defensive than increasePriority()
    /// and handles cases where the caller might use the wrong method.
    ///
    /// For a min-heap: increasing cost decreases priority
    /// For a max-heap: decreasing cost decreases priority
    ///
    /// Parameters:
    /// - `updated_item`: Item with same identity but new priority
    ///
    /// Returns: Error if item not found or allocation fails
    ///
    /// Time complexity: O(d · log_d n)
    ///
    /// Example:
    /// ```zig
    /// // For min-heap: higher cost = lower priority
    /// try heap.decreasePriority(Item{ .number = 42, .cost = 150 });
    /// ```
    pub fn decreasePriority(self: *Self, updated_item: Item) !void {
        const index = self.positions.getPtr(updated_item) orelse {
            return error.ItemNotFound;
        };
        self.container.items[index.*] = updated_item;
        self.moveUp(index.*);
        self.moveDown(index.*);
    }

    /// Remove and return the highest-priority item.
    ///
    /// Removes the root item and restructures the heap to maintain invariants.
    ///
    /// Returns: The highest-priority item, or null if heap is empty
    ///
    /// Time complexity: O(d · log_d n)
    ///
    /// Example:
    /// ```zig
    /// while (heap.pop()) |item| {
    ///     std.debug.print("Processing: {any}\n", .{item});
    /// }
    /// ```
    pub fn pop(self: *Self) ?Item {
        if (self.container.items.len == 0) return null;
        const top = self.container.items[0];
        self.swap(0, self.container.items.len - 1);
        _ = self.positions.remove(self.container.items[self.container.items.len - 1]);
        _ = self.container.pop();
        if (self.container.items.len > 0) {
            self.moveDown(0);
        }
        return top;
    }

    /// Clear all items from the heap, optionally changing the arity.
    ///
    /// Removes all items while retaining allocated capacity for efficiency.
    /// Can optionally change the heap's arity (d value).
    ///
    /// Parameters:
    /// - `new_depth`: Optional new arity value (must be >= 1 if provided)
    ///
    /// Returns: Error if new_depth is 0
    ///
    /// Time complexity: O(1)
    ///
    /// Example:
    /// ```zig
    /// try heap.clear(null);      // Clear, keep same d
    /// try heap.clear(4);          // Clear and change d to 4
    /// ```
    pub fn clear(self: *Self, new_depth: ?usize) !void {
        self.container.clearRetainingCapacity();
        self.positions.clearRetainingCapacity();
        if (new_depth) |new_d| {
            if (new_d == 0) return error.DepthMustBePositive;
            self.depth = new_d;
        }
    }

    /// Get a string representation of the heap contents.
    ///
    /// Returns a formatted string showing all items in heap order.
    /// The caller owns the returned memory and must free it.
    ///
    /// Returns: Allocated string containing heap contents
    ///
    /// Time complexity: O(n)
    ///
    /// Example:
    /// ```zig
    /// const str = try heap.toString();
    /// defer allocator.free(str);
    /// std.debug.print("Heap: {s}\n", .{str});
    /// ```
    pub fn toString(self: Self) ![]const u8 {
        var buffer: std.ArrayList(u8) = .empty;
        defer buffer.deinit(self.allocator);
        try buffer.appendSlice(self.allocator, "{");
        for (self.container.items, 0..) |item, i| {
            if (i != 0) try buffer.appendSlice(self.allocator, ", ");
            const item_str = try std.fmt.allocPrint(self.allocator, "{any}", .{item});
            defer self.allocator.free(item_str);
            try buffer.appendSlice(self.allocator, item_str);
        }
        try buffer.appendSlice(self.allocator, "}");
        return buffer.toOwnedSlice(self.allocator);
    }

    // --- Private methods ---

    /// Calculate the parent index of a node.
    ///
    /// In a d-ary heap stored as an array, the parent of node i is at (i-1)/d.
    ///
    /// Parameters:
    /// - `i`: Index of the child node
    ///
    /// Returns: Index of the parent node
    fn parent(self: Self, i: usize) usize {
        return (i - 1) / self.depth;
    }

    /// Find the child with highest priority among all children of node i.
    ///
    /// Examines all children of the given node and returns the index of the
    /// child with the highest priority according to the comparator.
    ///
    /// Parameters:
    /// - `i`: Index of the parent node
    ///
    /// Returns: Index of the best child, or an out-of-bounds index if no children exist
    fn bestChildPosition(self: *Self, i: usize) usize {
        const left = i * self.depth + 1;
        if (left >= self.container.items.len) return left;
        var best = left;
        const right = @min((i + 1) * self.depth, self.container.items.len - 1);
        var j: usize = left + 1;
        while (j <= right) : (j += 1) {
            if (self.comparator.higher_priority(self.container.items[j], self.container.items[best])) {
                best = j;
            }
        }
        return best;
    }

    /// Swap two items in the heap and update their position mappings.
    ///
    /// Exchanges the items at indices i and j in the container array and
    /// updates the position HashMap to reflect the new locations.
    ///
    /// Parameters:
    /// - `i`: Index of first item
    /// - `j`: Index of second item
    fn swap(self: *Self, i: usize, j: usize) void {
        if (i == j) return;
        std.mem.swap(Item, &self.container.items[i], &self.container.items[j]);
        self.positions.put(self.container.items[i], i) catch unreachable;
        self.positions.put(self.container.items[j], j) catch unreachable;
    }

    /// Move an item upward in the heap to restore heap property.
    ///
    /// Repeatedly swaps the item with its parent while it has higher priority
    /// than the parent, stopping when heap property is satisfied or root is reached.
    ///
    /// Parameters:
    /// - `i_param`: Starting index of the item to move up
    fn moveUp(self: *Self, i_param: usize) void {
        var i = i_param;
        while (i > 0) {
            const p = self.parent(i);
            if (self.comparator.higher_priority(self.container.items[i], self.container.items[p])) {
                self.swap(i, p);
                i = p;
            } else {
                break;
            }
        }
    }

    /// Move an item downward in the heap to restore heap property.
    ///
    /// Repeatedly swaps the item with its highest-priority child while any child
    /// has higher priority, stopping when heap property is satisfied or a leaf is reached.
    ///
    /// Parameters:
    /// - `i_param`: Starting index of the item to move down
    fn moveDown(self: *Self, i_param: usize) void {
        var i = i_param;
        while (true) {
            const first_child = i * self.depth + 1;
            if (first_child >= self.container.items.len) break;
            const best = self.bestChildPosition(i);
            if (self.comparator.higher_priority(self.container.items[best], self.container.items[i])) {
                self.swap(i, best);
                i = best;
            } else {
                break;
            }
        }
    }
};

// --- Pre-defined Comparators ---

/// Min-heap comparator based on cost field.
///
/// Lower cost values have higher priority (appear closer to root).
/// Use this for algorithms like Dijkstra's shortest path where you want
/// to process items with smallest cost first.
///
/// Example:
/// ```zig
/// var heap = try DHeap.init(3, MinByCost, allocator);
/// ```
pub const MinByCost = Comparator{ .higher_priority = minByCost };

/// Max-heap comparator based on cost field.
///
/// Higher cost values have higher priority (appear closer to root).
/// Use this when you want to process items with largest cost first.
///
/// Example:
/// ```zig
/// var heap = try DHeap.init(3, MaxByCost, allocator);
/// ```
pub const MaxByCost = Comparator{ .higher_priority = maxByCost };

/// Comparator function for min-heap: lower cost = higher priority.
fn minByCost(a: Item, b: Item) bool {
    return a.cost < b.cost;
}

/// Comparator function for max-heap: higher cost = higher priority.
fn maxByCost(a: Item, b: Item) bool {
    return a.cost > b.cost;
}

// --- Type Aliases ---

/// Type alias for position indices in the heap.
///
/// Used internally by the HashMap to track item positions.
/// Provided for API parity with C++ and Rust implementations.
pub const Position = usize;

// --- Error Types ---

/// Errors that can occur during heap operations.
pub const Error = error{
    /// Attempted to create heap with arity d = 0 (must be >= 1)
    DepthMustBePositive,

    /// Attempted to update priority of item not present in heap
    ItemNotFound,
};
