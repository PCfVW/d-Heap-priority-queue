/**
 * Instrumentation utilities for d-ary heap performance analysis.
 *
 * This module provides opt-in instrumentation to count comparisons performed
 * during heap operations. It is designed for:
 *
 * - **Educational purposes**: Understanding the theoretical vs actual cost of heap operations
 * - **Benchmarking**: Measuring real comparison counts across different arities
 * - **Visualization**: Powering interactive demos that show heap behavior
 *
 * ## Design Philosophy: Zero-Cost When Disabled
 *
 * Instrumentation follows these principles:
 *
 * 1. **Opt-in only**: No overhead when not using instrumentation
 * 2. **Non-breaking**: Existing code continues to work unchanged
 * 3. **Per-operation tracking**: Distinguish insert/pop/decreasePriority comparisons
 *
 * ## Cross-Language Consistency
 *
 * Currently, instrumentation is implemented in TypeScript only. The table below
 * shows the idiomatic zero-cost approach for each language, planned for v2.5.0:
 *
 * | Language   | Mechanism                        | Overhead When Disabled | Status |
 * |------------|----------------------------------|------------------------|--------|
 * | TypeScript | Optional hooks + instrumented comparator | Zero (JIT optimization) | ✅ Implemented |
 * | Go         | Nil stats pointer                | ~1 cycle (nil check)   | Planned v2.5.0 |
 * | Rust       | Generic over StatsCollector trait | Zero (monomorphization) | Planned v2.5.0 |
 * | C++        | Template policy class            | Zero (inlining)        | Planned v2.5.0 |
 * | Zig        | Comptime bool parameter          | Zero (branch elimination) | Planned v2.5.0 |
 *
 * ## Usage Example
 *
 * ```typescript
 * import { PriorityQueue, minBy, instrumentComparator } from 'd-ary-heap';
 *
 * // 1. Wrap your comparator with instrumentation
 * const comparator = instrumentComparator(minBy((v: Vertex) => v.distance));
 *
 * // 2. Create priority queue with operation hooks
 * const pq = new PriorityQueue({
 *   d: 4,
 *   comparator,
 *   keyExtractor: (v) => v.id,
 *   onBeforeOperation: (op) => comparator.startOperation(op),
 *   onAfterOperation: () => comparator.endOperation(),
 * });
 *
 * // 3. Use normally - comparisons are tracked automatically
 * pq.insert({ id: 'A', distance: 0 });
 * pq.insert({ id: 'B', distance: 5 });
 * pq.pop();
 *
 * // 4. Access statistics
 * console.log(comparator.stats);
 * // { insert: 1, pop: 2, decreasePriority: 0, increasePriority: 0, updatePriority: 0, total: 3 }
 *
 * // 5. Reset for next measurement
 * comparator.stats.reset();
 * ```
 *
 * ## Theoretical Complexity Reference
 *
 * For a d-ary heap with n elements:
 *
 * | Operation        | Comparisons (worst case)      |
 * |------------------|-------------------------------|
 * | insert           | ⌊log_d(n)⌋                    |
 * | pop              | d × ⌊log_d(n)⌋                |
 * | increasePriority | ⌊log_d(n)⌋ (moveUp only)      |
 * | decreasePriority | d × ⌊log_d(n)⌋ (moveDown only)|
 * | updatePriority   | (d+1) × ⌊log_d(n)⌋ (both)     |
 *
 * The demo visualization compares actual counts against these theoretical bounds.
 *
 * @module instrumentation
 * @version 2.4.0
 * @license Apache-2.0
 */

import type { Comparator } from './PriorityQueue';

/**
 * Operation types that can be tracked.
 *
 * Note: `updatePriority` is tracked separately for when the caller doesn't know
 * whether priority increased or decreased (checks both directions).
 */
export type OperationType = 'insert' | 'pop' | 'decreasePriority' | 'increasePriority' | 'updatePriority';

/**
 * Statistics tracking comparison counts per operation type.
 *
 * All counts start at zero and accumulate until `reset()` is called.
 */
export interface ComparisonStats {
  /** Comparisons during insert operations (moveUp) */
  insert: number;

  /** Comparisons during pop operations (moveDown + bestChildPosition) */
  pop: number;

  /** Comparisons during decreasePriority operations (moveDown only) */
  decreasePriority: number;

  /** Comparisons during increasePriority operations (moveUp only) */
  increasePriority: number;

  /** Comparisons during updatePriority operations (moveUp + moveDown) */
  updatePriority: number;

  /** Total comparisons across all operation types */
  readonly total: number;

  /** Reset all counters to zero */
  reset(): void;
}

/**
 * An instrumented comparator that tracks comparison counts.
 *
 * This extends a regular comparator with:
 * - `stats`: Current comparison counts
 * - `startOperation(type)`: Begin tracking for an operation
 * - `endOperation()`: Stop tracking current operation
 *
 * The comparator itself remains a valid `Comparator<T>` and can be used
 * anywhere a regular comparator is expected.
 */
export interface InstrumentedComparator<T> extends Comparator<T> {
  /** Current comparison statistics */
  readonly stats: ComparisonStats;

  /**
   * Signal the start of a heap operation.
   * Comparisons will be attributed to this operation type until `endOperation()`.
   *
   * @param type - The operation type being started
   */
  startOperation(type: OperationType): void;

  /**
   * Signal the end of the current heap operation.
   * Subsequent comparisons will not be counted until the next `startOperation()`.
   */
  endOperation(): void;
}

/**
 * Create comparison statistics tracker.
 *
 * @returns Fresh stats object with all counts at zero
 *
 * @example
 * ```typescript
 * const stats = createComparisonStats();
 * stats.insert = 5;
 * stats.pop = 10;
 * console.log(stats.total); // 15
 * stats.reset();
 * console.log(stats.total); // 0
 * ```
 */
export function createComparisonStats(): ComparisonStats {
  const stats: ComparisonStats = {
    insert: 0,
    pop: 0,
    decreasePriority: 0,
    increasePriority: 0,
    updatePriority: 0,

    get total(): number {
      return this.insert + this.pop + this.decreasePriority + this.increasePriority + this.updatePriority;
    },

    reset(): void {
      this.insert = 0;
      this.pop = 0;
      this.decreasePriority = 0;
      this.increasePriority = 0;
      this.updatePriority = 0;
    },
  };

  return stats;
}

/**
 * Wrap a comparator with instrumentation to track comparison counts.
 *
 * The returned comparator:
 * - Behaves identically to the original for comparison purposes
 * - Tracks how many times it's called, attributed to operation types
 * - Has zero overhead when `startOperation()` hasn't been called
 *
 * ## How It Works
 *
 * 1. Call `startOperation('insert')` before `pq.insert()`
 * 2. The comparator increments `stats.insert` for each comparison
 * 3. Call `endOperation()` after the operation completes
 * 4. Repeat for other operations
 *
 * The `PriorityQueue` class supports `onBeforeOperation` and `onAfterOperation`
 * hooks to automate this.
 *
 * ## Performance Note
 *
 * When `currentOperation` is null (between operations), the instrumented
 * comparator performs only a single null check before calling the original.
 * Modern JavaScript engines optimize this extremely well.
 *
 * @param comparator - The original comparator to instrument
 * @returns An instrumented comparator with stats tracking
 *
 * @example
 * ```typescript
 * import { minBy, instrumentComparator } from 'd-ary-heap';
 *
 * const cmp = instrumentComparator(minBy<number, number>(x => x));
 *
 * // Manual usage (without hooks)
 * cmp.startOperation('insert');
 * console.log(cmp(5, 3)); // false, and stats.insert++
 * console.log(cmp(3, 5)); // true, and stats.insert++
 * cmp.endOperation();
 *
 * console.log(cmp.stats.insert); // 2
 * ```
 */
export function instrumentComparator<T>(comparator: Comparator<T>): InstrumentedComparator<T> {
  const stats = createComparisonStats();
  let currentOperation: OperationType | null = null;

  // Create the instrumented function
  const instrumented = ((a: T, b: T): boolean => {
    // Only count when actively tracking an operation
    if (currentOperation !== null) {
      stats[currentOperation]++;
    }
    return comparator(a, b);
  }) as InstrumentedComparator<T>;

  // Attach stats (read-only from outside)
  Object.defineProperty(instrumented, 'stats', {
    value: stats,
    writable: false,
    enumerable: true,
  });

  // Attach operation control methods
  instrumented.startOperation = (type: OperationType): void => {
    currentOperation = type;
  };

  instrumented.endOperation = (): void => {
    currentOperation = null;
  };

  return instrumented;
}

/**
 * Calculate theoretical comparison count for an insert operation.
 *
 * Insert performs at most ⌊log_d(n)⌋ comparisons (one per level during moveUp).
 *
 * @param n - Number of elements in heap AFTER insert
 * @param d - Heap arity
 * @returns Theoretical worst-case comparison count
 */
export function theoreticalInsertComparisons(n: number, d: number): number {
  if (n <= 1) return 0;
  return Math.floor(Math.log(n) / Math.log(d));
}

/**
 * Calculate theoretical comparison count for a pop operation.
 *
 * Pop performs at most d × ⌊log_d(n)⌋ comparisons:
 * - At each level, find best among d children (d-1 comparisons)
 * - Compare best child with current (1 comparison)
 * - Total: d comparisons per level × ⌊log_d(n)⌋ levels
 *
 * @param n - Number of elements in heap BEFORE pop
 * @param d - Heap arity
 * @returns Theoretical worst-case comparison count
 */
export function theoreticalPopComparisons(n: number, d: number): number {
  if (n <= 1) return 0;
  const height = Math.floor(Math.log(n) / Math.log(d));
  return d * height;
}

/**
 * Calculate theoretical comparison count for an increasePriority operation.
 *
 * IncreasePriority performs only moveUp (item became more important).
 * Worst case: ⌊log_d(n)⌋ comparisons (one per level).
 *
 * @param n - Number of elements in heap
 * @param d - Heap arity
 * @returns Theoretical worst-case comparison count
 */
export function theoreticalIncreasePriorityComparisons(n: number, d: number): number {
  if (n <= 1) return 0;
  return Math.floor(Math.log(n) / Math.log(d));
}

/**
 * Calculate theoretical comparison count for a decreasePriority operation.
 *
 * DecreasePriority performs only moveDown (item became less important).
 * Worst case: d × ⌊log_d(n)⌋ comparisons.
 *
 * @param n - Number of elements in heap
 * @param d - Heap arity
 * @returns Theoretical worst-case comparison count
 */
export function theoreticalDecreasePriorityComparisons(n: number, d: number): number {
  if (n <= 1) return 0;
  const height = Math.floor(Math.log(n) / Math.log(d));
  return d * height;
}

/**
 * Calculate theoretical comparison count for an updatePriority operation.
 *
 * UpdatePriority performs both moveUp and moveDown (direction unknown).
 * Worst case: (d + 1) × ⌊log_d(n)⌋ comparisons.
 *
 * @param n - Number of elements in heap
 * @param d - Heap arity
 * @returns Theoretical worst-case comparison count
 */
export function theoreticalUpdatePriorityComparisons(n: number, d: number): number {
  if (n <= 1) return 0;
  const height = Math.floor(Math.log(n) / Math.log(d));
  return (d + 1) * height;
}
