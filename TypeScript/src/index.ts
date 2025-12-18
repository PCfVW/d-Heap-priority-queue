/**
 * d-ary Heap Priority Queue - TypeScript Implementation
 *
 * A high-performance, generic d-ary heap priority queue with O(1) item lookup.
 *
 * @packageDocumentation
 * @module d-ary-heap
 * @version 2.0.0
 * @license Apache-2.0
 * @copyright 2023-2025 Eric Jacopin
 */

export { PriorityQueue } from './PriorityQueue';
export type {
  Position,
  Comparator,
  KeyExtractor,
  PriorityQueueOptions,
} from './PriorityQueue';

export {
  minBy,
  maxBy,
  minNumber,
  maxNumber,
  minString,
  maxString,
  reverse,
  chain,
} from './comparators';
