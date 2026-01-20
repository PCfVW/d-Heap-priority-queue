Implement a d-ary heap priority queue in TypeScript based on the following type definitions and signatures.

Requirements:
1. Items have two distinct properties: an identity (for equality) and a priority (for ordering)
2. Two items are equal if they have the same identity, regardless of priority
3. The queue should support O(1) lookup to check if an item exists
4. Implement a min-heap where lower priority values have higher importance

/** Item represents an element in the priority queue. */
interface Item<K> {
    /** Identity field - determines equality */
    id: K;
    /** Priority field - determines heap ordering (lower = higher priority) */
    priority: number;
}

/** Function to extract the identity key from an item */
type KeyFn<T, K> = (item: T) => K;

/** Function to extract the priority from an item */
type PriorityFn<T> = (item: T) => number;

/** Comparator function for heap ordering */
type Comparator<T> = (a: T, b: T) => boolean;

/**
 * PriorityQueue is a d-ary min-heap with O(1) item lookup.
 * @typeParam T - The item type stored in the queue
 * @typeParam K - The key type used for identity (must be usable as Map key)
 */
class PriorityQueue<T, K = string | number> {
    private d: number;                    // arity
    private container: T[];               // heap array
    private positions: Map<K, number>;    // id -> index
    private keyFn: KeyFn<T, K>;           // extracts identity from item
    private priorityFn: PriorityFn<T>;    // extracts priority from item
    private comparator: Comparator<T>;    // returns true if a has higher priority than b

    /**
     * Creates a new priority queue.
     * @param d - Arity (children per node), must be >= 2
     * @param keyFn - Function to extract identity key from items
     * @param priorityFn - Function to extract priority from items
     * @param comparator - Returns true if first arg has higher priority
     */
    constructor(
        d: number,
        keyFn: KeyFn<T, K>,
        priorityFn: PriorityFn<T>,
        comparator: Comparator<T>
    );

    /** Adds an item to the queue. Throws if item with same key exists. */
    insert(item: T): void;

    /** Removes and returns the highest priority item. Returns undefined if empty. */
    pop(): T | undefined;

    /** Returns the highest priority item without removing it. Returns undefined if empty. */
    front(): T | undefined;

    /** Updates an item to have higher priority (lower value). Throws if not found. */
    increasePriority(item: T): void;

    /** Updates an item to have lower priority (higher value). Throws if not found. */
    decreasePriority(item: T): void;

    /** Returns true if an item with the given key exists. */
    contains(item: T): boolean;

    /** Returns the number of items in the queue. */
    len(): number;

    /** Returns true if the queue is empty. */
    isEmpty(): boolean;
}

Provide the complete implementation for all methods.