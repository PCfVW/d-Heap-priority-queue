class PriorityQueue<T, K = string | number> {
    private d: number;
    private container: T[] = [];
    private positions: Map<K, number> = new Map();
    private keyFn: KeyFn<T, K>;
    private priorityFn: PriorityFn<T>;
    private comparator: Comparator<T>;

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
        this.keyFn = keyFn;
        this.priorityFn = priorityFn;
        this.comparator = comparator;
    }

    insert(item: T): void {
        const key = this.keyFn(item);
        if (this.positions.has(key)) {
            throw new Error(`Item with key ${key} already exists`);
        }

        this.positions.set(key, this.container.length);
        this.container.push(item);
        this.bubbleUp(this.container.length - 1);
    }

    pop(): T | undefined {
        if (this.container.length === 0) {
            return undefined;
        }

        const root = this.container[0];
        const last = this.container.pop()!;
        const lastKey = this.keyFn(last);

        if (this.container.length > 0) {
            this.container[0] = last;
            this.positions.set(lastKey, 0);
            this.bubbleDown(0);
        }

        this.positions.delete(this.keyFn(root));
        return root;
    }

    front(): T | undefined {
        return this.container.length > 0 ? this.container[0] : undefined;
    }

    increasePriority(item: T): void {
        this.updatePriority(item, true);
    }

    decreasePriority(item: T): void {
        this.updatePriority(item, false);
    }

    contains(item: T): boolean {
        return this.positions.has(this.keyFn(item));
    }

    len(): number {
        return this.container.length;
    }

    isEmpty(): boolean {
        return this.container.length === 0;
    }

    private updatePriority(item: T, isIncrease: boolean): void {
        const key = this.keyFn(item);
        const index = this.positions.get(key);

        if (index === undefined) {
            throw new Error(`Item with key ${key} not found`);
        }

        const oldPriority = this.priorityFn(this.container[index]);
        const newPriority = this.priorityFn(item);

        if (isIncrease ? newPriority > oldPriority : newPriority < oldPriority) {
            throw new Error(`Priority update would ${isIncrease ? 'decrease' : 'increase'} priority`);
        }

        this.container[index] = item;

        if (isIncrease) {
            this.bubbleUp(index);
        } else {
            this.bubbleDown(index);
        }
    }

    private bubbleUp(index: number): void {
        while (index > 0) {
            const parentIndex = this.parent(index);
            if (this.comparator(this.container[index], this.container[parentIndex])) {
                this.swap(index, parentIndex);
                index = parentIndex;
            } else {
                break;
            }
        }
    }

    private bubbleDown(index: number): void {
        while (true) {
            const firstChild = this.firstChild(index);
            if (firstChild >= this.container.length) {
                break;
            }

            let minChild = firstChild;
            const end = Math.min(firstChild + this.d, this.container.length);

            for (let i = firstChild + 1; i < end; i++) {
                if (this.comparator(this.container[i], this.container[minChild])) {
                    minChild = i;
                }
            }

            if (this.comparator(this.container[minChild], this.container[index])) {
                this.swap(index, minChild);
                index = minChild;
            } else {
                break;
            }
        }
    }

    private parent(index: number): number {
        return Math.floor((index - 1) / this.d);
    }

    private firstChild(index: number): number {
        return index * this.d + 1;
    }

    private swap(i: number, j: number): void {
        const temp = this.container[i];
        this.container[i] = this.container[j];
        this.container[j] = temp;

        const keyI = this.keyFn(this.container[i]);
        const keyJ = this.keyFn(this.container[j]);

        this.positions.set(keyI, i);
        this.positions.set(keyJ, j);
    }
}