const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const testing = std.testing;
const expect = testing.expect;
const expectEqual = testing.expectEqual;
const expectError = testing.expectError;

pub fn DaryHeap(comptime T: type) type {
    return struct {
        const Self = @This();
        
        pub const Error = error{
            InvalidArity,
            ItemNotFound,
            ItemAlreadyExists,
            EmptyQueue,
        };

        // Item wrapper that holds both identity and priority
        const Item = struct {
            data: T,
            priority: i32,

            fn init(data: T, priority: i32) Item {
                return Item{ .data = data, .priority = priority };
            }
        };

        // Hash map context for using T as key
        const HashContext = struct {
            pub fn hash(self: @This(), key: T) u64 {
                _ = self;
                return std.hash_map.getAutoHashFn(T, void)({}, key);
            }

            pub fn eql(self: @This(), a: T, b: T) bool {
                _ = self;
                return std.hash_map.getAutoEqlFn(T, void)({}, a, b);
            }
        };

        const PositionMap = HashMap(T, usize, HashContext, std.hash_map.default_max_load_percentage);

        allocator: Allocator,
        heap: ArrayList(Item),
        position_map: PositionMap,
        d: usize, // arity

        pub fn init(allocator: Allocator, d: usize) Error!Self {
            if (d < 2) {
                return Error.InvalidArity;
            }

            return Self{
                .allocator = allocator,
                .heap = ArrayList(Item).init(allocator),
                .position_map = PositionMap.init(allocator),
                .d = d,
            };
        }

        pub fn deinit(self: *Self) void {
            self.heap.deinit();
            self.position_map.deinit();
        }

        pub fn insert(self: *Self, data: T, priority: i32) Error!void {
            // Check if item already exists
            if (self.position_map.contains(data)) {
                return Error.ItemAlreadyExists;
            }

            const item = Item.init(data, priority);
            
            // Add to end of heap
            try self.heap.append(item);
            const index = self.heap.items.len - 1;
            
            // Update position map
            try self.position_map.put(data, index);
            
            // Restore heap property
            self.sift_up(index);
        }

        pub fn pop(self: *Self) Error!Item {
            if (self.heap.items.len == 0) {
                return Error.EmptyQueue;
            }

            const root = self.heap.items[0];
            
            // Remove from position map
            _ = self.position_map.remove(root.data);

            if (self.heap.items.len == 1) {
                // Only one item, just remove it
                _ = self.heap.pop();
                return root;
            }

            // Move last item to root
            const last = self.heap.pop();
            self.heap.items[0] = last;
            
            // Update position map for moved item
            try self.position_map.put(last.data, 0);
            
            // Restore heap property
            self.sift_down(0);
            
            return root;
        }

        pub fn front(self: *Self) Error!Item {
            if (self.heap.items.len == 0) {
                return Error.EmptyQueue;
            }
            return self.heap.items[0];
        }

        pub fn increase_priority(self: *Self, data: T, new_priority: i32) Error!void {
            const index_ptr = self.position_map.getPtr(data) orelse return Error.ItemNotFound;
            const index = index_ptr.*;
            
            // Update priority (increase priority means lower value)
            self.heap.items[index].priority = new_priority;
            
            // Sift up since priority increased (value decreased)
            self.sift_up(index);
        }

        pub fn decrease_priority(self: *Self, data: T, new_priority: i32) Error!void {
            const index_ptr = self.position_map.getPtr(data) orelse return Error.ItemNotFound;
            const index = index_ptr.*;
            
            // Update priority (decrease priority means higher value)
            self.heap.items[index].priority = new_priority;
            
            // Sift down since priority decreased (value increased)
            self.sift_down(index);
        }

        pub fn contains(self: *Self, data: T) bool {
            return self.position_map.contains(data);
        }

        pub fn len(self: *Self) usize {
            return self.heap.items.len;
        }

        pub fn is_empty(self: *Self) bool {
            return self.heap.items.len == 0;
        }

        fn parent_index(self: *Self, index: usize) ?usize {
            if (index == 0) return null;
            return (index - 1) / self.d;
        }

        fn first_child_index(self: *Self, index: usize) usize {
            return self.d * index + 1;
        }

        fn sift_up(self: *Self, start_index: usize) void {
            var index = start_index;
            
            while (self.parent_index(index)) |parent_idx| {
                const item = self.heap.items[index];
                const parent = self.heap.items[parent_idx];
                
                // If item has higher priority (lower value) than parent, swap
                if (item.priority < parent.priority) {
                    self.swap(index, parent_idx);
                    index = parent_idx;
                } else {
                    break;
                }
            }
        }

        fn sift_down(self: *Self, start_index: usize) void {
            var index = start_index;
            
            while (true) {
                const first_child = self.first_child_index(index);
                
                // If no children, we're done
                if (first_child >= self.heap.items.len) {
                    break;
                }
                
                // Find child with highest priority (lowest value)
                var highest_priority_child = first_child;
                var highest_priority = self.heap.items[first_child].priority;
                
                var child_idx = first_child + 1;
                const last_child = @min(first_child + self.d, self.heap.items.len);
                
                while (child_idx < last_child) {
                    const child_priority = self.heap.items[child_idx].priority;
                    if (child_priority < highest_priority) {
                        highest_priority = child_priority;
                        highest_priority_child = child_idx;
                    }
                    child_idx += 1;
                }
                
                // If highest priority child has higher priority than current item, swap
                if (highest_priority < self.heap.items[index].priority) {
                    self.swap(index, highest_priority_child);
                    index = highest_priority_child;
                } else {
                    break;
                }
            }
        }

        fn swap(self: *Self, i: usize, j: usize) void {
            // Swap items in heap
            const temp = self.heap.items[i];
            self.heap.items[i] = self.heap.items[j];
            self.heap.items[j] = temp;
            
            // Update position map
            self.position_map.put(self.heap.items[i].data, i) catch unreachable;
            self.position_map.put(self.heap.items[j].data, j) catch unreachable;
        }

        // Helper function for testing - verify heap property
        pub fn verify_heap_property(self: *Self) bool {
            for (0..self.heap.items.len) |i| {
                const first_child = self.first_child_index(i);
                const last_child = @min(first_child + self.d, self.heap.items.len);
                
                for (first_child..last_child) |child_idx| {
                    if (self.heap.items[child_idx].priority < self.heap.items[i].priority) {
                        return false;
                    }
                }
            }
            return true;
        }

        // Helper function for testing - verify position map consistency
        pub fn verify_position_map(self: *Self) bool {
            if (self.position_map.count() != self.heap.items.len) {
                return false;
            }
            
            for (self.heap.items, 0..) |item, i| {
                if (self.position_map.get(item.data) != i) {
                    return false;
                }
            }
            return true;
        }
    };
}

// Tests
test "DaryHeap basic operations" {
    var heap = try DaryHeap(i32).init(testing.allocator, 3);
    defer heap.deinit();

    // Test empty heap
    try expect(heap.is_empty());
    try expectEqual(@as(usize, 0), heap.len());
    try expectError(DaryHeap(i32).Error.EmptyQueue, heap.pop());
    try expectError(DaryHeap(i32).Error.EmptyQueue, heap.front());

    // Test insert and basic properties
    try heap.insert(10, 5);
    try expect(!heap.is_empty());
    try expectEqual(@as(usize, 1), heap.len());
    try expect(heap.contains(10));
    try expect(!heap.contains(20));

    const front_item = try heap.front();
    try expectEqual(@as(i32, 10), front_item.data);
    try expectEqual(@as(i32, 5), front_item.priority);

    // Test multiple inserts
    try heap.insert(20, 3);
    try heap.insert(30, 7);
    try heap.insert(40, 1);

    try expectEqual(@as(usize, 4), heap.len());
    try expect(heap.verify_heap_property());
    try expect(heap.verify_position_map());

    // Root should be item with lowest priority value (highest priority)
    const new_front = try heap.front();
    try expectEqual(@as(i32, 40), new_front.data);
    try expectEqual(@as(i32, 1), new_front.priority);
}

test "DaryHeap pop operations" {
    var heap = try DaryHeap(i32).init(testing.allocator, 2);
    defer heap.deinit();

    // Insert items with different priorities
    try heap.insert(1, 10);
    try heap.insert(2, 5);
    try heap.insert(3, 15);
    try heap.insert(4, 3);
    try heap.insert(5, 8);

    // Pop items - should come out in priority order (lowest priority value first)
    var popped = try heap.pop();
    try expectEqual(@as(i32, 4), popped.data);
    try expectEqual(@as(i32, 3), popped.priority);
    try expect(heap.verify_heap_property());
    try expect(heap.verify_position_map());

    popped = try heap.pop();
    try expectEqual(@as(i32, 2), popped.data);
    try expectEqual(@as(i32, 5), popped.priority);

    popped = try heap.pop();
    try expectEqual(@as(i32, 5), popped.data);
    try expectEqual(@as(i32, 8), popped.priority);

    popped = try heap.pop();
    try expectEqual(@as(i32, 1), popped.data);
    try expectEqual(@as(i32, 10), popped.priority);

    popped = try heap.pop();
    try expectEqual(@as(i32, 3), popped.data);
    try expectEqual(@as(i32, 15), popped.priority);

    try expect(heap.is_empty());
}

test "DaryHeap priority updates" {
    var heap = try DaryHeap(i32).init(testing.allocator, 4);
    defer heap.deinit();

    try heap.insert(1, 10);
    try heap.insert(2, 20);
    try heap.insert(3, 30);

    // Increase priority (decrease value) - should move up
    try heap.increase_priority(3, 5);
    var front = try heap.front();
    try expectEqual(@as(i32, 3), front.data);
    try expectEqual(@as(i32, 5), front.priority);
    try expect(heap.verify_heap_property());

    // Decrease priority (increase value) - should move down
    try heap.decrease_priority(3, 25);
    front = try heap.front();
    try expectEqual(@as(i32, 1), front.data);
    try expectEqual(@as(i32, 10), front.priority);
    try expect(heap.verify_heap_property());

    // Test error cases
    try expectError(DaryHeap(i32).Error.ItemNotFound, heap.increase_priority(99, 1));
    try expectError(DaryHeap(i32).Error.ItemNotFound, heap.decrease_priority(99, 100));
}

test "DaryHeap duplicate insertion" {
    var heap = try DaryHeap(i32).init(testing.allocator, 3);
    defer heap.deinit();

    try heap.insert(1, 10);
    try expectError(DaryHeap(i32).Error.ItemAlreadyExists, heap.insert(1, 5));
}

test "DaryHeap invalid arity" {
    try expectError(DaryHeap(i32).Error.InvalidArity, DaryHeap(i32).init(testing.allocator, 1));
    try expectError(DaryHeap(i32).Error.InvalidArity, DaryHeap(i32).init(testing.allocator, 0));
}

test "DaryHeap with different arities" {
    // Test binary heap (d=2)
    var binary_heap = try DaryHeap(i32).init(testing.allocator, 2);
    defer binary_heap.deinit();

    // Test 5-ary heap
    var five_ary_heap = try DaryHeap(i32).init(testing.allocator, 5);
    defer five_ary_heap.deinit();

    // Insert same data in both heaps
    const data = [_]struct { data: i32, priority: i32 }{
        .{ .data = 1, .priority = 10 },
        .{ .data = 2, .priority = 5 },
        .{ .data = 3, .priority = 15 },
        .{ .data = 4, .priority = 3 },
        .{ .data = 5, .priority = 8 },
        .{ .data = 6, .priority = 12 },
        .{ .data = 7, .priority = 1 },
    };

    for (data) |item| {
        try binary_heap.insert(item.data, item.priority);
        try five_ary_heap.insert(item.data, item.priority);
    }

    try expect(binary_heap.verify_heap_property());
    try expect(five_ary_heap.verify_heap_property());

    // Both should pop items in same priority order
    while (!binary_heap.is_empty() and !five_ary_heap.is_empty()) {
        const binary_item = try binary_heap.pop();
        const five_ary_item = try five_ary_heap.pop();
        try expectEqual(binary_item.priority, five_ary_item.priority);
    }
}

test "DaryHeap stress test" {
    var heap = try DaryHeap(i32).init(testing.allocator, 3);
    defer heap.deinit();

    // Insert many items
    var i: i32 = 0;
    while (i < 100) : (i += 1) {
        try heap.insert(i, @rem(i * 17, 97)); // Some pseudo-random priorities
    }

    try expect(heap.verify_heap_property());
    try expect(heap.verify_position_map());
    try expectEqual(@as(usize, 100), heap.len());

    // Update some priorities
    try heap.increase_priority(50, -10);
    try heap.decrease_priority(25, 200);
    
    try expect(heap.verify_heap_property());
    try expect(heap.verify_position_map());

    // Pop all items - should maintain heap property throughout
    var last_priority: i32 = std.math.minInt(i32);
    while (!heap.is_empty()) {
        const item = try heap.pop();
        try expect(item.priority >= last_priority);
        last_priority = item.priority;
        try expect(heap.verify_heap_property());
        try expect(heap.verify_position_map());
    }
}