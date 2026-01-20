const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const testing = std.testing;

pub fn DaryHeapPriorityQueue(comptime T: type, comptime P: type) type {
    return struct {
        const Self = @This();
        
        pub const Item = struct {
            identity: T,
            priority: P,
            
            pub fn init(identity: T, priority: P) Item {
                return Item{
                    .identity = identity,
                    .priority = priority,
                };
            }
        };
        
        const IndexMap = HashMap(T, usize, HashContext, std.hash_map.default_max_load_percentage);
        
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
        
        allocator: Allocator,
        heap: ArrayList(Item),
        index_map: IndexMap,
        d: usize, // heap arity
        
        pub fn init(allocator: Allocator, d: usize) Self {
            std.debug.assert(d >= 2);
            return Self{
                .allocator = allocator,
                .heap = ArrayList(Item).init(allocator),
                .index_map = IndexMap.init(allocator),
                .d = d,
            };
        }
        
        pub fn deinit(self: *Self) void {
            self.heap.deinit();
            self.index_map.deinit();
        }
        
        pub fn len(self: *const Self) usize {
            return self.heap.items.len;
        }
        
        pub fn isEmpty(self: *const Self) bool {
            return self.heap.items.len == 0;
        }
        
        pub fn contains(self: *const Self, identity: T) bool {
            return self.index_map.contains(identity);
        }
        
        pub fn front(self: *const Self) ?Item {
            if (self.heap.items.len == 0) return null;
            return self.heap.items[0];
        }
        
        pub fn insert(self: *Self, item: Item) !void {
            // If item already exists, update its priority
            if (self.index_map.get(item.identity)) |index| {
                const old_priority = self.heap.items[index].priority;
                self.heap.items[index].priority = item.priority;
                
                if (item.priority < old_priority) {
                    try self.bubbleUp(index);
                } else if (item.priority > old_priority) {
                    try self.bubbleDown(index);
                }
                return;
            }
            
            // Add new item
            try self.heap.append(item);
            const new_index = self.heap.items.len - 1;
            try self.index_map.put(item.identity, new_index);
            try self.bubbleUp(new_index);
        }
        
        pub fn pop(self: *Self) ?Item {
            if (self.heap.items.len == 0) return null;
            
            const result = self.heap.items[0];
            _ = self.index_map.remove(result.identity);
            
            if (self.heap.items.len == 1) {
                _ = self.heap.pop();
                return result;
            }
            
            // Move last item to root and bubble down
            const last_item = self.heap.pop();
            self.heap.items[0] = last_item;
            self.index_map.put(last_item.identity, 0) catch unreachable;
            self.bubbleDown(0) catch unreachable;
            
            return result;
        }
        
        pub fn increasePriority(self: *Self, identity: T, new_priority: P) !bool {
            if (self.index_map.get(identity)) |index| {
                const current_priority = self.heap.items[index].priority;
                if (new_priority >= current_priority) return false; // Not an increase
                
                self.heap.items[index].priority = new_priority;
                try self.bubbleUp(index);
                return true;
            }
            return false;
        }
        
        pub fn decreasePriority(self: *Self, identity: T, new_priority: P) !bool {
            if (self.index_map.get(identity)) |index| {
                const current_priority = self.heap.items[index].priority;
                if (new_priority <= current_priority) return false; // Not a decrease
                
                self.heap.items[index].priority = new_priority;
                try self.bubbleDown(index);
                return true;
            }
            return false;
        }
        
        fn parentIndex(self: *const Self, index: usize) ?usize {
            if (index == 0) return null;
            return (index - 1) / self.d;
        }
        
        fn firstChildIndex(self: *const Self, index: usize) usize {
            return self.d * index + 1;
        }
        
        fn bubbleUp(self: *Self, start_index: usize) !void {
            var index = start_index;
            
            while (self.parentIndex(index)) |parent_idx| {
                if (self.heap.items[index].priority >= self.heap.items[parent_idx].priority) {
                    break;
                }
                
                try self.swapItems(index, parent_idx);
                index = parent_idx;
            }
        }
        
        fn bubbleDown(self: *Self, start_index: usize) !void {
            var index = start_index;
            
            while (true) {
                var min_index = index;
                const first_child = self.firstChildIndex(index);
                
                // Find the child with minimum priority
                var i: usize = 0;
                while (i < self.d) : (i += 1) {
                    const child_index = first_child + i;
                    if (child_index >= self.heap.items.len) break;
                    
                    if (self.heap.items[child_index].priority < self.heap.items[min_index].priority) {
                        min_index = child_index;
                    }
                }
                
                if (min_index == index) break;
                
                try self.swapItems(index, min_index);
                index = min_index;
            }
        }
        
        fn swapItems(self: *Self, i: usize, j: usize) !void {
            const temp = self.heap.items[i];
            self.heap.items[i] = self.heap.items[j];
            self.heap.items[j] = temp;
            
            // Update index map
            try self.index_map.put(self.heap.items[i].identity, i);
            try self.index_map.put(self.heap.items[j].identity, j);
        }
    };
}

// Test the implementation
test "DaryHeapPriorityQueue basic operations" {
    const allocator = testing.allocator;
    var pq = DaryHeapPriorityQueue(u32, f32).init(allocator, 3); // 3-ary heap
    defer pq.deinit();
    
    // Test empty queue
    try testing.expect(pq.isEmpty());
    try testing.expectEqual(@as(usize, 0), pq.len());
    try testing.expectEqual(@as(?DaryHeapPriorityQueue(u32, f32).Item, null), pq.front());
    try testing.expectEqual(@as(?DaryHeapPriorityQueue(u32, f32).Item, null), pq.pop());
    
    // Test insert and front
    try pq.insert(DaryHeapPriorityQueue(u32, f32).Item.init(1, 5.0));
    try testing.expect(!pq.isEmpty());
    try testing.expectEqual(@as(usize, 1), pq.len());
    try testing.expect(pq.contains(1));
    
    const front_item = pq.front().?;
    try testing.expectEqual(@as(u32, 1), front_item.identity);
    try testing.expectEqual(@as(f32, 5.0), front_item.priority);
    
    // Test multiple inserts (min-heap property)
    try pq.insert(DaryHeapPriorityQueue(u32, f32).Item.init(2, 3.0));
    try pq.insert(DaryHeapPriorityQueue(u32, f32).Item.init(3, 7.0));
    try pq.insert(DaryHeapPriorityQueue(u32, f32).Item.init(4, 1.0));
    
    try testing.expectEqual(@as(usize, 4), pq.len());
    
    // Item with priority 1.0 should be at front
    const min_item = pq.front().?;
    try testing.expectEqual(@as(u32, 4), min_item.identity);
    try testing.expectEqual(@as(f32, 1.0), min_item.priority);
    
    // Test pop (should return items in priority order)
    const popped1 = pq.pop().?;
    try testing.expectEqual(@as(u32, 4), popped1.identity);
    try testing.expectEqual(@as(f32, 1.0), popped1.priority);
    try testing.expect(!pq.contains(4));
    
    const popped2 = pq.pop().?;
    try testing.expectEqual(@as(u32, 2), popped2.identity);
    try testing.expectEqual(@as(f32, 3.0), popped2.priority);
    
    const popped3 = pq.pop().?;
    try testing.expectEqual(@as(u32, 1), popped3.identity);
    try testing.expectEqual(@as(f32, 5.0), popped3.priority);
    
    const popped4 = pq.pop().?;
    try testing.expectEqual(@as(u32, 3), popped4.identity);
    try testing.expectEqual(@as(f32, 7.0), popped4.priority);
    
    try testing.expect(pq.isEmpty());
}

test "DaryHeapPriorityQueue priority updates" {
    const allocator = testing.allocator;
    var pq = DaryHeapPriorityQueue(u32, f32).init(allocator, 2); // binary heap
    defer pq.deinit();
    
    try pq.insert(DaryHeapPriorityQueue(u32, f32).Item.init(1, 10.0));
    try pq.insert(DaryHeapPriorityQueue(u32, f32).Item.init(2, 20.0));
    try pq.insert(DaryHeapPriorityQueue(u32, f32).Item.init(3, 30.0));
    
    // Test increase priority (lower value)
    try testing.expect(try pq.increasePriority(3, 5.0));
    const front = pq.front().?;
    try testing.expectEqual(@as(u32, 3), front.identity);
    try testing.expectEqual(@as(f32, 5.0), front.priority);
    
    // Test decrease priority (higher value)
    try testing.expect(try pq.decreasePriority(3, 25.0));
    const new_front = pq.front().?;
    try testing.expectEqual(@as(u32, 1), new_front.identity);
    try testing.expectEqual(@as(f32, 10.0), new_front.priority);
    
    // Test invalid updates
    try testing.expect(!try pq.increasePriority(1, 15.0)); // Not an increase
    try testing.expect(!try pq.decreasePriority(1, 5.0));  // Not a decrease
    try testing.expect(!try pq.increasePriority(999, 1.0)); // Non-existent item
}

test "DaryHeapPriorityQueue duplicate identity handling" {
    const allocator = testing.allocator;
    var pq = DaryHeapPriorityQueue(u32, f32).init(allocator, 4); // 4-ary heap
    defer pq.deinit();
    
    try pq.insert(DaryHeapPriorityQueue(u32, f32).Item.init(1, 10.0));
    try testing.expectEqual(@as(usize, 1), pq.len());
    
    // Insert same identity with different priority - should update existing
    try pq.insert(DaryHeapPriorityQueue(u32, f32).Item.init(1, 5.0));
    try testing.expectEqual(@as(usize, 1), pq.len()); // Length shouldn't change
    
    const item = pq.front().?;
    try testing.expectEqual(@as(u32, 1), item.identity);
    try testing.expectEqual(@as(f32, 5.0), item.priority); // Priority should be updated
}

test "DaryHeapPriorityQueue with different d values" {
    const allocator = testing.allocator;
    
    // Test with different arity values
    for ([_]usize{ 2, 3, 4, 5, 8 }) |d| {
        var pq = DaryHeapPriorityQueue(u32, i32).init(allocator, d);
        defer pq.deinit();
        
        // Insert items in reverse priority order
        var i: i32 = 100;
        while (i >= 1) : (i -= 1) {
            try pq.insert(DaryHeapPriorityQueue(u32, i32).Item.init(@intCast(i), i));
        }
        
        try testing.expectEqual(@as(usize, 100), pq.len());
        
        // Pop all items - should come out in priority order
        i = 1;
        while (!pq.isEmpty()) {
            const item = pq.pop().?;
            try testing.expectEqual(@as(u32, @intCast(i)), item.identity);
            try testing.expectEqual(i, item.priority);
            i += 1;
        }
    }
}