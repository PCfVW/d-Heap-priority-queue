Implement a d-ary heap priority queue in TypeScript.

Requirements:
1. The heap arity (d) should be configurable at construction time
2. Items have two distinct properties: an identity (for equality) and a priority (for ordering)
3. Two items are equal if they have the same identity, regardless of priority
4. The queue should support O(1) lookup to check if an item exists
5. Implement a min-heap where lower priority values have higher importance

Required operations:
- insert(item): Add an item to the queue
- pop(): Remove and return the item with highest priority (lowest value)
- front(): Return the item with highest priority without removing it
- increase_priority(item): Update an existing item to have higher priority (lower value)
- decrease_priority(item): Update an existing item to have lower priority (higher value)
- contains(item): Check if an item with the given identity exists
- len(): Return the number of items in the queue
- is_empty(): Return whether the queue is empty

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


Provide a complete, working implementation that passes all tests.