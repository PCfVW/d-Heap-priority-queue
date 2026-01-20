Implement a d-ary heap priority queue in Go.

Requirements:
1. The heap arity (d) should be configurable at construction time
2. Items have two distinct properties: an identity (for equality) and a priority (for ordering)
3. Two items are equal if they have the same identity, regardless of priority
4. The queue should support O(1) lookup to check if an item exists
5. Implement a min-heap where lower priority values have higher importance

Required operations:
- Insert(item): Add an item to the queue
- Pop(): Remove and return the item with highest priority (lowest value)
- Front(): Return the item with highest priority without removing it
- IncreasePriority(item): Update an existing item to have higher priority (lower value)
- DecreasePriority(item): Update an existing item to have lower priority (higher value)
- Contains(item): Check if an item with the given identity exists
- Len(): Return the number of items in the queue
- IsEmpty(): Return whether the queue is empty

Provide a complete, working implementation.