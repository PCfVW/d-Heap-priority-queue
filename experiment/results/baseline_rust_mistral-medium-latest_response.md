# D-ary Heap Priority Queue in Rust

Here's a complete implementation of a d-ary heap priority queue that meets all your requirements:

```rust
use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct DaryHeap<T, P> {
    heap: Vec<(T, P)>,
    positions: HashMap<T, usize>,
    arity: usize,
}

impl<T, P> DaryHeap<T, P>
where
    T: Eq + Hash + Clone,
    P: Ord + Clone,
{
    /// Creates a new d-ary heap with the given arity
    pub fn new(arity: usize) -> Self {
        assert!(arity >= 2, "Heap arity must be at least 2");
        DaryHeap {
            heap: Vec::new(),
            positions: HashMap::new(),
            arity,
        }
    }

    /// Returns the number of elements in the heap
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Returns true if the heap is empty
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Checks if an item with the given identity exists in the heap
    pub fn contains(&self, item: &T) -> bool {
        self.positions.contains_key(item)
    }

    /// Returns a reference to the item with highest priority (lowest value)
    pub fn front(&self) -> Option<&T> {
        self.heap.get(0).map(|(item, _)| item)
    }

    /// Inserts a new item into the heap
    pub fn insert(&mut self, item: T, priority: P) {
        if self.positions.contains_key(&item) {
            panic!("Item already exists in the heap");
        }

        self.heap.push((item.clone(), priority));
        let index = self.heap.len() - 1;
        self.positions.insert(item, index);
        self.heapify_up(index);
    }

    /// Removes and returns the item with highest priority (lowest value)
    pub fn pop(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }

        let (item, _) = self.heap.swap_remove(0);
        self.positions.remove(&item);

        if !self.heap.is_empty() {
            // Update position of the item that was moved to index 0
            let moved_item = self.heap[0].0.clone();
            self.positions.insert(moved_item.clone(), 0);
            self.heapify_down(0);
        }

        Some(item)
    }

    /// Updates an existing item to have higher priority (lower value)
    pub fn increase_priority(&mut self, item: &T, new_priority: P) {
        if let Some(&index) = self.positions.get(item) {
            if new_priority >= self.heap[index].1 {
                panic!("New priority must be higher (lower value) than current priority");
            }

            self.heap[index].1 = new_priority;
            self.heapify_up(index);
        } else {
            panic!("Item not found in heap");
        }
    }

    /// Updates an existing item to have lower priority (higher value)
    pub fn decrease_priority(&mut self, item: &T, new_priority: P) {
        if let Some(&index) = self.positions.get(item) {
            if new_priority <= self.heap[index].1 {
                panic!("New priority must be lower (higher value) than current priority");
            }

            self.heap[index].1 = new_priority;
            self.heapify_down(index);
        } else {
            panic!("Item not found in heap");
        }
    }

    /// Moves an item up the heap until the heap property is restored
    fn heapify_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / self.arity;
            if self.heap[index].1 >= self.heap[parent].1 {
                break;
            }

            self.heap.swap(index, parent);
            self.update_positions(index, parent);
            index = parent;
        }
    }

    /// Moves an item down the heap until the heap property is restored
    fn heapify_down(&mut self, mut index: usize) {
        loop {
            let mut smallest = index;
            let first_child = index * self.arity + 1;

            // Check all children
            for i in 0..self.arity {
                let child = first_child + i;
                if child < self.heap.len() && self.heap[child].1 < self.heap[smallest].1 {
                    smallest = child;
                }
            }

            if smallest == index {
                break;
            }

            self.heap.swap(index, smallest);
            self.update_positions(index, smallest);
            index = smallest;
        }
    }

    /// Updates the positions of two items after they've been swapped
    fn update_positions(&mut self, index1: usize, index2: usize) {
        let item1 = self.heap[index1].0.clone();
        let item2 = self.heap[index2].0.clone();
        self.positions.insert(item1, index1);
        self.positions.insert(item2, index2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dary_heap() {
        let mut heap = DaryHeap::new(3);

        // Test empty heap
        assert!(heap.is_empty());
        assert_eq!(heap.len(), 0);
        assert_eq!(heap.pop(), None);
        assert_eq!(heap.front(), None);

        // Test insert and front
        heap.insert("a", 5);
        assert!(!heap.is_empty());
        assert_eq!(heap.len(), 1);
        assert_eq!(heap.front(), Some(&"a"));
        assert!(heap.contains(&"a"));
        assert!(!heap.contains(&"b"));

        // Test pop
        assert_eq!(heap.pop(), Some("a"));
        assert!(heap.is_empty());

        // Test multiple inserts
        heap.insert("a", 5);
        heap.insert("b", 3);
        heap.insert("c", 7);
        heap.insert("d", 1);
        heap.insert("e", 4);

        assert_eq!(heap.len(), 5);
        assert_eq!(heap.front(), Some(&"d"));

        // Test pop order
        assert_eq!(heap.pop(), Some("d"));
        assert_eq!(heap.pop(), Some("b"));
        assert_eq!(heap.pop(), Some("e"));
        assert_eq!(heap.pop(), Some("a"));
        assert_eq!(heap.pop(), Some("c"));
        assert_eq!(heap.pop(), None);

        // Test priority updates
        heap.insert("a", 5);
        heap.insert("b", 3);
        heap.insert("c", 7);

        heap.increase_priority(&"a", 1);
        assert_eq!(heap.front(), Some(&"a"));

        heap.decrease_priority(&"a", 6);
        assert_eq!(heap.front(), Some(&"b"));

        heap.increase_priority(&"c", 0);
        assert_eq!(heap.pop(), Some("c"));
        assert_eq!(heap.pop(), Some("b"));
        assert_eq!(heap.pop(), Some("a"));
    }

    #[test]
    fn test_duplicate_insert() {
        let mut heap = DaryHeap::new(2);
        heap.insert("a", 1);
        assert_eq!(heap.front(), Some(&"a"));

        // This should panic
        let result = std::panic::catch_unwind(|| {
            heap.insert("a", 2);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_priority_updates() {
        let mut heap = DaryHeap::new(2);
        heap.insert("a", 5);

        // These should panic
        let result = std::panic::catch_unwind(|| {
            heap.increase_priority(&"a", 6);
        });
        assert!(result.is_err());

        let result = std::panic::catch_unwind(|| {
            heap.decrease_priority(&"a", 4);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_different_arities() {
        for arity in 2..6 {
            let mut heap = DaryHeap::new(arity);

            // Insert items in reverse order
            for i in (0..10).rev() {
                heap.insert(i, i);
            }

            // Verify they come out in order
            for i in 0..10 {
                assert_eq!(heap.pop(), Some(i));
            }
        }
    }
}
```

## Key Features:

1. **Configurable Arity**: The heap's arity (d) is set at construction time and must be at least 2.

2. **Item Identity and Priority**: Items are stored with both an identity (for equality) and a priority (for ordering).

3. **O(1) Lookup**: Uses a HashMap to track item positions for O(1) contains checks.

4. **Min-Heap**: Implements a min-heap where lower priority values have higher importance.

5. **All Required Operations**:
   - `insert`: Adds a new item with O(log_d n) time complexity
   - `pop`: Removes and returns the highest priority item with O(d log_d n) time complexity
   - `front`: Returns the highest priority item without removal in O(1) time
   - `increase_priority`: Updates an item to have higher priority (lower value) with O(log_d n) time
   - `decrease_priority`: Updates an item to have lower priority (higher value) with O(d log_d n) time
   - `contains`: Checks if an item exists in O(1) time
   - `len`: Returns the number of items in O(1) time
   - `is_empty`: Checks if the heap is empty in O(1) time

6. **Error Handling**: Panics on invalid operations (duplicate inserts, invalid priority updates).

The implementation includes comprehensive tests to verify all functionality.