package dheap

import (
	"errors"
	"fmt"
)

// Common errors
var (
	ErrEmptyQueue   = errors.New("priority queue is empty")
	ErrItemNotFound = errors.New("item not found in priority queue")
)

// Comparator defines a comparison function for items
type Comparator[T any] func(a, b T) int

// Options configures the priority queue
type Options[T any, K comparable] struct {
	D            int
	Comparator   Comparator[T]
	KeyExtractor func(T) K
}

// PriorityQueue represents a d-ary heap priority queue
type PriorityQueue[T any, K comparable] struct {
	d            int
	items       []T
	keys        map[K]int // maps keys to indices in items slice
	comparator  Comparator[T]
	keyExtractor func(T) K
}

// New creates a new priority queue with the given options
func New[T any, K comparable](options Options[T, K]) *PriorityQueue[T, K] {
	if options.D < 2 {
		panic("d must be at least 2")
	}
	if options.Comparator == nil {
		panic("comparator must be provided")
	}
	if options.KeyExtractor == nil {
		panic("keyExtractor must be provided")
	}

	return &PriorityQueue[T, K]{
		d:            options.D,
		items:       make([]T, 0),
		keys:        make(map[K]int),
		comparator:  options.Comparator,
		keyExtractor: options.KeyExtractor,
	}
}

// D returns the arity of the heap
func (pq *PriorityQueue[T, K]) D() int {
	return pq.d
}

// Len returns the number of items in the queue
func (pq *PriorityQueue[T, K]) Len() int {
	return len(pq.items)
}

// IsEmpty checks if the queue is empty
func (pq *PriorityQueue[T, K]) IsEmpty() bool {
	return len(pq.items) == 0
}

// Contains checks if an item with the given identity exists
func (pq *PriorityQueue[T, K]) Contains(item T) bool {
	key := pq.keyExtractor(item)
	_, exists := pq.keys[key]
	return exists
}

// ContainsKey checks if an item with the given key exists
func (pq *PriorityQueue[T, K]) ContainsKey(key K) bool {
	_, exists := pq.keys[key]
	return exists
}

// Insert adds an item to the queue
func (pq *PriorityQueue[T, K]) Insert(item T) {
	key := pq.keyExtractor(item)
	if _, exists := pq.keys[key]; exists {
		// Item already exists, update its priority
		pq.updatePriority(item)
		return
	}

	// Add to the end and bubble up
	pq.items = append(pq.items, item)
	pq.keys[key] = len(pq.items) - 1
	pq.bubbleUp(len(pq.items) - 1)
}

// Front returns the item with highest priority without removing it
func (pq *PriorityQueue[T, K]) Front() (T, error) {
	if pq.IsEmpty() {
		var zero T
		return zero, ErrEmptyQueue
	}
	return pq.items[0], nil
}

// Pop removes and returns the item with highest priority
func (pq *PriorityQueue[T, K]) Pop() (T, bool) {
	if pq.IsEmpty() {
		var zero T
		return zero, false
	}

	// Save the front item
	front := pq.items[0]

	// Move last item to front
	lastIndex := len(pq.items) - 1
	pq.items[0] = pq.items[lastIndex]
	pq.keys[pq.keyExtractor(pq.items[0])] = 0

	// Remove last item
	pq.items = pq.items[:lastIndex]
	delete(pq.keys, pq.keyExtractor(front))

	// Bubble down the new front
	if !pq.IsEmpty() {
		pq.bubbleDown(0)
	}

	return front, true
}

// IncreasePriority updates an existing item to have higher priority (lower value in min-heap)
func (pq *PriorityQueue[T, K]) IncreasePriority(item T) error {
	return pq.updatePriority(item)
}

// DecreasePriority updates an existing item to have lower priority (higher value in min-heap)
func (pq *PriorityQueue[T, K]) DecreasePriority(item T) error {
	return pq.updatePriority(item)
}

// ToArray returns a copy of the underlying array (for testing)
func (pq *PriorityQueue[T, K]) ToArray() []T {
	arr := make([]T, len(pq.items))
	copy(arr, pq.items)
	return arr
}

// GetPosition returns the index of an item in the heap
func (pq *PriorityQueue[T, K]) GetPosition(item T) (int, error) {
	key := pq.keyExtractor(item)
	pos, exists := pq.keys[key]
	if !exists {
		return -1, ErrItemNotFound
	}
	return pos, nil
}

// GetPositionByKey returns the index of an item with the given key
func (pq *PriorityQueue[T, K]) GetPositionByKey(key K) (int, error) {
	pos, exists := pq.keys[key]
	if !exists {
		return -1, ErrItemNotFound
	}
	return pos, nil
}

// updatePriority handles both increasing and decreasing priority
func (pq *PriorityQueue[T, K]) updatePriority(newItem T) error {
	key := pq.keyExtractor(newItem)
	index, exists := pq.keys[key]
	if !exists {
		return ErrItemNotFound
	}

	oldItem := pq.items[index]
	comparison := pq.comparator(newItem, oldItem)

	// Update the item
	pq.items[index] = newItem

	if comparison < 0 {
		// New priority is higher (lower value in min-heap), bubble up
		pq.bubbleUp(index)
	} else if comparison > 0 {
		// New priority is lower (higher value in min-heap), bubble down
		pq.bubbleDown(index)
	}

	return nil
}

// bubbleUp moves an item up the heap until the heap property is restored
func (pq *PriorityQueue[T, K]) bubbleUp(index int) {
	for index > 0 {
		parent := (index - 1) / pq.d
		if pq.comparator(pq.items[index], pq.items[parent]) >= 0 {
			break // heap property is satisfied
		}
		// Swap with parent
		pq.swap(index, parent)
		index = parent
	}
}

// bubbleDown moves an item down the heap until the heap property is restored
func (pq *PriorityQueue[T, K]) bubbleDown(index int) {
	for {
		smallest := index
		firstChild := index*pq.d + 1

		// Check all children
		for i := 0; i < pq.d; i++ {
			child := firstChild + i
			if child < len(pq.items) && pq.comparator(pq.items[child], pq.items[smallest]) < 0 {
				smallest = child
			}
		}

		if smallest == index {
			break // heap property is satisfied
		}

		// Swap with smallest child
		pq.swap(index, smallest)
		index = smallest
	}
}

// swap exchanges two items in the heap
func (pq *PriorityQueue[T, K]) swap(i, j int) {
	pq.items[i], pq.items[j] = pq.items[j], pq.items[i]
	keyI := pq.keyExtractor(pq.items[i])
	keyJ := pq.keyExtractor(pq.items[j])
	pq.keys[keyI] = i
	pq.keys[keyJ] = j
}

// MinBy creates a comparator for min-heap based on a key function
func MinBy[T any, K ordered](keyFunc func(T) K) Comparator[T] {
	return func(a, b T) int {
		keyA := keyFunc(a)
		keyB := keyFunc(b)
		switch {
		case keyA < keyB:
			return -1
		case keyA > keyB:
			return 1
		default:
			return 0
		}
	}
}

// ordered is a constraint that matches any ordered type
type ordered interface {
	~int | ~int8 | ~int16 | ~int32 | ~int64 |
		~uint | ~uint8 | ~uint16 | ~uint32 | ~uint64 | ~uintptr |
		~float32 | ~float64 |
		~string
}