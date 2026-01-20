Implement a d-ary heap priority queue in TypeScript.

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

import { describe, it, expect } from 'vitest';
import { PriorityQueue } from 'd-ary-heap';

// Test item type
interface Item {
  id: string;
  priority: number;
}

// Helper: create min-heap of Items
function newItemMinHeap(d: number): PriorityQueue<Item, string> {
  return new PriorityQueue<Item, string>({
    d,
    comparator: (a, b) => a.priority < b.priority,
    keyExtractor: (item) => item.id,
  });
}

// =============================================================================
// insert() Tests
// =============================================================================

describe('insert()', () => {
  // Test: insert_postcondition_item_findable
  // Spec: specifications/insert.md
  // Property: inserted item can be found via contains() after insertion
  it('postcondition: item is findable after insertion', () => {
    const pq = newItemMinHeap(4);

    const item: Item = { id: 'test-item', priority: 50 };
    pq.insert(item);

    expect(pq.contains(item)).toBe(true);
    expect(pq.containsKey('test-item')).toBe(true);
  });

  // Test: insert_invariant_heap_property
  // Spec: specifications/insert.md
  // Property: heap invariant holds after insertion (front() returns minimum)
  it('invariant: heap property maintained after insertion', () => {
    const pq = newItemMinHeap(4);

    const items: Item[] = [
      { id: 'a', priority: 30 },
      { id: 'b', priority: 10 },
      { id: 'c', priority: 50 },
      { id: 'd', priority: 20 },
      { id: 'e', priority: 40 },
    ];

    for (const item of items) {
      pq.insert(item);
      expect(pq.front().priority).toBeLessThanOrEqual(30);
    }

    expect(pq.front().priority).toBe(10);
  });

  // Test: insert_size_increments
  // Spec: specifications/insert.md
  // Property: heap size increases by 1 after each insertion
  it('size: increments by 1 after each insertion', () => {
    const pq = newItemMinHeap(4);

    for (let i = 0; i < 5; i++) {
      const sizeBefore = pq.len();
      pq.insert({ id: `item${i}`, priority: i * 10 });
      expect(pq.len()).toBe(sizeBefore + 1);
    }
  });

  // Test: insert_edge_becomes_front_if_highest_priority
  // Spec: specifications/insert.md
  // Property: if inserted item has highest priority, it becomes front()
  it('edge: item becomes front if it has highest priority', () => {
    const pq = newItemMinHeap(4);

    pq.insert({ id: 'low', priority: 100 });
    pq.insert({ id: 'medium', priority: 50 });
    pq.insert({ id: 'high', priority: 10 });

    expect(pq.front().id).toBe('high');

    pq.insert({ id: 'urgent', priority: 1 });

    expect(pq.front().id).toBe('urgent');
  });
});


// --- pop.test.ts ---

// Test corpus for pop() operation
// Spec: specifications/pop.md
// Part of: Amphigraphic-Strict × d-ary Heap Priority Queue research

import { describe, it, expect } from 'vitest';
import { PriorityQueue } from 'd-ary-heap';

interface Item {
  id: string;
  priority: number;
}

function newItemMinHeap(d: number): PriorityQueue<Item, string> {
  return new PriorityQueue<Item, string>({
    d,
    comparator: (a, b) => a.priority < b.priority,
    keyExtractor: (item) => item.id,
  });
}

// =============================================================================
// pop() Tests
// =============================================================================

describe('pop()', () => {
  // Test: pop_postcondition_returns_minimum
  // Spec: specifications/pop.md
  // Property: pop() returns the item with lowest priority value (min-heap)
  it('postcondition: returns minimum priority item', () => {
    const pq = newItemMinHeap(4);

    pq.insert({ id: 'a', priority: 30 });
    pq.insert({ id: 'b', priority: 10 });
    pq.insert({ id: 'c', priority: 20 });

    const result = pq.pop();

    expect(result.priority).toBe(10);
    expect(result.id).toBe('b');
  });

  // Test: pop_invariant_maintains_heap_property
  // Spec: specifications/pop.md
  // Property: after pop(), heap invariant holds (front() is minimum of remaining)
  it('invariant: heap property maintained after pop', () => {
    const pq = newItemMinHeap(4);

    const items: Item[] = [
      { id: 'a', priority: 50 },
      { id: 'b', priority: 20 },
      { id: 'c', priority: 80 },
      { id: 'd', priority: 10 },
      { id: 'e', priority: 60 },
      { id: 'f', priority: 30 },
      { id: 'g', priority: 70 },
      { id: 'h', priority: 40 },
    ];

    for (const item of items) {
      pq.insert(item);
    }

    const expectedOrder = [10, 20, 30, 40];
    for (const expected of expectedOrder) {
      expect(pq.front().priority).toBe(expected);
      pq.pop();
    }
  });

  // Test: pop_size_decrements
  // Spec: specifications/pop.md
  // Property: size() decreases by 1 after successful pop()
  it('size: decrements by 1 after pop', () => {
    const pq = newItemMinHeap(4);

    pq.insert({ id: 'a', priority: 10 });
    pq.insert({ id: 'b', priority: 20 });
    pq.insert({ id: 'c', priority: 30 });

    for (let expectedSize = 2; expectedSize >= 0; expectedSize--) {
      const sizeBefore = pq.len();
      pq.pop();
      expect(pq.len()).toBe(sizeBefore - 1);
    }
  });

  // Test: pop_edge_empty_returns_undefined
  // Spec: specifications/pop.md
  // Property: pop() on empty heap returns undefined
  it('edge: returns undefined on empty heap', () => {
    const pq = newItemMinHeap(4);

    expect(pq.pop()).toBeUndefined();
  });
});


// --- front.test.ts ---

// Test corpus for front() operation
// Spec: specifications/front.md
// Part of: Amphigraphic-Strict × d-ary Heap Priority Queue research

import { describe, it, expect } from 'vitest';
import { PriorityQueue } from 'd-ary-heap';

interface Item {
  id: string;
  priority: number;
}

function newItemMinHeap(d: number): PriorityQueue<Item, string> {
  return new PriorityQueue<Item, string>({
    d,
    comparator: (a, b) => a.priority < b.priority,
    keyExtractor: (item) => item.id,
  });
}

// =============================================================================
// front() Tests
// =============================================================================

describe('front()', () => {
  // Test: front_postcondition_returns_minimum
  // Spec: specifications/front.md
  // Property: front() returns the item with lowest priority value (min-heap) without removal
  it('postcondition: returns minimum priority item', () => {
    const pq = newItemMinHeap(4);

    pq.insert({ id: 'a', priority: 30 });
    pq.insert({ id: 'b', priority: 10 });
    pq.insert({ id: 'c', priority: 20 });

    const front = pq.front();
    expect(front.priority).toBe(10);
    expect(front.id).toBe('b');
  });

  // Test: front_invariant_no_modification
  // Spec: specifications/front.md
  // Property: front() does not modify the heap (calling multiple times returns same result)
  it('invariant: does not modify heap', () => {
    const pq = newItemMinHeap(4);

    pq.insert({ id: 'a', priority: 30 });
    pq.insert({ id: 'b', priority: 10 });
    pq.insert({ id: 'c', priority: 20 });

    const first = pq.front();
    const second = pq.front();
    const third = pq.front();

    expect(first.id).toBe(second.id);
    expect(second.id).toBe(third.id);
  });

  // Test: front_size_unchanged
  // Spec: specifications/front.md
  // Property: size() remains the same after front()
  it('size: remains unchanged after front', () => {
    const pq = newItemMinHeap(4);

    pq.insert({ id: 'a', priority: 10 });
    pq.insert({ id: 'b', priority: 20 });
    pq.insert({ id: 'c', priority: 30 });

    const sizeBefore = pq.len();

    for (let i = 0; i < 5; i++) {
      pq.front();
    }

    expect(pq.len()).toBe(sizeBefore);
  });

  // Test: front_edge_empty_throws
  // Spec: specifications/front.md
  // Property: front() on empty heap throws error
  it('edge: throws error on empty heap', () => {
    const pq = newItemMinHeap(4);

    expect(() => pq.front()).toThrow();
  });
});


// --- increase_priority.test.ts ---

// Test corpus for increasePriority() operation
// Spec: specifications/increase_priority.md
// Part of: Amphigraphic-Strict × d-ary Heap Priority Queue research

import { describe, it, expect } from 'vitest';
import { PriorityQueue } from 'd-ary-heap';

interface Item {
  id: string;
  priority: number;
}

function newItemMinHeap(d: number): PriorityQueue<Item, string> {
  return new PriorityQueue<Item, string>({
    d,
    comparator: (a, b) => a.priority < b.priority,
    keyExtractor: (item) => item.id,
  });
}

// =============================================================================
// increasePriority() Tests
// =============================================================================

describe('increasePriority()', () => {
  // Test: increase_priority_postcondition_priority_changed
  // Spec: specifications/increase_priority.md
  // Property: item's priority is updated to the new value
  it('postcondition: priority is changed', () => {
    const pq = newItemMinHeap(4);

    pq.insert({ id: 'target', priority: 50 });
    pq.insert({ id: 'other', priority: 30 });

    expect(pq.front().id).toBe('other');

    pq.increasePriority({ id: 'target', priority: 10 });

    expect(pq.front().id).toBe('target');
    expect(pq.front().priority).toBe(10);
  });

  // Test: increase_priority_invariant_heap_property
  // Spec: specifications/increase_priority.md
  // Property: heap invariant holds after priority increase
  it('invariant: heap property maintained', () => {
    const pq = newItemMinHeap(4);

    const items: Item[] = [
      { id: 'a', priority: 80 },
      { id: 'b', priority: 60 },
      { id: 'c', priority: 40 },
      { id: 'd', priority: 20 },
      { id: 'e', priority: 100 },
      { id: 'f', priority: 50 },
    ];

    for (const item of items) {
      pq.insert(item);
    }

    expect(pq.front().priority).toBe(20);

    pq.increasePriority({ id: 'a', priority: 5 });

    expect(pq.front().id).toBe('a');
    expect(pq.front().priority).toBe(5);
  });

  // Test: increase_priority_position_item_moves_up
  // Spec: specifications/increase_priority.md
  // Property: item moves toward root after priority increase (becomes front if highest)
  it('position: item moves up toward root', () => {
    const pq = newItemMinHeap(4);

    pq.insert({ id: 'root', priority: 10 });
    pq.insert({ id: 'middle', priority: 50 });
    pq.insert({ id: 'leaf', priority: 100 });

    expect(pq.front().id).not.toBe('leaf');

    pq.increasePriority({ id: 'leaf', priority: 1 });

    expect(pq.front().id).toBe('leaf');
  });

  // Test: increase_priority_size_unchanged
  // Spec: specifications/increase_priority.md
  // Property: size() remains unchanged after priority update
  it('size: remains unchanged after priority update', () => {
    const pq = newItemMinHeap(4);

    pq.insert({ id: 'a', priority: 50 });
    pq.insert({ id: 'b', priority: 30 });
    pq.insert({ id: 'c', priority: 70 });

    const sizeBefore = pq.len();

    pq.increasePriority({ id: 'c', priority: 10 });

    expect(pq.len()).toBe(sizeBefore);
  });

  // Test: increase_priority_edge_not_found_throws
  // Spec: specifications/increase_priority.md
  // Property: throws error if item not in heap
  it('edge: throws error if item not found', () => {
    const pq = newItemMinHeap(4);

    pq.insert({ id: 'existing', priority: 50 });

    expect(() => pq.increasePriority({ id: 'nonexistent', priority: 10 })).toThrow();
  });
});


// --- decrease_priority.test.ts ---

// Test corpus for decreasePriority() operation
// Spec: specifications/decrease_priority.md
// Part of: Amphigraphic-Strict × d-ary Heap Priority Queue research

import { describe, it, expect } from 'vitest';
import { PriorityQueue } from 'd-ary-heap';

interface Item {
  id: string;
  priority: number;
}

function newItemMinHeap(d: number): PriorityQueue<Item, string> {
  return new PriorityQueue<Item, string>({
    d,
    comparator: (a, b) => a.priority < b.priority,
    keyExtractor: (item) => item.id,
  });
}

// =============================================================================
// decreasePriority() Tests
// =============================================================================

describe('decreasePriority()', () => {
  // Test: decrease_priority_postcondition_priority_changed
  // Spec: specifications/decrease_priority.md
  // Property: item's priority is updated to the new value
  it('postcondition: priority is changed', () => {
    const pq = newItemMinHeap(4);

    pq.insert({ id: 'target', priority: 10 });
    pq.insert({ id: 'other', priority: 30 });

    expect(pq.front().id).toBe('target');

    pq.decreasePriority({ id: 'target', priority: 50 });

    expect(pq.front().id).toBe('other');

    pq.pop();
    expect(pq.front().priority).toBe(50);
  });

  // Test: decrease_priority_invariant_heap_property
  // Spec: specifications/decrease_priority.md
  // Property: heap invariant holds after priority decrease
  it('invariant: heap property maintained', () => {
    const pq = newItemMinHeap(4);

    const items: Item[] = [
      { id: 'a', priority: 10 },
      { id: 'b', priority: 30 },
      { id: 'c', priority: 50 },
      { id: 'd', priority: 70 },
      { id: 'e', priority: 20 },
      { id: 'f', priority: 40 },
    ];

    for (const item of items) {
      pq.insert(item);
    }

    expect(pq.front().id).toBe('a');

    pq.decreasePriority({ id: 'a', priority: 100 });

    expect(pq.front().priority).toBe(20);
  });

  // Test: decrease_priority_position_item_moves_down
  // Spec: specifications/decrease_priority.md
  // Property: item moves toward leaves after priority decrease (no longer front if was)
  it('position: item moves down toward leaves', () => {
    const pq = newItemMinHeap(4);

    pq.insert({ id: 'root', priority: 10 });
    pq.insert({ id: 'child1', priority: 50 });
    pq.insert({ id: 'child2', priority: 60 });
    pq.insert({ id: 'child3', priority: 70 });

    expect(pq.front().id).toBe('root');

    pq.decreasePriority({ id: 'root', priority: 100 });

    expect(pq.front().id).not.toBe('root');
    expect(pq.front().id).toBe('child1');
  });

  // Test: decrease_priority_size_unchanged
  // Spec: specifications/decrease_priority.md
  // Property: size() remains unchanged after priority update
  it('size: remains unchanged after priority update', () => {
    const pq = newItemMinHeap(4);

    pq.insert({ id: 'a', priority: 10 });
    pq.insert({ id: 'b', priority: 30 });
    pq.insert({ id: 'c', priority: 50 });

    const sizeBefore = pq.len();

    pq.decreasePriority({ id: 'a', priority: 100 });

    expect(pq.len()).toBe(sizeBefore);
  });

  // Test: decrease_priority_edge_not_found_throws
  // Spec: specifications/decrease_priority.md
  // Property: throws error if item not in heap
  it('edge: throws error if item not found', () => {
    const pq = newItemMinHeap(4);

    pq.insert({ id: 'existing', priority: 50 });

    expect(() => pq.decreasePriority({ id: 'nonexistent', priority: 100 })).toThrow();
  });
});


Provide a complete, working implementation that satisfies the documentation, matches the type signatures, and passes all tests.