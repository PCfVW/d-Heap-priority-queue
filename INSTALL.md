# Installation Guide

## Go Installation

### Prerequisites
- **Go 1.21+** (for generics support)

### Install

```bash
go get github.com/PCfVW/d-Heap-priority-queue/Go/src@v2.4.0
```

### Usage

```go
package main

import (
    "fmt"
    dheap "github.com/PCfVW/d-Heap-priority-queue/Go/src"
)

type Task struct {
    ID       string
    Priority int
}

func main() {
    pq := dheap.New(dheap.Options[Task, string]{
        D:            4,
        Comparator:   dheap.MinBy(func(t Task) int { return t.Priority }),
        KeyExtractor: func(t Task) string { return t.ID },
    })

    pq.Insert(Task{ID: "task1", Priority: 10})
    pq.Insert(Task{ID: "task2", Priority: 5})

    top, _ := pq.Front()
    fmt.Printf("Top: %s\n", top.ID)  // task2
}
```

### Cross-Language Compatibility

Go provides both PascalCase (primary) and snake_case (compatibility) methods:

```go
// Primary Go style
pq.IsEmpty()
pq.IncreasePriority(item)
pq.String()

// Cross-language compatibility aliases
pq.Is_empty()
pq.Increase_priority(item)
pq.To_string()
```

---

## TypeScript Installation

### Prerequisites
- **Node.js 18+**
- npm or yarn

### Install from npm

```bash
npm install d-ary-heap
# or
yarn add d-ary-heap
```

### Usage

```typescript
import { PriorityQueue, minBy } from 'd-ary-heap';

interface Task {
  id: number;
  priority: number;
  name: string;
}

const pq = new PriorityQueue<Task, number>({
  d: 4,
  comparator: minBy(task => task.priority),
  keyExtractor: task => task.id
});

pq.insert({ id: 1, priority: 10, name: 'Low priority' });
pq.insert({ id: 2, priority: 5, name: 'High priority' });
console.log(pq.front()); // { id: 2, priority: 5, name: 'High priority' }
```

### Cross-Language Compatibility

TypeScript provides both camelCase (primary) and snake_case (compatibility) methods:

```typescript
// Primary TypeScript style
pq.isEmpty()
pq.increasePriority(item)
pq.toString()

// Cross-language compatibility aliases
pq.is_empty()
pq.increase_priority(item)  
pq.to_string()
```

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
            .url = "https://github.com/PCfVW/d-Heap-priority-queue/archive/refs/tags/v2.4.0.tar.gz",
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

### Cross-Language Compatibility (v2.3.0+)

Zig provides both camelCase (primary) and snake_case (compatibility) methods:

```zig
// Primary Zig style
heap.isEmpty()
heap.increasePriority(item)
heap.toString()

// Cross-language compatibility aliases
heap.to_string()  // Available in v2.2.0+
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
# rust_priority_queue = "2.4.0"

# Or use git dependency
rust_priority_queue = { git = "https://github.com/PCfVW/priority-queues", tag = "v2.4.0" }
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

## Cross-Language Method Compatibility

### Method Naming Conventions

Different languages follow their respective naming conventions:

| Function | C++ | Go | Rust | Zig | TypeScript |
|----------|-----|-----|------|-----|------------|
| **Check Empty** | `is_empty()` | `IsEmpty()` | `is_empty()` | `isEmpty()` | `isEmpty()` |
| **Increase Priority** | `increase_priority()` | `IncreasePriority()` | `increase_priority()` | `increasePriority()` | `increasePriority()` |
| **String Output** | `to_string()` | `String()` | `to_string()` | `toString()` / `to_string()` | `toString()` / `to_string()` |

### Compatibility Features (v2.3.0+)
- **Go**: Provides snake_case aliases (`Is_empty()`, `To_string()`) for cross-language consistency
- **Zig**: Added `to_string()` alias for cross-language consistency
- **TypeScript**: Provides complete snake_case aliases for all camelCase methods
- **Error Handling**: Each language uses idiomatic error handling (assertions, panics, error unions, exceptions, ok booleans)

## Building from Source

### Clone the repository

```bash
git clone https://github.com/PCfVW/priority-queues.git
cd priority-queues
```

### Go

```bash
cd Go
go build ./...   # Build all packages
go test ./...    # Run tests
go run examples/dijkstra/Go/main.go  # Run Dijkstra example
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

### TypeScript

```bash
cd TypeScript
npm install       # Install dependencies
npm run build     # Build the package
npm test          # Run tests
npm run lint      # Run linting
```

## Troubleshooting

## Troubleshooting

### TypeScript Issues

#### Package Not Found
```bash
npm ERR! 404 Not Found - GET https://registry.npmjs.org/d-ary-heap
```
Ensure you're using the correct package name: `d-ary-heap` (not `d_ary_heap`)

#### Type Errors
Make sure you're using TypeScript 5.0+ and have proper type definitions:
```bash
npm install --save-dev typescript@latest
```

### Zig Hash Mismatch
If you get a hash mismatch error, Zig will tell you the correct hash:
```
error: hash mismatch: expected 1220abc..., found 1220def...
```
Copy the "found" hash to your `build.zig.zon`.

### Zig Version Compatibility
- **v2.4.0+**: Requires Zig 0.15.2+
- **v2.0.0-v2.3.0**: Requires Zig 0.15.2+
- **v1.x**: For older Zig versions, use v1.1.0

### Version Compatibility Matrix

| Package Version | Go | Zig | C++ | Rust | TypeScript | Node.js |
|----------------|-----|-----|-----|------|------------|---------|
| **v2.4.0** | 1.21+ | 0.15.2+ | C++17+ | 2021+ | 5.0+ | 18+ |
| **v2.3.0** | 1.21+ | 0.15.2+ | C++17+ | 2021+ | 5.0+ | 18+ |
| **v2.2.0** | N/A | 0.15.2+ | C++17+ | 2021+ | 5.0+ | 18+ |
| **v2.1.1** | N/A | 0.15.2+ | C++17+ | 2021+ | 5.0+ | 18+ |
| **v2.0.0** | N/A | 0.15.2+ | C++17+ | 2021+ | N/A | N/A |

### Build Issues
- Ensure you have the correct compiler versions (see compatibility matrix above)
- Check that all dependencies are properly configured
- For TypeScript: Run `npm install` before building
- Refer to the language-specific README files for detailed instructions

## Error Handling Differences

Each language handles errors idiomatically:

- **C++**: Assertions (`assert()`) - check conditions before calling methods
- **Go**: Ok booleans - use `Peek()` for safe access, errors returned as second value
- **Rust**: Panics with messages - use `peek()` for safe access
- **Zig**: Error unions (`!void`) - handle with `try` or explicit checking
- **TypeScript**: Exceptions - use try-catch or `peek()` for safe access

## Examples

See the `examples/` directory in each language folder for more usage patterns and advanced examples.