package main

import (
    "fmt"
    "errors"
)

// Item represents an element in the priority queue
type Item struct {
    Identity interface{} // Unique identifier for the item
    Priority int         // Priority value (lower = higher priority)
}

// String returns a string representation of the item
func (i Item) String() string {
    return fmt.Sprintf("{ID: %v, Priority: %d}", i.Identity, i.Priority)
}

// DaryHeap represents a d-ary min-heap priority queue
type DaryHeap struct {
    d        int              // Arity of the heap
    heap     []Item           // The heap array
    indexMap map[interface{}]int // Maps item identity to heap index for O(1) lookup
}

// NewDaryHeap creates a new d-ary heap with the specified arity
func NewDaryHeap(d int) *DaryHeap {
    if d < 2 {
        panic("heap arity must be at least 2")
    }
    return &DaryHeap{
        d:        d,
        heap:     make([]Item, 0),
        indexMap: make(map[interface{}]int),
    }
}

// Len returns the number of items in the queue
func (h *DaryHeap) Len() int {
    return len(h.heap)
}

// IsEmpty returns whether the queue is empty
func (h *DaryHeap) IsEmpty() bool {
    return len(h.heap) == 0
}

// Contains checks if an item with the given identity exists
func (h *DaryHeap) Contains(identity interface{}) bool {
    _, exists := h.indexMap[identity]
    return exists
}

// parent returns the parent index of the given index
func (h *DaryHeap) parent(i int) int {
    return (i - 1) / h.d
}

// firstChild returns the index of the first child of the given index
func (h *DaryHeap) firstChild(i int) int {
    return h.d*i + 1
}

// swap swaps two elements in the heap and updates the index map
func (h *DaryHeap) swap(i, j int) {
    h.heap[i], h.heap[j] = h.heap[j], h.heap[i]
    h.indexMap[h.heap[i].Identity] = i
    h.indexMap[h.heap[j].Identity] = j
}

// bubbleUp moves an item up the heap until heap property is satisfied
func (h *DaryHeap) bubbleUp(index int) {
    for index > 0 {
        parentIndex := h.parent(index)
        if h.heap[index].Priority >= h.heap[parentIndex].Priority {
            break
        }
        h.swap(index, parentIndex)
        index = parentIndex
    }
}

// bubbleDown moves an item down the heap until heap property is satisfied
func (h *DaryHeap) bubbleDown(index int) {
    for {
        minIndex := index
        firstChildIndex := h.firstChild(index)
        
        // Find the child with minimum priority
        for i := 0; i < h.d; i++ {
            childIndex := firstChildIndex + i
            if childIndex >= len(h.heap) {
                break
            }
            if h.heap[childIndex].Priority < h.heap[minIndex].Priority {
                minIndex = childIndex
            }
        }
        
        // If no child has smaller priority, we're done
        if minIndex == index {
            break
        }
        
        h.swap(index, minIndex)
        index = minIndex
    }
}

// Insert adds an item to the queue
func (h *DaryHeap) Insert(item Item) error {
    // Check if item already exists
    if h.Contains(item.Identity) {
        return errors.New("item with this identity already exists")
    }
    
    // Add item to the end of heap
    h.heap = append(h.heap, item)
    index := len(h.heap) - 1
    h.indexMap[item.Identity] = index
    
    // Bubble up to maintain heap property
    h.bubbleUp(index)
    
    return nil
}

// Pop removes and returns the item with highest priority (lowest value)
func (h *DaryHeap) Pop() (Item, error) {
    if h.IsEmpty() {
        return Item{}, errors.New("heap is empty")
    }
    
    // Store the root item to return
    root := h.heap[0]
    delete(h.indexMap, root.Identity)
    
    // Move last item to root
    lastIndex := len(h.heap) - 1
    if lastIndex > 0 {
        h.heap[0] = h.heap[lastIndex]
        h.indexMap[h.heap[0].Identity] = 0
    }
    
    // Remove last element
    h.heap = h.heap[:lastIndex]
    
    // Bubble down if heap is not empty
    if len(h.heap) > 0 {
        h.bubbleDown(0)
    }
    
    return root, nil
}

// Front returns the item with highest priority without removing it
func (h *DaryHeap) Front() (Item, error) {
    if h.IsEmpty() {
        return Item{}, errors.New("heap is empty")
    }
    return h.heap[0], nil
}

// IncreasePriority updates an existing item to have higher priority (lower value)
func (h *DaryHeap) IncreasePriority(identity interface{}, newPriority int) error {
    index, exists := h.indexMap[identity]
    if !exists {
        return errors.New("item not found")
    }
    
    if newPriority >= h.heap[index].Priority {
        return errors.New("new priority must be lower than current priority")
    }
    
    h.heap[index].Priority = newPriority
    h.bubbleUp(index)
    
    return nil
}

// DecreasePriority updates an existing item to have lower priority (higher value)
func (h *DaryHeap) DecreasePriority(identity interface{}, newPriority int) error {
    index, exists := h.indexMap[identity]
    if !exists {
        return errors.New("item not found")
    }
    
    if newPriority <= h.heap[index].Priority {
        return errors.New("new priority must be higher than current priority")
    }
    
    h.heap[index].Priority = newPriority
    h.bubbleDown(index)
    
    return nil
}

// GetPriority returns the current priority of an item
func (h *DaryHeap) GetPriority(identity interface{}) (int, error) {
    index, exists := h.indexMap[identity]
    if !exists {
        return 0, errors.New("item not found")
    }
    return h.heap[index].Priority, nil
}

// String returns a string representation of the heap
func (h *DaryHeap) String() string {
    return fmt.Sprintf("DaryHeap(d=%d, len=%d): %v", h.d, len(h.heap), h.heap)
}

// Example usage and testing
func main() {
    // Create a 3-ary heap
    heap := NewDaryHeap(3)
    
    fmt.Println("=== D-ary Heap Priority Queue Demo ===")
    fmt.Printf("Created %d-ary heap\n\n", 3)
    
    // Test insertions
    items := []Item{
        {"task1", 5},
        {"task2", 1},
        {"task3", 8},
        {"task4", 3},
        {"task5", 2},
        {"task6", 7},
    }
    
    fmt.Println("Inserting items:")
    for _, item := range items {
        err := heap.Insert(item)
        if err != nil {
            fmt.Printf("Error inserting %v: %v\n", item, err)
        } else {
            fmt.Printf("Inserted: %v\n", item)
        }
    }
    
    fmt.Printf("\nHeap state: %v\n", heap)
    fmt.Printf("Length: %d, IsEmpty: %v\n\n", heap.Len(), heap.IsEmpty())
    
    // Test Front
    if front, err := heap.Front(); err == nil {
        fmt.Printf("Front item: %v\n\n", front)
    }
    
    // Test Contains
    fmt.Println("Testing Contains:")
    fmt.Printf("Contains 'task2': %v\n", heap.Contains("task2"))
    fmt.Printf("Contains 'task99': %v\n\n", heap.Contains("task99"))
    
    // Test priority updates
    fmt.Println("Testing priority updates:")
    if err := heap.IncreasePriority("task3", 0); err == nil {
        fmt.Println("Increased priority of task3 to 0")
        fmt.Printf("Heap state: %v\n", heap)
    }
    
    if err := heap.DecreasePriority("task2", 6); err == nil {
        fmt.Println("Decreased priority of task2 to 6")
        fmt.Printf("Heap state: %v\n\n", heap)
    }
    
    // Test popping all items
    fmt.Println("Popping all items in priority order:")
    for !heap.IsEmpty() {
        if item, err := heap.Pop(); err == nil {
            fmt.Printf("Popped: %v\n", item)
        }
    }
    
    fmt.Printf("\nFinal length: %d, IsEmpty: %v\n", heap.Len(), heap.IsEmpty())
    
    // Test error cases
    fmt.Println("\n=== Testing Error Cases ===")
    
    // Pop from empty heap
    if _, err := heap.Pop(); err != nil {
        fmt.Printf("Pop from empty heap: %v\n", err)
    }
    
    // Front from empty heap
    if _, err := heap.Front(); err != nil {
        fmt.Printf("Front from empty heap: %v\n", err)
    }
    
    // Insert duplicate
    heap.Insert(Item{"test", 1})
    if err := heap.Insert(Item{"test", 2}); err != nil {
        fmt.Printf("Insert duplicate: %v\n", err)
    }
    
    // Update non-existent item
    if err := heap.IncreasePriority("nonexistent", 0); err != nil {
        fmt.Printf("Update non-existent item: %v\n", err)
    }
    
    // Invalid priority update
    if err := heap.IncreasePriority("test", 5); err != nil {
        fmt.Printf("Invalid priority increase: %v\n", err)
    }
}