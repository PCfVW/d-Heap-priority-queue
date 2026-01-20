interface PriorityQueueConfig<T, K> {
  d: number;
  comparator: (a: T, b: T) => boolean;
  keyExtractor: (item: T) => K;
}

export class PriorityQueue<T, K> {
  private heap: T[] = [];
  private positionMap: Map<K, number> = new Map();
  private d: number;
  private comparator: (a: T, b: T) => boolean;
  private keyExtractor: (item: T) => K;

  constructor(config: PriorityQueueConfig<T, K>) {
    if (config.d < 2) {
      throw new Error('Arity d must be >= 2');
    }
    this.d = config.d;
    this.comparator = config.comparator;
    this.keyExtractor = config.keyExtractor;
  }

  private parentIndex(index: number): number {
    return Math.floor((index - 1) / this.d);
  }

  private firstChildIndex(index: number): number {
    return this.d * index + 1;
  }

  private lastChildIndex(index: number): number {
    return Math.min(this.d * index + this.d, this.heap.length - 1);
  }

  private swap(i: number, j: number): void {
    const temp = this.heap[i];
    this.heap[i] = this.heap[j];
    this.heap[j] = temp;

    // Update position map
    this.positionMap.set(this.keyExtractor(this.heap[i]), i);
    this.positionMap.set(this.keyExtractor(this.heap[j]), j);
  }

  private siftUp(index: number): void {
    while (index > 0) {
      const parentIdx = this.parentIndex(index);
      
      // If current item has higher priority (comparator returns true), swap with parent
      if (this.comparator(this.heap[index], this.heap[parentIdx])) {
        this.swap(index, parentIdx);
        index = parentIdx;
      } else {
        break;
      }
    }
  }

  private siftDown(index: number): void {
    while (true) {
      const firstChild = this.firstChildIndex(index);
      
      // No children
      if (firstChild >= this.heap.length) {
        break;
      }

      const lastChild = this.lastChildIndex(index);
      
      // Find child with highest priority (comparator returns true means higher priority)
      let bestChildIdx = firstChild;
      for (let i = firstChild + 1; i <= lastChild; i++) {
        if (this.comparator(this.heap[i], this.heap[bestChildIdx])) {
          bestChildIdx = i;
        }
      }

      // If best child has higher priority than current item, swap
      if (this.comparator(this.heap[bestChildIdx], this.heap[index])) {
        this.swap(index, bestChildIdx);
        index = bestChildIdx;
      } else {
        break;
      }
    }
  }

  insert(item: T): void {
    const key = this.keyExtractor(item);
    
    if (this.positionMap.has(key)) {
      throw new Error('Item with same identity already exists');
    }

    // Add to end of heap
    const index = this.heap.length;
    this.heap.push(item);
    this.positionMap.set(key, index);

    // Restore heap property
    this.siftUp(index);
  }

  pop(): T | undefined {
    if (this.heap.length === 0) {
      return undefined;
    }

    if (this.heap.length === 1) {
      const item = this.heap.pop()!;
      this.positionMap.delete(this.keyExtractor(item));
      return item;
    }

    // Store root to return
    const root = this.heap[0];
    this.positionMap.delete(this.keyExtractor(root));

    // Move last element to root
    const lastItem = this.heap.pop()!;
    this.heap[0] = lastItem;
    this.positionMap.set(this.keyExtractor(lastItem), 0);

    // Restore heap property
    this.siftDown(0);

    return root;
  }

  front(): T {
    if (this.heap.length === 0) {
      throw new Error('Cannot get front of empty heap');
    }
    return this.heap[0];
  }

  increasePriority(item: T): void {
    const key = this.keyExtractor(item);
    const index = this.positionMap.get(key);
    
    if (index === undefined) {
      throw new Error('Item not found in heap');
    }

    // Update the item at the found position
    this.heap[index] = item;

    // Since priority increased (became more important), sift up
    this.siftUp(index);
  }

  decreasePriority(item: T): void {
    const key = this.keyExtractor(item);
    const index = this.positionMap.get(key);
    
    if (index === undefined) {
      throw new Error('Item not found in heap');
    }

    // Update the item at the found position
    this.heap[index] = item;

    // Since priority decreased (became less important), sift down
    this.siftDown(index);
  }

  contains(item: T): boolean {
    const key = this.keyExtractor(item);
    return this.positionMap.has(key);
  }

  containsKey(key: K): boolean {
    return this.positionMap.has(key);
  }

  len(): number {
    return this.heap.length;
  }

  isEmpty(): boolean {
    return this.heap.length === 0;
  }
}