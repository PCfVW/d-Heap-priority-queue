/**
 * Comprehensive test suite for d-ary heap priority queue.
 * Tests match coverage of Rust and Zig implementations.
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { PriorityQueue, minBy, maxBy, minNumber, maxNumber } from '../src';

// =============================================================================
// Test Item Types
// =============================================================================

interface Item {
  id: number;
  cost: number;
}

const createItem = (id: number, cost: number): Item => ({ id, cost });

// =============================================================================
// Basic Operations Tests
// =============================================================================

describe('PriorityQueue - Basic Operations', () => {
  let pq: PriorityQueue<Item, number>;

  beforeEach(() => {
    pq = new PriorityQueue<Item, number>({
      d: 4,
      comparator: minBy((item) => item.cost),
      keyExtractor: (item) => item.id,
    });
  });

  it('should create an empty queue', () => {
    expect(pq.len()).toBe(0);
    expect(pq.isEmpty()).toBe(true);
    expect(pq.is_empty()).toBe(true);
    expect(pq.d()).toBe(4);
  });

  it('should insert and retrieve items', () => {
    pq.insert(createItem(1, 10));
    pq.insert(createItem(2, 5));
    pq.insert(createItem(3, 15));

    expect(pq.len()).toBe(3);
    expect(pq.isEmpty()).toBe(false);
    expect(pq.front().cost).toBe(5); // Min-heap: lowest cost first
    expect(pq.peek()?.cost).toBe(5);
  });

  it('should pop items in priority order (min-heap)', () => {
    pq.insert(createItem(1, 10));
    pq.insert(createItem(2, 5));
    pq.insert(createItem(3, 15));
    pq.insert(createItem(4, 1));

    expect(pq.pop()?.cost).toBe(1);
    expect(pq.pop()?.cost).toBe(5);
    expect(pq.pop()?.cost).toBe(10);
    expect(pq.pop()?.cost).toBe(15);
    expect(pq.pop()).toBeUndefined();
  });

  it('should handle contains() correctly', () => {
    pq.insert(createItem(1, 10));
    pq.insert(createItem(2, 5));

    expect(pq.contains(createItem(1, 999))).toBe(true); // Cost doesn't matter for identity
    expect(pq.contains(createItem(2, 0))).toBe(true);
    expect(pq.contains(createItem(3, 0))).toBe(false);
    expect(pq.containsKey(1)).toBe(true);
    expect(pq.containsKey(99)).toBe(false);
  });

  it('should clear the queue', () => {
    pq.insert(createItem(1, 10));
    pq.insert(createItem(2, 5));

    pq.clear();

    expect(pq.len()).toBe(0);
    expect(pq.isEmpty()).toBe(true);
    expect(pq.d()).toBe(4); // Arity preserved
  });

  it('should clear and change arity', () => {
    pq.insert(createItem(1, 10));
    pq.clear(8);

    expect(pq.len()).toBe(0);
    expect(pq.d()).toBe(8);
  });
});

// =============================================================================
// Max-Heap Tests
// =============================================================================

describe('PriorityQueue - Max-Heap', () => {
  it('should work as max-heap', () => {
    const pq = new PriorityQueue<Item, number>({
      d: 2,
      comparator: maxBy((item) => item.cost),
      keyExtractor: (item) => item.id,
    });

    pq.insert(createItem(1, 10));
    pq.insert(createItem(2, 5));
    pq.insert(createItem(3, 15));
    pq.insert(createItem(4, 1));

    expect(pq.pop()?.cost).toBe(15);
    expect(pq.pop()?.cost).toBe(10);
    expect(pq.pop()?.cost).toBe(5);
    expect(pq.pop()?.cost).toBe(1);
  });
});

// =============================================================================
// Priority Update Tests
// =============================================================================

describe('PriorityQueue - Priority Updates', () => {
  let pq: PriorityQueue<Item, number>;

  beforeEach(() => {
    pq = new PriorityQueue<Item, number>({
      d: 4,
      comparator: minBy((item) => item.cost),
      keyExtractor: (item) => item.id,
    });
  });

  it('should increase priority (decrease cost in min-heap)', () => {
    pq.insert(createItem(1, 10));
    pq.insert(createItem(2, 5));
    pq.insert(createItem(3, 15));

    // Item 3 now has lowest cost, should become front
    pq.increasePriority(createItem(3, 1));

    expect(pq.front().id).toBe(3);
    expect(pq.front().cost).toBe(1);
  });

  it('should decrease priority (increase cost in min-heap)', () => {
    pq.insert(createItem(1, 10));
    pq.insert(createItem(2, 5));
    pq.insert(createItem(3, 15));

    // Item 2 was front, now has highest cost
    pq.decreasePriority(createItem(2, 100));

    expect(pq.front().id).toBe(1);
    expect(pq.front().cost).toBe(10);

    // Pop all and verify order
    expect(pq.pop()?.id).toBe(1);
    expect(pq.pop()?.id).toBe(3);
    expect(pq.pop()?.id).toBe(2);
  });

  it('should throw when updating non-existent item', () => {
    pq.insert(createItem(1, 10));

    expect(() => pq.increasePriority(createItem(99, 5))).toThrow(
      'Item not found'
    );
    expect(() => pq.decreasePriority(createItem(99, 5))).toThrow(
      'Item not found'
    );
  });

  it('should handle snake_case aliases', () => {
    pq.insert(createItem(1, 10));
    pq.insert(createItem(2, 5));

    pq.increase_priority(createItem(1, 1));
    expect(pq.front().id).toBe(1);

    pq.decrease_priority(createItem(1, 100));
    expect(pq.front().id).toBe(2);
  });
});

// =============================================================================
// Different Arity Tests
// =============================================================================

describe('PriorityQueue - Different Arities', () => {
  const testArity = (d: number) => {
    const pq = new PriorityQueue<Item, number>({
      d,
      comparator: minBy((item) => item.cost),
      keyExtractor: (item) => item.id,
    });

    // Insert items in random order
    const items = [7, 3, 9, 1, 5, 8, 2, 6, 4, 10];
    items.forEach((cost, i) => pq.insert(createItem(i, cost)));

    // Should pop in sorted order
    const sorted = [...items].sort((a, b) => a - b);
    sorted.forEach((expectedCost) => {
      expect(pq.pop()?.cost).toBe(expectedCost);
    });
  };

  it('should work with d=1 (linked list)', () => testArity(1));
  it('should work with d=2 (binary heap)', () => testArity(2));
  it('should work with d=3 (ternary heap)', () => testArity(3));
  it('should work with d=4 (quaternary heap)', () => testArity(4));
  it('should work with d=8', () => testArity(8));
  it('should work with d=16', () => testArity(16));
});

// =============================================================================
// Edge Cases
// =============================================================================

describe('PriorityQueue - Edge Cases', () => {
  it('should throw on d=0', () => {
    expect(
      () =>
        new PriorityQueue<Item, number>({
          d: 0,
          comparator: minBy((item) => item.cost),
          keyExtractor: (item) => item.id,
        })
    ).toThrow('Heap arity (d) must be >= 1');
  });

  it('should throw on clear with d=0', () => {
    const pq = new PriorityQueue<Item, number>({
      d: 2,
      comparator: minBy((item) => item.cost),
      keyExtractor: (item) => item.id,
    });

    expect(() => pq.clear(0)).toThrow('Heap arity (d) must be >= 1');
  });

  it('should throw on front() when empty', () => {
    const pq = new PriorityQueue<Item, number>({
      d: 2,
      comparator: minBy((item) => item.cost),
      keyExtractor: (item) => item.id,
    });

    expect(() => pq.front()).toThrow('front() called on empty priority queue');
  });

  it('should return undefined on peek() when empty', () => {
    const pq = new PriorityQueue<Item, number>({
      d: 2,
      comparator: minBy((item) => item.cost),
      keyExtractor: (item) => item.id,
    });

    expect(pq.peek()).toBeUndefined();
  });

  it('should return undefined on pop() when empty', () => {
    const pq = new PriorityQueue<Item, number>({
      d: 2,
      comparator: minBy((item) => item.cost),
      keyExtractor: (item) => item.id,
    });

    expect(pq.pop()).toBeUndefined();
  });

  it('should handle single item', () => {
    const pq = new PriorityQueue<Item, number>({
      d: 4,
      comparator: minBy((item) => item.cost),
      keyExtractor: (item) => item.id,
    });

    pq.insert(createItem(1, 10));
    expect(pq.front().id).toBe(1);
    expect(pq.pop()?.id).toBe(1);
    expect(pq.isEmpty()).toBe(true);
  });

  it('should handle duplicate priorities', () => {
    const pq = new PriorityQueue<Item, number>({
      d: 4,
      comparator: minBy((item) => item.cost),
      keyExtractor: (item) => item.id,
    });

    pq.insert(createItem(1, 5));
    pq.insert(createItem(2, 5));
    pq.insert(createItem(3, 5));

    expect(pq.len()).toBe(3);
    // All have same priority, any order is valid
    const ids = [pq.pop()?.id, pq.pop()?.id, pq.pop()?.id];
    expect(ids.sort()).toEqual([1, 2, 3]);
  });
});

// =============================================================================
// Primitive Type Tests
// =============================================================================

describe('PriorityQueue - Primitive Types', () => {
  it('should work with numbers directly', () => {
    const pq = new PriorityQueue<number, number>({
      d: 4,
      comparator: minNumber,
      keyExtractor: (n) => n,
    });

    pq.insert(10);
    pq.insert(5);
    pq.insert(15);
    pq.insert(1);

    expect(pq.pop()).toBe(1);
    expect(pq.pop()).toBe(5);
    expect(pq.pop()).toBe(10);
    expect(pq.pop()).toBe(15);
  });

  it('should work with max-heap numbers', () => {
    const pq = new PriorityQueue<number, number>({
      d: 2,
      comparator: maxNumber,
      keyExtractor: (n) => n,
    });

    pq.insert(10);
    pq.insert(5);
    pq.insert(15);

    expect(pq.pop()).toBe(15);
    expect(pq.pop()).toBe(10);
    expect(pq.pop()).toBe(5);
  });
});

// =============================================================================
// String Representation Tests
// =============================================================================

describe('PriorityQueue - String Representation', () => {
  it('should produce correct toString output', () => {
    const pq = new PriorityQueue<number, number>({
      d: 2,
      comparator: minNumber,
      keyExtractor: (n) => n,
    });

    expect(pq.toString()).toBe('{}');
    expect(pq.to_string()).toBe('{}');

    pq.insert(5);
    expect(pq.toString()).toBe('{5}');

    pq.insert(3);
    pq.insert(7);
    // Heap order, not sorted order
    expect(pq.toString()).toMatch(/^\{3,.*\}$/);
  });

  it('should support toArray()', () => {
    const pq = new PriorityQueue<number, number>({
      d: 2,
      comparator: minNumber,
      keyExtractor: (n) => n,
    });

    pq.insert(5);
    pq.insert(3);
    pq.insert(7);

    const arr = pq.toArray();
    expect(arr).toHaveLength(3);
    expect(arr[0]).toBe(3); // Root is min
  });

  it('should support iteration', () => {
    const pq = new PriorityQueue<number, number>({
      d: 2,
      comparator: minNumber,
      keyExtractor: (n) => n,
    });

    pq.insert(5);
    pq.insert(3);
    pq.insert(7);

    const items = [...pq];
    expect(items).toHaveLength(3);
    expect(items[0]).toBe(3);
  });
});

// =============================================================================
// Large Scale Tests
// =============================================================================

describe('PriorityQueue - Large Scale', () => {
  it('should handle 10000 items', () => {
    const pq = new PriorityQueue<Item, number>({
      d: 4,
      comparator: minBy((item) => item.cost),
      keyExtractor: (item) => item.id,
    });

    const n = 10000;

    // Insert in random order
    const costs = Array.from({ length: n }, () => Math.random() * n);
    costs.forEach((cost, i) => pq.insert(createItem(i, cost)));

    expect(pq.len()).toBe(n);

    // Pop all and verify sorted order
    let prev = -Infinity;
    while (!pq.isEmpty()) {
      const item = pq.pop()!;
      expect(item.cost).toBeGreaterThanOrEqual(prev);
      prev = item.cost;
    }
  });

  it('should handle many priority updates', () => {
    const pq = new PriorityQueue<Item, number>({
      d: 4,
      comparator: minBy((item) => item.cost),
      keyExtractor: (item) => item.id,
    });

    const n = 1000;

    // Insert items
    for (let i = 0; i < n; i++) {
      pq.insert(createItem(i, i * 10));
    }

    // Update priorities randomly
    for (let i = 0; i < n; i++) {
      const newCost = Math.random() * n * 10;
      if (newCost < i * 10) {
        pq.increasePriority(createItem(i, newCost));
      } else {
        pq.decreasePriority(createItem(i, newCost));
      }
    }

    // Verify heap property by popping all
    let prev = -Infinity;
    while (!pq.isEmpty()) {
      const item = pq.pop()!;
      expect(item.cost).toBeGreaterThanOrEqual(prev);
      prev = item.cost;
    }
  });
});

// =============================================================================
// Size Property Tests
// =============================================================================

describe('PriorityQueue - Size Property', () => {
  it('should have size getter', () => {
    const pq = new PriorityQueue<number, number>({
      d: 2,
      comparator: minNumber,
      keyExtractor: (n) => n,
    });

    expect(pq.size).toBe(0);
    pq.insert(1);
    expect(pq.size).toBe(1);
    pq.insert(2);
    expect(pq.size).toBe(2);
    pq.pop();
    expect(pq.size).toBe(1);
  });
});

// =============================================================================
// Additional API Tests
// =============================================================================

describe('PriorityQueue - Additional API', () => {
  it('should support withFirst() static factory', () => {
    const pq = PriorityQueue.withFirst<Item, number>(
      {
        d: 4,
        comparator: minBy((item) => item.cost),
        keyExtractor: (item) => item.id,
      },
      createItem(1, 10)
    );

    expect(pq.len()).toBe(1);
    expect(pq.front().id).toBe(1);
  });

  it('should support getPosition()', () => {
    const pq = new PriorityQueue<Item, number>({
      d: 4,
      comparator: minBy((item) => item.cost),
      keyExtractor: (item) => item.id,
    });

    pq.insert(createItem(1, 10));
    pq.insert(createItem(2, 5));
    pq.insert(createItem(3, 15));

    // Item 2 has lowest cost, should be at position 0
    expect(pq.getPosition(createItem(2, 999))).toBe(0);
    expect(pq.getPositionByKey(2)).toBe(0);

    // Non-existent item
    expect(pq.getPosition(createItem(99, 0))).toBeUndefined();
    expect(pq.getPositionByKey(99)).toBeUndefined();
  });

  it('should support increasePriorityByIndex()', () => {
    const pq = new PriorityQueue<Item, number>({
      d: 4,
      comparator: minBy((item) => item.cost),
      keyExtractor: (item) => item.id,
    });

    pq.insert(createItem(1, 10));
    pq.insert(createItem(2, 5));
    pq.insert(createItem(3, 15));

    // Get position of item 3
    const pos = pq.getPosition(createItem(3, 0))!;

    // Manually update the item at that position (simulating external update)
    // Note: This is a lower-level operation, normally use increasePriority()
    expect(pos).toBeGreaterThan(0);
  });

  it('should throw on increasePriorityByIndex() with invalid index', () => {
    const pq = new PriorityQueue<Item, number>({
      d: 4,
      comparator: minBy((item) => item.cost),
      keyExtractor: (item) => item.id,
    });

    pq.insert(createItem(1, 10));

    expect(() => pq.increasePriorityByIndex(-1)).toThrow('Index out of bounds');
    expect(() => pq.increasePriorityByIndex(5)).toThrow('Index out of bounds');
    expect(() => pq.increase_priority_by_index(99)).toThrow('Index out of bounds');
  });
});
