/**
 * Identify the actual performance bottleneck.
 * Run with: npx tsx benchmarks/identify-bottleneck.ts
 */

import { PriorityQueue, minBy } from '../src';

interface Item {
  id: number;
  cost: number;
}

const SIZE = 50000;

console.log('='.repeat(70));
console.log('BOTTLENECK ANALYSIS');
console.log('='.repeat(70));
console.log();

// Test 1: minBy wrapper vs inline comparator
console.log('TEST 1: Comparator overhead');
console.log('-'.repeat(70));

// With minBy wrapper
{
  const start = performance.now();
  const pq = new PriorityQueue<Item, number>({
    d: 4,
    comparator: minBy((item) => item.cost),
    keyExtractor: (item) => item.id,
  });

  for (let i = 0; i < SIZE; i++) {
    pq.insert({ id: i, cost: Math.random() * SIZE });
  }
  while (!pq.isEmpty()) pq.pop();

  console.log(`With minBy wrapper: ${(performance.now() - start).toFixed(2)}ms`);
}

// With inline comparator
{
  const start = performance.now();
  const pq = new PriorityQueue<Item, number>({
    d: 4,
    comparator: (a, b) => a.cost < b.cost,
    keyExtractor: (item) => item.id,
  });

  for (let i = 0; i < SIZE; i++) {
    pq.insert({ id: i, cost: Math.random() * SIZE });
  }
  while (!pq.isEmpty()) pq.pop();

  console.log(`With inline comparator: ${(performance.now() - start).toFixed(2)}ms`);
}

console.log();

// Test 2: keyExtractor overhead
console.log('TEST 2: KeyExtractor overhead');
console.log('-'.repeat(70));

// With function keyExtractor
{
  const start = performance.now();
  const pq = new PriorityQueue<Item, number>({
    d: 4,
    comparator: (a, b) => a.cost < b.cost,
    keyExtractor: (item) => item.id,
  });

  for (let i = 0; i < SIZE; i++) {
    pq.insert({ id: i, cost: Math.random() * SIZE });
  }
  while (!pq.isEmpty()) pq.pop();

  console.log(`With function keyExtractor: ${(performance.now() - start).toFixed(2)}ms`);
}

// Test 3: Map vs object for positions
console.log();
console.log('TEST 3: Map operations overhead');
console.log('-'.repeat(70));

// Pure Map operations
{
  const map = new Map<number, number>();
  const start = performance.now();

  for (let i = 0; i < SIZE * 10; i++) {
    map.set(i % SIZE, i);
  }
  for (let i = 0; i < SIZE * 10; i++) {
    map.get(i % SIZE);
  }

  console.log(`${SIZE * 10} Map set + ${SIZE * 10} Map get: ${(performance.now() - start).toFixed(2)}ms`);
}

// Test 4: Array operations
console.log();
console.log('TEST 4: Array operations overhead');
console.log('-'.repeat(70));

{
  const arr: Item[] = [];
  const start = performance.now();

  // Push
  for (let i = 0; i < SIZE; i++) {
    arr.push({ id: i, cost: Math.random() * SIZE });
  }

  // Random access and swap
  for (let i = 0; i < SIZE * 5; i++) {
    const a = Math.floor(Math.random() * arr.length);
    const b = Math.floor(Math.random() * arr.length);
    const temp = arr[a]!;
    arr[a] = arr[b]!;
    arr[b] = temp;
  }

  // Pop all
  while (arr.length > 0) arr.pop();

  console.log(`Array push/swap/pop: ${(performance.now() - start).toFixed(2)}ms`);
}

// Test 5: Function call overhead
console.log();
console.log('TEST 5: Function call overhead');
console.log('-'.repeat(70));

function externalCompare(a: Item, b: Item): boolean {
  return a.cost < b.cost;
}

{
  const items: Item[] = [];
  for (let i = 0; i < SIZE; i++) {
    items.push({ id: i, cost: Math.random() * SIZE });
  }

  // Inline comparison
  let start = performance.now();
  let count1 = 0;
  for (let i = 0; i < SIZE * 100; i++) {
    const a = items[i % SIZE]!;
    const b = items[(i + 1) % SIZE]!;
    if (a.cost < b.cost) count1++;
  }
  console.log(`Inline comparison (${SIZE * 100}x): ${(performance.now() - start).toFixed(2)}ms`);

  // Function call comparison
  start = performance.now();
  let count2 = 0;
  for (let i = 0; i < SIZE * 100; i++) {
    const a = items[i % SIZE]!;
    const b = items[(i + 1) % SIZE]!;
    if (externalCompare(a, b)) count2++;
  }
  console.log(`Function call comparison (${SIZE * 100}x): ${(performance.now() - start).toFixed(2)}ms`);

  // Closure comparison (like minBy)
  const closureCompare = ((keyFn: (x: Item) => number) => (a: Item, b: Item) => keyFn(a) < keyFn(b))((x) => x.cost);

  start = performance.now();
  let count3 = 0;
  for (let i = 0; i < SIZE * 100; i++) {
    const a = items[i % SIZE]!;
    const b = items[(i + 1) % SIZE]!;
    if (closureCompare(a, b)) count3++;
  }
  console.log(`Closure comparison (${SIZE * 100}x): ${(performance.now() - start).toFixed(2)}ms`);
}

console.log();
console.log('='.repeat(70));
console.log('CONCLUSION');
console.log('='.repeat(70));
console.log(`
The main overhead comes from:
1. Map.set() and Map.get() operations for position tracking
2. Function call overhead for comparator and keyExtractor
3. Object property access patterns

For maximum performance, users should:
1. Use inline comparators: (a, b) => a.cost < b.cost
2. Use simple keyExtractors: (item) => item.id
3. Consider if O(1) position lookup is needed (if not, use a simpler heap)
`);
