# d-Heap Priority Queue v2.0.0 - Zig 0.15.2 Support

🚀 **Major Release**: Complete Zig 0.15.2 compatibility with generic implementation and comprehensive testing.

## 🎯 What's New

### ✨ Zig 0.15.2 Support
- **Full compatibility** with latest Zig version
- Updated ArrayList, HashMap, and build system APIs
- New format function signature support

### 🔧 Generic Implementation
- **Truly generic**: Use your own item types in Zig
- Type-safe with `HashContext(T)` and `Comparator(T)`
- Backward compatible with built-in `Item` type via `DHeapItem`

### 🧪 Comprehensive Testing
- **20+ tests** covering all functionality
- Edge cases, error conditions, and heap property maintenance
- Passes in Debug, ReleaseSafe, and ReleaseFast modes

### 📦 Module Export
- Can be used as a dependency in other Zig projects
- Proper `build.zig.zon` integration
- Clean module interface with `d-heap` import

### ➕ New Features
- `peek()` alias for `front()` method
- `initCapacity()` for pre-allocation
- Better error handling (no `unreachable` in user paths)

## 🔄 API Consistency

All three implementations (C++, Rust, Zig) now have **identical APIs**:

| Operation | C++ | Rust | Zig |
|-----------|-----|------|-----|
| Create | `PriorityQueue(d)` | `PriorityQueue::new(d, cmp)` | `DHeapItem.init(d, cmp, alloc)` |
| Insert | `insert(item)` | `insert(item)` | `insert(item)` |
| Front | `front()` | `front()` | `front()` / `peek()` |
| Pop | `pop()` | `pop()` | `pop()` |
| Contains | `contains(item)` | `contains(&item)` | `contains(item)` |
| Size | `len()` | `len()` | `len()` |
| Empty | `is_empty()` | `is_empty()` | `isEmpty()` |

## 📚 Quick Start

### Built-in Item Type
```zig
const d_heap = @import("d-heap");
const DHeapItem = d_heap.DHeapItem;
const MinByCost = d_heap.MinByCost;
const Item = d_heap.Item;

var heap = try DHeapItem.init(3, MinByCost, allocator);
defer heap.deinit();

try heap.insert(Item.init(1, 10));
try heap.insert(Item.init(2, 5));

while (heap.pop()) |item| {
    std.debug.print("Popped: {f}\n", .{item});
}
```

### Custom Item Types
```zig
const Task = struct {
    id: u64,
    priority: i32,
    
    pub fn hash(self: Task) u64 { /* ... */ }
    pub fn eql(a: Task, b: Task) bool { /* ... */ }
};

const TaskHeap = d_heap.DHeap(Task, d_heap.HashContext(Task), d_heap.Comparator(Task));
```

## 🛠 Installation

Add to your `build.zig.zon`:
```zig
.dependencies = .{
    .d_heap = .{
        .url = "https://github.com/your-username/priority-queues/archive/refs/tags/v2.0.0.tar.gz",
        .hash = "...", // Zig will calculate this
    },
},
```

See [INSTALL.md](INSTALL.md) for complete installation instructions.

## 🔄 Migration from v1.x

- **Zig users**: Update to Zig 0.15.2, use `DHeapItem` for built-in types
- **Custom types**: Use new generic API with `DHeap(T, Context, Comparator)`
- **API**: All method names remain the same (camelCase in Zig)

## 🏆 Competitive Advantage

This is the **first professional-grade, generic, well-tested** d-ary heap implementation in Zig with:
- ✅ O(1) item lookup via HashMap
- ✅ Configurable arity (d-ary heap)
- ✅ Min/max heap support
- ✅ Cross-language API parity
- ✅ Comprehensive documentation
- ✅ 20+ unit tests

## 📊 Performance

- **O(1)**: `front()`, `peek()`, `contains()`, `len()`, `isEmpty()`, `d()`
- **O(log_d n)**: `insert()`, `increasePriority()`
- **O(d·log_d n)**: `pop()`, `decreasePriority()`

## 🔗 Links

- **Documentation**: See language-specific README files
- **Examples**: Check `src/main.zig` for usage patterns
- **Tests**: Run `zig build test` to see all functionality
- **Issues**: Report bugs or feature requests on GitHub

---

**Full Changelog**: [v1.1.0...v2.0.0](https://github.com/your-username/priority-queues/compare/v1.1.0...v2.0.0)