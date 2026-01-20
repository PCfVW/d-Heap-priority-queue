# D-ary Heap Priority Queue in Rust

Here's a complete implementation of a d-ary heap priority queue in Rust with all the required functionality:

```rust
use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Ord;

#[derive(Debug)]
pub struct DaryHeap<T, P>
where
    T: Eq + Hash + Clone,
    P: Ord + Clone,
{
    d: usize, // arity of the heap
    heap: Vec<(T, P)>, // the heap array
    position_map: HashMap<T, usize>, // maps item identity to its index in the heap
}

impl<T, P> DaryHeap<T, P>
where
    T: Eq + Hash + Clone,
    P: Ord + Clone,
{
    /// Create a new priority queue with the specified arity.
    ///
    /// # Arguments
    /// * `d` - The arity of the heap (number of children per node), must be >= 2
    ///
    /// # Returns
    /// A new empty priority queue
    pub fn new(d: usize) -> Self {
        assert!(d >= 2, "Arity must be at least 2");
        DaryHeap {
            d,
            heap: Vec::new(),
            position_map: HashMap::new(),
        }
    }

    /// Add an item to the queue.
    ///
    /// # Arguments
    /// * `item` - The item to insert (identity)
    /// * `priority` - The priority of the item
    ///
    /// # Precondition
    /// Item with same identity must not already exist
    pub fn insert(&mut self, item: T, priority: P) {
        assert!(!self.contains(&item), "Item already exists in the heap");

        let index = self.heap.len();
        self.heap.push((item.clone(), priority));
        self.position_map.insert(item, index);
        self.sift_up(index);
    }

    /// Remove and return the item with highest priority (lowest priority value).
    ///
    /// # Returns
    /// The item with highest priority
    ///
    /// # Precondition
    /// Queue is not empty
    pub fn pop(&mut self) -> Option<(T, P)> {
        if self.is_empty() {
            return None;
        }

        let last_index = self.heap.len() - 1;
        self.swap(0, last_index);

        let item = self.heap.pop().unwrap();
        self.position_map.remove(&item.0);

        if !self.is_empty() {
            self.sift_down(0);
        }

        Some(item)
    }

    /// Return the item with highest priority without removing it.
    ///
    /// # Returns
    /// The item with highest priority
    ///
    /// # Precondition
    /// Queue is not empty
    pub fn front(&self) -> Option<&(T, P)> {
        self.heap.get(0)
    }

    /// Update an existing item to have higher priority (lower priority value).
    ///
    /// # Arguments
    /// * `item` - The item to update (identity)
    /// * `new_priority` - The new priority value (must be lower than current)
    ///
    /// # Precondition
    /// Item with same identity must exist in queue
    pub fn increase_priority(&mut self, item: &T, new_priority: P) {
        let index = *self.position_map.get(item).expect("Item not found in heap");
        let current_priority = &self.heap[index].1;

        assert!(new_priority <= current_priority.clone(), "New priority must be higher (lower value) than current");

        self.heap[index].1 = new_priority;
        self.sift_up(index);
    }

    /// Update an existing item to have lower priority (higher priority value).
    ///
    /// # Arguments
    /// * `item` - The item to update (identity)
    /// * `new_priority` - The new priority value (must be higher than current)
    ///
    /// # Precondition
    /// Item with same identity must exist in queue
    pub fn decrease_priority(&mut self, item: &T, new_priority: P) {
        let index = *self.position_map.get(item).expect("Item not found in heap");
        let current_priority = &self.heap[index].1;

        assert!(new_priority >= current_priority.clone(), "New priority must be lower (higher value) than current");

        self.heap[index].1 = new_priority;
        self.sift_down(index);
    }

    /// Check if an item with the given identity exists in the queue.
    ///
    /// # Arguments
    /// * `item` - The item to check
    ///
    /// # Returns
    /// true if item with same identity exists, false otherwise
    pub fn contains(&self, item: &T) -> bool {
        self.position_map.contains_key(item)
    }

    /// Return the number of items in the queue.
    ///
    /// # Returns
    /// Non-negative integer count
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Return whether the queue contains no items.
    ///
    /// # Returns
    /// true if len() == 0, false otherwise
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Restore heap property by moving an item up toward the root.
    fn sift_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent_index = (index - 1) / self.d;
            if self.heap[index].1 < self.heap[parent_index].1 {
                self.swap(index, parent_index);
                index = parent_index;
            } else {
                break;
            }
        }
    }

    /// Restore heap property by moving an item down toward the leaves.
    fn sift_down(&mut self, mut index: usize) {
        loop {
            let first_child = self.d * index + 1;
            if first_child >= self.heap.len() {
                break; // no children
            }

            // Find the child with the highest priority (lowest value)
            let last_child = std::cmp::min(self.d * index + self.d, self.heap.len());
            let mut min_child = first_child;

            for child in first_child + 1..last_child {
                if self.heap[child].1 < self.heap[min_child].1 {
                    min_child = child;
                }
            }

            if self.heap[index].1 > self.heap[min_child].1 {
                self.swap(index, min_child);
                index = min_child;
            } else {
                break;
            }
        }
    }

    /// Swap two elements in the heap and update their positions in the map.
    fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
        let item_i = self.heap[i].0.clone();
        let item_j = self.heap[j].0.clone();
        self.position_map.insert(item_i, i);
        self.position_map.insert(item_j, j);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let heap = DaryHeap::<i32, i32>::new(3);
        assert_eq!(heap.len(), 0);
        assert!(heap.is_empty());
    }

    #[test]
    fn test_insert_and_front() {
        let mut heap = DaryHeap::new(3);
        heap.insert(1, 10);
        heap.insert(2, 5);
        heap.insert(3, 15);

        assert_eq!(heap.len(), 3);
        assert_eq!(heap.front().unwrap().0, 2);
    }

    #[test]
    fn test_pop() {
        let mut heap = DaryHeap::new(3);
        heap.insert(1, 10);
        heap.insert(2, 5);
        heap.insert(3, 15);

        assert_eq!(heap.pop().unwrap().0, 2);
        assert_eq!(heap.pop().unwrap().0, 1);
        assert_eq!(heap.pop().unwrap().0, 3);
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_increase_priority() {
        let mut heap = DaryHeap::new(3);
        heap.insert(1, 10);
        heap.insert(2, 5);
        heap.insert(3, 15);

        heap.increase_priority(&1, 2);
        assert_eq!(heap.pop().unwrap().0, 1);
    }

    #[test]
    fn test_decrease_priority() {
        let mut heap = DaryHeap::new(3);
        heap.insert(1, 10);
        heap.insert(2, 5);
        heap.insert(3, 15);

        heap.decrease_priority(&2, 20);
        assert_eq!(heap.pop().unwrap().0, 1);
    }

    #[test]
    fn test_contains() {
        let mut heap = DaryHeap::new(3);
        heap.insert(1, 10);
        heap.insert(2, 5);

        assert!(heap.contains(&1));
        assert!(heap.contains(&2));
        assert!(!heap.contains(&3));
    }

    #[test]
    fn test_sift_up() {
        let mut heap = DaryHeap::new(3);
        heap.insert(1, 10);
        heap.insert(2, 5);
        heap.insert(3, 15);
        heap.insert(4, 1);

        assert_eq!(heap.front().unwrap().0, 4);
    }

    #[test]
    fn test_sift_down() {
        let mut heap = DaryHeap::new(3);
        heap.insert(1, 10);
        heap.insert(2, 5);
        heap.insert(3, 15);

        heap.decrease_priority(&2, 20);
        assert_eq!(heap.pop().unwrap().0, 1);
    }
}
```

## Key Features

1. **Generic Implementation**: Works with any identity type `T` (must be `Eq + Hash + Clone`) and any priority type `P` (must be `Ord + Clone`).

2. **Position Map**: Uses a `HashMap` to track each item's position in the heap for O(1) lookups.

3. **Complete API**: Implements all required methods:
   - `new`, `insert`, `pop`, `front`
   - `increase_priority`, `decrease_priority`
   - `contains`, `len`, `is_empty`

4. **Heap Operations**:
   - `sift_up` and `sift_down` maintain the heap property
   - `swap` helper method updates both the heap array and position map

5. **Error Handling**: Uses assertions for preconditions (could be replaced with proper error handling in production code).

6. **Test Coverage**: Includes comprehensive unit tests for all functionality.

The implementation maintains all the required time complexities:
- Insert: O(log_d n)
- Pop: O(d * log_d n)
- Front: O(1)
- Priority updates: O(log_d n) or O(d * log_d n)
- Contains: O(1)