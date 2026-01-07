![C++17](https://img.shields.io/badge/C%2B%2B-17-blue.svg)
![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-green.svg)

# d-Heap Priority Queue (C++17) v2.4.0

This is a generic d-ary heap priority queue supporting both min-queue and max-queue behavior through a comparator.

## Strengths

- **Flexible behavior**: min-heap or max-heap via a comparator (`std::less<T>` by default), and configurable arity `d` at construction time.
- **Efficient operations** on n items (see reference below):
  - O(1): access the highest-priority item (`front()`).
  - O(log_d n): `insert()` and upward reheapification.
  - O(d · log_d n): delete-top (`pop()`), and child selection per level in a d-ary heap.
- **O(1) item lookup**: an internal dictionary maps each item to its position, enabling efficient priority updates by item identity.
- **Practical API**: `insert`, `front`, `pop`, `empty`, `size`, `clear(optional new d)`, stream output via `put(std::ostream&)`.
- **Unified API**: Cross-language standardized methods: `len()`, `is_empty()`, `d()`, `contains()`, `to_string()`, `decrease_priority()`, and `Position` type alias.

## How to use (basic example)

Define your item type `T` and provide:
- **Hash** for `T` (e.g., by stable id/number),
- **Equality** for `T` (same identity as hash),
- **Comparator** on priority (e.g., by `cost`) determining min- or max-queue.

```cpp
#include "PriorityQueue.h"
using namespace TOOLS;

struct Item {
    uint32_t id;
    uint32_t cost; // priority
};

struct ItemHash { size_t operator()(const Item& x) const { return std::hash<uint32_t>()(x.id); } };
struct ItemEq   { bool operator()(const Item& a, const Item& b) const { return a.id == b.id; } };
struct LessCost { bool operator()(const Item& a, const Item& b) const { return a.cost < b.cost; } }; // min-queue

TOOLS::PriorityQueue<Item, ItemHash, LessCost, ItemEq> pq(/* d = */ 3);

pq.insert({.id=1, .cost=10});
pq.insert({.id=2, .cost=5});

auto top = pq.front();      // highest priority (lowest cost here)

// Increase priority of an existing item (e.g., decrease cost in a min-queue)
Item i{.id=1, .cost=3};     // same identity, new priority
pq.increase_priority(i);    // repositions item upward

// Decrease priority of an existing item (e.g., increase cost in a min-queue)
Item j{.id=2, .cost=8};     // same identity, new priority
pq.decrease_priority(j);    // repositions item downward

pq.pop();                   // remove current highest-priority item

pq.clear(/* optional new d */);

// Unified API methods (cross-language consistency)
std::cout << "Size: " << pq.len() << std::endl;           // Same as pq.size()
std::cout << "Empty: " << pq.is_empty() << std::endl;     // Same as pq.empty()
std::cout << "Arity: " << pq.d() << std::endl;            // Get d value
std::cout << "Contains: " << pq.contains({.id=1, .cost=0}) << std::endl; // O(1) membership test
std::cout << "Contents: " << pq.to_string() << std::endl; // String output
```

## Compilation

```bash
# Compile with C++17 support
g++ -std=c++17 -O2 your_program.cpp -o your_program
# or with MSVC
cl /std:c++17 /EHsc /O2 your_program.cpp
```

Notes:
- Identity (hash/equality) should be stable; priority values may change over time.
- Only the highest-priority item can be removed directly (`pop()`).

## What is a d-Heap?

- A [d-ary heap](https://en.wikipedia.org/wiki/D-ary_heap) is a tree where each node has up to d children, the root holds the highest priority, children are unordered, and priorities decrease along any root-to-leaf path.
- Time complexities over n items (cf. reference):
  - O(1): access top
  - O(d · log_d n): delete-top
  - O(log_d n): insert and upward update

## Reference

Section A.3, [d-Heaps](https://en.wikipedia.org/wiki/D-ary_heap), pp. 773–778 of Ravindra Ahuja, Thomas Magnanti & James Orlin, **Network Flows** (Prentice Hall, 1993). Book info: https://mitmgmtfaculty.mit.edu/jorlin/network-flows/
