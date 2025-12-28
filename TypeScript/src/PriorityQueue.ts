/**
 * d-ary Heap Priority Queue - TypeScript Implementation
 *
 * A generic d-ary heap (d-heap) priority queue with:
 * - Configurable arity (d): number of children per node
 * - Min-heap or max-heap behavior via comparator functions
 * - O(1) item lookup using Map for efficient priority updates
 * - O(1) access to highest-priority item
 * - O(log_d n) insert and priority increase operations
 * - O(d · log_d n) pop and priority decrease operations
 *
 * @version 2.3.0
 * @license Apache-2.0
 * @copyright 2023-2025 Eric Jacopin
 */

/** Type alias for position indices (cross-language consistency) */
export type Position = number;

/**
 * Comparator function type for priority comparison.
 * Returns true if `a` has higher priority than `b`.
 */
export type Comparator<T> = (a: T, b: T) => boolean;

/**
 * Key extractor function type for identity-based lookup.
 * Must return a value that can be used as a Map key (string or number recommended).
 */
export type KeyExtractor<T, K> = (item: T) => K;

/**
 * Configuration options for PriorityQueue construction.
 */
export interface PriorityQueueOptions<T, K> {
  /** Number of children per node (arity). Must be >= 1. Default: 2 */
  d?: number;
  /** Comparator function. Returns true if first arg has higher priority. */
  comparator: Comparator<T>;
  /** Key extractor for identity-based lookup. Required for decrease/increase priority. */
  keyExtractor: KeyExtractor<T, K>;
  /** Initial capacity hint for pre-allocation */
  initialCapacity?: number;
}

/**
 * Generic d-ary heap priority queue with O(1) lookup.
 *
 * A d-ary heap is a tree structure where:
 * - Each node has at most d children
 * - The root contains the highest-priority item
 * - Each parent has higher priority than all its children
 * - The tree is complete (filled left-to-right, level by level)
 *
 * This implementation uses an array-based representation with O(1) item lookup
 * via a Map that tracks each item's position in the heap.
 *
 * ## Time Complexities
 * - front(), peek(): O(1)
 * - insert(): O(log_d n)
 * - pop(): O(d · log_d n)
 * - increasePriority(): O(log_d n)
 * - decreasePriority(): O(d · log_d n)
 * - contains(): O(1)
 * - len(), isEmpty(), d(): O(1)
 *
 * @typeParam T - Item type stored in the queue
 * @typeParam K - Key type for identity lookup (typically string or number)
 */
export class PriorityQueue<T, K = string | number> {
  /** Array-based heap storage (complete tree representation) */
  private container: T[];

  /** Maps each item's key to its position in the container for O(1) lookup */
  private positions: Map<K, Position>;

  /** Number of children per node (arity of the heap) */
  private depth: number;

  /** Comparator determining heap order (min vs max) */
  private readonly comparator: Comparator<T>;

  /** Key extractor for identity-based lookup */
  private readonly keyExtractor: KeyExtractor<T, K>;

  /**
   * Create a new d-ary heap priority queue.
   *
   * @param options - Configuration options
   * @throws Error if d < 1
   *
   * @example
   * ```typescript
   * // Min-heap by cost
   * const pq = new PriorityQueue<Item, number>({
   *   d: 4,
   *   comparator: (a, b) => a.cost < b.cost,
   *   keyExtractor: (item) => item.id
   * });
   * ```
   */
  constructor(options: PriorityQueueOptions<T, K>) {
    const d = options.d ?? 2;
    if (d < 1) {
      throw new Error('Heap arity (d) must be >= 1');
    }

    this.depth = d;
    this.comparator = options.comparator;
    this.keyExtractor = options.keyExtractor;

    // Pre-allocate if capacity hint provided
    const capacity = options.initialCapacity ?? 0;
    this.container = capacity > 0 ? new Array<T>(capacity) : [];
    if (capacity > 0) this.container.length = 0; // Reset length but keep capacity
    this.positions = new Map<K, Position>();
  }

  /**
   * Create a new priority queue with an initial item already inserted.
   * Equivalent to Rust's `with_first()` constructor.
   *
   * @param options - Configuration options
   * @param firstItem - First item to insert
   * @returns New PriorityQueue with the item already inserted
   */
  static withFirst<T, K>(
    options: PriorityQueueOptions<T, K>,
    firstItem: T
  ): PriorityQueue<T, K> {
    const pq = new PriorityQueue<T, K>(options);
    pq.insert(firstItem);
    return pq;
  }

  // ===========================================================================
  // Public API - Query Operations
  // ===========================================================================

  /**
   * Get the number of items in the heap.
   * Time complexity: O(1)
   */
  len(): number {
    return this.container.length;
  }

  /** Alias for len() - backward compatibility */
  get size(): number {
    return this.container.length;
  }

  /**
   * Check if the heap is empty.
   * Time complexity: O(1)
   */
  isEmpty(): boolean {
    return this.container.length === 0;
  }

  /** Alias for isEmpty() - snake_case for cross-language consistency */
  is_empty(): boolean {
    return this.isEmpty();
  }

  /**
   * Get the arity (number of children per node) of the heap.
   * Time complexity: O(1)
   */
  d(): number {
    return this.depth;
  }

  /**
   * Check if an item with the given key exists in the heap.
   * Time complexity: O(1)
   *
   * @param item - Item to check (uses keyExtractor for identity)
   */
  contains(item: T): boolean {
    return this.positions.has(this.keyExtractor(item));
  }

  /**
   * Check if an item with the given key exists in the heap.
   * Time complexity: O(1)
   *
   * @param key - Key to check directly
   */
  containsKey(key: K): boolean {
    return this.positions.has(key);
  }

  /**
   * Get the current position (index) of an item in the heap.
   * Time complexity: O(1)
   *
   * @param item - Item to find (uses keyExtractor for identity)
   * @returns Position index, or undefined if not found
   */
  getPosition(item: T): Position | undefined {
    return this.positions.get(this.keyExtractor(item));
  }

  /**
   * Get the current position (index) of an item by its key.
   * Time complexity: O(1)
   *
   * @param key - Key to find
   * @returns Position index, or undefined if not found
   */
  getPositionByKey(key: K): Position | undefined {
    return this.positions.get(key);
  }

  /**
   * Get the highest-priority item without removing it.
   * Time complexity: O(1)
   *
   * @returns The highest-priority item
   * @throws Error if heap is empty
   */
  front(): T {
    const item = this.container[0];
    if (item === undefined) {
      throw new Error('front() called on empty priority queue');
    }
    return item;
  }

  /**
   * Get the highest-priority item without removing it.
   * Safe alternative to front().
   * Time complexity: O(1)
   *
   * @returns The highest-priority item, or undefined if empty
   */
  peek(): T | undefined {
    return this.container.length > 0 ? this.container[0] : undefined;
  }

  // ===========================================================================
  // Public API - Modification Operations
  // ===========================================================================

  /**
   * Insert a new item into the heap.
   * Time complexity: O(log_d n)
   *
   * @param item - Item to insert
   *
   * @remarks
   * If an item with the same key already exists, behavior is undefined.
   * Use contains() to check first, or use increasePriority()/decreasePriority()
   * to update existing items.
   */
  insert(item: T): void {
    const index = this.container.length;
    this.container.push(item);

    // Fast path: first item doesn't need sift-up
    if (index === 0) {
      this.positions.set(this.keyExtractor(item), 0);
      return;
    }

    this.positions.set(this.keyExtractor(item), index);
    this.moveUp(index);
  }

  /**
   * Insert multiple items into the heap.
   * Uses heapify algorithm which is O(n) for bulk insertion vs O(n log n) for individual inserts.
   * Time complexity: O(n) where n is total items after insertion
   *
   * @param items - Array of items to insert
   *
   * @remarks
   * More efficient than calling insert() repeatedly when adding many items at once.
   * If any item has a key that already exists, behavior is undefined.
   */
  insertMany(items: T[]): void {
    if (items.length === 0) return;

    const keyExtractor = this.keyExtractor;
    const container = this.container;
    const positions = this.positions;
    const startIndex = container.length;

    // Add all items to container and positions map
    for (let i = 0; i < items.length; i++) {
      const item = items[i]!;
      container.push(item);
      positions.set(keyExtractor(item), startIndex + i);
    }

    // If this was an empty heap, use heapify (O(n)) instead of n insertions (O(n log n))
    if (startIndex === 0 && items.length > 1) {
      this.heapify();
    } else {
      // Otherwise, sift up each new item
      for (let i = startIndex; i < container.length; i++) {
        this.moveUp(i);
      }
    }
  }

  /**
   * Build heap property from unordered array.
   * Uses Floyd's algorithm - O(n) time complexity.
   * Called internally by insertMany when starting from empty heap.
   */
  private heapify(): void {
    const n = this.container.length;
    if (n <= 1) return;

    const d = this.depth;
    // Start from last non-leaf node and sift down each
    // Last non-leaf is parent of last element: floor((n-2)/d)
    const lastNonLeaf = ((n - 2) / d) | 0;

    for (let i = lastNonLeaf; i >= 0; i--) {
      this.moveDown(i);
    }
  }

  /**
   * Increase the priority of an existing item (move toward root).
   * Time complexity: O(log_d n)
   *
   * @param updatedItem - Item with same identity but updated priority
   * @throws Error if item not found
   *
   * @remarks
   * For min-heap: decreasing the priority value increases importance.
   * For max-heap: increasing the priority value increases importance.
   * This method only moves items upward for performance.
   */
  increasePriority(updatedItem: T): void {
    const key = this.keyExtractor(updatedItem);
    const index = this.positions.get(key);

    if (index === undefined) {
      throw new Error('Item not found in priority queue');
    }

    this.container[index] = updatedItem;
    this.moveUp(index);
  }

  /** Alias for increasePriority() - snake_case for cross-language consistency */
  increase_priority(updatedItem: T): void {
    this.increasePriority(updatedItem);
  }

  /**
   * Increase the priority of the item at the given index.
   * Time complexity: O(log_d n)
   *
   * @param index - Index of the item in the heap array
   * @throws Error if index is out of bounds
   *
   * @remarks
   * This is a lower-level method. Prefer increasePriority() with the item itself.
   */
  increasePriorityByIndex(index: number): void {
    if (index < 0 || index >= this.container.length) {
      throw new Error('Index out of bounds');
    }
    this.moveUp(index);
  }

  /** Alias for increasePriorityByIndex() - snake_case for cross-language consistency */
  increase_priority_by_index(index: number): void {
    this.increasePriorityByIndex(index);
  }

  /**
   * Decrease the priority of an existing item (move toward leaves).
   * Time complexity: O(d · log_d n)
   *
   * @param updatedItem - Item with same identity but updated priority
   * @throws Error if item not found
   *
   * @remarks
   * For min-heap: increasing the priority value decreases importance.
   * For max-heap: decreasing the priority value decreases importance.
   * This method checks both directions for robustness.
   */
  decreasePriority(updatedItem: T): void {
    const key = this.keyExtractor(updatedItem);
    const index = this.positions.get(key);

    if (index === undefined) {
      throw new Error('Item not found in priority queue');
    }

    this.container[index] = updatedItem;
    // Check both directions since we don't know if priority actually decreased
    this.moveUp(index);
    this.moveDown(index);
  }

  /** Alias for decreasePriority() - snake_case for cross-language consistency */
  decrease_priority(updatedItem: T): void {
    this.decreasePriority(updatedItem);
  }

  /**
   * Remove and return the highest-priority item.
   * Time complexity: O(d · log_d n)
   *
   * @returns The removed item, or undefined if empty
   */
  pop(): T | undefined {
    const container = this.container;
    const n = container.length;

    if (n === 0) {
      return undefined;
    }

    const keyExtractor = this.keyExtractor;
    const top = container[0]!;
    this.positions.delete(keyExtractor(top));

    if (n === 1) {
      container.length = 0;
      return top;
    }

    // Move last item to root and sift down
    const lastItem = container[n - 1]!;
    container[0] = lastItem;
    this.positions.set(keyExtractor(lastItem), 0);
    container.length = n - 1;

    this.moveDown(0);

    return top;
  }

  /**
   * Remove and return multiple highest-priority items.
   * More efficient than calling pop() repeatedly.
   * Time complexity: O(count · d · log_d n)
   *
   * @param count - Number of items to remove
   * @returns Array of removed items in priority order
   */
  popMany(count: number): T[] {
    const result: T[] = [];
    const actualCount = count < this.container.length ? count : this.container.length;

    for (let i = 0; i < actualCount; i++) {
      const item = this.pop();
      if (item !== undefined) {
        result.push(item);
      }
    }

    return result;
  }

  /**
   * Clear all items from the heap, optionally changing the arity.
   * Time complexity: O(1) (references cleared, GC handles memory)
   *
   * @param newD - Optional new arity value (must be >= 1 if provided)
   * @throws Error if newD < 1
   */
  clear(newD?: number): void {
    this.container.length = 0;
    this.positions.clear();

    if (newD !== undefined) {
      if (newD < 1) {
        throw new Error('Heap arity (d) must be >= 1');
      }
      this.depth = newD;
    }
  }

  /**
   * Get a string representation of the heap contents.
   * Time complexity: O(n)
   *
   * @returns Formatted string showing all items in heap order
   */
  toString(): string {
    return '{' + this.container.map(String).join(', ') + '}';
  }

  /** Alias for toString() - snake_case for cross-language consistency */
  to_string(): string {
    return this.toString();
  }

  /**
   * Get all items in heap order (for debugging/iteration).
   * Time complexity: O(n) - creates a copy
   *
   * @returns Copy of internal array
   */
  toArray(): T[] {
    return [...this.container];
  }

  /**
   * Iterate over items in heap order (not priority order).
   */
  *[Symbol.iterator](): Iterator<T> {
    for (const item of this.container) {
      yield item;
    }
  }

  // ===========================================================================
  // Private Methods - Heap Operations
  // ===========================================================================

  /**
   * Swap two items in the heap and update their position mappings.
   * V8 optimizes simple swap patterns well.
   */
  private swap(i: number, j: number): void {
    const container = this.container;
    const temp = container[i]!;
    container[i] = container[j]!;
    container[j] = temp;

    // Update positions
    this.positions.set(this.keyExtractor(container[i]!), i);
    this.positions.set(this.keyExtractor(container[j]!), j);
  }

  /**
   * Find the child with highest priority among all children of node i.
   */
  private bestChildPosition(i: number): number {
    const d = this.depth;
    const container = this.container;
    const n = container.length;
    const left = i * d + 1;

    if (left >= n) return left;

    let best = left;
    const right = Math.min((i + 1) * d, n - 1);

    for (let j = left + 1; j <= right; j++) {
      if (this.comparator(container[j]!, container[best]!)) {
        best = j;
      }
    }

    return best;
  }

  /**
   * Move an item upward in the heap to restore heap property.
   * Uses simple swap pattern which V8 optimizes well.
   */
  private moveUp(i: number): void {
    const d = this.depth;
    const container = this.container;

    while (i > 0) {
      const p = Math.floor((i - 1) / d);
      if (this.comparator(container[i]!, container[p]!)) {
        this.swap(i, p);
        i = p;
      } else {
        break;
      }
    }
  }

  /**
   * Move an item downward in the heap to restore heap property.
   */
  private moveDown(i: number): void {
    const d = this.depth;
    const container = this.container;

    while (true) {
      const firstChild = i * d + 1;
      if (firstChild >= container.length) break;

      const best = this.bestChildPosition(i);
      if (this.comparator(container[best]!, container[i]!)) {
        this.swap(i, best);
        i = best;
      } else {
        break;
      }
    }
  }
}
