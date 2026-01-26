package dheap

import (
	"errors"
	"fmt"
	"strings"
)

// Position is the unified type alias for heap indices (cross-language consistency).
//
// Cross-language equivalents:
//   - C++: TOOLS::PriorityQueue<T>::Position
//   - Rust: d_ary_heap::Position
//   - Zig: DHeap.Position
//   - TypeScript: Position type alias
type Position = int

// Comparator returns true if a has higher priority than b.
//
// Cross-language equivalents:
//   - C++: std::less<T> / std::greater<T>
//   - Rust: PriorityCompare<T> trait
//   - Zig: Comparator(T)
//   - TypeScript: Comparator<T> function
type Comparator[T any] func(a, b T) bool

// KeyExtractor extracts a comparable key from an item for O(1) lookup.
//
// The key must be unique per item and stable (not change during the item's
// lifetime in the queue). This enables O(1) position lookup for priority updates.
type KeyExtractor[T any, K comparable] func(item T) K

// Options configures a new PriorityQueue.
//
// Cross-language equivalents:
//   - TypeScript: PriorityQueueOptions<T, K>
type Options[T any, K comparable] struct {
	// D is the arity (number of children per node). Must be >= 1. Default: 2
	D int

	// Comparator defines priority order. Returns true if first arg has higher priority.
	// Required.
	Comparator Comparator[T]

	// KeyExtractor extracts a unique identity key from each item.
	// Required for O(1) lookup and priority updates.
	KeyExtractor KeyExtractor[T, K]

	// InitialCapacity is a hint for pre-allocation.
	InitialCapacity int
}

// PriorityQueue is a generic d-ary heap with O(1) item lookup.
//
// Type Parameters:
//   - T: Item type stored in the queue
//   - K: Key type for identity lookup (must be comparable)
//
// Time Complexities (n = number of items, d = arity):
//   - New(): O(1)
//   - Len(), IsEmpty(), D(): O(1)
//   - Front(), Peek(): O(1)
//   - Contains(), ContainsKey(): O(1)
//   - GetPosition(), GetPositionByKey(): O(1)
//   - Insert(): O(log_d n)
//   - IncreasePriority(): O(log_d n)
//   - Pop(): O(d · log_d n)
//   - DecreasePriority(): O(d · log_d n)
//   - UpdatePriority(): O((d+1) · log_d n)
//   - Clear(): O(1)
//   - String(): O(n)
//
// Cross-language equivalents:
//   - C++: TOOLS::PriorityQueue<T, THash, TComparisonPredicate, TEqual>
//   - Rust: d_ary_heap::PriorityQueue<T, C>
//   - Zig: DHeap(T, HashContext(T), Comparator(T))
//   - TypeScript: PriorityQueue<T, K>
type PriorityQueue[T any, K comparable] struct {
	container    []T
	positions    map[K]Position
	depth        int
	comparator   Comparator[T]
	keyExtractor KeyExtractor[T, K]
}

// ErrEmptyQueue is returned when attempting to access items in an empty queue.
var ErrEmptyQueue = errors.New("priority queue is empty")

// ErrItemNotFound is returned when an item is not found in the queue.
var ErrItemNotFound = errors.New("item not found in priority queue")

// ErrInvalidArity is returned when arity d < 1.
var ErrInvalidArity = errors.New("arity (d) must be >= 1")

// New creates a new d-ary heap priority queue.
//
// Panics if D < 1 or if Comparator/KeyExtractor is nil.
//
// Example:
//
//	pq := dheap.New(dheap.Options[int, int]{
//		D:            4,
//		Comparator:   dheap.MinNumber,
//		KeyExtractor: func(x int) int { return x },
//	})
//
// Cross-language equivalents:
//   - C++: PriorityQueue<T>(d)
//   - Rust: PriorityQueue::new(d, comparator)
//   - Zig: DHeap.init(d, comparator, allocator)
//   - TypeScript: new PriorityQueue(options)
func New[T any, K comparable](opts Options[T, K]) *PriorityQueue[T, K] {
	d := opts.D
	if d == 0 {
		d = 2 // Default to binary heap
	}
	if d < 1 {
		panic(ErrInvalidArity)
	}
	if opts.Comparator == nil {
		panic("Comparator is required")
	}
	if opts.KeyExtractor == nil {
		panic("KeyExtractor is required")
	}

	capacity := opts.InitialCapacity
	if capacity < 0 {
		capacity = 0
	}

	return &PriorityQueue[T, K]{
		container:    make([]T, 0, capacity),
		positions:    make(map[K]Position, capacity),
		depth:        d,
		comparator:   opts.Comparator,
		keyExtractor: opts.KeyExtractor,
	}
}

// WithFirst creates a new priority queue with an initial item already inserted.
//
// Equivalent to Rust's with_first() constructor.
//
// Example:
//
//	pq := dheap.WithFirst(dheap.Options[int, int]{
//		D:            4,
//		Comparator:   dheap.MinNumber,
//		KeyExtractor: func(x int) int { return x },
//	}, 42)
//
// Cross-language equivalents:
//   - Rust: PriorityQueue::with_first(d, comparator, item)
//   - TypeScript: PriorityQueue.withFirst(options, item)
func WithFirst[T any, K comparable](opts Options[T, K], firstItem T) *PriorityQueue[T, K] {
	pq := New(opts)
	pq.Insert(firstItem)
	return pq
}

// ===========================================================================
// Query Operations
// ===========================================================================

// Len returns the number of items in the heap.
//
// Time Complexity: O(1)
//
// Cross-language equivalents:
//   - C++: len()
//   - Rust: len()
//   - Zig: len()
//   - TypeScript: len()
func (pq *PriorityQueue[T, K]) Len() int {
	return len(pq.container)
}

// IsEmpty returns true if the heap is empty.
//
// Time Complexity: O(1)
//
// Cross-language equivalents:
//   - C++: is_empty()
//   - Rust: is_empty()
//   - Zig: isEmpty()
//   - TypeScript: isEmpty()
func (pq *PriorityQueue[T, K]) IsEmpty() bool {
	return len(pq.container) == 0
}

// Is_empty is a snake_case alias for IsEmpty (cross-language consistency).
func (pq *PriorityQueue[T, K]) Is_empty() bool {
	return pq.IsEmpty()
}

// D returns the arity (number of children per node) of the heap.
//
// Time Complexity: O(1)
//
// Cross-language equivalents:
//   - C++: d()
//   - Rust: d()
//   - Zig: d()
//   - TypeScript: d()
func (pq *PriorityQueue[T, K]) D() int {
	return pq.depth
}

// Contains checks if an item with the same key exists in the heap.
//
// Time Complexity: O(1)
//
// Cross-language equivalents:
//   - C++: contains(item)
//   - Rust: contains(item)
//   - Zig: contains(item)
//   - TypeScript: contains(item)
func (pq *PriorityQueue[T, K]) Contains(item T) bool {
	key := pq.keyExtractor(item)
	_, exists := pq.positions[key]
	return exists
}

// ContainsKey checks if an item with the given key exists in the heap.
//
// Time Complexity: O(1)
//
// Cross-language equivalents:
//   - TypeScript: containsKey(key)
func (pq *PriorityQueue[T, K]) ContainsKey(key K) bool {
	_, exists := pq.positions[key]
	return exists
}

// GetPosition returns the current position (index) of an item in the heap.
//
// Time Complexity: O(1)
//
// Cross-language equivalents:
//   - TypeScript: getPosition(item)
func (pq *PriorityQueue[T, K]) GetPosition(item T) (Position, bool) {
	key := pq.keyExtractor(item)
	pos, exists := pq.positions[key]
	return pos, exists
}

// GetPositionByKey returns the current position (index) of an item by its key.
//
// Time Complexity: O(1)
//
// Cross-language equivalents:
//   - TypeScript: getPositionByKey(key)
func (pq *PriorityQueue[T, K]) GetPositionByKey(key K) (Position, bool) {
	pos, exists := pq.positions[key]
	return pos, exists
}

// Front returns the highest-priority item without removing it.
//
// Returns ErrEmptyQueue if the heap is empty.
//
// Time Complexity: O(1)
//
// Cross-language equivalents:
//   - C++: front() (UB if empty)
//   - Rust: front() (panics if empty)
//   - Zig: front() (returns null if empty)
//   - TypeScript: front() (throws if empty)
func (pq *PriorityQueue[T, K]) Front() (T, error) {
	if len(pq.container) == 0 {
		var zero T
		return zero, ErrEmptyQueue
	}
	return pq.container[0], nil
}

// Peek returns the highest-priority item without removing it.
//
// Safe alternative to Front(). Returns (item, true) if found, or (zero, false) if empty.
//
// Time Complexity: O(1)
//
// Cross-language equivalents:
//   - Rust: peek()
//   - TypeScript: peek()
func (pq *PriorityQueue[T, K]) Peek() (T, bool) {
	if len(pq.container) == 0 {
		var zero T
		return zero, false
	}
	return pq.container[0], true
}

// ===========================================================================
// Modification Operations
// ===========================================================================

// Insert adds a new item into the heap according to its priority.
//
// Time Complexity: O(log_d n)
//
// Note: If an item with the same key already exists, behavior is undefined.
// Use Contains() to check first, or use IncreasePriority()/DecreasePriority()
// to update existing items.
//
// Cross-language equivalents:
//   - C++: insert(item)
//   - Rust: insert(item)
//   - Zig: insert(item)
//   - TypeScript: insert(item)
func (pq *PriorityQueue[T, K]) Insert(item T) {
	index := len(pq.container)
	pq.container = append(pq.container, item)

	// Fast path: first item doesn't need sift-up
	if index == 0 {
		pq.positions[pq.keyExtractor(item)] = 0
		return
	}

	pq.positions[pq.keyExtractor(item)] = index
	pq.moveUp(index)
}

// InsertMany inserts multiple items into the heap.
//
// Uses heapify algorithm which is O(n) for bulk insertion vs O(n log n)
// for individual inserts when starting from empty.
//
// Time Complexity: O(n) where n is total items after insertion
//
// Cross-language equivalents:
//   - TypeScript: insertMany(items)
func (pq *PriorityQueue[T, K]) InsertMany(items []T) {
	if len(items) == 0 {
		return
	}

	startIndex := len(pq.container)

	// Add all items to container and positions map
	for i, item := range items {
		pq.container = append(pq.container, item)
		pq.positions[pq.keyExtractor(item)] = startIndex + i
	}

	// If this was an empty heap, use heapify O(n) instead of n insertions O(n log n)
	if startIndex == 0 && len(items) > 1 {
		pq.heapify()
	} else {
		// Otherwise, sift up each new item
		for i := startIndex; i < len(pq.container); i++ {
			pq.moveUp(i)
		}
	}
}

// heapify builds heap property from unordered array using Floyd's algorithm.
// Called internally by InsertMany when starting from empty heap.
// Time Complexity: O(n)
func (pq *PriorityQueue[T, K]) heapify() {
	n := len(pq.container)
	if n <= 1 {
		return
	}

	d := pq.depth
	// Start from last non-leaf node and sift down each
	// Last non-leaf is parent of last element: floor((n-2)/d)
	lastNonLeaf := (n - 2) / d

	for i := lastNonLeaf; i >= 0; i-- {
		pq.moveDown(i)
	}
}

// IncreasePriority updates an existing item to have higher priority (moves toward root).
//
// Time Complexity: O(log_d n)
//
// Returns ErrItemNotFound if the item is not in the queue.
//
// Note:
//   - For min-heap: decreasing the priority value increases importance.
//   - For max-heap: increasing the priority value increases importance.
//
// Cross-language equivalents:
//   - C++: increase_priority(item)
//   - Rust: increase_priority(item)
//   - Zig: increasePriority(item)
//   - TypeScript: increasePriority(item)
func (pq *PriorityQueue[T, K]) IncreasePriority(updatedItem T) error {
	key := pq.keyExtractor(updatedItem)
	index, exists := pq.positions[key]
	if !exists {
		return ErrItemNotFound
	}

	pq.container[index] = updatedItem
	pq.moveUp(index)
	return nil
}

// Increase_priority is a snake_case alias for IncreasePriority.
func (pq *PriorityQueue[T, K]) Increase_priority(updatedItem T) error {
	return pq.IncreasePriority(updatedItem)
}

// IncreasePriorityByIndex increases the priority of the item at the given index.
//
// Time Complexity: O(log_d n)
//
// Panics if index is out of bounds.
//
// Cross-language equivalents:
//   - Rust: increase_priority_by_index(index)
//   - TypeScript: increasePriorityByIndex(index)
func (pq *PriorityQueue[T, K]) IncreasePriorityByIndex(index Position) {
	if index < 0 || index >= len(pq.container) {
		panic("index out of bounds")
	}
	pq.moveUp(index)
}

// Increase_priority_by_index is a snake_case alias for IncreasePriorityByIndex (cross-language consistency).
func (pq *PriorityQueue[T, K]) Increase_priority_by_index(index Position) {
	pq.IncreasePriorityByIndex(index)
}

// DecreasePriorityByIndex decreases the priority of the item at the given index.
//
// Time Complexity: O(d · log_d n)
//
// Panics if index is out of bounds.
//
// Note: The item at the given index should already have its priority value updated
// in the container before calling this method. This is primarily useful for
// internal operations or when you have direct access to the heap array.
//
// Cross-language equivalents:
//   - TypeScript: decreasePriorityByIndex(index)
func (pq *PriorityQueue[T, K]) DecreasePriorityByIndex(index Position) {
	if index < 0 || index >= len(pq.container) {
		panic("index out of bounds")
	}
	pq.moveDown(index)
}

// Decrease_priority_by_index is a snake_case alias for DecreasePriorityByIndex (cross-language consistency).
func (pq *PriorityQueue[T, K]) Decrease_priority_by_index(index Position) {
	pq.DecreasePriorityByIndex(index)
}

// DecreasePriority updates an existing item to have lower priority (moves toward leaves).
//
// Time Complexity: O(d · log_d n)
//
// Returns ErrItemNotFound if the item is not in the queue.
//
// Note: This method only moves down. Use UpdatePriority() if direction is unknown.
//
// Cross-language equivalents:
//   - C++: decrease_priority(item)
//   - Rust: decrease_priority(item)
//   - Zig: decreasePriority(item)
//   - TypeScript: decreasePriority(item)
func (pq *PriorityQueue[T, K]) DecreasePriority(updatedItem T) error {
	key := pq.keyExtractor(updatedItem)
	index, exists := pq.positions[key]
	if !exists {
		return ErrItemNotFound
	}

	pq.container[index] = updatedItem
	pq.moveDown(index)
	return nil
}

// Decrease_priority is a snake_case alias for DecreasePriority.
func (pq *PriorityQueue[T, K]) Decrease_priority(updatedItem T) error {
	return pq.DecreasePriority(updatedItem)
}

// UpdatePriority updates an existing item when the direction of priority change is unknown.
//
// Time Complexity: O((d+1) · log_d n)
//
// Returns ErrItemNotFound if the item is not in the queue.
//
// This method checks both directions (moveUp then moveDown), making it safe to use
// when you don't know whether the priority increased or decreased.
//
// Use IncreasePriority() or DecreasePriority() for better performance when you
// know the direction of the change.
//
// Cross-language equivalents:
//   - TypeScript: updatePriority(item)
func (pq *PriorityQueue[T, K]) UpdatePriority(updatedItem T) error {
	key := pq.keyExtractor(updatedItem)
	index, exists := pq.positions[key]
	if !exists {
		return ErrItemNotFound
	}

	pq.container[index] = updatedItem
	pq.moveUp(index)
	// Re-fetch position in case moveUp changed it
	index = pq.positions[key]
	pq.moveDown(index)
	return nil
}

// Update_priority is a snake_case alias for UpdatePriority.
func (pq *PriorityQueue[T, K]) Update_priority(updatedItem T) error {
	return pq.UpdatePriority(updatedItem)
}

// Pop removes and returns the highest-priority item from the heap.
//
// Returns (item, true) if successful, or (zero, false) if the heap is empty.
//
// Time Complexity: O(d · log_d n)
//
// Cross-language equivalents:
//   - C++: pop()
//   - Rust: pop() (no return)
//   - Zig: pop()
//   - TypeScript: pop()
func (pq *PriorityQueue[T, K]) Pop() (T, bool) {
	n := len(pq.container)
	if n == 0 {
		var zero T
		return zero, false
	}

	top := pq.container[0]
	delete(pq.positions, pq.keyExtractor(top))

	if n == 1 {
		pq.container = pq.container[:0]
		return top, true
	}

	// Move last item to root and sift down
	lastItem := pq.container[n-1]
	pq.container[0] = lastItem
	pq.positions[pq.keyExtractor(lastItem)] = 0
	pq.container = pq.container[:n-1]

	pq.moveDown(0)

	return top, true
}

// PopMany removes and returns multiple highest-priority items.
//
// Time Complexity: O(count · d · log_d n)
//
// Cross-language equivalents:
//   - TypeScript: popMany(count)
func (pq *PriorityQueue[T, K]) PopMany(count int) []T {
	if count <= 0 {
		return nil
	}

	actualCount := count
	if actualCount > len(pq.container) {
		actualCount = len(pq.container)
	}

	result := make([]T, 0, actualCount)
	for i := 0; i < actualCount; i++ {
		if item, ok := pq.Pop(); ok {
			result = append(result, item)
		}
	}

	return result
}

// Clear removes all items from the heap, optionally changing the arity.
//
// Time Complexity: O(1)
//
// Cross-language equivalents:
//   - C++: clear(opt_d)
//   - Rust: clear(opt_d)
//   - Zig: clear(new_depth?)
//   - TypeScript: clear(newD?)
func (pq *PriorityQueue[T, K]) Clear(newD ...int) {
	pq.container = pq.container[:0]
	pq.positions = make(map[K]Position)

	if len(newD) > 0 && newD[0] >= 1 {
		pq.depth = newD[0]
	}
}

// ToArray returns a copy of all items in heap order (not priority order).
//
// Time Complexity: O(n)
//
// Cross-language equivalents:
//   - TypeScript: toArray()
func (pq *PriorityQueue[T, K]) ToArray() []T {
	result := make([]T, len(pq.container))
	copy(result, pq.container)
	return result
}

// String returns a string representation of the heap contents.
//
// Implements fmt.Stringer interface.
//
// Time Complexity: O(n)
//
// Cross-language equivalents:
//   - C++: to_string() / put(ostream)
//   - Rust: to_string() / Display trait
//   - Zig: toString()
//   - TypeScript: toString()
func (pq *PriorityQueue[T, K]) String() string {
	if len(pq.container) == 0 {
		return "{}"
	}

	var sb strings.Builder
	sb.WriteString("{")
	for i, item := range pq.container {
		if i > 0 {
			sb.WriteString(", ")
		}
		sb.WriteString(fmt.Sprintf("%v", item))
	}
	sb.WriteString("}")
	return sb.String()
}

// To_string is a snake_case alias for String (cross-language consistency).
func (pq *PriorityQueue[T, K]) To_string() string {
	return pq.String()
}

// ===========================================================================
// Private Methods - Heap Operations
// ===========================================================================

// swap exchanges two items in the heap and updates their position mappings.
func (pq *PriorityQueue[T, K]) swap(i, j Position) {
	pq.container[i], pq.container[j] = pq.container[j], pq.container[i]

	// Update positions
	pq.positions[pq.keyExtractor(pq.container[i])] = i
	pq.positions[pq.keyExtractor(pq.container[j])] = j
}

// bestChildPosition finds the child with highest priority among all children of node i.
// Children of node i are at indices: left = i*d+1 through i*d+d (inclusive), i.e., [left, left+d-1].
func (pq *PriorityQueue[T, K]) bestChildPosition(i Position) Position {
	d := pq.depth
	n := len(pq.container)
	left := i*d + 1

	if left >= n {
		return left
	}

	best := left
	// rightBound is the exclusive upper bound for iteration (one past the last child index)
	rightBound := (i+1)*d + 1
	if rightBound > n {
		rightBound = n
	}

	for j := left + 1; j < rightBound; j++ {
		if pq.comparator(pq.container[j], pq.container[best]) {
			best = j
		}
	}

	return best
}

// moveUp sifts an item up to restore heap property.
func (pq *PriorityQueue[T, K]) moveUp(i Position) {
	d := pq.depth

	for i > 0 {
		p := (i - 1) / d
		if pq.comparator(pq.container[i], pq.container[p]) {
			pq.swap(i, p)
			i = p
		} else {
			break
		}
	}
}

// moveDown sifts an item down to restore heap property.
func (pq *PriorityQueue[T, K]) moveDown(i Position) {
	d := pq.depth
	n := len(pq.container)

	for {
		firstChild := i*d + 1
		if firstChild >= n {
			break
		}

		best := pq.bestChildPosition(i)
		if pq.comparator(pq.container[best], pq.container[i]) {
			pq.swap(i, best)
			i = best
		} else {
			break
		}
	}
}
