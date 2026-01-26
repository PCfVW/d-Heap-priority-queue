//! Generic d-ary heap priority queue implementation.
//!
//! This module provides a generic d-ary heap (d-heap) priority queue with:
//! - Configurable arity (d): number of children per node
//! - Min-heap or max-heap behavior via comparator functions
//! - O(1) item lookup using HashMap for efficient priority updates
//! - O(1) access to highest-priority item
//! - O(log_d n) insert and priority increase operations
//! - O(d · log_d n) pop and priority decrease operations
//!
//! The implementation is generic over the item type T, allowing users to
//! define their own item types with custom hash and equality functions.
//!
//! ## Example Usage
//!
//! ```zig
//! const std = @import("std");
//! const d_heap = @import("d_heap.zig");
//!
//! // Define your item type
//! const MyItem = struct {
//!     id: u32,
//!     priority: i32,
//!
//!     pub fn hash(self: MyItem) u64 {
//!         var hasher = std.hash.Wyhash.init(0);
//!         std.hash.autoHash(&hasher, self.id);
//!         return hasher.final();
//!     }
//!
//!     pub fn eql(a: MyItem, b: MyItem) bool {
//!         return a.id == b.id;
//!     }
//! };
//!
//! // Create comparator
//! fn lessByPriority(a: MyItem, b: MyItem) bool {
//!     return a.priority < b.priority;
//! }
//!
//! // Create heap type
//! const MyHeap = d_heap.DHeap(MyItem, d_heap.HashContext(MyItem), d_heap.Comparator(MyItem));
//!
//! // Use it
//! var heap = try MyHeap.init(3, .{ .cmp = lessByPriority }, allocator);
//! defer heap.deinit();
//! ```

const std = @import("std");

// Re-export the default Item type for convenience and backward compatibility
pub const Item = @import("types.zig").Item;

/// Generic hash context for types that implement hash() and eql() methods.
///
/// This context can be used with any type T that has:
/// - `pub fn hash(self: T) u64`
/// - `pub fn eql(a: T, b: T) bool` (or `pub fn eq(a: T, b: T) bool`)
///
/// Example:
/// ```zig
/// const MyContext = HashContext(MyItem);
/// const MyHeap = DHeap(MyItem, MyContext, Comparator(MyItem));
/// ```
pub fn HashContext(comptime T: type) type {
    return struct {
        const Self = @This();

        /// Compute hash for an item using its hash() method.
        pub fn hash(_: Self, key: T) u64 {
            return T.hash(key);
        }

        /// Check equality using the item's eql() or eq() method.
        pub fn eql(_: Self, a: T, b: T) bool {
            // Support both eql() and eq() method names
            if (@hasDecl(T, "eql")) {
                return T.eql(a, b);
            } else if (@hasDecl(T, "eq")) {
                return T.eq(a, b);
            } else {
                @compileError("Type " ++ @typeName(T) ++ " must have eql() or eq() method");
            }
        }
    };
}

/// Generic comparator wrapper for priority comparison functions.
///
/// Wraps a comparison function that determines priority ordering.
/// The function should return true if `a` has higher priority than `b`.
///
/// Example:
/// ```zig
/// fn minByPriority(a: MyItem, b: MyItem) bool {
///     return a.priority < b.priority;  // Lower value = higher priority
/// }
///
/// const cmp = Comparator(MyItem){ .cmp = minByPriority };
/// ```
pub fn Comparator(comptime T: type) type {
    return struct {
        /// Function pointer that returns true if `a` has higher priority than `b`
        cmp: *const fn (a: T, b: T) bool,

        /// Check if a has higher priority than b.
        pub fn higherPriority(self: @This(), a: T, b: T) bool {
            return self.cmp(a, b);
        }
    };
}

/// Generic d-ary heap priority queue.
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
/// ## Type Parameters
///
/// - `T`: Item type. Must provide `hash(self: T) u64` method for identity.
/// - `Context`: HashMap context type providing hash() and eql() for T.
///              Use `HashContext(T)` for types with standard hash/eql methods.
/// - `ComparatorType`: Comparator type with `higherPriority(a: T, b: T) bool`.
///                     Use `Comparator(T)` wrapper for function pointers.
///
/// ## Time Complexities
///
/// - front(): O(1)
/// - insert(): O(log_d n)
/// - pop(): O(d · log_d n)
/// - increasePriority(): O(log_d n)
/// - decreasePriority(): O(d · log_d n)
/// - contains(): O(1)
/// - len(), isEmpty(), d(): O(1)
///
/// ## Memory
///
/// O(n) for n items, plus HashMap overhead.
pub fn DHeap(
    comptime T: type,
    comptime Context: type,
    comptime ComparatorType: type,
) type {
    return struct {
        const Self = @This();
        // Zig 0.15.2: ArrayList is now unmanaged - allocator passed to methods
        const ArrayList = std.ArrayList(T);
        const HashMap = std.HashMap(T, usize, Context, std.hash_map.default_max_load_percentage);

        /// Number of children per node (arity of the heap)
        depth: usize,

        /// Array-based heap storage (complete tree representation)
        container: ArrayList,

        /// Maps each item to its position in the container for O(1) lookup
        positions: HashMap,

        /// Comparator determining heap order (min vs max)
        comparator: ComparatorType,

        /// Allocator used for all dynamic memory allocations
        allocator: std.mem.Allocator,

        // =====================================================================
        // Initialization and Cleanup
        // =====================================================================

        /// Initialize a new d-ary heap.
        ///
        /// Creates an empty priority queue with the specified arity and comparator.
        ///
        /// Parameters:
        /// - `depth`: Number of children per node (must be >= 1)
        /// - `comparator`: Comparator determining priority order
        /// - `allocator`: Memory allocator for heap operations
        ///
        /// Returns: Initialized heap or error if depth is 0
        ///
        /// Example:
        /// ```zig
        /// var heap = try MyHeap.init(3, .{ .cmp = minByPriority }, allocator);
        /// defer heap.deinit();
        /// ```
        pub fn init(
            depth: usize,
            comparator: ComparatorType,
            allocator: std.mem.Allocator,
        ) !Self {
            if (depth == 0) return error.DepthMustBePositive;
            return Self{
                .depth = depth,
                .container = ArrayList.empty,
                .positions = HashMap.init(allocator),
                .comparator = comparator,
                .allocator = allocator,
            };
        }

        /// Initialize a new d-ary heap with pre-allocated capacity.
        ///
        /// Use this when you know the approximate number of items to avoid
        /// reallocations during insertion.
        ///
        /// Parameters:
        /// - `depth`: Number of children per node (must be >= 1)
        /// - `comparator`: Comparator determining priority order
        /// - `allocator`: Memory allocator for heap operations
        /// - `capacity`: Initial capacity to pre-allocate
        ///
        /// Returns: Initialized heap or error
        pub fn initCapacity(
            depth: usize,
            comparator: ComparatorType,
            allocator: std.mem.Allocator,
            capacity: usize,
        ) !Self {
            if (depth == 0) return error.DepthMustBePositive;
            const container = try ArrayList.initCapacity(allocator, capacity);
            var positions = HashMap.init(allocator);
            try positions.ensureTotalCapacity(@intCast(capacity));
            return Self{
                .depth = depth,
                .container = container,
                .positions = positions,
                .comparator = comparator,
                .allocator = allocator,
            };
        }

        /// Free all resources used by the heap.
        ///
        /// Must be called to prevent memory leaks. After calling deinit(),
        /// the heap should not be used again.
        pub fn deinit(self: *Self) void {
            self.container.deinit(self.allocator);
            self.positions.deinit();
        }

        // =====================================================================
        // Public API - Query Operations
        // =====================================================================

        /// Get the number of items in the heap.
        ///
        /// Time complexity: O(1)
        pub fn len(self: Self) usize {
            return self.container.items.len;
        }

        /// Check if the heap is empty.
        ///
        /// Time complexity: O(1)
        ///
        /// Cross-language equivalents:
        ///   - Go: IsEmpty()
        ///   - TypeScript: isEmpty()
        pub fn isEmpty(self: Self) bool {
            return self.container.items.len == 0;
        }

        /// Alias for isEmpty() - snake_case for cross-language consistency.
        pub fn is_empty(self: Self) bool {
            return self.isEmpty();
        }

        /// Get the arity (number of children per node) of the heap.
        ///
        /// Time complexity: O(1)
        pub fn d(self: Self) usize {
            return self.depth;
        }

        /// Check if an item with the given identity exists in the heap.
        ///
        /// The item is looked up by identity (hash/equality), not by
        /// priority value. You can pass an item with any priority value
        /// as long as the identity fields match.
        ///
        /// Time complexity: O(1)
        ///
        /// Cross-language equivalents:
        ///   - Go: Contains()
        ///   - TypeScript: contains()
        ///
        /// Example:
        /// ```zig
        /// if (heap.contains(Item{ .number = 42, .cost = 0 })) {
        ///     // Item with number=42 exists (cost doesn't matter for lookup)
        /// }
        /// ```
        pub fn contains(self: *const Self, item: T) bool {
            return self.positions.contains(item);
        }

        /// Get the position (index) of an item in the heap array.
        ///
        /// The item is looked up by identity (hash/equality), not by
        /// priority value. Returns null if the item is not found.
        ///
        /// Time complexity: O(1)
        ///
        /// Cross-language equivalents:
        ///   - Go: GetPosition()
        ///   - TypeScript: getPosition()
        ///
        /// Example:
        /// ```zig
        /// if (heap.getPosition(Item{ .number = 42, .cost = 0 })) |pos| {
        ///     // Item found at position `pos`
        /// }
        /// ```
        pub fn getPosition(self: *const Self, item: T) ?usize {
            return self.positions.get(item);
        }

        /// Alias for getPosition() - snake_case for cross-language consistency.
        pub fn get_position(self: *const Self, item: T) ?usize {
            return self.getPosition(item);
        }

        /// Get the highest-priority item without removing it.
        ///
        /// Returns null if the heap is empty.
        ///
        /// Time complexity: O(1)
        pub fn front(self: *const Self) ?T {
            if (self.container.items.len == 0) return null;
            return self.container.items[0];
        }

        /// Alias for front() - get highest-priority item without removing.
        ///
        /// Provided for API consistency with other priority queue implementations.
        ///
        /// Cross-language equivalents:
        ///   - Go: Peek()
        ///   - TypeScript: peek()
        ///   - C++: peek() (returns std::optional)
        ///   - Rust: peek()
        pub fn peek(self: *const Self) ?T {
            return self.front();
        }

        // =====================================================================
        // Public API - Modification Operations
        // =====================================================================

        /// Insert a new item into the heap.
        ///
        /// The item is added to the heap and repositioned to maintain heap invariants.
        ///
        /// **Important**: If an item with the same identity already exists,
        /// behavior is undefined. Use `contains()` to check first, or use
        /// `increasePriority()`/`decreasePriority()` to update existing items.
        ///
        /// Time complexity: O(log_d n)
        ///
        /// Returns: Error if allocation fails
        pub fn insert(self: *Self, item: T) !void {
            try self.container.append(self.allocator, item);
            const index = self.container.items.len - 1;
            try self.positions.put(item, index);
            try self.moveUp(index);
        }

        /// Insert multiple items into the heap.
        ///
        /// Uses Floyd's heapify algorithm which is O(n) for bulk insertion when
        /// starting from an empty heap, vs O(n log n) for individual inserts.
        ///
        /// Time complexity: O(n) where n is total items after insertion (when starting empty)
        ///                  O(k log n) when inserting k items into non-empty heap
        ///
        /// Cross-language equivalents:
        ///   - Go: InsertMany()
        ///   - TypeScript: insertMany()
        ///
        /// Returns: Error if allocation fails
        pub fn insertMany(self: *Self, items: []const T) !void {
            if (items.len == 0) return;

            const start_index = self.container.items.len;

            // Add all items to container and positions map
            for (items, 0..) |item, i| {
                try self.container.append(self.allocator, item);
                try self.positions.put(item, start_index + i);
            }

            // If this was an empty heap, use heapify O(n) instead of n insertions O(n log n)
            if (start_index == 0 and items.len > 1) {
                try self.heapify();
            } else {
                // Otherwise, sift up each new item
                var i: usize = start_index;
                while (i < self.container.items.len) : (i += 1) {
                    try self.moveUp(i);
                }
            }
        }

        /// Alias for insertMany() - snake_case for cross-language consistency.
        pub fn insert_many(self: *Self, items: []const T) !void {
            return self.insertMany(items);
        }

        /// Increase the priority of an existing item (move toward root).
        ///
        /// Updates an item's priority and repositions it upward in the heap.
        /// The item is identified by its identity (hash/equality), and its
        /// priority is updated to the new value.
        ///
        /// For a min-heap: decreasing the priority value increases importance
        /// For a max-heap: increasing the priority value increases importance
        ///
        /// This method only moves items upward for performance. If the new
        /// priority is actually lower, use `decreasePriority()` instead.
        ///
        /// Time complexity: O(log_d n)
        ///
        /// Cross-language equivalents:
        ///   - Go: IncreasePriority()
        ///   - TypeScript: increasePriority()
        ///
        /// Returns: Error if item not found
        pub fn increasePriority(self: *Self, updated_item: T) !void {
            const index_ptr = self.positions.getPtr(updated_item) orelse {
                return error.ItemNotFound;
            };
            const index = index_ptr.*;

            // Update the item in the container
            // Note: HashMap key doesn't need updating because hash/equality
            // are based on identity, not priority value.
            self.container.items[index] = updated_item;

            try self.moveUp(index);
        }

        /// Alias for increasePriority() - snake_case for cross-language consistency.
        pub fn increase_priority(self: *Self, updated_item: T) !void {
            return self.increasePriority(updated_item);
        }

        /// Increase the priority of the item at the given index.
        ///
        /// This is a lower-level method that only calls moveUp on the item
        /// at the specified index. The caller is responsible for ensuring
        /// the item's priority value has been updated appropriately.
        ///
        /// Time complexity: O(log_d n)
        ///
        /// Cross-language equivalents:
        ///   - Go: IncreasePriorityByIndex()
        ///   - TypeScript: increasePriorityByIndex()
        ///
        /// Returns: Error if index is out of bounds
        pub fn increasePriorityByIndex(self: *Self, index: usize) !void {
            if (index >= self.container.items.len) {
                return error.IndexOutOfBounds;
            }
            try self.moveUp(index);
        }

        /// Alias for increasePriorityByIndex() - snake_case for cross-language consistency.
        pub fn increase_priority_by_index(self: *Self, index: usize) !void {
            return self.increasePriorityByIndex(index);
        }

        /// Decrease the priority of the item at the given index.
        ///
        /// This is a lower-level method that only calls moveDown on the item
        /// at the specified index. The caller is responsible for ensuring
        /// the item's priority value has been updated appropriately.
        ///
        /// Time complexity: O(d · log_d n)
        ///
        /// Cross-language equivalents:
        ///   - Go: DecreasePriorityByIndex()
        ///   - TypeScript: decreasePriorityByIndex()
        ///
        /// Returns: Error if index is out of bounds
        pub fn decreasePriorityByIndex(self: *Self, index: usize) !void {
            if (index >= self.container.items.len) {
                return error.IndexOutOfBounds;
            }
            try self.moveDown(index);
        }

        /// Alias for decreasePriorityByIndex() - snake_case for cross-language consistency.
        pub fn decrease_priority_by_index(self: *Self, index: usize) !void {
            return self.decreasePriorityByIndex(index);
        }

        /// Decrease the priority of an existing item (move toward leaves).
        ///
        /// Updates an item's priority and repositions it downward in the heap.
        /// The item is identified by its identity (hash/equality), and its
        /// priority is updated to the new value.
        ///
        /// For a min-heap: increasing the priority value decreases importance
        /// For a max-heap: decreasing the priority value decreases importance
        ///
        /// This method only moves items downward for performance. If the new
        /// priority is actually higher, use `increasePriority()` instead.
        /// If direction is unknown, use `updatePriority()`.
        ///
        /// Time complexity: O(d · log_d n)
        ///
        /// Cross-language equivalents:
        ///   - Go: DecreasePriority()
        ///   - TypeScript: decreasePriority()
        ///
        /// Returns: Error if item not found
        pub fn decreasePriority(self: *Self, updated_item: T) !void {
            const index_ptr = self.positions.getPtr(updated_item) orelse {
                return error.ItemNotFound;
            };
            const index = index_ptr.*;

            // Update the item in the container
            self.container.items[index] = updated_item;

            // Only move down - caller asserts priority decreased
            try self.moveDown(index);
        }

        /// Alias for decreasePriority() - snake_case for cross-language consistency.
        pub fn decrease_priority(self: *Self, updated_item: T) !void {
            return self.decreasePriority(updated_item);
        }

        /// Update the priority of an existing item when direction is unknown.
        ///
        /// Updates an item's priority and repositions it in the heap, checking
        /// both upward and downward directions. Use this when you don't know
        /// whether the priority increased or decreased.
        ///
        /// For better performance when direction is known, use:
        /// - `increasePriority()` when item became more important
        /// - `decreasePriority()` when item became less important
        ///
        /// Time complexity: O((d+1) · log_d n) - checks both directions
        ///
        /// Cross-language equivalents:
        ///   - Go: UpdatePriority()
        ///   - TypeScript: updatePriority()
        ///
        /// Returns: Error if item not found
        pub fn updatePriority(self: *Self, updated_item: T) !void {
            const index_ptr = self.positions.getPtr(updated_item) orelse {
                return error.ItemNotFound;
            };
            const index = index_ptr.*;

            // Update the item in the container
            self.container.items[index] = updated_item;

            // Check both directions since we don't know which way priority changed
            try self.moveUp(index);
            // Re-fetch position in case moveUp changed it
            const new_index = self.positions.get(updated_item) orelse return error.ItemNotFound;
            try self.moveDown(new_index);
        }

        /// Alias for updatePriority() - snake_case for cross-language consistency.
        pub fn update_priority(self: *Self, updated_item: T) !void {
            return self.updatePriority(updated_item);
        }

        /// Remove and return the highest-priority item.
        ///
        /// Removes the root item and restructures the heap to maintain invariants.
        /// Returns null if the heap is empty.
        ///
        /// Time complexity: O(d · log_d n)
        ///
        /// Returns: The highest-priority item, null if empty, or error if internal operation fails
        pub fn pop(self: *Self) !?T {
            if (self.container.items.len == 0) return null;

            const top = self.container.items[0];
            const last_index = self.container.items.len - 1;

            if (last_index > 0) {
                // Swap root with last element
                try self.swapItems(0, last_index);
            }

            // Remove the last element (which is now the old root)
            _ = self.positions.remove(self.container.items[last_index]);
            _ = self.container.pop();

            // Restore heap property if there are remaining items
            if (self.container.items.len > 0) {
                try self.moveDown(0);
            }

            return top;
        }

        /// Remove and return multiple highest-priority items.
        ///
        /// Returns a slice containing up to `count` items in priority order.
        /// The caller owns the returned memory and must free it.
        ///
        /// Time complexity: O(count · d · log_d n)
        ///
        /// Cross-language equivalents:
        ///   - Go: PopMany()
        ///   - TypeScript: popMany()
        ///
        /// Returns: Allocated slice of popped items, or error if allocation fails
        pub fn popMany(self: *Self, count: usize) ![]T {
            const actual_count = @min(count, self.container.items.len);
            if (actual_count == 0) {
                return &[_]T{};
            }

            var result = try self.allocator.alloc(T, actual_count);
            errdefer self.allocator.free(result);

            for (0..actual_count) |i| {
                if (try self.pop()) |item| {
                    result[i] = item;
                }
            }

            return result;
        }

        /// Alias for popMany() - snake_case for cross-language consistency.
        pub fn pop_many(self: *Self, count: usize) ![]T {
            return self.popMany(count);
        }

        /// Get a copy of all items in heap order (not priority order).
        ///
        /// Returns a newly allocated slice containing all items.
        /// The caller owns the returned memory and must free it.
        ///
        /// Time complexity: O(n)
        ///
        /// Cross-language equivalents:
        ///   - Go: ToArray()
        ///   - TypeScript: toArray()
        ///
        /// Returns: Allocated slice of all items, or error if allocation fails
        pub fn toArray(self: *const Self) ![]T {
            if (self.container.items.len == 0) {
                return &[_]T{};
            }

            const result = try self.allocator.alloc(T, self.container.items.len);
            @memcpy(result, self.container.items);
            return result;
        }

        /// Alias for toArray() - snake_case for cross-language consistency.
        pub fn to_array(self: *const Self) ![]T {
            return self.toArray();
        }

        /// Clear all items from the heap, optionally changing the arity.
        ///
        /// Removes all items while retaining allocated capacity for efficiency.
        ///
        /// Parameters:
        /// - `new_depth`: Optional new arity value (must be >= 1 if provided)
        ///
        /// Time complexity: O(1) (capacity retained)
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
        /// **Note**: Requires T to implement the `format` function for std.fmt.
        ///
        /// Time complexity: O(n)
        pub fn toString(self: Self) ![]const u8 {
            var buffer = std.ArrayList(u8).empty;
            errdefer buffer.deinit(self.allocator);

            const writer = buffer.writer(self.allocator);
            try writer.writeAll("{");
            for (self.container.items, 0..) |item, i| {
                if (i != 0) try writer.writeAll(", ");
                // Use {f} to invoke the item's format method
                try writer.print("{f}", .{item});
            }
            try writer.writeAll("}");
            return buffer.toOwnedSlice(self.allocator);
        }

        /// Alias for toString() - snake_case for cross-language consistency.
        ///
        /// Provides the same functionality as `toString()` but with snake_case naming
        /// to match C++ and Rust implementations for easier cross-language usage.
        ///
        /// Time complexity: O(n)
        pub fn to_string(self: Self) ![]const u8 {
            return self.toString();
        }

        // =====================================================================
        // Private Methods - Heap Operations
        // =====================================================================

        /// Calculate the parent index of a node.
        /// In a d-ary heap stored as an array, the parent of node i is at (i-1)/d.
        fn parent(self: Self, i: usize) usize {
            return (i - 1) / self.depth;
        }

        /// Find the child with highest priority among all children of node i.
        fn bestChildPosition(self: *Self, i: usize) usize {
            const left = i * self.depth + 1;
            if (left >= self.container.items.len) return left;

            var best = left;
            const right = @min((i + 1) * self.depth, self.container.items.len - 1);

            var j: usize = left + 1;
            while (j <= right) : (j += 1) {
                if (self.comparator.higherPriority(self.container.items[j], self.container.items[best])) {
                    best = j;
                }
            }
            return best;
        }

        /// Swap two items in the heap and update their position mappings.
        ///
        /// This is a low-level operation that updates both the container and
        /// the position HashMap.
        ///
        /// Returns: Error if position map update fails (should not happen for existing keys)
        fn swapItems(self: *Self, i: usize, j: usize) !void {
            if (i == j) return;

            // Swap in container
            std.mem.swap(T, &self.container.items[i], &self.container.items[j]);

            // Update positions in HashMap
            // These items are already in the HashMap (inserted during insert()).
            // We're only updating the position values, not adding new keys.
            // The put() operation for existing keys should not require allocation
            // in most cases, but we propagate any errors to maintain heap invariants.
            try self.positions.put(self.container.items[i], i);
            try self.positions.put(self.container.items[j], j);
        }

        /// Build heap property from unordered array using Floyd's algorithm.
        ///
        /// Called internally by insertMany when starting from empty heap.
        /// Time complexity: O(n)
        fn heapify(self: *Self) !void {
            const n = self.container.items.len;
            if (n <= 1) return;

            // Start from last non-leaf node and sift down each
            // Last non-leaf is parent of last element: floor((n-2)/d)
            // Use saturating subtraction to handle n < 2 case safely
            const last_non_leaf = (n -| 2) / self.depth;

            // Iterate from last_non_leaf down to 0 (inclusive)
            var i: usize = last_non_leaf + 1;
            while (i > 0) {
                i -= 1;
                try self.moveDown(i);
            }
        }

        /// Move an item upward in the heap to restore heap property.
        fn moveUp(self: *Self, i_param: usize) !void {
            var i = i_param;
            while (i > 0) {
                const p = self.parent(i);
                if (self.comparator.higherPriority(self.container.items[i], self.container.items[p])) {
                    try self.swapItems(i, p);
                    i = p;
                } else {
                    break;
                }
            }
        }

        /// Move an item downward in the heap to restore heap property.
        fn moveDown(self: *Self, i_param: usize) !void {
            var i = i_param;
            while (true) {
                const first_child = i * self.depth + 1;
                if (first_child >= self.container.items.len) break;

                const best = self.bestChildPosition(i);
                if (self.comparator.higherPriority(self.container.items[best], self.container.items[i])) {
                    try self.swapItems(i, best);
                    i = best;
                } else {
                    break;
                }
            }
        }
    };
}

// =============================================================================
// Convenience Types for Default Item
// =============================================================================

/// Default hash context for the built-in Item type.
/// Provided for backward compatibility and convenience.
pub const ItemContext = HashContext(Item);

/// Default comparator type for the built-in Item type.
pub const ItemComparator = Comparator(Item);

/// Pre-configured DHeap type using the default Item type.
/// This provides backward compatibility with the non-generic API.
///
/// Example:
/// ```zig
/// var heap = try DHeapItem.init(3, MinByCost, allocator);
/// ```
pub const DHeapItem = DHeap(Item, ItemContext, ItemComparator);

// =============================================================================
// Pre-defined Comparators for Default Item Type
// =============================================================================

/// Min-heap comparator based on cost field.
///
/// Lower cost values have higher priority (appear closer to root).
/// Use this for algorithms like Dijkstra's shortest path.
pub const MinByCost = ItemComparator{ .cmp = minByCost };

/// Max-heap comparator based on cost field.
///
/// Higher cost values have higher priority (appear closer to root).
pub const MaxByCost = ItemComparator{ .cmp = maxByCost };

/// Comparator function for min-heap: lower cost = higher priority.
fn minByCost(a: Item, b: Item) bool {
    return a.cost < b.cost;
}

/// Comparator function for max-heap: higher cost = higher priority.
fn maxByCost(a: Item, b: Item) bool {
    return a.cost > b.cost;
}

// =============================================================================
// Type Aliases
// =============================================================================

/// Type alias for position indices in the heap.
/// Provided for API parity with C++ and Rust implementations.
pub const Position = usize;

// =============================================================================
// Error Types
// =============================================================================

/// Errors that can occur during heap operations.
pub const Error = error{
    /// Attempted to create heap with arity d = 0 (must be >= 1)
    DepthMustBePositive,

    /// Attempted to update priority of item not present in heap
    ItemNotFound,

    /// Attempted to access item at invalid index
    IndexOutOfBounds,
};
