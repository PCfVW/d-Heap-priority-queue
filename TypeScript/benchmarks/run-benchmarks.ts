/**
 * Comprehensive benchmarks for d-ary heap priority queue.
 * Run with: npx tsx benchmarks/run-benchmarks.ts
 */

import { PriorityQueue, minBy, maxBy } from '../src';

interface Item {
  id: number;
  cost: number;
}

const createItem = (id: number, cost: number): Item => ({ id, cost });

// Utility to measure execution time
function benchmark(name: string, fn: () => void, iterations = 1): number {
  // Warmup
  fn();

  const times: number[] = [];
  for (let i = 0; i < iterations; i++) {
    const start = performance.now();
    fn();
    times.push(performance.now() - start);
  }

  const avg = times.reduce((a, b) => a + b, 0) / times.length;
  const min = Math.min(...times);
  const max = Math.max(...times);

  return avg;
}

function formatTime(ms: number): string {
  if (ms < 1) return `${(ms * 1000).toFixed(2)}µs`;
  if (ms < 1000) return `${ms.toFixed(2)}ms`;
  return `${(ms / 1000).toFixed(2)}s`;
}

console.log('='.repeat(70));
console.log('D-ARY HEAP PRIORITY QUEUE - PERFORMANCE BENCHMARKS');
console.log('='.repeat(70));
console.log();

// ============================================================================
// Benchmark 1: Insert Performance
// ============================================================================
console.log('BENCHMARK 1: INSERT PERFORMANCE');
console.log('-'.repeat(70));

const insertSizes = [1000, 10000, 100000, 500000];
const insertResults: Record<number, Record<number, number>> = {};

for (const size of insertSizes) {
  insertResults[size] = {};

  for (const d of [2, 4, 8]) {
    const time = benchmark(
      `Insert ${size} items (d=${d})`,
      () => {
        const pq = new PriorityQueue<Item, number>({
          d,
          comparator: minBy((item) => item.cost),
          keyExtractor: (item) => item.id,
          initialCapacity: size,
        });

        for (let i = 0; i < size; i++) {
          pq.insert(createItem(i, Math.random() * size));
        }
      },
      3
    );

    insertResults[size][d] = time;
  }
}

console.log('Insert times (ms):');
console.log('Size\t\td=2\t\td=4\t\td=8');
for (const size of insertSizes) {
  const r = insertResults[size];
  console.log(
    `${size}\t\t${formatTime(r[2])}\t\t${formatTime(r[4])}\t\t${formatTime(r[8])}`
  );
}
console.log();

// ============================================================================
// Benchmark 2: Pop Performance
// ============================================================================
console.log('BENCHMARK 2: POP PERFORMANCE (drain entire heap)');
console.log('-'.repeat(70));

const popSizes = [1000, 10000, 100000];
const popResults: Record<number, Record<number, number>> = {};

for (const size of popSizes) {
  popResults[size] = {};

  for (const d of [2, 4, 8]) {
    // Pre-build the heap
    const pq = new PriorityQueue<Item, number>({
      d,
      comparator: minBy((item) => item.cost),
      keyExtractor: (item) => item.id,
    });

    for (let i = 0; i < size; i++) {
      pq.insert(createItem(i, Math.random() * size));
    }

    // Measure pop time
    const time = benchmark(
      `Pop ${size} items (d=${d})`,
      () => {
        // Clone for each iteration
        const testPq = new PriorityQueue<Item, number>({
          d,
          comparator: minBy((item) => item.cost),
          keyExtractor: (item) => item.id,
        });
        for (let i = 0; i < size; i++) {
          testPq.insert(createItem(i, Math.random() * size));
        }

        while (!testPq.isEmpty()) {
          testPq.pop();
        }
      },
      3
    );

    popResults[size][d] = time;
  }
}

console.log('Pop times (ms) - includes rebuild:');
console.log('Size\t\td=2\t\td=4\t\td=8');
for (const size of popSizes) {
  const r = popResults[size];
  console.log(
    `${size}\t\t${formatTime(r[2])}\t\t${formatTime(r[4])}\t\t${formatTime(r[8])}`
  );
}
console.log();

// ============================================================================
// Benchmark 3: Priority Update Performance
// ============================================================================
console.log('BENCHMARK 3: PRIORITY UPDATE PERFORMANCE');
console.log('-'.repeat(70));

const updateSizes = [1000, 10000, 50000];
const updateResults: Record<number, { increase: number; decrease: number }> = {};

for (const size of updateSizes) {
  const pq = new PriorityQueue<Item, number>({
    d: 4,
    comparator: minBy((item) => item.cost),
    keyExtractor: (item) => item.id,
  });

  for (let i = 0; i < size; i++) {
    pq.insert(createItem(i, size / 2 + Math.random() * size));
  }

  // Measure increase priority
  const increaseTime = benchmark(
    `Increase priority ${size} times`,
    () => {
      for (let i = 0; i < size; i++) {
        pq.increasePriority(createItem(i, Math.random() * (size / 4)));
      }
    },
    3
  );

  // Reset priorities
  for (let i = 0; i < size; i++) {
    pq.decreasePriority(createItem(i, size / 2 + Math.random() * size));
  }

  // Measure decrease priority
  const decreaseTime = benchmark(
    `Decrease priority ${size} times`,
    () => {
      for (let i = 0; i < size; i++) {
        pq.decreasePriority(createItem(i, size + Math.random() * size));
      }
    },
    3
  );

  updateResults[size] = { increase: increaseTime, decrease: decreaseTime };
}

console.log('Priority update times (d=4):');
console.log('Size\t\tIncrease\tDecrease');
for (const size of updateSizes) {
  const r = updateResults[size];
  console.log(`${size}\t\t${formatTime(r.increase)}\t\t${formatTime(r.decrease)}`);
}
console.log();

// ============================================================================
// Benchmark 4: Mixed Workload (Dijkstra-like)
// ============================================================================
console.log('BENCHMARK 4: MIXED WORKLOAD (Dijkstra-like pattern)');
console.log('-'.repeat(70));

const mixedSizes = [10000, 50000, 100000];
const mixedResults: Record<number, Record<number, number>> = {};

for (const size of mixedSizes) {
  mixedResults[size] = {};

  for (const d of [2, 4, 8]) {
    const time = benchmark(
      `Mixed workload ${size} (d=${d})`,
      () => {
        const pq = new PriorityQueue<Item, number>({
          d,
          comparator: minBy((item) => item.cost),
          keyExtractor: (item) => item.id,
        });

        // Insert initial items
        for (let i = 0; i < size; i++) {
          pq.insert(createItem(i, Math.random() * size * 10));
        }

        // Simulate Dijkstra: pop min, update neighbors
        let processed = 0;
        while (!pq.isEmpty() && processed < size / 2) {
          const current = pq.pop()!;
          processed++;

          // Update ~3 "neighbors" still in queue
          for (let j = 0; j < 3; j++) {
            const neighborId = (current.id + j + 1) % size;
            if (pq.containsKey(neighborId)) {
              const newCost = current.cost + Math.random() * 10;
              pq.increasePriority(createItem(neighborId, newCost));
            }
          }
        }
      },
      3
    );

    mixedResults[size][d] = time;
  }
}

console.log('Mixed workload times:');
console.log('Size\t\td=2\t\td=4\t\td=8');
for (const size of mixedSizes) {
  const r = mixedResults[size];
  console.log(
    `${size}\t\t${formatTime(r[2])}\t\t${formatTime(r[4])}\t\t${formatTime(r[8])}`
  );
}
console.log();

// ============================================================================
// Benchmark 5: Throughput (ops/sec)
// ============================================================================
console.log('BENCHMARK 5: THROUGHPUT (operations per second)');
console.log('-'.repeat(70));

const throughputDuration = 1000; // 1 second per test

function measureThroughput(
  name: string,
  setup: () => PriorityQueue<Item, number>,
  operation: (pq: PriorityQueue<Item, number>, i: number) => void
): number {
  const pq = setup();
  let ops = 0;
  const start = performance.now();

  while (performance.now() - start < throughputDuration) {
    operation(pq, ops);
    ops++;
  }

  return ops;
}

const insertOps = measureThroughput(
  'Insert throughput',
  () =>
    new PriorityQueue<Item, number>({
      d: 4,
      comparator: minBy((item) => item.cost),
      keyExtractor: (item) => item.id,
    }),
  (pq, i) => pq.insert(createItem(i, Math.random() * 1000000))
);

console.log(`Insert throughput: ${(insertOps / 1000).toFixed(0)}k ops/sec`);

// Pop throughput (with refill)
let popOps = 0;
{
  const pq = new PriorityQueue<Item, number>({
    d: 4,
    comparator: minBy((item) => item.cost),
    keyExtractor: (item) => item.id,
  });

  // Pre-fill
  for (let i = 0; i < 10000; i++) {
    pq.insert(createItem(i, Math.random() * 10000));
  }

  const start = performance.now();
  let nextId = 10000;

  while (performance.now() - start < throughputDuration) {
    pq.pop();
    pq.insert(createItem(nextId++, Math.random() * 10000));
    popOps++;
  }
}

console.log(`Pop+Insert throughput: ${(popOps / 1000).toFixed(0)}k ops/sec`);

console.log();
console.log('='.repeat(70));
console.log('BENCHMARK COMPLETE');
console.log('='.repeat(70));
