type Comparator<T> = (a: T, b: T) => boolean;

interface PriorityQueueOptions<T, K> {
    d: number;
    comparator: Comparator<T>;
    keyExtractor: (item: T) => K;
}

export class PriorityQueue<T, K> {
    private heap: T[] = [];
    private keyToIndexMap: Map<K, number> = new Map();
    private options: PriorityQueueOptions<T, K>;

    constructor(options: PriorityQueueOptions<T, K>) {
        if (options.d < 2) {
            throw new Error("Heap arity (d) must be at least 2");
        }
        this.options = options;
    }

    len(): number {
        return this.heap.length;
    }

    is_empty(): boolean {
        return this.heap.length === 0;
    }

    contains(item: T): boolean {
        const key = this.options.keyExtractor(item);
        return this.keyToIndexMap.has(key);
    }

    containsKey(key: K): boolean {
        return this.keyToIndexMap.has(key);
    }

    insert(item: T): void {
        const key = this.options.keyExtractor(item);
        if (this.keyToIndexMap.has(key)) {
            throw new Error("Item with this key already exists in the heap");
        }

        this.heap.push(item);
        const index = this.heap.length - 1;
        this.keyToIndexMap.set(key, index);
        this.bubbleUp(index);
    }

    front(): T {
        if (this.is_empty()) {
            throw new Error("Heap is empty");
        }
        return this.heap[0];
    }

    pop(): T | undefined {
        if (this.is_empty()) {
            return undefined;
        }

        const root = this.heap[0];
        const rootKey = this.options.keyExtractor(root);

        if (this.heap.length === 1) {
            this.heap.pop();
            this.keyToIndexMap.delete(rootKey);
            return root;
        }

        // Move last element to root
        const lastItem = this.heap.pop()!;
        const lastKey = this.options.keyExtractor(lastItem);
        this.heap[0] = lastItem;
        this.keyToIndexMap.set(lastKey, 0);
        this.keyToIndexMap.delete(rootKey);

        // Bubble down the new root
        this.bubbleDown(0);

        return root;
    }

    increasePriority(item: T): void {
        const key = this.options.keyExtractor(item);
        const index = this.keyToIndexMap.get(key);
        if (index === undefined) {
            throw new Error("Item not found in heap");
        }

        const oldPriority = this.heap[index];
        this.heap[index] = item;

        // If the new priority is actually higher (lower value in min-heap)
        if (this.options.comparator(item, oldPriority)) {
            this.bubbleUp(index);
        } else {
            // If the new priority is lower (higher value in min-heap), we need to bubble down
            this.bubbleDown(index);
        }
    }

    decreasePriority(item: T): void {
        const key = this.options.keyExtractor(item);
        const index = this.keyToIndexMap.get(key);
        if (index === undefined) {
            throw new Error("Item not found in heap");
        }

        const oldPriority = this.heap[index];
        this.heap[index] = item;

        // If the new priority is actually lower (higher value in min-heap)
        if (!this.options.comparator(item, oldPriority) && !this.options.comparator(oldPriority, item)) {
            this.bubbleDown(index);
        } else {
            // If the new priority is higher (lower value in min-heap), we need to bubble up
            this.bubbleUp(index);
        }
    }

    private bubbleUp(index: number): void {
        while (index > 0) {
            const parentIndex = this.getParentIndex(index);
            if (this.options.comparator(this.heap[index], this.heap[parentIndex])) {
                this.swap(index, parentIndex);
                index = parentIndex;
            } else {
                break;
            }
        }
    }

    private bubbleDown(index: number): void {
        while (true) {
            const firstChildIndex = this.getFirstChildIndex(index);
            if (firstChildIndex >= this.heap.length) {
                break; // No children
            }

            // Find the smallest child
            let smallestChildIndex = firstChildIndex;
            const lastChildIndex = Math.min(
                this.getFirstChildIndex(index) + this.options.d,
                this.heap.length
            );

            for (let i = firstChildIndex + 1; i < lastChildIndex; i++) {
                if (this.options.comparator(this.heap[i], this.heap[smallestChildIndex])) {
                    smallestChildIndex = i;
                }
            }

            // If the current node is larger than the smallest child, swap
            if (this.options.comparator(this.heap[smallestChildIndex], this.heap[index])) {
                this.swap(index, smallestChildIndex);
                index = smallestChildIndex;
            } else {
                break;
            }
        }
    }

    private getParentIndex(childIndex: number): number {
        return Math.floor((childIndex - 1) / this.options.d);
    }

    private getFirstChildIndex(parentIndex: number): number {
        return parentIndex * this.options.d + 1;
    }

    private swap(i: number, j: number): void {
        const temp = this.heap[i];
        this.heap[i] = this.heap[j];
        this.heap[j] = temp;

        // Update the index map
        const keyI = this.options.keyExtractor(this.heap[i]);
        const keyJ = this.options.keyExtractor(this.heap[j]);
        this.keyToIndexMap.set(keyI, i);
        this.keyToIndexMap.set(keyJ, j);
    }
}