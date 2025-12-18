/**
 * Compare optimized vs naive implementation to quantify improvements.
 * Run with: npx tsx benchmarks/compare-implementations.ts
 */

import { PriorityQueue, minBy } from '../src';

interface Item {
  id: number;
  cost: number;
}

const createItem = (id: number, cost: number): Item => ({ id, cost });

// ============================================================================
// NAIVE IMPLEMENTATION (for comparison)
// ============================================================================

type Comparator<T> = (a: T, b: T) => boolean;
type KeyExtractor<T, K> = (item: T) => K;

/**
 * Naive implementation without optimizations:
 * - No specialized paths for d=2 or d=4
 * - Uses swap() instead of sift pattern
 * - No property caching
 * - Uses Math.min
 */
class NaivePriorityQueue<T, K> {
  private container: T[] = [];
  private positions: Map<K, number> = new Map();
  private readonly d: number;
  private readonly comparator: Comparator<T>;
  private readonly keyExtractor: KeyExtractor<T, K>;

  constructor(d: number, comparator: Comparator<T>, keyExtractor: KeyExtractor<T, K>) {
    this.d = d;
    this.comparator = comparator;
    this.keyExtractor = keyExtractor;
  }

  len(): number {
    return this.container.length;
  }

  isEmpty(): boolean {
    return this.container.length === 0;
  }

  containsKey(key: K): boolean {
    return this.positions.has(key);
  }

  insert(item: T): void {
    const index = this.container.length;
    this.container.push(item);
    this.positions.set(this.keyExtractor(item), index);
    this.moveUp(index);
  }

  pop(): T | undefined {
    if (this.container.length === 0) return undefined;

    const top = this.container[0]!;
    this.positions.delete(this.keyExtractor(top));

    if (this.container.length === 1) {
      this.container.pop();
      return top;
    }

    const last = this.container.pop()!;
    this.container[0] = last;
    this.positions.set(this.keyExtractor(last), 0);
    this.moveDown(0);

    return top;
  }

  increasePriority(item: T): void {
    const key = this.keyExtractor(item);
    const index = this.positions.get(key);
    if (index === undefined) throw new Error('Item not found');

    this.container[index] = item;
    this.moveUp(index);
  }

  // NAIVE: Uses swap pattern (2 writes per level)
  private swap(i: number, j: number): void {
    const temp = this.container[i]!;
    this.container[i] = this.container[j]!;
    this.container[j] = temp;

    this.positions.set(this.keyExtractor(this.container[i]!), i);
    this.positions.set(this.keyExtractor(this.container[j]!), j);
  }

  // NAIVE: No property caching, uses swap
  private moveUp(i: number): void {
    while (i > 0) {
      const p = Math.floor((i - 1) / this.d); // Uses Math.floor
      if (this.comparator(this.container[i]!, this.container[p]!)) {
        this.swap(i, p);
        i = p;
      } else {
        break;
      }
    }
  }

  // NAIVE: Separate bestChild function, uses Math.min
  private bestChildPosition(i: number): number {
    const left = i * this.d + 1;
    if (left >= this.container.length) return left;

    let best = left;
    const right = Math.min((i + 1) * this.d, this.container.length - 1);

    for (let j = left + 1; j <= right; j++) {
      if (this.comparator(this.container[j]!, this.container[best]!)) {
        best = j;
      }
    }
    return best;
  }

  // NAIVE: Uses swap, calls bestChildPosition
  private moveDown(i: number): void {
    while (true) {
      const firstChild = i * this.d + 1;
      if (firstChild >= this.container.length) break;

      const best = this.bestChildPosition(i);
      if (this.comparator(this.container[best]!, this.container[i]!)) {
        this.swap(i, best);
        i = best;
      } else {
        break;
      }
    }
  }
}

// ============================================================================
// BENCHMARK UTILITIES
// ============================================================================

function benchmark(name: string, fn: () => void, iterations = 5): number {
  // Warmup
  fn();
  fn();

  const times: number[] = [];
  for (let i = 0; i < iterations; i++) {
    const start = performance.now();
    fn();
    times.push(performance.now() - start);
  }

  return times.reduce((a, b) => a + b, 0) / times.length;
}

function formatTime(ms: number): string {
  if (ms < 1) return `${(ms * 1000).toFixed(1)}µs`;
  return `${ms.toFixed(2)}ms`;
}

function formatSpeedup(naive: number, optimized: number): string {
  const speedup = naive / optimized;
  if (speedup >= 1) {
    return `${speedup.toFixed(2)}x faster ✓`;
  } else {
    return `${(1 / speedup).toFixed(2)}x slower ✗`;
  }
}

// ============================================================================
// RUN COMPARISONS
// ============================================================================

console.log('='.repeat(75));
console.log('OPTIMIZED vs NAIVE IMPLEMENTATION COMPARISON');
console.log('='.repeat(75));
console.log();

const sizes = [10000, 50000, 100000];
const arities = [2, 4, 8];

// Test 1: Insert Performance
console.log('TEST 1: INSERT PERFORMANCE');
console.log('-'.repeat(75));
console.log('Size\t\td\tNaive\t\tOptimized\tSpeedup');

for (const size of sizes) {
  for (const d of arities) {
    const naiveTime = benchmark(`Naive insert ${size}`, () => {
      const pq = new NaivePriorityQueue<Item, number>(
        d,
        (a, b) => a.cost < b.cost,
        (item) => item.id
      );
      for (let i = 0; i < size; i++) {
        pq.insert(createItem(i, Math.random() * size));
      }
    });

    const optTime = benchmark(`Optimized insert ${size}`, () => {
      const pq = new PriorityQueue<Item, number>({
        d,
        comparator: (a, b) => a.cost < b.cost, // Same comparator as naive
        keyExtractor: (item) => item.id,
        initialCapacity: size,
      });
      for (let i = 0; i < size; i++) {
        pq.insert(createItem(i, Math.random() * size));
      }
    });

    console.log(
      `${size}\t\t${d}\t${formatTime(naiveTime)}\t\t${formatTime(optTime)}\t\t${formatSpeedup(naiveTime, optTime)}`
    );
  }
}

console.log();

// Test 2: Pop Performance
console.log('TEST 2: POP PERFORMANCE (drain heap)');
console.log('-'.repeat(75));
console.log('Size\t\td\tNaive\t\tOptimized\tSpeedup');

for (const size of [10000, 50000]) {
  for (const d of arities) {
    const naiveTime = benchmark(`Naive pop ${size}`, () => {
      const pq = new NaivePriorityQueue<Item, number>(
        d,
        (a, b) => a.cost < b.cost,
        (item) => item.id
      );
      for (let i = 0; i < size; i++) {
        pq.insert(createItem(i, Math.random() * size));
      }
      while (!pq.isEmpty()) {
        pq.pop();
      }
    });

    const optTime = benchmark(`Optimized pop ${size}`, () => {
      const pq = new PriorityQueue<Item, number>({
        d,
        comparator: (a, b) => a.cost < b.cost,
        keyExtractor: (item) => item.id,
      });
      for (let i = 0; i < size; i++) {
        pq.insert(createItem(i, Math.random() * size));
      }
      while (!pq.isEmpty()) {
        pq.pop();
      }
    });

    console.log(
      `${size}\t\t${d}\t${formatTime(naiveTime)}\t\t${formatTime(optTime)}\t\t${formatSpeedup(naiveTime, optTime)}`
    );
  }
}

console.log();

// Test 3: Priority Update Performance
console.log('TEST 3: PRIORITY UPDATE PERFORMANCE');
console.log('-'.repeat(75));
console.log('Size\t\td\tNaive\t\tOptimized\tSpeedup');

for (const size of [10000, 50000]) {
  for (const d of arities) {
    // Naive
    const naivePq = new NaivePriorityQueue<Item, number>(
      d,
      (a, b) => a.cost < b.cost,
      (item) => item.id
    );
    for (let i = 0; i < size; i++) {
      naivePq.insert(createItem(i, size + Math.random() * size));
    }

    const naiveTime = benchmark(`Naive update ${size}`, () => {
      for (let i = 0; i < size; i++) {
        naivePq.increasePriority(createItem(i, Math.random() * size));
      }
    });

    // Optimized
    const optPq = new PriorityQueue<Item, number>({
      d,
      comparator: (a, b) => a.cost < b.cost,
      keyExtractor: (item) => item.id,
    });
    for (let i = 0; i < size; i++) {
      optPq.insert(createItem(i, size + Math.random() * size));
    }

    const optTime = benchmark(`Optimized update ${size}`, () => {
      for (let i = 0; i < size; i++) {
        optPq.increasePriority(createItem(i, Math.random() * size));
      }
    });

    console.log(
      `${size}\t\t${d}\t${formatTime(naiveTime)}\t\t${formatTime(optTime)}\t\t${formatSpeedup(naiveTime, optTime)}`
    );
  }
}

console.log();

// Test 4: Mixed Workload
console.log('TEST 4: MIXED WORKLOAD (Dijkstra-like)');
console.log('-'.repeat(75));
console.log('Size\t\td\tNaive\t\tOptimized\tSpeedup');

for (const size of [10000, 50000]) {
  for (const d of [2, 4]) {
    const naiveTime = benchmark(`Naive mixed ${size}`, () => {
      const pq = new NaivePriorityQueue<Item, number>(
        d,
        (a, b) => a.cost < b.cost,
        (item) => item.id
      );

      for (let i = 0; i < size; i++) {
        pq.insert(createItem(i, Math.random() * size * 10));
      }

      let processed = 0;
      while (!pq.isEmpty() && processed < size / 2) {
        const current = pq.pop()!;
        processed++;

        for (let j = 0; j < 3; j++) {
          const neighborId = (current.id + j + 1) % size;
          if (pq.containsKey(neighborId)) {
            pq.increasePriority(createItem(neighborId, current.cost + Math.random() * 10));
          }
        }
      }
    });

    const optTime = benchmark(`Optimized mixed ${size}`, () => {
      const pq = new PriorityQueue<Item, number>({
        d,
        comparator: (a, b) => a.cost < b.cost,
        keyExtractor: (item) => item.id,
      });

      for (let i = 0; i < size; i++) {
        pq.insert(createItem(i, Math.random() * size * 10));
      }

      let processed = 0;
      while (!pq.isEmpty() && processed < size / 2) {
        const current = pq.pop()!;
        processed++;

        for (let j = 0; j < 3; j++) {
          const neighborId = (current.id + j + 1) % size;
          if (pq.containsKey(neighborId)) {
            pq.increasePriority(createItem(neighborId, current.cost + Math.random() * 10));
          }
        }
      }
    });

    console.log(
      `${size}\t\t${d}\t${formatTime(naiveTime)}\t\t${formatTime(optTime)}\t\t${formatSpeedup(naiveTime, optTime)}`
    );
  }
}

console.log();
console.log('='.repeat(75));
console.log('SUMMARY');
console.log('='.repeat(75));
console.log(`
Key optimizations applied:
1. Sift pattern (single placement) vs swap pattern (2 writes per level)
2. Property caching in local variables
3. Specialized fast paths for d=2 (binary) and d=4 (quaternary) heaps
4. Bit shifts for d=2 parent/child calculations
5. Inlined bestChildPosition to eliminate function call overhead
6. Avoided Math.min/Math.floor in hot paths
7. Pre-allocation support for known sizes
`);
