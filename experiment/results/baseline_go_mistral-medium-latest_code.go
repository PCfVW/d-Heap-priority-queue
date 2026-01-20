package main

import (
	"errors"
	"fmt"
)

// Item represents an item in the priority queue with identity and priority
type Item struct {
	Identity interface{} // Used for equality comparison
	Priority int         // Used for ordering
}

// DaryHeap implements a d-ary min-heap priority queue
type DaryHeap struct {
	d      int          // arity of the heap
	items  []*Item      // heap storage
	lookup map[interface{}]int // maps identity to index in items
}

// NewDaryHeap creates a new d-ary heap with the given arity
func NewDaryHeap(d int) *DaryHeap {
	if d < 2 {
		panic("heap arity must be at least 2")
	}
	return &DaryHeap{
		d:      d,
		items:  make([]*Item, 0),
		lookup: make(map[interface{}]int),
	}
}

// Len returns the number of items in the queue
func (h *DaryHeap) Len() int {
	return len(h.items)
}

// IsEmpty returns whether the queue is empty
func (h *DaryHeap) IsEmpty() bool {
	return len(h.items) == 0
}

// Contains checks if an item with the given identity exists
func (h *DaryHeap) Contains(item Item) bool {
	_, exists := h.lookup[item.Identity]
	return exists
}

// Insert adds an item to the queue
func (h *DaryHeap) Insert(item Item) {
	if h.Contains(item) {
		return // or could return an error
	}

	h.items = append(h.items, &item)
	h.lookup[item.Identity] = len(h.items) - 1
	h.heapifyUp(len(h.items) - 1)
}

// Front returns the item with highest priority without removing it
func (h *DaryHeap) Front() (*Item, error) {
	if h.IsEmpty() {
		return nil, errors.New("heap is empty")
	}
	return h.items[0], nil
}

// Pop removes and returns the item with highest priority (lowest value)
func (h *DaryHeap) Pop() (*Item, error) {
	if h.IsEmpty() {
		return nil, errors.New("heap is empty")
	}

	item := h.items[0]
	delete(h.lookup, item.Identity)

	// Move last item to root
	last := h.items[len(h.items)-1]
	h.items[0] = last
	h.lookup[last.Identity] = 0
	h.items = h.items[:len(h.items)-1]

	// Heapify down
	if !h.IsEmpty() {
		h.heapifyDown(0)
	}

	return item, nil
}

// IncreasePriority updates an existing item to have higher priority (lower value)
func (h *DaryHeap) IncreasePriority(item Item) error {
	index, exists := h.lookup[item.Identity]
	if !exists {
		return errors.New("item not found")
	}

	if item.Priority > h.items[index].Priority {
		return errors.New("new priority is not higher (lower value)")
	}

	h.items[index].Priority = item.Priority
	h.heapifyUp(index)
	return nil
}

// DecreasePriority updates an existing item to have lower priority (higher value)
func (h *DaryHeap) DecreasePriority(item Item) error {
	index, exists := h.lookup[item.Identity]
	if !exists {
		return errors.New("item not found")
	}

	if item.Priority < h.items[index].Priority {
		return errors.New("new priority is not lower (higher value)")
	}

	h.items[index].Priority = item.Priority
	h.heapifyDown(index)
	return nil
}

// heapifyUp moves an item up the heap until the heap property is restored
func (h *DaryHeap) heapifyUp(index int) {
	for index > 0 {
		parent := (index - 1) / h.d
		if h.items[index].Priority >= h.items[parent].Priority {
			break
		}
		h.swap(index, parent)
		index = parent
	}
}

// heapifyDown moves an item down the heap until the heap property is restored
func (h *DaryHeap) heapifyDown(index int) {
	for {
		smallest := index
		firstChild := index*h.d + 1

		// Find the smallest child
		for i := 0; i < h.d; i++ {
			child := firstChild + i
			if child >= len(h.items) {
				break
			}
			if h.items[child].Priority < h.items[smallest].Priority {
				smallest = child
			}
		}

		if smallest == index {
			break
		}

		h.swap(index, smallest)
		index = smallest
	}
}

// swap exchanges two items in the heap and updates their positions in the lookup map
func (h *DaryHeap) swap(i, j int) {
	h.items[i], h.items[j] = h.items[j], h.items[i]
	h.lookup[h.items[i].Identity] = i
	h.lookup[h.items[j].Identity] = j
}

// Example usage
func main() {
	heap := NewDaryHeap(3) // ternary heap

	// Insert items
	heap.Insert(Item{Identity: "task1", Priority: 5})
	heap.Insert(Item{Identity: "task2", Priority: 3})
	heap.Insert(Item{Identity: "task3", Priority: 7})
	heap.Insert(Item{Identity: "task4", Priority: 1})

	fmt.Println("Heap size:", heap.Len()) // 4

	// Check front
	front, _ := heap.Front()
	fmt.Println("Front item:", front.Identity, front.Priority) // task4 1

	// Pop items
	for !heap.IsEmpty() {
		item, _ := heap.Pop()
		fmt.Printf("Popped: %v (priority %d)\n", item.Identity, item.Priority)
	}
	// Output order: task4 (1), task2 (3), task1 (5), task3 (7)

	// Test priority updates
	heap.Insert(Item{Identity: "taskA", Priority: 10})
	heap.Insert(Item{Identity: "taskB", Priority: 20})

	heap.IncreasePriority(Item{Identity: "taskA", Priority: 5}) // higher priority (lower value)
	heap.DecreasePriority(Item{Identity: "taskB", Priority: 30}) // lower priority (higher value)

	front, _ = heap.Front()
	fmt.Println("New front:", front.Identity, front.Priority) // taskA 5
}