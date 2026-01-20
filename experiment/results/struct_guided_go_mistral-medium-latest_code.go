package main

import (
	"errors"
	"fmt"
)

// Item represents an element in the priority queue.
// The ID field determines identity (equality).
// The Priority field determines ordering in the heap.
type Item[K comparable] struct {
	ID       K
	Priority int
}

// PriorityQueue is a d-ary min-heap with O(1) item lookup.
type PriorityQueue[K comparable] struct {
	d         int            // arity (children per node)
	container []Item[K]      // heap array
	positions map[K]int      // maps item ID to index in container
}

// New creates a new priority queue with the given arity d.
// Panics if d < 2.
func New[K comparable](d int) *PriorityQueue[K] {
	if d < 2 {
		panic("arity must be at least 2")
	}
	return &PriorityQueue[K]{
		d:         d,
		container: make([]Item[K], 0),
		positions: make(map[K]int),
	}
}

// Insert adds an item to the queue.
// Returns an error if an item with the same ID already exists.
func (pq *PriorityQueue[K]) Insert(item Item[K]) error {
	if _, exists := pq.positions[item.ID]; exists {
		return errors.New("item with this ID already exists")
	}

	// Add to the end of the container
	pq.container = append(pq.container, item)
	pq.positions[item.ID] = len(pq.container) - 1

	// Bubble up to maintain heap property
	pq.bubbleUp(len(pq.container) - 1)

	return nil
}

// Pop removes and returns the item with highest priority (lowest value).
// Returns the item and true, or zero value and false if empty.
func (pq *PriorityQueue[K]) Pop() (Item[K], bool) {
	if pq.IsEmpty() {
		var zero Item[K]
		return zero, false
	}

	// Get the root item
	root := pq.container[0]

	// Move last item to root
	last := pq.container[len(pq.container)-1]
	pq.container[0] = last
	pq.positions[last.ID] = 0

	// Remove last item
	pq.container = pq.container[:len(pq.container)-1]
	delete(pq.positions, root.ID)

	// Bubble down to maintain heap property
	if !pq.IsEmpty() {
		pq.bubbleDown(0)
	}

	return root, true
}

// Front returns the item with highest priority without removing it.
// Returns the item and true, or zero value and false if empty.
func (pq *PriorityQueue[K]) Front() (Item[K], bool) {
	if pq.IsEmpty() {
		var zero Item[K]
		return zero, false
	}
	return pq.container[0], true
}

// IncreasePriority updates an item to have higher priority (lower value).
// The item parameter contains the ID to find and the new priority.
// Returns an error if the item doesn't exist.
func (pq *PriorityQueue[K]) IncreasePriority(item Item[K]) error {
	return pq.updatePriority(item, true)
}

// DecreasePriority updates an item to have lower priority (higher value).
// The item parameter contains the ID to find and the new priority.
// Returns an error if the item doesn't exist.
func (pq *PriorityQueue[K]) DecreasePriority(item Item[K]) error {
	return pq.updatePriority(item, false)
}

// updatePriority handles both increasing and decreasing priority
func (pq *PriorityQueue[K]) updatePriority(item Item[K], isIncrease bool) error {
	index, exists := pq.positions[item.ID]
	if !exists {
		return errors.New("item not found")
	}

	oldPriority := pq.container[index].Priority
	pq.container[index].Priority = item.Priority

	if isIncrease {
		// If priority increased (value decreased), bubble up
		if item.Priority < oldPriority {
			pq.bubbleUp(index)
		} else {
			pq.bubbleDown(index)
		}
	} else {
		// If priority decreased (value increased), bubble down
		if item.Priority > oldPriority {
			pq.bubbleDown(index)
		} else {
			pq.bubbleUp(index)
		}
	}

	return nil
}

// Contains returns true if an item with the given ID exists.
func (pq *PriorityQueue[K]) Contains(item Item[K]) bool {
	_, exists := pq.positions[item.ID]
	return exists
}

// Len returns the number of items in the queue.
func (pq *PriorityQueue[K]) Len() int {
	return len(pq.container)
}

// IsEmpty returns true if the queue contains no items.
func (pq *PriorityQueue[K]) IsEmpty() bool {
	return len(pq.container) == 0
}

// bubbleUp moves an item up the heap until the heap property is restored
func (pq *PriorityQueue[K]) bubbleUp(index int) {
	for index > 0 {
		parent := (index - 1) / pq.d
		if pq.container[index].Priority >= pq.container[parent].Priority {
			break
		}
		pq.swap(index, parent)
		index = parent
	}
}

// bubbleDown moves an item down the heap until the heap property is restored
func (pq *PriorityQueue[K]) bubbleDown(index int) {
	for {
		child := pq.d*index + 1 // first child
		if child >= len(pq.container) {
			break
		}

		// Find the smallest child
		minChild := child
		for i := 1; i < pq.d; i++ {
			c := child + i
			if c >= len(pq.container) {
				break
			}
			if pq.container[c].Priority < pq.container[minChild].Priority {
				minChild = c
			}
		}

		// If current is smaller than smallest child, we're done
		if pq.container[index].Priority <= pq.container[minChild].Priority {
			break
		}

		// Otherwise, swap and continue
		pq.swap(index, minChild)
		index = minChild
	}
}

// swap exchanges two items in the container and updates their positions
func (pq *PriorityQueue[K]) swap(i, j int) {
	pq.container[i], pq.container[j] = pq.container[j], pq.container[i]
	pq.positions[pq.container[i].ID] = i
	pq.positions[pq.container[j].ID] = j
}