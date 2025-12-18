# Installation Guide

## Zig Installation

### Prerequisites
- **Zig 0.15.2** or later
- Git (for dependency management)

### Using as a Dependency

#### 1. Add to your `build.zig.zon`

```zig
.{
    .name = "your-project",
    .version = "0.1.0",
    .dependencies = .{
        .d_heap = .{
            .url = "https://github.com/your-username/priority-queues/archive/refs/tags/v2.0.0.tar.gz",
            .hash = "1220abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
        },
    },
}
```

**Note**: Replace the hash with the actual hash. Zig will calculate and tell you the correct hash when you first run `zig build`.

#### 2. Update your `build.zig`

```zig
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Get the d-heap dependency
    const d_heap = b.dependency("d_heap", .{
        .target = target,
        .optimize = optimize,
    });

    const exe = b.addExecutable(.{
        .name = "your-app",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/main.zig"),
            .target = target,
            .optimize = optimize,
            .imports = &.{
                .{ .name = "d-heap", .module = d_heap.module("d-heap") },
            },
        }),
    });

    b.installArtifact(exe);
}
```

#### 3. Use in your code

```zig
const std = @import("std");
const d_heap = @import("d-heap");

// Use pre-configured type for built-in Item
const DHeapItem = d_heap.DHeapItem;
const MinByCost = d_heap.MinByCost;
const Item = d_heap.Item;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var heap = try DHeapItem.init(3, MinByCost, allocator);
    defer heap.deinit();

    try heap.insert(Item.init(1, 10));
    try heap.insert(Item.init(2, 5));

    while (heap.pop()) |item| {
        std.debug.print("Popped: {f}\n", .{item});
    }
}
```

### Custom Item Types

```zig
const std = @import("std");
const d_heap = @import("d-heap");

// Define your item type
const Task = struct {
    id: u64,
    priority: i32,
    name: []const u8,

    pub fn hash(self: Task) u64 {
        var hasher = std.hash.Wyhash.init(0);
        std.hash.autoHash(&hasher, self.id);
        return hasher.final();
    }

    pub fn eql(a: Task, b: Task) bool {
        return a.id == b.id;
    }
};

// Create comparator
fn taskLessThan(a: Task, b: Task) bool {
    return a.priority < b.priority;
}

// Create heap type
const TaskHeap = d_heap.DHeap(
    Task,
    d_heap.HashContext(Task),
    d_heap.Comparator(Task),
);

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const comparator = d_heap.Comparator(Task){ .cmp = taskLessThan };
    var heap = try TaskHeap.init(4, comparator, allocator);
    defer heap.deinit();

    try heap.insert(.{ .id = 1, .priority = 10, .name = "Build" });
    try heap.insert(.{ .id = 2, .priority = 5, .name = "Test" });

    if (heap.front()) |task| {
        std.debug.print("Next: {s}\n", .{task.name});
    }
}
```

## C++ Installation

### Prerequisites
- **C++17** compatible compiler (GCC 7+, Clang 5+, MSVC 2017+)
- CMake 3.10+ (optional, for building examples)

### Usage
Simply include the header file in your project:

```cpp
#include "Cpp/PriorityQueue.h"

int main() {
    PriorityQueue<int> pq(3);  // 3-ary heap
    pq.insert(10);
    pq.insert(5);
    std::cout << pq.front() << std::endl;  // 5 (min-heap by default)
    return 0;
}
```

## Rust Installation

### Prerequisites
- **Rust Edition 2021** (Rust 1.56+)

### Add to your `Cargo.toml`

```toml
[dependencies]
# If published to crates.io (not recommended based on analysis)
# rust_priority_queue = "2.0.0"

# Or use git dependency
rust_priority_queue = { git = "https://github.com/your-username/priority-queues", tag = "v2.0.0" }
```

### Usage

```rust
use priority_queue::{PriorityQueue, MinBy};

fn main() {
    let mut pq = PriorityQueue::new(3, MinBy(|x: &i32| *x));
    pq.insert(10);
    pq.insert(5);
    println!("{}", pq.front());  // 5
}
```

## Building from Source

### Clone the repository

```bash
git clone https://github.com/your-username/priority-queues.git
cd priority-queues
```

### Zig

```bash
cd zig
zig build        # Build demo
zig build run    # Run demo
zig build test   # Run tests
```

### C++

```bash
cd Cpp
g++ -std=c++17 -O2 main.cpp -o demo
./demo
```

### Rust

```bash
cd Rust
cargo build --release
cargo run --bin demo
cargo test
```

## Troubleshooting

### Zig Hash Mismatch
If you get a hash mismatch error, Zig will tell you the correct hash:
```
error: hash mismatch: expected 1220abc..., found 1220def...
```
Copy the "found" hash to your `build.zig.zon`.

### Zig Version Compatibility
This library requires Zig 0.15.2+. For older Zig versions, use v1.1.0.

### Build Issues
- Ensure you have the correct compiler versions
- Check that all dependencies are properly configured
- Refer to the language-specific README files for detailed instructions

## Examples

See the `examples/` directory in each language folder for more usage patterns and advanced examples.