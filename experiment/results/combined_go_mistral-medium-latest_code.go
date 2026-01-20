package dheap

import (
	"errors"
	"fmt"
)

// Common errors
var (
	ErrEmptyQueue   = errors.New("priority queue is empty")
	ErrItemNotFound = errors.New("item not found in priority queue")
	ErrItemExists   = errors.New("item already exists in priority queue")
)

// Comparator defines a comparison function for items
type Comparator[T any] func(a, b T) int

// MinBy creates a comparator that orders items by a key function (min-heap)
func MinBy[T any, K ordered](keyFunc func(T) K) Comparator[T] {
	return func(a, b T) int {
		aKey, bKey := keyFunc(a), keyFunc(b)
		switch {
		case aKey < bKey:
			return -1
		case aKey > bKey:
			return 1
		default:
			return 0
		}
	}
}

// MaxBy creates a comparator that orders items by a key function (max-heap)
func MaxBy[T any, K ordered](keyFunc func(T) K) Comparator[T] {
	return func(a, b T) int {
		aKey, bKey := keyFunc(a), keyFunc(b)
		switch {
		case aKey > bKey:
			return -1
		case aKey < bKey:
			return 1
		default:
			return 0
		}
	}
}

// Options configures the priority queue
type Options[T any, K comparable] struct {
	D            int         // Arity of the heap (number of children per node)
	Comparator   Comparator[T] // Comparison function for items
	KeyExtractor func(T) K    // Function to extract the key from an item
}

// PriorityQueue implements a d-ary heap priority queue
type PriorityQueue[T any, K comparable] struct {
	heap         []T
	positionMap  map[K]int
	d            int
	compare      Comparator[T]
	keyExtractor func(T) K
}

// New creates a new priority queue with the given options
func New[T any, K comparable](options Options[T, K]) *PriorityQueue[T, K] {
	if options.D < 2 {
		panic("d must be >= 2")
	}
	if options.Comparator == nil {
		panic("comparator must not be nil")
	}
	if options.KeyExtractor == nil {
		panic("keyExtractor must not be nil")
	}

	return &PriorityQueue[T, K]{
		heap:         make([]T, 0),
		positionMap:  make(map[K]int),
		d:            options.D,
		compare:      options.Comparator,
		keyExtractor: options.KeyExtractor,
	}
}

// D returns the arity of the heap
func (pq *PriorityQueue[T, K]) D() int {
	return pq.d
}

// Len returns the number of items in the queue
func (pq *PriorityQueue[T, K]) Len() int {
	return len(pq.heap)
}

// IsEmpty returns true if the queue is empty
func (pq *PriorityQueue[T, K]) IsEmpty() bool {
	return len(pq.heap) == 0
}

// Contains checks if an item with the same key exists in the queue
func (pq *PriorityQueue[T, K]) Contains(item T) bool {
	key := pq.keyExtractor(item)
	_, exists := pq.positionMap[key]
	return exists
}

// ContainsKey checks if an item with the given key exists in the queue
func (pq *PriorityQueue[T, K]) ContainsKey(key K) bool {
	_, exists := pq.positionMap[key]
	return exists
}

// GetPosition returns the index of an item in the heap
func (pq *PriorityQueue[T, K]) GetPosition(item T) (int, error) {
	key := pq.keyExtractor(item)
	pos, exists := pq.positionMap[key]
	if !exists {
		return -1, ErrItemNotFound
	}
	return pos, nil
}

// GetPositionByKey returns the index of an item with the given key in the heap
func (pq *PriorityQueue[T, K]) GetPositionByKey(key K) (int, error) {
	pos, exists := pq.positionMap[key]
	if !exists {
		return -1, ErrItemNotFound
	}
	return pos, nil
}

// Insert adds an item to the queue
func (pq *PriorityQueue[T, K]) Insert(item T) error {
	key := pq.keyExtractor(item)
	if _, exists := pq.positionMap[key]; exists {
		return ErrItemExists
	}

	// Add to the end of the heap
	pq.heap = append(pq.heap, item)
	pos := len(pq.heap) - 1
	pq.positionMap[key] = pos

	// Sift up to maintain heap property
	pq.siftUp(pos)
	return nil
}

// Front returns the item with highest priority without removing it
func (pq *PriorityQueue[T, K]) Front() (T, error) {
	if pq.IsEmpty() {
		var zero T
		return zero, ErrEmptyQueue
	}
	return pq.heap[0], nil
}

// Pop removes and returns the item with highest priority
func (pq *PriorityQueue[T, K]) Pop() (T, bool) {
	if pq.IsEmpty() {
		var zero T
		return zero, false
	}

	// Get the root item
	root := pq.heap[0]

	// Remove the last item and put it at root
	last := pq.heap[len(pq.heap)-1]
	pq.heap = pq.heap[:len(pq.heap)-1]

	// If there are items left, move the last item to root and sift down
	if len(pq.heap) > 0 {
		// Update position map for the moved item
		lastKey := pq.keyExtractor(last)
		pq.positionMap[lastKey] = 0
		pq.heap[0] = last

		// Sift down to maintain heap property
		pq.siftDown(0)
	}

	// Remove the root from position map
	rootKey := pq.keyExtractor(root)
	delete(pq.positionMap, rootKey)

	return root, true
}

// IncreasePriority updates an existing item to have higher priority (lower value in min-heap)
func (pq *PriorityQueue[T, K]) IncreasePriority(item T) error {
	key := pq.keyExtractor(item)
	pos, exists := pq.positionMap[key]
	if !exists {
		return ErrItemNotFound
	}

	// Update the item in the heap
	pq.heap[pos] = item

	// Sift up to maintain heap property
	pq.siftUp(pos)
	return nil
}

// DecreasePriority updates an existing item to have lower priority (higher value in min-heap)
func (pq *PriorityQueue[T, K]) DecreasePriority(item T) error {
	key := pq.keyExtractor(item)
	pos, exists := pq.positionMap[key]
	if !exists {
		return ErrItemNotFound
	}

	// Update the item in the heap
	pq.heap[pos] = item

	// Sift down to maintain heap property
	pq.siftDown(pos)
	return nil
}

// ToArray returns a copy of the heap array (for testing purposes)
func (pq *PriorityQueue[T, K]) ToArray() []T {
	result := make([]T, len(pq.heap))
	copy(result, pq.heap)
	return result
}

// parent returns the index of the parent of the item at index i
func (pq *PriorityQueue[T, K]) parent(i int) int {
	return (i - 1) / pq.d
}

// firstChild returns the index of the first child of the item at index i
func (pq *PriorityQueue[T, K]) firstChild(i int) int {
	return i*pq.d + 1
}

// siftUp moves the item at index i up to maintain the heap property
func (pq *PriorityQueue[T, K]) siftUp(i int) {
	for i > 0 {
		p := pq.parent(i)
		if pq.compare(pq.heap[i], pq.heap[p]) >= 0 {
			break // heap property is satisfied
		}

		// Swap items
		pq.heap[i], pq.heap[p] = pq.heap[p], pq.heap[i]

		// Update position map
		pKey := pq.keyExtractor(pq.heap[p])
		iKey := pq.keyExtractor(pq.heap[i])
		pq.positionMap[pKey] = p
		pq.positionMap[iKey] = i

		i = p
	}
}

// siftDown moves the item at index i down to maintain the heap property
func (pq *PriorityQueue[T, K]) siftDown(i int) {
	for {
		// Find the child with highest priority (min-heap: lowest value)
		minIndex := i
		first := pq.firstChild(i)
		last := first + pq.d

		// Check all children
		for j := first; j < last && j < len(pq.heap); j++ {
			if pq.compare(pq.heap[j], pq.heap[minIndex]) < 0 {
				minIndex = j
			}
		}

		// If the item is already in the right place
		if minIndex == i {
			break
		}

		// Swap with the child
		pq.heap[i], pq.heap[minIndex] = pq.heap[minIndex], pq.heap[i]

		// Update position map
		iKey := pq.keyExtractor(pq.heap[i])
		minKey := pq.keyExtractor(pq.heap[minIndex])
		pq.positionMap[iKey] = i
		pq.positionMap[minKey] = minIndex

		i = minIndex
	}
}

// ordered is a constraint that matches any ordered type
type ordered interface {
	~int | ~int8 | ~int16 | ~int32 | ~int64 |
		~uint | ~uint8 | ~uint16 | ~uint32 | ~uint64 | ~uintptr |
		~float32 | ~float64 |
		~string
}