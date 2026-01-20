type Identity = string | number | symbol;
type Priority = number;

interface HeapItem {
    identity: Identity;
    priority: Priority;
}

class DaryHeap {
    private heap: HeapItem[];
    private positionMap: Map<Identity, number>;
    private d: number;

    constructor(d: number) {
        if (d < 2) {
            throw new Error("Ary must be at least 2");
        }
        this.d = d;
        this.heap = [];
        this.positionMap = new Map();
    }

    // Public API methods

    insert(item: HeapItem): void {
        if (this.positionMap.has(item.identity)) {
            throw new Error("Item with this identity already exists in the heap");
        }

        this.heap.push(item);
        this.positionMap.set(item.identity, this.heap.length - 1);
        this.siftUp(this.heap.length - 1);
    }

    pop(): HeapItem | null {
        if (this.heap.length === 0) {
            return null;
        }

        const root = this.heap[0];
        this.positionMap.delete(root.identity);

        if (this.heap.length > 1) {
            // Move last element to root
            const last = this.heap.pop()!;
            this.heap[0] = last;
            this.positionMap.set(last.identity, 0);
            this.siftDown(0);
        } else {
            this.heap.pop();
        }

        return root;
    }

    front(): HeapItem | null {
        return this.heap.length > 0 ? this.heap[0] : null;
    }

    increasePriority(item: HeapItem): void {
        const index = this.positionMap.get(item.identity);
        if (index === undefined) {
            throw new Error("Item not found in heap");
        }

        if (item.priority > this.heap[index].priority) {
            throw new Error("New priority is not higher (lower value) than current priority");
        }

        this.heap[index].priority = item.priority;
        this.siftUp(index);
    }

    decreasePriority(item: HeapItem): void {
        const index = this.positionMap.get(item.identity);
        if (index === undefined) {
            throw new Error("Item not found in heap");
        }

        if (item.priority < this.heap[index].priority) {
            throw new Error("New priority is not lower (higher value) than current priority");
        }

        this.heap[index].priority = item.priority;
        this.siftDown(index);
    }

    contains(item: HeapItem): boolean {
        return this.positionMap.has(item.identity);
    }

    len(): number {
        return this.heap.length;
    }

    isEmpty(): boolean {
        return this.heap.length === 0;
    }

    // Private helper methods

    private siftUp(index: number): void {
        while (index > 0) {
            const parentIndex = Math.floor((index - 1) / this.d);
            if (this.heap[index].priority >= this.heap[parentIndex].priority) {
                break;
            }

            // Swap with parent
            this.swap(index, parentIndex);
            index = parentIndex;
        }
    }

    private siftDown(index: number): void {
        while (true) {
            const firstChildIndex = this.d * index + 1;
            if (firstChildIndex >= this.heap.length) {
                break; // No children
            }

            // Find the child with minimum priority
            let minChildIndex = firstChildIndex;
            const lastChildIndex = Math.min(this.d * index + this.d, this.heap.length - 1);

            for (let i = firstChildIndex + 1; i <= lastChildIndex; i++) {
                if (this.heap[i].priority < this.heap[minChildIndex].priority) {
                    minChildIndex = i;
                }
            }

            if (this.heap[index].priority <= this.heap[minChildIndex].priority) {
                break; // Heap property is satisfied
            }

            // Swap with the minimum child
            this.swap(index, minChildIndex);
            index = minChildIndex;
        }
    }

    private swap(i: number, j: number): void {
        // Update position map
        this.positionMap.set(this.heap[i].identity, j);
        this.positionMap.set(this.heap[j].identity, i);

        // Swap in heap array
        [this.heap[i], this.heap[j]] = [this.heap[j], this.heap[i]];
    }
}