Here's the complete implementation of the d-ary heap priority queue in TypeScript:

```typescript
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
    ) {
        if (d < 2) {
            throw new Error("Arity must be >= 2");
        }
        this.d = d;
        this.container = [];
        this.positions = new Map();
        this.keyFn = keyFn;
        this.priorityFn = priorityFn;
        this.comparator = comparator;
    }

    /** Adds an item to the queue. Throws if item with same key exists. */
    insert(item: T): void {
        const key = this.keyFn(item);
        if (this.positions.has(key)) {
            throw new Error(`Item with key ${key} already exists`);
        }

        const index = this.container.length;
        this.container.push(item);
        this.positions.set(key, index);
        this.heapifyUp(index);
    }

    /** Removes and returns the highest priority item. Returns undefined if empty. */
    pop(): T | undefined {
        if (this.container.length === 0) {
            return undefined;
        }

        const result = this.container[0];
        const key = this.keyFn(result);
        this.positions.delete(key);

        if (this.container.length === 1) {
            this.container.pop();
            return result;
        }

        // Move last element to root and heapify down
        const lastItem = this.container.pop()!;
        this.container[0] = lastItem;
        this.positions.set(this.keyFn(lastItem), 0);
        this.heapifyDown(0);

        return result;
    }

    /** Returns the highest priority item without removing it. Returns undefined if empty. */
    front(): T | undefined {
        return this.container.length > 0 ? this.container[0] : undefined;
    }

    /** Updates an item to have higher priority (lower value). Throws if not found. */
    increasePriority(item: T): void {
        const key = this.keyFn(item);
        const index = this.positions.get(key);
        if (index === undefined) {
            throw new Error(`Item with key ${key} not found`);
        }

        this.container[index] = item;
        this.heapifyUp(index);
    }

    /** Updates an item to have lower priority (higher value). Throws if not found. */
    decreasePriority(item: T): void {
        const key = this.keyFn(item);
        const index = this.positions.get(key);
        if (index === undefined) {
            throw new Error(`Item with key ${key} not found`);
        }

        this.container[index] = item;
        this.heapifyDown(index);
    }

    /** Returns true if an item with the given key exists. */
    contains(item: T): boolean {
        const key = this.keyFn(item);
        return this.positions.has(key);
    }

    /** Returns the number of items in the queue. */
    len(): number {
        return this.container.length;
    }

    /** Returns true if the queue is empty. */
    isEmpty(): boolean {
        return this.container.length === 0;
    }

    /** Helper method to restore heap property by moving element up */
    private heapifyUp(index: number): void {
        while (index > 0) {
            const parentIndex = Math.floor((index - 1) / this.d);
            
            if (!this.comparator(this.container[index], this.container[parentIndex])) {
                break;
            }

            this.swap(index, parentIndex);
            index = parentIndex;
        }
    }

    /** Helper method to restore heap property by moving element down */
    private heapifyDown(index: number): void {
        while (true) {
            let bestIndex = index;
            const firstChild = index * this.d + 1;

            // Check all children to find the one with highest priority
            for (let i = 0; i < this.d; i++) {
                const childIndex = firstChild + i;
                if (childIndex >= this.container.length) {
                    break;
                }

                if (this.comparator(this.container[childIndex], this.container[bestIndex])) {
                    bestIndex = childIndex;
                }
            }

            if (bestIndex === index) {
                break;
            }

            this.swap(index, bestIndex);
            index = bestIndex;
        }
    }

    /** Helper method to swap two elements and update position map */
    private swap(i: number, j: number): void {
        const temp = this.container[i];
        this.container[i] = this.container[j];
        this.container[j] = temp;

        // Update position map
        this.positions.set(this.keyFn(this.container[i]), i);
        this.positions.set(this.keyFn(this.container[j]), j);
    }
}
```

## Key Implementation Details:

1. **D-ary Heap Structure**: 
   - Parent of node at index `i` is at `Math.floor((i-1)/d)`
   - First child of node at index `i` is at `i*d + 1`
   - Children are at indices `i*d + 1` through `i*d + d`

2. **O(1) Lookup**: The `positions` Map maintains the mapping from item identity to heap index, enabling constant-time existence checks.

3. **Heap Operations**:
   - `heapifyUp`: Restores heap property by moving an element up the tree
   - `heapifyDown`: Restores heap property by moving an element down the tree
   - Both operations maintain the position map during swaps

4. **Priority Updates**:
   - `increasePriority`: Updates item and calls `heapifyUp` (since higher priority means it should move up)
   - `decreasePriority`: Updates item and calls `heapifyDown` (since lower priority means it should move down)

5. **Error Handling**: Throws appropriate errors for duplicate insertions and operations on non-existent items.

The implementation maintains all heap invariants while providing the required O(1) lookup capability through the position map.