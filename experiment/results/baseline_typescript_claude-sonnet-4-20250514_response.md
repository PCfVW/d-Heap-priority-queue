Here's a complete implementation of a d-ary heap priority queue in TypeScript:

```typescript
interface PriorityItem {
  identity: string | number;
  priority: number;
}

class DaryHeapPriorityQueue<T extends PriorityItem> {
  private heap: T[] = [];
  private indexMap: Map<string | number, number> = new Map();
  private readonly arity: number;

  constructor(arity: number = 2) {
    if (arity < 2) {
      throw new Error("Heap arity must be at least 2");
    }
    this.arity = arity;
  }

  /**
   * Add an item to the queue
   */
  insert(item: T): void {
    if (this.indexMap.has(item.identity)) {
      throw new Error(`Item with identity ${item.identity} already exists`);
    }

    const index = this.heap.length;
    this.heap.push(item);
    this.indexMap.set(item.identity, index);
    this.heapifyUp(index);
  }

  /**
   * Remove and return the item with highest priority (lowest value)
   */
  pop(): T | undefined {
    if (this.heap.length === 0) {
      return undefined;
    }

    const root = this.heap[0];
    this.indexMap.delete(root.identity);

    if (this.heap.length === 1) {
      this.heap.pop();
      return root;
    }

    // Move last element to root and heapify down
    const lastItem = this.heap.pop()!;
    this.heap[0] = lastItem;
    this.indexMap.set(lastItem.identity, 0);
    this.heapifyDown(0);

    return root;
  }

  /**
   * Return the item with highest priority without removing it
   */
  front(): T | undefined {
    return this.heap.length > 0 ? this.heap[0] : undefined;
  }

  /**
   * Update an existing item to have higher priority (lower value)
   */
  increasePriority(item: T): void {
    const index = this.indexMap.get(item.identity);
    if (index === undefined) {
      throw new Error(`Item with identity ${item.identity} not found`);
    }

    if (item.priority >= this.heap[index].priority) {
      throw new Error("New priority must be lower (higher importance) than current priority");
    }

    this.heap[index] = item;
    this.heapifyUp(index);
  }

  /**
   * Update an existing item to have lower priority (higher value)
   */
  decreasePriority(item: T): void {
    const index = this.indexMap.get(item.identity);
    if (index === undefined) {
      throw new Error(`Item with identity ${item.identity} not found`);
    }

    if (item.priority <= this.heap[index].priority) {
      throw new Error("New priority must be higher (lower importance) than current priority");
    }

    this.heap[index] = item;
    this.heapifyDown(index);
  }

  /**
   * Check if an item with the given identity exists
   */
  contains(item: T): boolean {
    return this.indexMap.has(item.identity);
  }

  /**
   * Return the number of items in the queue
   */
  len(): number {
    return this.heap.length;
  }

  /**
   * Return whether the queue is empty
   */
  isEmpty(): boolean {
    return this.heap.length === 0;
  }

  /**
   * Get the parent index of a given index
   */
  private getParentIndex(index: number): number {
    return Math.floor((index - 1) / this.arity);
  }

  /**
   * Get the first child index of a given index
   */
  private getFirstChildIndex(index: number): number {
    return this.arity * index + 1;
  }

  /**
   * Move an item up the heap to maintain heap property
   */
  private heapifyUp(index: number): void {
    while (index > 0) {
      const parentIndex = this.getParentIndex(index);
      
      if (this.heap[index].priority >= this.heap[parentIndex].priority) {
        break;
      }

      this.swap(index, parentIndex);
      index = parentIndex;
    }
  }

  /**
   * Move an item down the heap to maintain heap property
   */
  private heapifyDown(index: number): void {
    while (true) {
      let minIndex = index;
      const firstChildIndex = this.getFirstChildIndex(index);

      // Check all children to find the one with minimum priority
      for (let i = 0; i < this.arity; i++) {
        const childIndex = firstChildIndex + i;
        
        if (childIndex >= this.heap.length) {
          break;
        }

        if (this.heap[childIndex].priority < this.heap[minIndex].priority) {
          minIndex = childIndex;
        }
      }

      if (minIndex === index) {
        break;
      }

      this.swap(index, minIndex);
      index = minIndex;
    }
  }

  /**
   * Swap two elements in the heap and update the index map
   */
  private swap(i: number, j: number): void {
    const temp = this.heap[i];
    this.heap[i] = this.heap[j];
    this.heap[j] = temp;

    // Update index map
    this.indexMap.set(this.heap[i].identity, i);
    this.indexMap.set(this.heap[j].identity, j);
  }

  /**
   * Get a string representation of the heap (for debugging)
   */
  toString(): string {
    return this.heap.map(item => `{id: ${item.identity}, priority: ${item.priority}}`).join(', ');
  }

  /**
   * Validate the heap property (for testing)
   */
  private isValidHeap(): boolean {
    for (let i = 0; i < this.heap.length; i++) {
      const firstChildIndex = this.getFirstChildIndex(i);
      
      for (let j = 0; j < this.arity; j++) {
        const childIndex = firstChildIndex + j;
        
        if (childIndex >= this.heap.length) {
          break;
        }

        if (this.heap[i].priority > this.heap[childIndex].priority) {
          return false;
        }
      }
    }
    return true;
  }
}

// Example usage and testing
interface Task extends PriorityItem {
  identity: string;
  priority: number;
  description: string;
}

// Example usage
const pq = new DaryHeapPriorityQueue<Task>(3); // 3-ary heap

// Insert some tasks
pq.insert({ identity: "task1", priority: 5, description: "Low priority task" });
pq.insert({ identity: "task2", priority: 1, description: "High priority task" });
pq.insert({ identity: "task3", priority: 3, description: "Medium priority task" });
pq.insert({ identity: "task4", priority: 2, description: "High-medium priority task" });

console.log("Queue length:", pq.len()); // 4
console.log("Is empty:", pq.isEmpty()); // false

console.log("Front item:", pq.front()); // task2 (priority 1)

console.log("Contains task2:", pq.contains({ identity: "task2", priority: 1, description: "" })); // true

// Increase priority of task3 (lower the priority value)
pq.increasePriority({ identity: "task3", priority: 0.5, description: "Updated medium priority task" });

console.log("Front item after increase:", pq.front()); // task3 (priority 0.5)

// Pop items in priority order
while (!pq.isEmpty()) {
  console.log("Popped:", pq.pop());
}

export { DaryHeapPriorityQueue, PriorityItem };
```

This implementation provides:

1. **Configurable arity**: The heap arity `d` is set at construction time and must be at least 2.

2. **Identity and priority separation**: Items have distinct `identity` and `priority` properties. Equality is based on identity only.

3. **O(1) lookup**: The `indexMap` provides constant-time lookup to check if an item exists and to find its position in the heap.

4. **Min-heap property**: Lower priority values have higher importance and bubble up to the root.

5. **All required operations**:
   - `insert()`: O(log_d n) - adds item and maintains heap property
   - `pop()`: O(d log_d n) - removes root and reheapifies
   - `front()`: O(1) - returns root without removal
   - `increasePriority()`: O(log_d n) - decreases priority value and bubbles up
   - `decreasePriority()`: O(d log_d n) - increases priority value and bubbles down
   - `contains()`: O(1) - checks existence via index map
   - `len()`: O(1) - returns heap size
   - `isEmpty()`: O(1) - checks if heap is empty

6. **Proper TypeScript types**: Uses generics with constraints to ensure type safety.

The implementation handles edge cases, maintains the heap invariant, and provides clear error messages for invalid operations.