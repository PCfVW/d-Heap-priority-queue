Implement a d-ary heap priority queue in Go.

## Overview

A d-ary heap is a generalization of a binary heap where each node has up to d children instead of 2. This implementation requires O(1) item lookup via a position map (hash map tracking each item's index in the heap array).

## Core Concepts

### Identity vs Priority
- **Identity**: Determines equality between items. Used as the key in the position map.
- **Priority**: Determines ordering in the heap. Lower values = higher priority (min-heap).
- Two items with the same identity but different priorities are considered equal.

### Heap Structure
- Array-based complete tree representation
- For a node at index i:
  - Parent index: (i - 1) / d
  - First child index: d * i + 1
  - Last child index: d * i + d (if it exists)
- Root is at index 0

### Position Map
- Hash map from item identity to array index
- Enables O(1) contains() and O(1) lookup for priority updates
- Must be kept synchronized with heap array at all times

## API Documentation

### Constructor
Create a new priority queue with the specified arity.
- **Parameter**: d (arity) - number of children per node, must be >= 2
- **Returns**: New empty priority queue

### insert(item)
Add an item to the queue.
- **Precondition**: Item with same identity must not already exist
- **Postcondition**: Item is in the queue and findable via contains()
- **Postcondition**: Heap property is maintained
- **Postcondition**: Size increases by 1
- **Algorithm**: Add to end of array, then sift up to restore heap property
- **Time complexity**: O(log_d n)

### pop()
Remove and return the item with highest priority (lowest priority value).
- **Precondition**: Queue is not empty
- **Postcondition**: Returned item is no longer in the queue
- **Postcondition**: Heap property is maintained
- **Postcondition**: Size decreases by 1
- **Algorithm**: Swap root with last element, remove last, sift down from root
- **Time complexity**: O(d * log_d n)
- **Edge case**: Return null/None/error if queue is empty

### front()
Return the item with highest priority without removing it.
- **Precondition**: Queue is not empty
- **Postcondition**: Queue is unchanged (same size, same items)
- **Returns**: Item at root (index 0)
- **Time complexity**: O(1)
- **Edge case**: Return null/None/error if queue is empty

### increase_priority(item)
Update an existing item to have higher priority (lower priority value).
- **Precondition**: Item with same identity must exist in queue
- **Input**: Item with the identity to find and the new (lower) priority value
- **Postcondition**: Item's priority is updated to the new value
- **Postcondition**: Heap property is maintained (item may move up)
- **Postcondition**: Size is unchanged
- **Algorithm**: Update priority at current position, then sift up
- **Time complexity**: O(log_d n)
- **Note**: "Increase priority" means making it MORE important (lower value in min-heap)

### decrease_priority(item)
Update an existing item to have lower priority (higher priority value).
- **Precondition**: Item with same identity must exist in queue
- **Input**: Item with the identity to find and the new (higher) priority value
- **Postcondition**: Item's priority is updated to the new value
- **Postcondition**: Heap property is maintained (item may move down)
- **Postcondition**: Size is unchanged
- **Algorithm**: Update priority at current position, then sift down
- **Time complexity**: O(d * log_d n)
- **Note**: "Decrease priority" means making it LESS important (higher value in min-heap)

### contains(item)
Check if an item with the given identity exists in the queue.
- **Returns**: true if item with same identity exists, false otherwise
- **Note**: Compares by identity only, not priority
- **Time complexity**: O(1) via position map lookup

### len()
Return the number of items in the queue.
- **Returns**: Non-negative integer count
- **Time complexity**: O(1)

### is_empty()
Return whether the queue contains no items.
- **Returns**: true if len() == 0, false otherwise
- **Time complexity**: O(1)

## Sift Operations

### sift_up(index)
Restore heap property by moving an item up toward the root.
- Compare item at index with its parent
- If item has higher priority (lower value) than parent, swap them
- Repeat until item is at root or parent has higher/equal priority
- Update position map after each swap

### sift_down(index)
Restore heap property by moving an item down toward the leaves.
- Find the child with highest priority (lowest value) among all children
- If that child has higher priority than the item, swap them
- Repeat until item has no children or no child has higher priority
- Update position map after each swap

## Type Definitions

{TYPE_STUBS}

## Test Corpus

Your implementation must pass all of the following tests:

// Test corpus for insert() operation
// Spec: specifications/insert.md
// Part of: Amphigraphic-Strict × d-ary Heap Priority Queue research

package corpus

import (
	"testing"

	dheap "github.com/PCfVW/d-Heap-priority-queue/Go/v2/src"
)

// Test item type
type Item struct {
	ID       string
	Priority int
}

// Helper: create min-heap of Items
func newItemMinHeap(d int) *dheap.PriorityQueue[Item, string] {
	return dheap.New(dheap.Options[Item, string]{
		D:            d,
		Comparator:   dheap.MinBy(func(i Item) int { return i.Priority }),
		KeyExtractor: func(i Item) string { return i.ID },
	})
}

// Helper: verify heap invariant for min-heap
func verifyHeapInvariant(t *testing.T, pq *dheap.PriorityQueue[Item, string]) bool {
	t.Helper()
	arr := pq.ToArray()
	d := pq.D()
	for i := 0; i < len(arr); i++ {
		for j := 1; j <= d; j++ {
			childIdx := i*d + j
			if childIdx < len(arr) {
				if arr[i].Priority > arr[childIdx].Priority {
					t.Errorf("Heap invariant violated: parent[%d].Priority=%d > child[%d].Priority=%d",
						i, arr[i].Priority, childIdx, arr[childIdx].Priority)
					return false
				}
			}
		}
	}
	return true
}

// =============================================================================
// insert() Tests
// =============================================================================

// Test: insert_postcondition_item_findable
// Spec: specifications/insert.md
// Property: inserted item can be found via Contains() after insertion
func TestInsert_Postcondition_ItemFindable(t *testing.T) {
	pq := newItemMinHeap(4)

	item := Item{ID: "test-item", Priority: 50}
	pq.Insert(item)

	if !pq.Contains(item) {
		t.Error("inserted item should be findable via Contains()")
	}
	if !pq.ContainsKey("test-item") {
		t.Error("inserted item should be findable via ContainsKey()")
	}
}

// Test: insert_invariant_heap_property
// Spec: specifications/insert.md
// Property: heap invariant holds after insertion (parent priority <= children)
func TestInsert_Invariant_HeapProperty(t *testing.T) {
	pq := newItemMinHeap(4)

	// Insert items in arbitrary order
	items := []Item{
		{ID: "a", Priority: 30},
		{ID: "b", Priority: 10},
		{ID: "c", Priority: 50},
		{ID: "d", Priority: 20},
		{ID: "e", Priority: 40},
	}

	for _, item := range items {
		pq.Insert(item)
		if !verifyHeapInvariant(t, pq) {
			t.Fatalf("heap invariant violated after inserting %v", item)
		}
	}
}

// Test: insert_size_increments
// Spec: specifications/insert.md
// Property: heap size increases by 1 after each insertion
func TestInsert_Size_Increments(t *testing.T) {
	pq := newItemMinHeap(4)

	for i := 0; i < 5; i++ {
		sizeBefore := pq.Len()
		pq.Insert(Item{ID: string(rune('a' + i)), Priority: i * 10})
		sizeAfter := pq.Len()

		if sizeAfter != sizeBefore+1 {
			t.Errorf("expected size %d after insert, got %d", sizeBefore+1, sizeAfter)
		}
	}
}

// Test: insert_edge_becomes_front_if_highest_priority
// Spec: specifications/insert.md
// Property: if inserted item has highest priority, it becomes front()
func TestInsert_Edge_BecomesFrontIfHighestPriority(t *testing.T) {
	pq := newItemMinHeap(4)

	// Insert items with decreasing priority values (increasing importance in min-heap)
	pq.Insert(Item{ID: "low", Priority: 100})
	pq.Insert(Item{ID: "medium", Priority: 50})
	pq.Insert(Item{ID: "high", Priority: 10})

	front, err := pq.Front()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if front.ID != "high" {
		t.Errorf("expected 'high' at front (priority 10), got '%s' (priority %d)", front.ID, front.Priority)
	}

	// Insert new highest priority item
	pq.Insert(Item{ID: "urgent", Priority: 1})

	front, err = pq.Front()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if front.ID != "urgent" {
		t.Errorf("expected 'urgent' at front after insert, got '%s'", front.ID)
	}
}


// --- pop_test.go ---

// Test corpus for pop() operation
// Spec: specifications/pop.md
// Part of: Amphigraphic-Strict × d-ary Heap Priority Queue research

package corpus

import (
	"testing"
)

// =============================================================================
// pop() Tests
// =============================================================================

// Test: pop_postcondition_returns_minimum
// Spec: specifications/pop.md
// Property: pop() returns the item with lowest priority value (min-heap)
func TestPop_Postcondition_ReturnsMinimum(t *testing.T) {
	pq := newItemMinHeap(4)

	pq.Insert(Item{ID: "a", Priority: 30})
	pq.Insert(Item{ID: "b", Priority: 10})
	pq.Insert(Item{ID: "c", Priority: 20})

	result, ok := pq.Pop()
	if !ok {
		t.Fatal("pop() should return ok=true on non-empty heap")
	}

	if result.Priority != 10 {
		t.Errorf("expected priority 10, got %d", result.Priority)
	}
	if result.ID != "b" {
		t.Errorf("expected item 'b', got '%s'", result.ID)
	}
}

// Test: pop_invariant_maintains_heap_property
// Spec: specifications/pop.md
// Property: after pop(), heap invariant holds for all remaining elements
func TestPop_Invariant_MaintainsHeapProperty(t *testing.T) {
	pq := newItemMinHeap(4)

	// Insert multiple items
	items := []Item{
		{ID: "a", Priority: 50},
		{ID: "b", Priority: 20},
		{ID: "c", Priority: 80},
		{ID: "d", Priority: 10},
		{ID: "e", Priority: 60},
		{ID: "f", Priority: 30},
		{ID: "g", Priority: 70},
		{ID: "h", Priority: 40},
	}

	for _, item := range items {
		pq.Insert(item)
	}

	// Pop half and verify invariant after each pop
	for i := 0; i < 4; i++ {
		pq.Pop()
		if !verifyHeapInvariant(t, pq) {
			t.Fatalf("heap invariant violated after pop #%d", i+1)
		}
	}
}

// Test: pop_size_decrements
// Spec: specifications/pop.md
// Property: size() decreases by 1 after successful pop()
func TestPop_Size_Decrements(t *testing.T) {
	pq := newItemMinHeap(4)

	pq.Insert(Item{ID: "a", Priority: 10})
	pq.Insert(Item{ID: "b", Priority: 20})
	pq.Insert(Item{ID: "c", Priority: 30})

	for expectedSize := 2; expectedSize >= 0; expectedSize-- {
		sizeBefore := pq.Len()
		pq.Pop()
		sizeAfter := pq.Len()

		if sizeAfter != sizeBefore-1 {
			t.Errorf("expected size %d after pop, got %d", sizeBefore-1, sizeAfter)
		}
	}
}

// Test: pop_edge_empty_returns_false
// Spec: specifications/pop.md
// Property: pop() on empty heap returns (zero, false), not panic
func TestPop_Edge_EmptyReturnsFalse(t *testing.T) {
	pq := newItemMinHeap(4)

	_, ok := pq.Pop()
	if ok {
		t.Error("pop() on empty heap should return ok=false")
	}

	// Verify no panic occurred and heap is still valid
	if pq.Len() != 0 {
		t.Error("heap should remain empty after failed pop")
	}
}


// --- front_test.go ---

// Test corpus for front() operation
// Spec: specifications/front.md
// Part of: Amphigraphic-Strict × d-ary Heap Priority Queue research

package corpus

import (
	"testing"

	dheap "github.com/PCfVW/d-Heap-priority-queue/Go/v2/src"
)

// =============================================================================
// front() Tests
// =============================================================================

// Test: front_postcondition_returns_minimum
// Spec: specifications/front.md
// Property: front() returns the item with lowest priority value (min-heap) without removal
func TestFront_Postcondition_ReturnsMinimum(t *testing.T) {
	pq := newItemMinHeap(4)

	pq.Insert(Item{ID: "a", Priority: 30})
	pq.Insert(Item{ID: "b", Priority: 10})
	pq.Insert(Item{ID: "c", Priority: 20})

	result, err := pq.Front()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if result.Priority != 10 {
		t.Errorf("expected priority 10, got %d", result.Priority)
	}
	if result.ID != "b" {
		t.Errorf("expected item 'b', got '%s'", result.ID)
	}
}

// Test: front_invariant_no_modification
// Spec: specifications/front.md
// Property: front() does not modify the heap (calling multiple times returns same result)
func TestFront_Invariant_NoModification(t *testing.T) {
	pq := newItemMinHeap(4)

	pq.Insert(Item{ID: "a", Priority: 30})
	pq.Insert(Item{ID: "b", Priority: 10})
	pq.Insert(Item{ID: "c", Priority: 20})

	// Call front() multiple times
	first, _ := pq.Front()
	second, _ := pq.Front()
	third, _ := pq.Front()

	// All should return the same item
	if first.ID != second.ID || second.ID != third.ID {
		t.Errorf("front() returned different items: %s, %s, %s", first.ID, second.ID, third.ID)
	}

	// Heap should still be intact
	if !verifyHeapInvariant(t, pq) {
		t.Error("heap invariant violated after multiple front() calls")
	}
}

// Test: front_size_unchanged
// Spec: specifications/front.md
// Property: size() remains the same after front()
func TestFront_Size_Unchanged(t *testing.T) {
	pq := newItemMinHeap(4)

	pq.Insert(Item{ID: "a", Priority: 10})
	pq.Insert(Item{ID: "b", Priority: 20})
	pq.Insert(Item{ID: "c", Priority: 30})

	sizeBefore := pq.Len()

	// Call front() multiple times
	for i := 0; i < 5; i++ {
		pq.Front()
	}

	sizeAfter := pq.Len()

	if sizeAfter != sizeBefore {
		t.Errorf("expected size %d unchanged, got %d", sizeBefore, sizeAfter)
	}
}

// Test: front_edge_empty_returns_error
// Spec: specifications/front.md
// Property: front() on empty heap returns error, not panic
func TestFront_Edge_EmptyReturnsError(t *testing.T) {
	pq := newItemMinHeap(4)

	_, err := pq.Front()
	if err != dheap.ErrEmptyQueue {
		t.Errorf("expected ErrEmptyQueue, got %v", err)
	}
}


// --- increase_priority_test.go ---

// Test corpus for increase_priority() operation
// Spec: specifications/increase_priority.md
// Part of: Amphigraphic-Strict × d-ary Heap Priority Queue research

package corpus

import (
	"testing"

	dheap "github.com/PCfVW/d-Heap-priority-queue/Go/v2/src"
)

// =============================================================================
// increase_priority() Tests
// =============================================================================

// Test: increase_priority_postcondition_priority_changed
// Spec: specifications/increase_priority.md
// Property: item's priority is updated to the new value
func TestIncreasePriority_Postcondition_PriorityChanged(t *testing.T) {
	pq := newItemMinHeap(4)

	pq.Insert(Item{ID: "target", Priority: 50})
	pq.Insert(Item{ID: "other", Priority: 30})

	// Increase priority of "target" (in min-heap: lower value = higher priority)
	err := pq.IncreasePriority(Item{ID: "target", Priority: 10})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify "target" is now at front (highest priority)
	front, _ := pq.Front()
	if front.ID != "target" {
		t.Errorf("expected 'target' at front after priority increase, got '%s'", front.ID)
	}
	if front.Priority != 10 {
		t.Errorf("expected priority 10, got %d", front.Priority)
	}
}

// Test: increase_priority_invariant_heap_property
// Spec: specifications/increase_priority.md
// Property: heap invariant holds after priority increase
func TestIncreasePriority_Invariant_HeapProperty(t *testing.T) {
	pq := newItemMinHeap(4)

	// Create a larger heap
	items := []Item{
		{ID: "a", Priority: 80},
		{ID: "b", Priority: 60},
		{ID: "c", Priority: 40},
		{ID: "d", Priority: 20},
		{ID: "e", Priority: 100},
		{ID: "f", Priority: 50},
	}

	for _, item := range items {
		pq.Insert(item)
	}

	// Increase priority of items from the back
	err := pq.IncreasePriority(Item{ID: "a", Priority: 5})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if !verifyHeapInvariant(t, pq) {
		t.Error("heap invariant violated after increase_priority")
	}
}

// Test: increase_priority_position_item_moves_up
// Spec: specifications/increase_priority.md
// Property: item moves toward root after priority increase (bubbles up)
func TestIncreasePriority_Position_ItemMovesUp(t *testing.T) {
	pq := newItemMinHeap(4)

	pq.Insert(Item{ID: "root", Priority: 10})
	pq.Insert(Item{ID: "middle", Priority: 50})
	pq.Insert(Item{ID: "leaf", Priority: 100})

	// Get position before
	posBefore, _ := pq.GetPosition(Item{ID: "leaf", Priority: 100})

	// Increase priority of "leaf" to become highest priority
	err := pq.IncreasePriority(Item{ID: "leaf", Priority: 1})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Get position after
	posAfter, _ := pq.GetPositionByKey("leaf")

	// Position should have decreased (moved toward root, which is position 0)
	if posAfter >= posBefore {
		t.Errorf("expected item to move up (position decrease), but position went from %d to %d", posBefore, posAfter)
	}

	// Should now be at root
	if posAfter != 0 {
		t.Errorf("expected item at root (position 0), got position %d", posAfter)
	}
}

// Test: increase_priority_size_unchanged
// Spec: specifications/increase_priority.md
// Property: size() remains unchanged after priority update
func TestIncreasePriority_Size_Unchanged(t *testing.T) {
	pq := newItemMinHeap(4)

	pq.Insert(Item{ID: "a", Priority: 50})
	pq.Insert(Item{ID: "b", Priority: 30})
	pq.Insert(Item{ID: "c", Priority: 70})

	sizeBefore := pq.Len()

	err := pq.IncreasePriority(Item{ID: "c", Priority: 10})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	sizeAfter := pq.Len()

	if sizeAfter != sizeBefore {
		t.Errorf("expected size %d unchanged, got %d", sizeBefore, sizeAfter)
	}
}

// Test: increase_priority_edge_not_found_returns_error
// Spec: specifications/increase_priority.md
// Property: returns error if item not in heap
func TestIncreasePriority_Edge_NotFoundReturnsError(t *testing.T) {
	pq := newItemMinHeap(4)

	pq.Insert(Item{ID: "existing", Priority: 50})

	err := pq.IncreasePriority(Item{ID: "nonexistent", Priority: 10})
	if err != dheap.ErrItemNotFound {
		t.Errorf("expected ErrItemNotFound, got %v", err)
	}
}


// --- decrease_priority_test.go ---

// Test corpus for decrease_priority() operation
// Spec: specifications/decrease_priority.md
// Part of: Amphigraphic-Strict × d-ary Heap Priority Queue research

package corpus

import (
	"testing"

	dheap "github.com/PCfVW/d-Heap-priority-queue/Go/v2/src"
)

// =============================================================================
// decrease_priority() Tests
// =============================================================================

// Test: decrease_priority_postcondition_priority_changed
// Spec: specifications/decrease_priority.md
// Property: item's priority is updated to the new value
func TestDecreasePriority_Postcondition_PriorityChanged(t *testing.T) {
	pq := newItemMinHeap(4)

	pq.Insert(Item{ID: "target", Priority: 10})
	pq.Insert(Item{ID: "other", Priority: 30})

	// "target" starts at front
	front, _ := pq.Front()
	if front.ID != "target" {
		t.Fatalf("expected 'target' at front initially, got '%s'", front.ID)
	}

	// Decrease priority of "target" (in min-heap: higher value = lower priority)
	err := pq.DecreasePriority(Item{ID: "target", Priority: 50})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify "other" is now at front (it has higher priority now)
	front, _ = pq.Front()
	if front.ID != "other" {
		t.Errorf("expected 'other' at front after priority decrease, got '%s'", front.ID)
	}

	// Verify "target" priority was actually changed
	// Pop "other" first, then check "target"
	pq.Pop()
	remaining, _ := pq.Front()
	if remaining.Priority != 50 {
		t.Errorf("expected 'target' priority to be 50, got %d", remaining.Priority)
	}
}

// Test: decrease_priority_invariant_heap_property
// Spec: specifications/decrease_priority.md
// Property: heap invariant holds after priority decrease
func TestDecreasePriority_Invariant_HeapProperty(t *testing.T) {
	pq := newItemMinHeap(4)

	// Create a larger heap
	items := []Item{
		{ID: "a", Priority: 10},
		{ID: "b", Priority: 30},
		{ID: "c", Priority: 50},
		{ID: "d", Priority: 70},
		{ID: "e", Priority: 20},
		{ID: "f", Priority: 40},
	}

	for _, item := range items {
		pq.Insert(item)
	}

	// Decrease priority of root item
	err := pq.DecreasePriority(Item{ID: "a", Priority: 100})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if !verifyHeapInvariant(t, pq) {
		t.Error("heap invariant violated after decrease_priority")
	}
}

// Test: decrease_priority_position_item_moves_down
// Spec: specifications/decrease_priority.md
// Property: item moves toward leaves after priority decrease (sinks down)
func TestDecreasePriority_Position_ItemMovesDown(t *testing.T) {
	pq := newItemMinHeap(4)

	pq.Insert(Item{ID: "root", Priority: 10})
	pq.Insert(Item{ID: "child1", Priority: 50})
	pq.Insert(Item{ID: "child2", Priority: 60})
	pq.Insert(Item{ID: "child3", Priority: 70})

	// Verify "root" is at position 0
	posBefore, _ := pq.GetPositionByKey("root")
	if posBefore != 0 {
		t.Fatalf("expected 'root' at position 0, got %d", posBefore)
	}

	// Decrease priority of "root" to become lowest priority
	err := pq.DecreasePriority(Item{ID: "root", Priority: 100})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Get position after
	posAfter, _ := pq.GetPositionByKey("root")

	// Position should have increased (moved toward leaves, away from position 0)
	if posAfter <= posBefore {
		t.Errorf("expected item to move down (position increase), but position went from %d to %d", posBefore, posAfter)
	}
}

// Test: decrease_priority_size_unchanged
// Spec: specifications/decrease_priority.md
// Property: size() remains unchanged after priority update
func TestDecreasePriority_Size_Unchanged(t *testing.T) {
	pq := newItemMinHeap(4)

	pq.Insert(Item{ID: "a", Priority: 10})
	pq.Insert(Item{ID: "b", Priority: 30})
	pq.Insert(Item{ID: "c", Priority: 50})

	sizeBefore := pq.Len()

	err := pq.DecreasePriority(Item{ID: "a", Priority: 100})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	sizeAfter := pq.Len()

	if sizeAfter != sizeBefore {
		t.Errorf("expected size %d unchanged, got %d", sizeBefore, sizeAfter)
	}
}

// Test: decrease_priority_edge_not_found_returns_error
// Spec: specifications/decrease_priority.md
// Property: returns error if item not in heap
func TestDecreasePriority_Edge_NotFoundReturnsError(t *testing.T) {
	pq := newItemMinHeap(4)

	pq.Insert(Item{ID: "existing", Priority: 50})

	err := pq.DecreasePriority(Item{ID: "nonexistent", Priority: 100})
	if err != dheap.ErrItemNotFound {
		t.Errorf("expected ErrItemNotFound, got %v", err)
	}
}


Provide a complete, working implementation that satisfies the documentation, matches the type signatures, and passes all tests.