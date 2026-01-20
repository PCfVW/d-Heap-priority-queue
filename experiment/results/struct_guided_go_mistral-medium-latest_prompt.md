Implement a d-ary heap priority queue in Go based on the following type definitions and signatures.

Requirements:
1. Items have two distinct properties: an identity (for equality) and a priority (for ordering)
2. Two items are equal if they have the same identity, regardless of priority
3. The queue should support O(1) lookup to check if an item exists
4. Implement a min-heap where lower priority values have higher importance

// Item represents an element in the priority queue.
// The ID field determines identity (equality).
// The Priority field determines ordering in the heap.
type Item[K comparable] struct {
    ID       K
    Priority int
}

// PriorityQueue is a d-ary min-heap with O(1) item lookup.
type PriorityQueue[K comparable] struct {
    d         int                // arity (children per node)
    container []Item[K]          // heap array
    positions map[K]int          // maps item ID to index in container
}

// New creates a new priority queue with the given arity d.
// Panics if d < 2.
func New[K comparable](d int) *PriorityQueue[K]

// Insert adds an item to the queue.
// Returns an error if an item with the same ID already exists.
func (pq *PriorityQueue[K]) Insert(item Item[K]) error

// Pop removes and returns the item with highest priority (lowest value).
// Returns the item and true, or zero value and false if empty.
func (pq *PriorityQueue[K]) Pop() (Item[K], bool)

// Front returns the item with highest priority without removing it.
// Returns the item and true, or zero value and false if empty.
func (pq *PriorityQueue[K]) Front() (Item[K], bool)

// IncreasePriority updates an item to have higher priority (lower value).
// The item parameter contains the ID to find and the new priority.
// Returns an error if the item doesn't exist.
func (pq *PriorityQueue[K]) IncreasePriority(item Item[K]) error

// DecreasePriority updates an item to have lower priority (higher value).
// The item parameter contains the ID to find and the new priority.
// Returns an error if the item doesn't exist.
func (pq *PriorityQueue[K]) DecreasePriority(item Item[K]) error

// Contains returns true if an item with the given ID exists.
func (pq *PriorityQueue[K]) Contains(item Item[K]) bool

// Len returns the number of items in the queue.
func (pq *PriorityQueue[K]) Len() int

// IsEmpty returns true if the queue contains no items.
func (pq *PriorityQueue[K]) IsEmpty() bool

Provide the complete implementation for all methods.