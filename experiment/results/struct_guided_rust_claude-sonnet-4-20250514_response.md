Here's a complete implementation of the d-ary heap priority queue in Rust:

```rust
use std::collections::HashMap;
use std::hash::Hash;

/// Item represents an element in the priority queue.
/// The `id` field determines identity (equality via Hash + Eq).
/// The `priority` field determines ordering in the heap.
#[derive(Clone)]
pub struct Item<K> {
    pub id: K,
    pub priority: i32,
}

/// PriorityQueue is a d-ary min-heap with O(1) item lookup.
pub struct PriorityQueue<K: Hash + Eq + Clone> {
    d: usize,                          // arity (children per node)
    container: Vec<Item<K>>,           // heap array
    positions: HashMap<K, usize>,      // maps item id to index in container
}

impl<K: Hash + Eq + Clone> PriorityQueue<K> {
    /// Creates a new priority queue with the given arity d.
    /// Panics if d < 2.
    pub fn new(d: usize) -> Self {
        if d < 2 {
            panic!("Arity d must be at least 2");
        }
        Self {
            d,
            container: Vec::new(),
            positions: HashMap::new(),
        }
    }

    /// Adds an item to the queue.
    /// Returns Err if an item with the same id already exists.
    pub fn insert(&mut self, item: Item<K>) -> Result<(), &'static str> {
        if self.positions.contains_key(&item.id) {
            return Err("Item with this id already exists");
        }

        let index = self.container.len();
        self.positions.insert(item.id.clone(), index);
        self.container.push(item);
        self.heapify_up(index);
        Ok(())
    }

    /// Removes and returns the item with highest priority (lowest value).
    /// Returns None if empty.
    pub fn pop(&mut self) -> Option<Item<K>> {
        if self.container.is_empty() {
            return None;
        }

        let root = self.container[0].clone();
        self.positions.remove(&root.id);

        if self.container.len() == 1 {
            self.container.pop();
            return Some(root);
        }

        // Move last element to root
        let last = self.container.pop().unwrap();
        self.container[0] = last;
        self.positions.insert(self.container[0].id.clone(), 0);
        self.heapify_down(0);

        Some(root)
    }

    /// Returns a reference to the item with highest priority without removing it.
    /// Returns None if empty.
    pub fn front(&self) -> Option<&Item<K>> {
        self.container.first()
    }

    /// Updates an item to have higher priority (lower value).
    /// The item parameter contains the id to find and the new priority.
    /// Returns Err if the item doesn't exist.
    pub fn increase_priority(&mut self, item: Item<K>) -> Result<(), &'static str> {
        let index = self.positions.get(&item.id)
            .ok_or("Item not found")?;
        let index = *index;

        if item.priority >= self.container[index].priority {
            return Err("New priority is not higher (lower value) than current priority");
        }

        self.container[index].priority = item.priority;
        self.heapify_up(index);
        Ok(())
    }

    /// Updates an item to have lower priority (higher value).
    /// The item parameter contains the id to find and the new priority.
    /// Returns Err if the item doesn't exist.
    pub fn decrease_priority(&mut self, item: Item<K>) -> Result<(), &'static str> {
        let index = self.positions.get(&item.id)
            .ok_or("Item not found")?;
        let index = *index;

        if item.priority <= self.container[index].priority {
            return Err("New priority is not lower (higher value) than current priority");
        }

        self.container[index].priority = item.priority;
        self.heapify_down(index);
        Ok(())
    }

    /// Returns true if an item with the given id exists.
    pub fn contains(&self, item: &Item<K>) -> bool {
        self.positions.contains_key(&item.id)
    }

    /// Returns the number of items in the queue.
    pub fn len(&self) -> usize {
        self.container.len()
    }

    /// Returns true if the queue contains no items.
    pub fn is_empty(&self) -> bool {
        self.container.is_empty()
    }

    // Helper methods

    /// Returns the parent index of the given index.
    fn parent(&self, index: usize) -> Option<usize> {
        if index == 0 {
            None
        } else {
            Some((index - 1) / self.d)
        }
    }

    /// Returns the first child index of the given index.
    fn first_child(&self, index: usize) -> usize {
        self.d * index + 1
    }

    /// Moves an item up the heap until heap property is satisfied.
    fn heapify_up(&mut self, mut index: usize) {
        while let Some(parent_index) = self.parent(index) {
            if self.container[index].priority >= self.container[parent_index].priority {
                break;
            }
            self.swap(index, parent_index);
            index = parent_index;
        }
    }

    /// Moves an item down the heap until heap property is satisfied.
    fn heapify_down(&mut self, mut index: usize) {
        loop {
            let first_child = self.first_child(index);
            if first_child >= self.container.len() {
                break; // No children
            }

            // Find the child with minimum priority
            let mut min_child = first_child;
            let last_child = std::cmp::min(first_child + self.d, self.container.len());
            
            for child in first_child + 1..last_child {
                if self.container[child].priority < self.container[min_child].priority {
                    min_child = child;
                }
            }

            // If current node has lower or equal priority than min child, we're done
            if self.container[index].priority <= self.container[min_child].priority {
                break;
            }

            self.swap(index, min_child);
            index = min_child;
        }
    }

    /// Swaps two items in the heap and updates their positions in the hashmap.
    fn swap(&mut self, i: usize, j: usize) {
        self.container.swap(i, j);
        self.positions.insert(self.container[i].id.clone(), i);
        self.positions.insert(self.container[j].id.clone(), j);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut pq = PriorityQueue::new(2);
        
        assert!(pq.is_empty());
        assert_eq!(pq.len(), 0);
        assert!(pq.front().is_none());
        assert!(pq.pop().is_none());

        // Insert items
        pq.insert(Item { id: "a", priority: 5 }).unwrap();
        pq.insert(Item { id: "b", priority: 3 }).unwrap();
        pq.insert(Item { id: "c", priority: 7 }).unwrap();

        assert_eq!(pq.len(), 3);
        assert!(!pq.is_empty());
        assert_eq!(pq.front().unwrap().id, "b");
        assert_eq!(pq.front().unwrap().priority, 3);

        // Test contains
        assert!(pq.contains(&Item { id: "a", priority: 0 })); // priority doesn't matter for contains
        assert!(pq.contains(&Item { id: "b", priority: 0 }));
        assert!(!pq.contains(&Item { id: "d", priority: 0 }));

        // Pop items (should come out in priority order)
        let item = pq.pop().unwrap();
        assert_eq!(item.id, "b");
        assert_eq!(item.priority, 3);

        let item = pq.pop().unwrap();
        assert_eq!(item.id, "a");
        assert_eq!(item.priority, 5);

        let item = pq.pop().unwrap();
        assert_eq!(item.id, "c");
        assert_eq!(item.priority, 7);

        assert!(pq.is_empty());
    }

    #[test]
    fn test_priority_updates() {
        let mut pq = PriorityQueue::new(3);
        
        pq.insert(Item { id: 1, priority: 10 }).unwrap();
        pq.insert(Item { id: 2, priority: 20 }).unwrap();
        pq.insert(Item { id: 3, priority: 30 }).unwrap();

        // Increase priority (decrease value)
        pq.increase_priority(Item { id: 3, priority: 5 }).unwrap();
        assert_eq!(pq.front().unwrap().id, 3);
        assert_eq!(pq.front().unwrap().priority, 5);

        // Decrease priority (increase value)
        pq.decrease_priority(Item { id: 3, priority: 25 }).unwrap();
        assert_eq!(pq.front().unwrap().id, 1);
        assert_eq!(pq.front().unwrap().priority, 10);
    }

    #[test]
    fn test_duplicate_insert() {
        let mut pq = PriorityQueue::new(2);
        
        pq.insert(Item { id: "test", priority: 5 }).unwrap();
        let result = pq.insert(Item { id: "test", priority: 10 });
        assert!(result.is_err());
    }

    #[test]
    #[should_panic]
    fn test_invalid_arity() {
        PriorityQueue::<i32>::new(1);
    }
}
```

This implementation provides:

1. **Complete d-ary heap functionality**: The heap maintains the min-heap property where each parent has priority ≤ all its children.

2. **O(1) lookup**: The `positions` HashMap provides constant-time lookup to check if an item exists and find its position.

3. **Proper identity vs priority handling**: Items are considered equal based on their `id` field only, while ordering uses the `priority` field.

4. **Efficient operations**:
   - `insert`: O(log_d n) - adds item and heapifies up
   - `pop`: O(d log_d n) - removes root and heapifies down
   - `increase_priority`/`decrease_priority`: O(log_d n) or O(d log_d n) depending on direction
   - `contains`: O(1) - HashMap lookup
   - `front`: O(1) - returns first element

5. **Proper error handling**: Returns appropriate errors for duplicate inserts, missing items, and invalid priority updates.

The implementation includes comprehensive helper methods for heap operations and maintains consistency between the heap array and the position HashMap at all times.