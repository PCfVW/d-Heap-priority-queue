export interface PriorityQueueConfig<T, K> {
  d: number;
  comparator: (a: T, b: T) => boolean;
  keyExtractor: (item: T) => K;
}

export class PriorityQueue<T, K> {
  private heap: T[] = [];
  private keyToIndex: Map<K, number> = new Map();
  private d: number;
  private comparator: (a: T, b: T) => boolean;
  private keyExtractor: (item: T) => K;

  constructor(config: PriorityQueueConfig<T, K>) {
    this.d = config.d;
    this.comparator = config.comparator;
    this.keyExtractor = config.keyExtractor;
  }

  private parentIndex(index: number): number {
    return Math.floor((index - 1) / this.d);
  }

  private childrenIndices(index: number): number[] {
    const children: number[] = [];
    const firstChild = this.d * index + 1;
    for (let i = 0; i < this.d; i++) {
      const childIndex = firstChild + i;
      if (childIndex < this.heap.length) {
        children.push(childIndex);
      }
    }
    return children;
  }

  private swap(i: number, j: number): void {
    // Update key-to-index mapping
    const keyI = this.keyExtractor(this.heap[i]);
    const keyJ = this.keyExtractor(this.heap[j]);
    this.keyToIndex.set(keyI, j);
    this.keyToIndex.set(keyJ, i);

    // Swap elements
    [this.heap[i], this.heap[j]] = [this.heap[j], this.heap[i]];
  }

  private bubbleUp(index: number): void {
    while (index > 0) {
      const parentIdx = this.parentIndex(index);
      if (!this.comparator(this.heap[index], this.heap[parentIdx])) {
        break;
      }
      this.swap(index, parentIdx);
      index = parentIdx;
    }
  }

  private bubbleDown(index: number): void {
    while (true) {
      const children = this.childrenIndices(index);
      if (children.length === 0) {
        break;
      }

      // Find the child with highest priority (minimum value for min-heap)
      let bestChildIndex = children[0];
      for (let i = 1; i < children.length; i++) {
        if (this.comparator(this.heap[children[i]], this.heap[bestChildIndex])) {
          bestChildIndex = children[i];
        }
      }

      // If current node has higher priority than best child, we're done
      if (!this.comparator(this.heap[bestChildIndex], this.heap[index])) {
        break;
      }

      this.swap(index, bestChildIndex);
      index = bestChildIndex;
    }
  }

  insert(item: T): void {
    const key = this.keyExtractor(item);
    
    // If item already exists, update it
    if (this.keyToIndex.has(key)) {
      const existingIndex = this.keyToIndex.get(key)!;
      const oldItem = this.heap[existingIndex];
      this.heap[existingIndex] = item;
      
      // Determine if we need to bubble up or down
      if (this.comparator(item, oldItem)) {
        this.bubbleUp(existingIndex);
      } else {
        this.bubbleDown(existingIndex);
      }
      return;
    }

    // Add new item
    const newIndex = this.heap.length;
    this.heap.push(item);
    this.keyToIndex.set(key, newIndex);
    this.bubbleUp(newIndex);
  }

  pop(): T | undefined {
    if (this.heap.length === 0) {
      return undefined;
    }

    const result = this.heap[0];
    const key = this.keyExtractor(result);
    this.keyToIndex.delete(key);

    if (this.heap.length === 1) {
      this.heap.pop();
      return result;
    }

    // Move last element to root and bubble down
    const lastItem = this.heap.pop()!;
    const lastKey = this.keyExtractor(lastItem);
    this.heap[0] = lastItem;
    this.keyToIndex.set(lastKey, 0);
    this.bubbleDown(0);

    return result;
  }

  front(): T {
    if (this.heap.length === 0) {
      throw new Error('Cannot get front of empty priority queue');
    }
    return this.heap[0];
  }

  private updatePriority(item: T): void {
    const key = this.keyExtractor(item);
    const index = this.keyToIndex.get(key);
    
    if (index === undefined) {
      throw new Error('Item not found in priority queue');
    }

    const oldItem = this.heap[index];
    this.heap[index] = item;

    // Determine if we need to bubble up or down
    if (this.comparator(item, oldItem)) {
      this.bubbleUp(index);
    } else {
      this.bubbleDown(index);
    }
  }

  increase_priority(item: T): void {
    this.updatePriority(item);
  }

  decrease_priority(item: T): void {
    this.updatePriority(item);
  }

  contains(item: T): boolean {
    const key = this.keyExtractor(item);
    return this.keyToIndex.has(key);
  }

  containsKey(key: K): boolean {
    return this.keyToIndex.has(key);
  }

  len(): number {
    return this.heap.length;
  }

  is_empty(): boolean {
    return this.heap.length === 0;
  }
}