# D-ary Heap Priority Queue in Go

Here's a complete implementation of a d-ary heap priority queue in Go with all the required functionality:

```go
package main

import (
	"errors"
	"fmt"
)

// Item represents an item in the priority queue with identity and priority
type Item struct {
	Identity  interface{} // Used for equality comparison
	Priority  int         // Used for ordering (lower = higher priority)
}

// DaryHeap implements a d-ary heap priority queue
type DaryHeap struct {
	d          int         // arity of the heap
	heap       []*Item     // array-based heap storage
	position   map[interface{}]int // maps item identity to heap index
}

// NewDaryHeap creates a new d-ary heap with the given arity
func NewDaryHeap(d int) (*DaryHeap, error) {
	if d < 2 {
		return nil, errors.New("arity must be >= 2")
	}
	return &DaryHeap{
		d:        d,
		heap:     make([]*Item, 0),
		position: make(map[interface{}]int),
	}, nil
}

// Len returns the number of items in the heap
func (h *DaryHeap) Len() int {
	return len(h.heap)
}

// IsEmpty checks if the heap is empty
func (h *DaryHeap) IsEmpty() bool {
	return len(h.heap) == 0
}

// Contains checks if an item with the given identity exists in the heap
func (h *DaryHeap) Contains(item *Item) bool {
	_, exists := h.position[item.Identity]
	return exists
}

// Insert adds a new item to the heap
func (h *DaryHeap) Insert(item *Item) error {
	if h.Contains(item) {
		return errors.New("item with this identity already exists")
	}

	// Add to the end of the heap
	h.heap = append(h.heap, item)
	h.position[item.Identity] = len(h.heap) - 1

	// Sift up to maintain heap property
	h.siftUp(len(h.heap) - 1)
	return nil
}

// Front returns the highest priority item without removing it
func (h *DaryHeap) Front() (*Item, error) {
	if h.IsEmpty() {
		return nil, errors.New("heap is empty")
	}
	return h.heap[0], nil
}

// Pop removes and returns the highest priority item
func (h *DaryHeap) Pop() (*Item, error) {
	if h.IsEmpty() {
		return nil, errors.New("heap is empty")
	}

	// Get the root item
	root := h.heap[0]

	// Move last item to root
	last := h.heap[len(h.heap)-1]
	h.heap[0] = last
	h.position[last.Identity] = 0

	// Remove last item
	h.heap = h.heap[:len(h.heap)-1]
	delete(h.position, root.Identity)

	// Sift down to maintain heap property
	if !h.IsEmpty() {
		h.siftDown(0)
	}

	return root, nil
}

// IncreasePriority updates an item's priority to be higher (lower value)
func (h *DaryHeap) IncreasePriority(item *Item) error {
	index, exists := h.position[item.Identity]
	if !exists {
		return errors.New("item not found in heap")
	}

	// Only update if the new priority is actually higher (lower value)
	if item.Priority < h.heap[index].Priority {
		h.heap[index].Priority = item.Priority
		h.siftUp(index)
	}
	return nil
}

// DecreasePriority updates an item's priority to be lower (higher value)
func (h *DaryHeap) DecreasePriority(item *Item) error {
	index, exists := h.position[item.Identity]
	if !exists {
		return errors.New("item not found in heap")
	}

	// Only update if the new priority is actually lower (higher value)
	if item.Priority > h.heap[index].Priority {
		h.heap[index].Priority = item.Priority
		h.siftDown(index)
	}
	return nil
}

// siftUp moves an item up the heap to maintain the heap property
func (h *DaryHeap) siftUp(index int) {
	for index > 0 {
		parent := (index - 1) / h.d
		if h.heap[index].Priority >= h.heap[parent].Priority {
			break // heap property is satisfied
		}

		// Swap with parent
		h.swap(index, parent)
		index = parent
	}
}

// siftDown moves an item down the heap to maintain the heap property
func (h *DaryHeap) siftDown(index int) {
	for {
		// Find the child with the highest priority (lowest value)
		child := h.findMinChild(index)
		if child == -1 || h.heap[index].Priority <= h.heap[child].Priority {
			break // no children or heap property is satisfied
		}

		// Swap with the child
		h.swap(index, child)
		index = child
	}
}

// findMinChild finds the child with the highest priority (lowest value)
func (h *DaryHeap) findMinChild(index int) int {
	firstChild := h.d*index + 1
	lastChild := h.d*index + h.d

	if firstChild >= len(h.heap) {
		return -1 // no children
	}

	minChild := firstChild
	for i := firstChild + 1; i <= lastChild && i < len(h.heap); i++ {
		if h.heap[i].Priority < h.heap[minChild].Priority {
			minChild = i
		}
	}
	return minChild
}

// swap exchanges two items in the heap and updates their positions
func (h *DaryHeap) swap(i, j int) {
	h.heap[i], h.heap[j] = h.heap[j], h.heap[i]
	h.position[h.heap[i].Identity] = i
	h.position[h.heap[j].Identity] = j
}

// String returns a string representation of the heap
func (h *DaryHeap) String() string {
	return fmt.Sprintf("%v", h.heap)
}

// Example usage
func main() {
	heap, _ := NewDaryHeap(3) // ternary heap

	// Insert some items
	heap.Insert(&Item{Identity: "A", Priority: 5})
	heap.Insert(&Item{Identity: "B", Priority: 3})
	heap.Insert(&Item{Identity: "C", Priority: 7})
	heap.Insert(&Item{Identity: "D", Priority: 1})
	heap.Insert(&Item{Identity: "E", Priority: 4})

	fmt.Println("Heap after inserts:", heap)

	// Pop items
	for !heap.IsEmpty() {
		item, _ := heap.Pop()
		fmt.Printf("Popped: %v\n", item)
	}

	// Test priority updates
	heap.Insert(&Item{Identity: "X", Priority: 10})
	heap.Insert(&Item{Identity: "Y", Priority: 20})
	fmt.Println("\nHeap before priority updates:", heap)

	heap.IncreasePriority(&Item{Identity: "Y", Priority: 5})
	fmt.Println("After increasing Y's priority:", heap)

	heap.DecreasePriority(&Item{Identity: "X", Priority: 30})
	fmt.Println("After decreasing X's priority:", heap)
}
```

## Key Features

1. **D-ary Heap Structure**: Implements a heap where each node has up to d children (configurable at creation).

2. **Position Map**: Uses a hash map to track each item's position in the heap array for O(1) lookups.

3. **Complete API**:
   - `Insert`: Adds a new item to the heap
   - `Pop`: Removes and returns the highest priority item
   - `Front`: Peeks at the highest priority item
   - `IncreasePriority`: Updates an item to have higher priority
   - `DecreasePriority`: Updates an item to have lower priority
   - `Contains`: Checks if an item exists in the heap
   - `Len` and `IsEmpty`: Basic size operations

4. **Heap Property Maintenance**:
   - `siftUp`: Moves an item up the heap when its priority increases
   - `siftDown`: Moves an item down the heap when its priority decreases

5. **Error Handling**: Proper error handling for edge cases like empty heap operations.

The implementation maintains all the required time complexities:
- Insert: O(log_d n)
- Pop: O(d * log_d n)
- Front: O(1)
- Priority updates: O(log_d n) or O(d * log_d n) depending on direction
- Contains: O(1)