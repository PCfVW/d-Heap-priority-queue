type Item<T> = {
    identity: T;
    priority: number;
};

type HeapNode<T> = {
    item: Item<T>;
    index: number;
};

export class DaryHeapPriorityQueue<T> {
    private heap: HeapNode<T>[];
    private itemMap: Map<T, HeapNode<T>>;
    private d: number;

    constructor(d: number = 2) {
        if (d < 2) {
            throw new Error("Heap arity must be at least 2");
        }
        this.d = d;
        this.heap = [];
        this.itemMap = new Map();
    }

    /**
     * Insert an item into the priority queue
     * @param item The item to insert
     */
    insert(item: Item<T>): void {
        if (this.itemMap.has(item.identity)) {
            throw new Error("Item with this identity already exists in the queue");
        }

        const node: HeapNode<T> = { item, index: this.heap.length };
        this.heap.push(node);
        this.itemMap.set(item.identity, node);
        this.heapifyUp(node.index);
    }

    /**
     * Remove and return the item with highest priority (lowest value)
     * @returns The item with highest priority
     */
    pop(): Item<T> | undefined {
        if (this.heap.length === 0) {
            return undefined;
        }

        const root = this.heap[0];
        this.itemMap.delete(root.item.identity);

        if (this.heap.length === 1) {
            this.heap.pop();
            return root.item;
        }

        // Move last element to root
        const last = this.heap.pop()!;
        this.heap[0] = last;
        this.heap[0].index = 0;
        this.itemMap.set(last.item.identity, this.heap[0]);

        // Heapify down
        this.heapifyDown(0);

        return root.item;
    }

    /**
     * Return the item with highest priority without removing it
     * @returns The item with highest priority
     */
    front(): Item<T> | undefined {
        return this.heap[0]?.item;
    }

    /**
     * Update an existing item to have higher priority (lower value)
     * @param item The item to update
     * @param newPriority The new priority value
     */
    increasePriority(item: Item<T>, newPriority: number): void {
        this.updatePriority(item, newPriority, true);
    }

    /**
     * Update an existing item to have lower priority (higher value)
     * @param item The item to update
     * @param newPriority The new priority value
     */
    decreasePriority(item: Item<T>, newPriority: number): void {
        this.updatePriority(item, newPriority, false);
    }

    /**
     * Check if an item with the given identity exists
     * @param identity The identity to check
     * @returns True if the item exists, false otherwise
     */
    contains(identity: T): boolean {
        return this.itemMap.has(identity);
    }

    /**
     * Return the number of items in the queue
     * @returns The number of items
     */
    len(): number {
        return this.heap.length;
    }

    /**
     * Return whether the queue is empty
     * @returns True if the queue is empty, false otherwise
     */
    isEmpty(): boolean {
        return this.heap.length === 0;
    }

    private updatePriority(item: Item<T>, newPriority: number, isIncrease: boolean): void {
        const node = this.itemMap.get(item.identity);
        if (!node) {
            throw new Error("Item not found in the queue");
        }

        if (isIncrease && newPriority > node.item.priority) {
            throw new Error("New priority must be lower than current priority for increasePriority");
        }

        if (!isIncrease && newPriority < node.item.priority) {
            throw new Error("New priority must be higher than current priority for decreasePriority");
        }

        node.item.priority = newPriority;

        if (isIncrease) {
            this.heapifyUp(node.index);
        } else {
            this.heapifyDown(node.index);
        }
    }

    private heapifyUp(index: number): void {
        while (index > 0) {
            const parentIndex = this.getParentIndex(index);
            if (this.heap[index].item.priority >= this.heap[parentIndex].item.priority) {
                break;
            }
            this.swap(index, parentIndex);
            index = parentIndex;
        }
    }

    private heapifyDown(index: number): void {
        while (true) {
            const smallest = this.getSmallestChildIndex(index);
            if (smallest === -1 || this.heap[index].item.priority <= this.heap[smallest].item.priority) {
                break;
            }
            this.swap(index, smallest);
            index = smallest;
        }
    }

    private getSmallestChildIndex(index: number): number {
        const firstChildIndex = this.getFirstChildIndex(index);
        if (firstChildIndex >= this.heap.length) {
            return -1;
        }

        let smallestIndex = firstChildIndex;
        const lastChildIndex = Math.min(firstChildIndex + this.d, this.heap.length);

        for (let i = firstChildIndex + 1; i < lastChildIndex; i++) {
            if (this.heap[i].item.priority < this.heap[smallestIndex].item.priority) {
                smallestIndex = i;
            }
        }

        return smallestIndex;
    }

    private getParentIndex(index: number): number {
        return Math.floor((index - 1) / this.d);
    }

    private getFirstChildIndex(index: number): number {
        return index * this.d + 1;
    }

    private swap(i: number, j: number): void {
        const temp = this.heap[i];
        this.heap[i] = this.heap[j];
        this.heap[j] = temp;

        // Update the index references in the nodes
        this.heap[i].index = i;
        this.heap[j].index = j;

        // Update the map references
        this.itemMap.set(this.heap[i].item.identity, this.heap[i]);
        this.itemMap.set(this.heap[j].item.identity, this.heap[j]);
    }
}