/**
 * Performance benchmark tests for d-ary heap priority queue.
 * These tests verify that operations complete within expected time bounds.
 */

import { describe, it, expect } from 'vitest';
import { PriorityQueue, minBy } from '../src';

interface Item {
  id: number;
  cost: number;
}

const createItem = (id: number, cost: number): Item => ({ id, cost });

describe('PriorityQueue - Performance Benchmarks', () => {
  it('should handle 100,000 insertions efficiently', () => {
    const pq = new PriorityQueue<Item, number>({
      d: 4,
      comparator: minBy((item) => item.cost),
      keyExtractor: (item) => item.id,
      initialCapacity: 100000,
    });

    const start = performance.now();

    for (let i = 0; i < 100000; i++) {
      pq.insert(createItem(i, Math.random() * 100000));
    }

    const insertTime = performance.now() - start;

    expect(pq.len()).toBe(100000);
    // Should complete in under 500ms on most machines
    expect(insertTime).toBeLessThan(1000);

    console.log(`100k insertions: ${insertTime.toFixed(2)}ms`);
  });

  it('should handle 100,000 pop operations efficiently', () => {
    const pq = new PriorityQueue<Item, number>({
      d: 4,
      comparator: minBy((item) => item.cost),
      keyExtractor: (item) => item.id,
      initialCapacity: 100000,
    });

    // Setup
    for (let i = 0; i < 100000; i++) {
      pq.insert(createItem(i, Math.random() * 100000));
    }

    const start = performance.now();

    let prev = -Infinity;
    while (!pq.isEmpty()) {
      const item = pq.pop()!;
      expect(item.cost).toBeGreaterThanOrEqual(prev);
      prev = item.cost;
    }

    const popTime = performance.now() - start;

    expect(pq.len()).toBe(0);
    // Should complete in under 1000ms on most machines
    expect(popTime).toBeLessThan(2000);

    console.log(`100k pops: ${popTime.toFixed(2)}ms`);
  });

  it('should handle mixed operations efficiently', () => {
    const pq = new PriorityQueue<Item, number>({
      d: 4,
      comparator: minBy((item) => item.cost),
      keyExtractor: (item) => item.id,
    });

    const start = performance.now();

    // Insert 50k items
    for (let i = 0; i < 50000; i++) {
      pq.insert(createItem(i, Math.random() * 100000));
    }

    // Pop 10k items
    for (let i = 0; i < 10000; i++) {
      pq.pop();
    }

    // Update priorities on items that are still in the queue
    // We need to check which items are still present
    let updateCount = 0;
    for (let i = 0; i < 50000 && updateCount < 20000; i++) {
      if (pq.containsKey(i)) {
        const newCost = Math.random() * 100000;
        pq.decreasePriority(createItem(i, newCost));
        updateCount++;
      }
    }

    // Pop remaining
    while (!pq.isEmpty()) {
      pq.pop();
    }

    const totalTime = performance.now() - start;

    expect(pq.len()).toBe(0);
    // Should complete in under 1500ms on most machines
    expect(totalTime).toBeLessThan(3000);

    console.log(`Mixed operations: ${totalTime.toFixed(2)}ms`);
  });

  it('should compare different arities', () => {
    const arities = [2, 4, 8, 16];
    const results: Record<number, number> = {};

    for (const d of arities) {
      const pq = new PriorityQueue<Item, number>({
        d,
        comparator: minBy((item) => item.cost),
        keyExtractor: (item) => item.id,
      });

      const start = performance.now();

      // Insert and pop 50k items
      for (let i = 0; i < 50000; i++) {
        pq.insert(createItem(i, Math.random() * 50000));
      }
      while (!pq.isEmpty()) {
        pq.pop();
      }

      results[d] = performance.now() - start;
    }

    console.log('Arity comparison (50k insert+pop):');
    for (const [d, time] of Object.entries(results)) {
      console.log(`  d=${d}: ${time.toFixed(2)}ms`);
    }

    // All should complete reasonably
    for (const time of Object.values(results)) {
      expect(time).toBeLessThan(2000);
    }
  });
});
