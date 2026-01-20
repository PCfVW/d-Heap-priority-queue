# D-ary Heap Priority Queue Implementation in Rust

Here's a complete implementation of a d-ary min-heap priority queue with O(1) lookup:

```rust
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone)]
pub struct Item<K> {
    pub id: K,
    pub priority: i32,
}

pub struct PriorityQueue<K: Hash + Eq + Clone> {
    d: usize,
    container: Vec<Item<K>>,
    positions: HashMap<K, usize>,
}

impl<K: Hash + Eq + Clone> PriorityQueue<K> {
    pub fn new(d: usize) -> Self {
        if d < 2 {
            panic!("A d-ary heap must have arity d >= 2");
        }
        PriorityQueue {
            d,
            container: Vec::new(),
            positions: HashMap::new(),
        }
    }

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

    pub fn pop(&mut self) -> Option<Item<K>> {
        if self.container.is_empty() {
            return None;
        }

        let min_item = self.container[0].clone();
        self.positions.remove(&min_item.id);

        if self.container.len() > 1 {
            let last_index = self.container.len() - 1;
            self.container.swap(0, last_index);
            self.positions.insert(self.container[0].id.clone(), 0);
        }

        self.container.pop();
        if !self.container.is_empty() {
            self.heapify_down(0);
        }

        Some(min_item)
    }

    pub fn front(&self) -> Option<&Item<K>> {
        self.container.get(0)
    }

    pub fn increase_priority(&mut self, item: Item<K>) -> Result<(), &'static str> {
        if let Some(&index) = self.positions.get(&item.id) {
            if item.priority < self.container[index].priority {
                self.container[index].priority = item.priority;
                self.heapify_up(index);
                Ok(())
            } else {
                Err("New priority is not higher (lower value) than current")
            }
        } else {
            Err("Item not found")
        }
    }

    pub fn decrease_priority(&mut self, item: Item<K>) -> Result<(), &'static str> {
        if let Some(&index) = self.positions.get(&item.id) {
            if item.priority > self.container[index].priority {
                self.container[index].priority = item.priority;
                self.heapify_down(index);
                Ok(())
            } else {
                Err("New priority is not lower (higher value) than current")
            }
        } else {
            Err("Item not found")
        }
    }

    pub fn contains(&self, item: &Item<K>) -> bool {
        self.positions.contains_key(&item.id)
    }

    pub fn len(&self) -> usize {
        self.container.len()
    }

    pub fn is_empty(&self) -> bool {
        self.container.is_empty()
    }

    // Helper methods
    fn heapify_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / self.d;
            if self.container[index].priority < self.container[parent].priority {
                self.swap(index, parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    fn heapify_down(&mut self, mut index: usize) {
        loop {
            let mut smallest = index;
            let first_child = index * self.d + 1;

            // Check all children
            for i in 0..self.d {
                let child = first_child + i;
                if child < self.container.len() &&
                   self.container[child].priority < self.container[smallest].priority {
                    smallest = child;
                }
            }

            if smallest != index {
                self.swap(index, smallest);
                index = smallest;
            } else {
                break;
            }
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.container.swap(i, j);
        self.positions.insert(self.container[i].id.clone(), i);
        self.positions.insert(self.container[j].id.clone(), j);
    }
}
```

## Implementation Notes:

1. **Heap Structure**: The implementation uses a vector to store the heap elements, with the heap property maintained where each parent node has a priority less than or equal to its children.

2. **D-ary Operations**:
   - `heapify_up`: Moves an element up the heap until the heap property is restored
   - `heapify_down`: Moves an element down the heap until the heap property is restored
   - Both operations consider all d children when determining the next position

3. **Position Tracking**: The `positions` HashMap maintains O(1) lookup for any item by its ID.

4. **Priority Updates**:
   - `increase_priority` (lower value) triggers a `heapify_up`
   - `decrease_priority` (higher value) triggers a `heapify_down`

5. **Error Handling**: Methods return appropriate error messages when operations can't be performed (e.g., item not found, invalid priority change).

6. **Efficiency**:
   - Insert: O(log_d n)
   - Pop: O(d log_d n)
   - Priority updates: O(log_d n)
   - Contains: O(1)

The implementation maintains all the required properties while providing efficient operations for a priority queue.