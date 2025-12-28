/**
 * Pre-built comparator factories for common use cases.
 *
 * @module comparators
 * @version 2.3.0
 * @license Apache-2.0
 */

import type { Comparator } from './PriorityQueue';

/**
 * Create a min-heap comparator using a key extractor.
 * Lower key values have higher priority (appear closer to root).
 *
 * @param keyFn - Function to extract the comparable key from an item
 * @returns Comparator function for min-heap behavior
 *
 * @example
 * ```typescript
 * const minByCost = minBy<Item, number>(item => item.cost);
 * ```
 */
export function minBy<T, K>(keyFn: (item: T) => K): Comparator<T> {
  return (a: T, b: T) => keyFn(a) < keyFn(b);
}

/**
 * Create a max-heap comparator using a key extractor.
 * Higher key values have higher priority (appear closer to root).
 *
 * @param keyFn - Function to extract the comparable key from an item
 * @returns Comparator function for max-heap behavior
 *
 * @example
 * ```typescript
 * const maxByCost = maxBy<Item, number>(item => item.cost);
 * ```
 */
export function maxBy<T, K>(keyFn: (item: T) => K): Comparator<T> {
  return (a: T, b: T) => keyFn(a) > keyFn(b);
}

/**
 * Min-heap comparator for primitive number values.
 * Lower numbers have higher priority.
 */
export const minNumber: Comparator<number> = (a, b) => a < b;

/**
 * Max-heap comparator for primitive number values.
 * Higher numbers have higher priority.
 */
export const maxNumber: Comparator<number> = (a, b) => a > b;

/**
 * Min-heap comparator for primitive string values.
 * Lexicographically smaller strings have higher priority.
 */
export const minString: Comparator<string> = (a, b) => a < b;

/**
 * Max-heap comparator for primitive string values.
 * Lexicographically larger strings have higher priority.
 */
export const maxString: Comparator<string> = (a, b) => a > b;

/**
 * Create a comparator that reverses another comparator.
 *
 * @param cmp - Original comparator to reverse
 * @returns Reversed comparator
 *
 * @example
 * ```typescript
 * const maxByCost = reverse(minBy<Item, number>(item => item.cost));
 * ```
 */
export function reverse<T>(cmp: Comparator<T>): Comparator<T> {
  return (a: T, b: T) => cmp(b, a);
}

/**
 * Create a comparator that compares by multiple keys in order.
 * Falls back to subsequent comparators when items are equal.
 *
 * @param comparators - Array of comparators to apply in order
 * @returns Combined comparator
 *
 * @example
 * ```typescript
 * // Sort by priority first, then by timestamp
 * const cmp = chain(
 *   minBy<Task, number>(t => t.priority),
 *   minBy<Task, number>(t => t.timestamp)
 * );
 * ```
 */
export function chain<T>(...comparators: Comparator<T>[]): Comparator<T> {
  return (a: T, b: T) => {
    for (const cmp of comparators) {
      if (cmp(a, b)) return true;
      if (cmp(b, a)) return false;
    }
    return false;
  };
}
