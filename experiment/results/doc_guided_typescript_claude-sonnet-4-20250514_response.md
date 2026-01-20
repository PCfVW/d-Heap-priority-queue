Here's a complete implementation of a d-ary heap priority queue in TypeScript:

```typescript
interface PriorityItem<T> {
  identity: T;
  priority: number;
}

class DaryHeapPriorityQueue<T> {
  private heap: PriorityItem<T>[] = [];
  private positionMap: Map<T, number> = new Map();
  private readonly d: number;

  constructor(d: number) {
    if (d < 2) {
      throw new Error("Arity must be at least 2");
    }
    this.d = d;
  }

  /**
   * Add an item to the queue
   */
  insert(item: PriorityItem<T>): void {
    if (this.contains(item)) {
      throw new Error("Item with same identity already exists");
    }

    // Add to end of heap
    const index = this.heap.length;
    this.heap.push(item);
    this.positionMap.set(item.identity, index);

    // Restore heap property by sifting up
    this.siftUp(index);
  }

  /**
   * Remove and return the item with highest priority (lowest priority value)
   */
  pop(): PriorityItem<T> | null {
    if (this.isEmpty()) {
      return null;
    }

    const root = this.heap[0];
    const lastItem = this.heap[this.heap.length - 1];

    // Remove from position map
    this.positionMap.delete(root.identity);

    if (this.heap.length === 1) {
      this.heap.pop();
      return root;
    }

    // Move last item to root
    this.heap[0] = lastItem;
    this.positionMap.set(lastItem.identity, 0);
    this.heap.pop();

    // Restore heap property by sifting down
    this.siftDown(0);

    return root;
  }

  /**
   * Return the item with highest priority without removing it
   */
  front(): PriorityItem<T> | null {
    if (this.isEmpty()) {
      return null;
    }
    return this.heap[0];
  }

  /**
   * Update an existing item to have higher priority (lower priority value)
   */
  increasePriority(item: PriorityItem<T>): void {
    const index = this.positionMap.get(item.identity);
    if (index === undefined) {
      throw new Error("Item not found in queue");
    }

    const currentPriority = this.heap[index].priority;
    if (item.priority >= currentPriority) {
      throw new Error("New priority must be lower (higher priority) than current priority");
    }

    // Update priority
    this.heap[index] = item;

    // Sift up since priority decreased (higher priority)
    this.siftUp(index);
  }

  /**
   * Update an existing item to have lower priority (higher priority value)
   */
  decreasePriority(item: PriorityItem<T>): void {
    const index = this.positionMap.get(item.identity);
    if (index === undefined) {
      throw new Error("Item not found in queue");
    }

    const currentPriority = this.heap[index].priority;
    if (item.priority <= currentPriority) {
      throw new Error("New priority must be higher (lower priority) than current priority");
    }

    // Update priority
    this.heap[index] = item;

    // Sift down since priority increased (lower priority)
    this.siftDown(index);
  }

  /**
   * Check if an item with the given identity exists in the queue
   */
  contains(item: PriorityItem<T>): boolean {
    return this.positionMap.has(item.identity);
  }

  /**
   * Return the number of items in the queue
   */
  len(): number {
    return this.heap.length;
  }

  /**
   * Return whether the queue contains no items
   */
  isEmpty(): boolean {
    return this.heap.length === 0;
  }

  /**
   * Get parent index for a given index
   */
  private getParentIndex(index: number): number {
    return Math.floor((index - 1) / this.d);
  }

  /**
   * Get first child index for a given index
   */
  private getFirstChildIndex(index: number): number {
    return this.d * index + 1;
  }

  /**
   * Get last child index for a given index
   */
  private getLastChildIndex(index: number): number {
    return Math.min(this.d * index + this.d, this.heap.length - 1);
  }

  /**
   * Swap two items in the heap and update position map
   */
  private swap(i: number, j: number): void {
    const temp = this.heap[i];
    this.heap[i] = this.heap[j];
    this.heap[j] = temp;

    // Update position map
    this.positionMap.set(this.heap[i].identity, i);
    this.positionMap.set(this.heap[j].identity, j);
  }

  /**
   * Restore heap property by moving an item up toward the root
   */
  private siftUp(index: number): void {
    while (index > 0) {
      const parentIndex = this.getParentIndex(index);
      
      // If current item has higher priority (lower value) than parent, swap
      if (this.heap[index].priority < this.heap[parentIndex].priority) {
        this.swap(index, parentIndex);
        index = parentIndex;
      } else {
        break;
      }
    }
  }

  /**
   * Restore heap property by moving an item down toward the leaves
   */
  private siftDown(index: number): void {
    while (true) {
      const firstChildIndex = this.getFirstChildIndex(index);
      
      // If no children, we're done
      if (firstChildIndex >= this.heap.length) {
        break;
      }

      const lastChildIndex = this.getLastChildIndex(index);
      
      // Find child with highest priority (lowest priority value)
      let minChildIndex = firstChildIndex;
      for (let i = firstChildIndex + 1; i <= lastChildIndex; i++) {
        if (this.heap[i].priority < this.heap[minChildIndex].priority) {
          minChildIndex = i;
        }
      }

      // If the child with highest priority has higher priority than current item, swap
      if (this.heap[minChildIndex].priority < this.heap[index].priority) {
        this.swap(index, minChildIndex);
        index = minChildIndex;
      } else {
        break;
      }
    }
  }

  /**
   * Debug method to verify heap property
   */
  private verifyHeapProperty(): boolean {
    for (let i = 0; i < this.heap.length; i++) {
      const firstChildIndex = this.getFirstChildIndex(i);
      const lastChildIndex = this.getLastChildIndex(i);
      
      for (let j = firstChildIndex; j <= lastChildIndex && j < this.heap.length; j++) {
        if (this.heap[i].priority > this.heap[j].priority) {
          return false;
        }
      }
    }
    return true;
  }

  /**
   * Debug method to verify position map consistency
   */
  private verifyPositionMap(): boolean {
    if (this.positionMap.size !== this.heap.length) {
      return false;
    }

    for (let i = 0; i < this.heap.length; i++) {
      const mappedIndex = this.positionMap.get(this.heap[i].identity);
      if (mappedIndex !== i) {
        return false;
      }
    }

    return true;
  }

  /**
   * Debug method to verify all invariants
   */
  verify(): boolean {
    return this.verifyHeapProperty() && this.verifyPositionMap();
  }
}

// Example usage and tests
function example() {
  // Create a 3-ary heap
  const pq = new DaryHeapPriorityQueue<string>(3);

  // Insert some items
  pq.insert({ identity: "task1", priority: 5 });
  pq.insert({ identity: "task2", priority: 1 });
  pq.insert({ identity: "task3", priority: 3 });
  pq.insert({ identity: "task4", priority: 2 });
  pq.insert({ identity: "task5", priority: 4 });

  console.log("Size:", pq.len()); // 5
  console.log("Front:", pq.front()); // { identity: "task2", priority: 1 }

  // Pop items in priority order
  while (!pq.isEmpty()) {
    console.log("Popped:", pq.pop());
  }
  // Should print in order: task2(1), task4(2), task3(3), task5(4), task1(5)

  // Test priority updates
  pq.insert({ identity: "A", priority: 10 });
  pq.insert({ identity: "B", priority: 20 });
  pq.insert({ identity: "C", priority: 15 });

  console.log("Before update:", pq.front()); // A(10)
  
  pq.increasePriority({ identity: "B", priority: 5 });
  console.log("After increase priority:", pq.front()); // B(5)
  
  pq.decreasePriority({ identity: "B", priority: 25 });
  console.log("After decrease priority:", pq.front()); // A(10)

  console.log("Contains A:", pq.contains({ identity: "A", priority: 999 })); // true
  console.log("Contains D:", pq.contains({ identity: "D", priority: 1 })); // false
}

// Run example
example();

export { DaryHeapPriorityQueue, PriorityItem };
```

This implementation provides:

1. **Complete d-ary heap structure** with configurable arity
2. **O(1) position map** for fast lookups and updates
3. **All required operations** with correct time complexities:
   - `insert`: O(log_d n)
   - `pop`: O(d * log_d n)
   - `front`: O(1)
   - `increasePriority`: O(log_d n)
   - `decreasePriority`: O(d * log_d n)
   - `contains`: O(1)
   - `len`: O(1)
   - `isEmpty`: O(1)

4. **Proper error handling** for precondition violations
5. **Debug methods** to verify heap property and position map consistency
6. **Type safety** with TypeScript generics
7. **Complete example usage** demonstrating all features

The implementation maintains all invariants:
- Heap property (parent priority ≤ all children priorities)
- Position map consistency (maps identity to correct array index)
- Proper synchronization between heap array and position map