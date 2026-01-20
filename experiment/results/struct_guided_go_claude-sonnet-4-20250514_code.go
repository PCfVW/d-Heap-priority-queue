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
    d         int                // arity (children per node)
    container []Item[K]          // heap array
    positions map[K]int          // maps item ID to index in container
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

// parent returns the index of the parent node
func (pq *PriorityQueue[K]) parent(i int) int {
    return (i - 1) / pq.d
}

// firstChild returns the index of the first child node
func (pq *PriorityQueue[K]) firstChild(i int) int {
    return pq.d*i + 1
}

// swap exchanges two items in the heap and updates their positions
func (pq *PriorityQueue[K]) swap(i, j int) {
    pq.container[i], pq.container[j] = pq.container[j], pq.container[i]
    pq.positions[pq.container[i].ID] = i
    pq.positions[pq.container[j].ID] = j
}

// bubbleUp moves an item up the heap until heap property is satisfied
func (pq *PriorityQueue[K]) bubbleUp(i int) {
    for i > 0 {
        p := pq.parent(i)
        if pq.container[i].Priority >= pq.container[p].Priority {
            break
        }
        pq.swap(i, p)
        i = p
    }
}

// bubbleDown moves an item down the heap until heap property is satisfied
func (pq *PriorityQueue[K]) bubbleDown(i int) {
    for {
        minChild := -1
        firstChildIdx := pq.firstChild(i)
        
        // Find the child with minimum priority
        for j := 0; j < pq.d; j++ {
            childIdx := firstChildIdx + j
            if childIdx >= len(pq.container) {
                break
            }
            if minChild == -1 || pq.container[childIdx].Priority < pq.container[minChild].Priority {
                minChild = childIdx
            }
        }
        
        // If no child or current item has lower priority than min child, stop
        if minChild == -1 || pq.container[i].Priority <= pq.container[minChild].Priority {
            break
        }
        
        pq.swap(i, minChild)
        i = minChild
    }
}

// Insert adds an item to the queue.
// Returns an error if an item with the same ID already exists.
func (pq *PriorityQueue[K]) Insert(item Item[K]) error {
    if _, exists := pq.positions[item.ID]; exists {
        return errors.New("item with this ID already exists")
    }
    
    // Add item to the end of the heap
    pq.container = append(pq.container, item)
    idx := len(pq.container) - 1
    pq.positions[item.ID] = idx
    
    // Restore heap property
    pq.bubbleUp(idx)
    
    return nil
}

// Pop removes and returns the item with highest priority (lowest value).
// Returns the item and true, or zero value and false if empty.
func (pq *PriorityQueue[K]) Pop() (Item[K], bool) {
    if len(pq.container) == 0 {
        var zero Item[K]
        return zero, false
    }
    
    // Store the root item to return
    root := pq.container[0]
    delete(pq.positions, root.ID)
    
    // Move last item to root
    lastIdx := len(pq.container) - 1
    if lastIdx > 0 {
        pq.container[0] = pq.container[lastIdx]
        pq.positions[pq.container[0].ID] = 0
    }
    
    // Remove last item
    pq.container = pq.container[:lastIdx]
    
    // Restore heap property if there are still items
    if len(pq.container) > 0 {
        pq.bubbleDown(0)
    }
    
    return root, true
}

// Front returns the item with highest priority without removing it.
// Returns the item and true, or zero value and false if empty.
func (pq *PriorityQueue[K]) Front() (Item[K], bool) {
    if len(pq.container) == 0 {
        var zero Item[K]
        return zero, false
    }
    return pq.container[0], true
}

// IncreasePriority updates an item to have higher priority (lower value).
// The item parameter contains the ID to find and the new priority.
// Returns an error if the item doesn't exist.
func (pq *PriorityQueue[K]) IncreasePriority(item Item[K]) error {
    idx, exists := pq.positions[item.ID]
    if !exists {
        return errors.New("item not found")
    }
    
    pq.container[idx].Priority = item.Priority
    pq.bubbleUp(idx)
    
    return nil
}

// DecreasePriority updates an item to have lower priority (higher value).
// The item parameter contains the ID to find and the new priority.
// Returns an error if the item doesn't exist.
func (pq *PriorityQueue[K]) DecreasePriority(item Item[K]) error {
    idx, exists := pq.positions[item.ID]
    if !exists {
        return errors.New("item not found")
    }
    
    pq.container[idx].Priority = item.Priority
    pq.bubbleDown(idx)
    
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

// Example usage and testing
func main() {
    // Create a 3-ary heap
    pq := New[string](3)
    
    // Insert some items
    pq.Insert(Item[string]{ID: "task1", Priority: 5})
    pq.Insert(Item[string]{ID: "task2", Priority: 1})
    pq.Insert(Item[string]{ID: "task3", Priority: 3})
    pq.Insert(Item[string]{ID: "task4", Priority: 2})
    
    fmt.Printf("Queue length: %d\n", pq.Len())
    
    // Check front item
    if front, ok := pq.Front(); ok {
        fmt.Printf("Front item: %+v\n", front)
    }
    
    // Pop items in priority order
    for !pq.IsEmpty() {
        if item, ok := pq.Pop(); ok {
            fmt.Printf("Popped: %+v\n", item)
        }
    }
    
    // Test priority updates
    pq.Insert(Item[string]{ID: "A", Priority: 10})
    pq.Insert(Item[string]{ID: "B", Priority: 20})
    pq.Insert(Item[string]{ID: "C", Priority: 15})
    
    fmt.Println("\nBefore priority update:")
    if front, ok := pq.Front(); ok {
        fmt.Printf("Front: %+v\n", front)
    }
    
    // Increase priority of B (lower its value)
    pq.IncreasePriority(Item[string]{ID: "B", Priority: 5})
    
    fmt.Println("After increasing priority of B:")
    if front, ok := pq.Front(); ok {
        fmt.Printf("Front: %+v\n", front)
    }
    
    // Test Contains
    fmt.Printf("Contains A: %t\n", pq.Contains(Item[string]{ID: "A", Priority: 0}))
    fmt.Printf("Contains D: %t\n", pq.Contains(Item[string]{ID: "D", Priority: 0}))
}