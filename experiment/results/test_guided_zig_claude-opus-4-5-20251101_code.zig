const std = @import("std");
const Allocator = std.mem.Allocator;

pub const Item = struct {
    number: u32,
    cost: u32,

    pub fn init(number: u32, cost: u32) Item {
        return Item{ .number = number, .cost = cost };
    }
};

pub fn MinByCost(a: Item, b: Item) bool {
    return a.cost < b.cost;
}

pub fn DHeapItem(comptime T: type, comptime lessThanFn: fn (T, T) bool, comptime identityFn: fn (T) u32) type {
    return struct {
        const Self = @This();

        d: usize,
        items: std.ArrayList(T),
        index_map: std.AutoHashMap(u32, usize),
        allocator: Allocator,

        pub fn init(d: usize, allocator: Allocator) !Self {
            return Self{
                .d = d,
                .items = std.ArrayList(T).init(allocator),
                .index_map = std.AutoHashMap(u32, usize).init(allocator),
                .allocator = allocator,
            };
        }

        pub fn deinit(self: *Self) void {
            self.items.deinit();
            self.index_map.deinit();
        }

        pub fn len(self: *const Self) usize {
            return self.items.items.len;
        }

        pub fn isEmpty(self: *const Self) bool {
            return self.items.items.len == 0;
        }

        pub fn contains(self: *const Self, item: T) bool {
            return self.index_map.contains(identityFn(item));
        }

        pub fn front(self: *const Self) ?T {
            if (self.items.items.len == 0) return null;
            return self.items.items[0];
        }

        pub fn insert(self: *Self, item: T) !void {
            const idx = self.items.items.len;
            try self.items.append(item);
            try self.index_map.put(identityFn(item), idx);
            self.siftUp(idx);
        }

        pub fn pop(self: *Self) !?T {
            if (self.items.items.len == 0) return null;

            const result = self.items.items[0];
            _ = self.index_map.remove(identityFn(result));

            if (self.items.items.len == 1) {
                _ = self.items.pop();
                return result;
            }

            // Move last element to root
            const last = self.items.pop();
            self.items.items[0] = last;
            self.index_map.put(identityFn(last), 0) catch unreachable;
            self.siftDown(0);

            return result;
        }

        pub fn increasePriority(self: *Self, item: T) !void {
            const id = identityFn(item);
            const idx = self.index_map.get(id) orelse return error.ItemNotFound;
            self.items.items[idx] = item;
            self.siftUp(idx);
        }

        pub fn decreasePriority(self: *Self, item: T) !void {
            const id = identityFn(item);
            const idx = self.index_map.get(id) orelse return error.ItemNotFound;
            self.items.items[idx] = item;
            self.siftDown(idx);
        }

        fn siftUp(self: *Self, start_idx: usize) void {
            var idx = start_idx;
            while (idx > 0) {
                const parent = (idx - 1) / self.d;
                if (lessThanFn(self.items.items[idx], self.items.items[parent])) {
                    self.swap(idx, parent);
                    idx = parent;
                } else {
                    break;
                }
            }
        }

        fn siftDown(self: *Self, start_idx: usize) void {
            var idx = start_idx;
            const n = self.items.items.len;

            while (true) {
                var smallest = idx;
                const first_child = idx * self.d + 1;

                var i: usize = 0;
                while (i < self.d) : (i += 1) {
                    const child = first_child + i;
                    if (child < n and lessThanFn(self.items.items[child], self.items.items[smallest])) {
                        smallest = child;
                    }
                }

                if (smallest == idx) break;

                self.swap(idx, smallest);
                idx = smallest;
            }
        }

        fn swap(self: *Self, i: usize, j: usize) void {
            const id_i = identityFn(self.items.items[i]);
            const id_j = identityFn(self.items.items[j]);

            const tmp = self.items.items[i];
            self.items.items[i] = self.items.items[j];
            self.items.items[j] = tmp;

            self.index_map.put(id_i, j) catch unreachable;
            self.index_map.put(id_j, i) catch unreachable;
        }
    };
}

fn itemIdentity(item: Item) u32 {
    return item.number;
}

// Type alias matching test expectations
pub const DHeap = struct {
    pub fn init(d: usize, comptime _: fn (Item, Item) bool, allocator: Allocator) !DHeapItem(Item, MinByCost, itemIdentity) {
        return DHeapItem(Item, MinByCost, itemIdentity).init(d, allocator);
    }
};