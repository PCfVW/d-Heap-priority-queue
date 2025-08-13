# Min/Max d-Heap Priority Queues (C++ and Rust)

This repository contains generic d-ary heap (d-heap) priority queue implementations with O(1) lookup for item updates and configurable arity d.

- Min-heap or max-heap behavior via comparator
- Efficient operations: O(1) front, O(log_d n) insert/update, O(d · log_d n) pop
- Examples and unit tests included in each language subproject
- Both implementations provide the exact same set of operations (API parity between C++ and Rust).
- <u>Provided</u>: access top (front), insert, update priority of existing item, delete-top (pop), size/length, emptiness check.
- <u>Not provided</u>: erase/remove arbitrary item by identity, meld/merge of heaps, stable ordering for equal priorities, or iterators supporting removal during traversal.

Explore the language-specific implementations:

| Language | README |
| --- | --- |
| ![C++17](https://img.shields.io/badge/C%2B%2B-17-blue.svg) | [Cpp/README.md](Cpp/README.md) |
| ![Rust Edition 2021](https://img.shields.io/badge/Rust-Edition_2021-orange.svg) | [Rust/README.md](Rust/README.md) |

References:
- Ahuja, Magnanti & Orlin, **Network Flows** (1993), Section A.3 on d-Heaps
