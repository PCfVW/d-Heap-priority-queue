// Package dheap provides a generic d-ary heap priority queue with O(1) item lookup.
//
// A d-ary heap is a tree structure where:
//   - Each node has at most d children (configurable arity)
//   - The root contains the highest-priority item
//   - Each parent has higher priority than all its children
//   - The tree is complete (filled left-to-right, level by level)
//
// This implementation uses an array-based representation with O(1) item lookup
// via a map that tracks each item's position in the heap.
//
// # Features
//
//   - Configurable arity (d): number of children per node
//   - Min-heap or max-heap behavior via comparator functions
//   - O(1) item lookup using map for efficient priority updates
//   - O(1) access to highest-priority item
//   - O(log_d n) insert and priority increase operations
//   - O(d · log_d n) pop and priority decrease operations
//
// # Cross-Language Compatibility
//
// This Go implementation maintains API parity with:
//   - C++: PriorityQueue<T> in Cpp/PriorityQueue.h
//   - Rust: d_ary_heap::PriorityQueue in Rust/src/lib.rs
//   - Zig: DHeap(T) in zig/src/dheap.zig
//   - TypeScript: PriorityQueue<T> in TypeScript/src/PriorityQueue.ts
//
// All implementations share identical time complexities and method semantics.
//
// # Basic Usage
//
//	// Create a min-heap by integer value
//	pq := dheap.New(dheap.Options[int, int]{
//		D:            4,
//		Comparator:   dheap.MinNumber,
//		KeyExtractor: func(x int) int { return x },
//	})
//
//	pq.Insert(5)
//	pq.Insert(3)
//	pq.Insert(7)
//
//	top, _ := pq.Front()  // Returns 3
//	pq.Pop()              // Removes 3
//
// # Custom Types
//
//	type Task struct {
//		ID       string
//		Priority int
//	}
//
//	pq := dheap.New(dheap.Options[Task, string]{
//		D:            4,
//		Comparator:   dheap.MinBy(func(t Task) int { return t.Priority }),
//		KeyExtractor: func(t Task) string { return t.ID },
//	})
//
// # Reference
//
// Section A.3, d-Heaps, pp. 773–778 of Ravindra Ahuja, Thomas Magnanti & James Orlin,
// Network Flows (Prentice Hall, 1993).
//
// See also: https://en.wikipedia.org/wiki/D-ary_heap
//
// Version: 2.4.0
// License: Apache-2.0
// Copyright: 2023-2026 Eric Jacopin
package dheap
