Looking at the test requirements, I need to implement a d-ary heap priority queue in Go. Let me analyze the test structure and create the implementation.

```go
// src/priority_queue.go
package dheap

import (
	"errors"
)

var (
	ErrEmptyQueue   = errors.New("queue is empty")
	ErrItemNotFound = errors.New("item not found")
)

// Options configures the priority queue
type Options[T any, K comparable] struct {
	D            int                    // heap arity
	Comparator   func(T, T) bool       // comparison function (true if first has higher priority)
	KeyExtractor func(T) K             // extracts key for equality comparison
}

// PriorityQueue implements a d-ary heap
type PriorityQueue[T any, K comparable] struct {
	d            int
	items        []T
	keyToIndex   map[K]int
	comparator   func(T, T) bool
	keyExtractor func(T) K
}

// New creates a new d-ary heap priority queue
func New[T any, K comparable](opts Options[T, K]) *PriorityQueue[T, K] {
	return &PriorityQueue[T, K]{
		d:            opts.D,
		items:        make([]T, 0),
		keyToIndex:   make(map[K]int),
		comparator:   opts.Comparator,
		keyExtractor: opts.KeyExtractor,
	}
}

// MinBy creates a min-heap comparator using the given priority extractor
func MinBy[T any, P interface{ ~int | ~float64 }](priorityExtractor func(T) P) func(T, T) bool {
	return func(a, b T) bool {
		return priorityExtractor(a) < priorityExtractor(b)
	}
}

// D returns the heap arity
func (pq *PriorityQueue[T, K]) D() int {
	return pq.d
}

// Len returns the number of items in the queue
func (pq *PriorityQueue[T, K]) Len() int {
	return len(pq.items)
}

// IsEmpty returns whether the queue is empty
func (pq *PriorityQueue[T, K]) IsEmpty() bool {
	return len(pq.items) == 0
}

// Contains checks if an item with the given identity exists
func (pq *PriorityQueue[T, K]) Contains(item T) bool {
	key := pq.keyExtractor(item)
	_, exists := pq.keyToIndex[key]
	return exists
}

// ContainsKey checks if an item with the given key exists
func (pq *PriorityQueue[T, K]) ContainsKey(key K) bool {
	_, exists := pq.keyToIndex[key]
	return exists
}

// Insert adds an item to the queue
func (pq *PriorityQueue[T, K]) Insert(item T) {
	key := pq.keyExtractor(item)
	
	// If item already exists, update it
	if idx, exists := pq.keyToIndex[key]; exists {
		oldItem := pq.items[idx]
		pq.items[idx] = item
		
		// Determine if we need to bubble up or sink down
		if pq.comparator(item, oldItem) {
			pq.bubbleUp(idx)
		} else {
			pq.sinkDown(idx)
		}
		return
	}
	
	// Add new item
	pq.items = append(pq.items, item)
	idx := len(pq.items) - 1
	pq.keyToIndex[key] = idx
	pq.bubbleUp(idx)
}

// Pop removes and returns the item with highest priority
func (pq *PriorityQueue[T, K]) Pop() (T, bool) {
	var zero T
	if len(pq.items) == 0 {
		return zero, false
	}
	
	if len(pq.items) == 1 {
		item := pq.items[0]
		key := pq.keyExtractor(item)
		pq.items = pq.items[:0]
		delete(pq.keyToIndex, key)
		return item, true
	}
	
	// Store the root item to return
	root := pq.items[0]
	rootKey := pq.keyExtractor(root)
	
	// Move last item to root
	lastItem := pq.items[len(pq.items)-1]
	lastKey := pq.keyExtractor(lastItem)
	
	pq.items[0] = lastItem
	pq.items = pq.items[:len(pq.items)-1]
	
	// Update key mappings
	delete(pq.keyToIndex, rootKey)
	pq.keyToIndex[lastKey] = 0
	
	// Restore heap property
	if len(pq.items) > 0 {
		pq.sinkDown(0)
	}
	
	return root, true
}

// Front returns the item with highest priority without removing it
func (pq *PriorityQueue[T, K]) Front() (T, error) {
	var zero T
	if len(pq.items) == 0 {
		return zero, ErrEmptyQueue
	}
	return pq.items[0], nil
}

// IncreasePriority updates an existing item to have higher priority
func (pq *PriorityQueue[T, K]) IncreasePriority(item T) error {
	key := pq.keyExtractor(item)
	idx, exists := pq.keyToIndex[key]
	if !exists {
		return ErrItemNotFound
	}
	
	pq.items[idx] = item
	pq.bubbleUp(idx)
	return nil
}

// DecreasePriority updates an existing item to have lower priority
func (pq *PriorityQueue[T, K]) DecreasePriority(item T) error {
	key := pq.keyExtractor(item)
	idx, exists := pq.keyToIndex[key]
	if !exists {
		return ErrItemNotFound
	}
	
	pq.items[idx] = item
	pq.sinkDown(idx)
	return nil
}

// GetPosition returns the position of an item in the heap
func (pq *PriorityQueue[T, K]) GetPosition(item T) (int, bool) {
	key := pq.keyExtractor(item)
	idx, exists := pq.keyToIndex[key]
	return idx, exists
}

// GetPositionByKey returns the position of an item by its key
func (pq *PriorityQueue[T, K]) GetPositionByKey(key K) (int, bool) {
	idx, exists := pq.keyToIndex[key]
	return idx, exists
}

// ToArray returns a copy of the internal array (for testing)
func (pq *PriorityQueue[T, K]) ToArray() []T {
	result := make([]T, len(pq.items))
	copy(result, pq.items)
	return result
}

// Helper methods

func (pq *PriorityQueue[T, K]) parentIndex(i int) int {
	return (i - 1) / pq.d
}

func (pq *PriorityQueue[T, K]) firstChildIndex(i int) int {
	return i*pq.d + 1
}

func (pq *PriorityQueue[T, K]) bubbleUp(idx int) {
	for idx > 0 {
		parentIdx := pq.parentIndex(idx)
		if !pq.comparator(pq.items[idx], pq.items[parentIdx]) {
			break
		}
		pq.swap(idx, parentIdx)
		idx = parentIdx
	}
}

func (pq *PriorityQueue[T, K]) sinkDown(idx int) {
	for {
		bestIdx := idx
		firstChild := pq.firstChildIndex(idx)
		
		// Check all children
		for i := 0; i < pq.d; i++ {
			childIdx := firstChild + i
			if childIdx >= len(pq.items) {
				break
			}
			if pq.comparator(pq.items[childIdx], pq.items[bestIdx]) {
				bestIdx = childIdx
			}
		}
		
		if bestIdx == idx {
			break
		}
		
		pq.swap(idx, bestIdx)
		idx = bestIdx
	}
}

func (pq *PriorityQueue[T, K]) swap(i, j int) {
	// Swap items
	pq.items[i], pq.items[j] = pq.items[j], pq.items[i]
	
	// Update key mappings
	keyI := pq.keyExtractor(pq.items[i])
	keyJ := pq.keyExtractor(pq.items[j])
	pq.keyToIndex[keyI] = i
	pq.keyToIndex[keyJ] = j
}
```

This implementation provides:

1. **Configurable d-ary heap**: The arity `d` is set during construction and determines how many children each node can have.

2. **Generic types**: Uses Go generics to support any item type `T` and key type `K`.

3. **Identity vs Priority**: Items are compared for equality using their key (extracted via `KeyExtractor`), while ordering uses the `Comparator` function.

4. **O(1) lookup**: The `keyToIndex` map provides constant-time lookup to check if an item exists and find its position.

5. **Min-heap support**: The `MinBy` helper creates a comparator for min-heaps where lower values have higher priority.

6. **All required operations**:
   - `Insert`: Adds items and maintains heap property via bubble-up
   - `Pop`: Removes root, moves last item to root, and sinks down
   - `Front`: Returns root without modification
   - `IncreasePriority`/`DecreasePriority`: Updates items and restores heap property
   - `Contains`/`ContainsKey`: O(1) existence checks
   - `Len`/`IsEmpty`: Size queries

7. **Heap invariant maintenance**: The `bubbleUp` and `sinkDown` methods ensure the heap property is maintained after all operations.

8. **Error handling**: Returns appropriate errors for empty queue and missing items instead of panicking.

The implementation handles all the edge cases tested in the corpus, including empty queues, non-existent items, and maintains the heap invariant throughout all operations.