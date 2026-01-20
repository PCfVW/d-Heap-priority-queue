//! D-ary heap priority queue implementation in Zig
//! 
//! This module provides a configurable d-ary heap where:
//! - Items have identity (for equality) and priority (for ordering)
//! - Two items are equal if they have the same identity, regardless of priority
//! - O(1) lookup to check if an item exists
//! - Min-heap where lower priority values have higher importance

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

pub const Item = struct {
    number: u32, // identity
    cost: u32,   // priority

    pub fn init(number: u32, cost: u32) Item {
        return Item{ .number = number, .cost = cost };
    }

    pub fn eql(self: Item, other: Item) bool {
        return self.number == other.number;
    }
};

pub const MinByCost = struct {
    pub fn lessThan(a: Item, b: Item) bool {
        return a.cost < b.cost;
    }
};

pub const DHeapItem = struct {
    const Self = @This();
    
    heap: ArrayList(Item),
    index_map: HashMap(u32, usize, std.hash_map.DefaultContext(u32), std.hash_map.default_max_load_percentage),
    d: usize, // heap arity
    compareFn: type,
    allocator: Allocator,

    pub fn init(d: usize, comptime compareFn: type, allocator: Allocator) !Self {
        return Self{
            .heap = ArrayList(Item).init(allocator),
            .index_map = HashMap(u32, usize, std.hash_map.DefaultContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
            .d = d,
            .compareFn = compareFn,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Self) void {
        self.heap.deinit();
        self.index_map.deinit();
    }

    pub fn insert(self: *Self, item: Item) !void {
        // If item already exists, this is effectively an update
        if (self.index_map.get(item.number)) |index| {
            const old_item = self.heap.items[index];
            self.heap.items[index] = item;
            
            // Determine if we need to bubble up or down
            if (self.compareFn.lessThan(item, old_item)) {
                self.bubbleUp(index);
            } else {
                self.bubbleDown(index);
            }
            return;
        }

        // Add new item
        try self.heap.append(item);
        const index = self.heap.items.len - 1;
        try self.index_map.put(item.number, index);
        self.bubbleUp(index);
    }

    pub fn pop(self: *Self) !?Item {
        if (self.heap.items.len == 0) {
            return null;
        }

        const min_item = self.heap.items[0];
        _ = self.index_map.remove(min_item.number);

        if (self.heap.items.len == 1) {
            _ = self.heap.pop();
            return min_item;
        }

        // Move last item to root
        const last_item = self.heap.pop();
        self.heap.items[0] = last_item;
        try self.index_map.put(last_item.number, 0);
        
        self.bubbleDown(0);
        return min_item;
    }

    pub fn front(self: *Self) ?Item {
        if (self.heap.items.len == 0) {
            return null;
        }
        return self.heap.items[0];
    }

    pub fn increasePriority(self: *Self, item: Item) !void {
        const index = self.index_map.get(item.number) orelse return error.ItemNotFound;
        
        self.heap.items[index] = item;
        self.bubbleUp(index);
    }

    pub fn decreasePriority(self: *Self, item: Item) !void {
        const index = self.index_map.get(item.number) orelse return error.ItemNotFound;
        
        self.heap.items[index] = item;
        self.bubbleDown(index);
    }

    pub fn contains(self: *Self, item: Item) bool {
        return self.index_map.contains(item.number);
    }

    pub fn len(self: *Self) usize {
        return self.heap.items.len;
    }

    pub fn isEmpty(self: *Self) bool {
        return self.heap.items.len == 0;
    }

    fn parent(self: *Self, index: usize) usize {
        if (index == 0) return 0;
        return (index - 1) / self.d;
    }

    fn firstChild(self: *Self, index: usize) usize {
        return self.d * index + 1;
    }

    fn bubbleUp(self: *Self, start_index: usize) void {
        var index = start_index;
        
        while (index > 0) {
            const parent_index = self.parent(index);
            
            if (!self.compareFn.lessThan(self.heap.items[index], self.heap.items[parent_index])) {
                break;
            }
            
            self.swap(index, parent_index);
            index = parent_index;
        }
    }

    fn bubbleDown(self: *Self, start_index: usize) void {
        var index = start_index;
        
        while (true) {
            var min_index = index;
            const first_child_index = self.firstChild(index);
            
            // Check all children
            var i: usize = 0;
            while (i < self.d) : (i += 1) {
                const child_index = first_child_index + i;
                if (child_index >= self.heap.items.len) break;
                
                if (self.compareFn.lessThan(self.heap.items[child_index], self.heap.items[min_index])) {
                    min_index = child_index;
                }
            }
            
            if (min_index == index) break;
            
            self.swap(index, min_index);
            index = min_index;
        }
    }

    fn swap(self: *Self, i: usize, j: usize) void {
        const temp = self.heap.items[i];
        self.heap.items[i] = self.heap.items[j];
        self.heap.items[j] = temp;
        
        // Update index map
        self.index_map.put(self.heap.items[i].number, i) catch unreachable;
        self.index_map.put(self.heap.items[j].number, j) catch unreachable;
    }
};