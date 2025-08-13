![Rust Edition 2021](https://img.shields.io/badge/Rust-Edition_2021-orange.svg)
![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-green.svg)

# d-Heap Priority Queue (Rust, Edition 2021)

This is a generic d-ary heap priority queue supporting both min-queue and max-queue behavior through a comparator wrapper.

## Strengths

- **Flexible behavior**: min-heap or max-heap via comparator adaptors (`MinBy`/`MaxBy`), and configurable arity `d` at construction time.
- **Efficient operations** on n items (see reference below):
  - O(1): access the highest-priority item (`front()`).
  - O(log_d n): `insert()` and upward reheapification.
  - O(d · log_d n): delete-top (`pop()`), with up to d children examined per level.
- **O(1) item lookup**: internal hash map tracks positions by item identity, enabling efficient priority updates for existing items.
- **Practical API**: `insert`, `front`, `pop`, `increase_priority`, `is_empty`, `len`.

## How to use (basic example)

Define your item type `T` and derive:
- **Hash + Eq** for identity (e.g., by stable id/number),
- Choose a comparator wrapper on priority (e.g., by `cost`) determining min- or max-queue.

```rust
use priority_queue::{PriorityQueue, MinBy, MaxBy};

#[derive(Clone, Eq, PartialEq, Hash)]
struct Item { id: u32, cost: u32 } // identity by id; priority by cost

// Min-queue by "cost"
let mut pq: PriorityQueue<Item, MinBy<_>> =
    PriorityQueue::new(3, MinBy(|x: &Item| x.cost));

pq.insert(Item { id: 1, cost: 10 });
pq.insert(Item { id: 2, cost: 5 });

let top = pq.front().clone(); // highest priority (lowest cost here)

// Increase priority of an existing item (e.g., decrease cost in a min-queue)
let updated = Item { id: 1, cost: 3 }; // same identity, new priority
pq.increase_priority(&updated);        // repositions item upward as needed

pq.pop(); // remove current highest-priority item
```

Notes:
- Identity should be stable (`Eq` + `Hash`); priority values may change over time.
- Only the highest-priority item can be removed directly (`pop()`).

## What is a d-Heap?

- A [d-ary heap](https://en.wikipedia.org/wiki/D-ary_heap) is a tree where each node has up to d children, the root holds the highest priority, children are unordered, and priorities decrease along any root-to-leaf path.
- Time complexities over n items (cf. reference):
  - O(1): access top
  - O(d · log_d n): delete-top
  - O(log_d n): insert and upward update

## Reference

Section A.3, [d-Heaps](https://en.wikipedia.org/wiki/D-ary_heap), pp. 773–778 of Ravindra Ahuja, Thomas Magnanti & James Orlin, **Network Flows** (Prentice Hall, 1993). Book info: https://mitmgmtfaculty.mit.edu/jorlin/network-flows/
