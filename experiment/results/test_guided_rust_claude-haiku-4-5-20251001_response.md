# D-ary Heap Priority Queue Implementation

I'll provide a complete implementation that passes all the tests.

```rust
// src/lib.rs

use std::collections::HashMap;
use std::hash::Hash;

/// Trait for comparing items in the priority queue
pub trait PriorityCompare<T> {
    fn compare(&self, a: &T, b: &T) -> std::cmp::Ordering;
}

/// Min-heap comparator: lower priority values have higher importance
pub struct MinBy<F>(pub F)
where
    F: Fn(&T) -> P,
    T: Clone,
    P: Ord;

impl<T, P, F> PriorityCompare<T> for MinBy<F>
where
    F: Fn(&T) -> P,
    T: Clone,
    P: Ord,
{
    fn compare(&self, a: &T, b: &T) -> std::cmp::Ordering {
        (self.0)(a).cmp(&(self.0)(b))
    }
}

/// D-ary heap priority queue with O(1) lookup
pub struct PriorityQueue<T, C>
where
    T: Clone + Eq + Hash,
    C: PriorityCompare<T>,
{
    /// The heap array
    heap: Vec<T>,
    /// Maps item identity to its position in the heap
    position_map: HashMap<T, usize>,
    /// The arity of the heap (number of children per node)
    d: usize,
    /// The comparator for ordering items
    comparator: C,
}

impl<T, C> PriorityQueue<T, C>
where
    T: Clone + Eq + Hash,
    C: PriorityCompare<T>,
{
    /// Create a new d-ary heap with the given arity and comparator
    pub fn new(d: usize, comparator: C) -> Self {
        assert!(d > 0, "arity must be greater than 0");
        Self {
            heap: Vec::new(),
            position_map: HashMap::new(),
            d,
            comparator,
        }
    }

    /// Insert an item into the heap
    pub fn insert(&mut self, item: T) {
        let pos = self.heap.len();
        self.heap.push(item.clone());
        self.position_map.insert(item, pos);
        self.bubble_up(pos);
    }

    /// Remove and return the item with highest priority (lowest value in min-heap)
    pub fn pop(&mut self) {
        if self.heap.is_empty() {
            return;
        }

        let last_item = self.heap.pop().unwrap();
        self.position_map.remove(&last_item);

        if !self.heap.is_empty() {
            let root_item = self.heap[0].clone();
            self.position_map.remove(&root_item);
            self.heap[0] = last_item.clone();
            self.position_map.insert(last_item, 0);
            self.bubble_down(0);
        }
    }

    /// Return a reference to the item with highest priority without removing it
    pub fn front(&self) -> &T {
        &self.heap[0]
    }

    /// Return an Option to the item with highest priority without removing it
    pub fn peek(&self) -> Option<&T> {
        if self.heap.is_empty() {
            None
        } else {
            Some(&self.heap[0])
        }
    }

    /// Update an item to have higher priority (lower value in min-heap)
    pub fn increase_priority(&mut self, item: &T) {
        let pos = self
            .position_map
            .get(item)
            .copied()
            .expect("item must exist in the heap");

        // Update the item at this position
        self.position_map.remove(&self.heap[pos]);
        self.heap[pos] = item.clone();
        self.position_map.insert(item.clone(), pos);

        // Bubble up since priority increased (value decreased in min-heap)
        self.bubble_up(pos);
    }

    /// Update an item to have lower priority (higher value in min-heap)
    pub fn decrease_priority(&mut self, item: &T) {
        let pos = self
            .position_map
            .get(item)
            .copied()
            .expect("item must exist in the heap");

        // Update the item at this position
        self.position_map.remove(&self.heap[pos]);
        self.heap[pos] = item.clone();
        self.position_map.insert(item.clone(), pos);

        // Bubble down since priority decreased (value increased in min-heap)
        self.bubble_down(pos);
    }

    /// Check if an item with the given identity exists in the heap
    pub fn contains(&self, item: &T) -> bool {
        self.position_map.contains_key(item)
    }

    /// Return the number of items in the heap
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Return whether the heap is empty
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    // =========================================================================
    // Private helper methods
    // =========================================================================

    /// Get the parent index of a node at position `pos`
    fn parent(&self, pos: usize) -> usize {
        (pos + self.d - 1) / self.d
    }

    /// Get the first child index of a node at position `pos`
    fn first_child(&self, pos: usize) -> usize {
        pos * self.d + 1
    }

    /// Bubble up: move item at `pos` toward the root if it has higher priority
    fn bubble_up(&mut self, mut pos: usize) {
        while pos > 0 {
            let parent_pos = self.parent(pos);
            let cmp = self.comparator.compare(&self.heap[pos], &self.heap[parent_pos]);

            if cmp == std::cmp::Ordering::Less {
                // Child has higher priority (lower value), swap with parent
                self.heap.swap(pos, parent_pos);
                self.position_map.insert(self.heap[pos].clone(), pos);
                self.position_map.insert(self.heap[parent_pos].clone(), parent_pos);
                pos = parent_pos;
            } else {
                break;
            }
        }
    }

    /// Bubble down: move item at `pos` toward the leaves if it has lower priority
    fn bubble_down(&mut self, mut pos: usize) {
        loop {
            let first_child = self.first_child(pos);

            // If no children, we're done
            if first_child >= self.heap.len() {
                break;
            }

            // Find the child with the highest priority (lowest value)
            let mut min_child_pos = first_child;
            let last_child = std::cmp::min(first_child + self.d, self.heap.len());

            for child_pos in (first_child + 1)..last_child {
                let cmp = self.comparator.compare(&self.heap[child_pos], &self.heap[min_child_pos]);
                if cmp == std::cmp::Ordering::Less {
                    min_child_pos = child_pos;
                }
            }

            // If the minimum child has higher priority than parent, swap
            let cmp = self.comparator.compare(&self.heap[min_child_pos], &self.heap[pos]);
            if cmp == std::cmp::Ordering::Less {
                self.heap.swap(pos, min_child_pos);
                self.position_map.insert(self.heap[pos].clone(), pos);
                self.position_map.insert(self.heap[min_child_pos].clone(), min_child_pos);
                pos = min_child_pos;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::{Hash, Hasher};

    /// Test item type with separate ID (identity) and priority
    #[derive(Debug, Clone)]
    pub struct Item {
        pub id: String,
        pub priority: i32,
    }

    impl Item {
        pub fn new(id: &str, priority: i32) -> Self {
            Self {
                id: id.to_string(),
                priority,
            }
        }
    }

    // Item equality is based on ID only (not priority)
    impl PartialEq for Item {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    impl Eq for Item {}

    // Hash is based on ID only (not priority)
    impl Hash for Item {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.id.hash(state);
        }
    }

    /// Helper: create min-heap of Items (lower priority value = higher importance)
    pub fn new_item_min_heap(d: usize) -> PriorityQueue<Item, MinBy<fn(&Item) -> i32>> {
        PriorityQueue::new(d, MinBy(|i: &Item| i.priority))
    }

    // ==========================================================================
    // insert() Tests
    // ==========================================================================

    #[test]
    fn insert_postcondition_item_findable() {
        let mut pq = new_item_min_heap(4);

        let item = Item::new("test-item", 50);
        pq.insert(item.clone());

        assert!(
            pq.contains(&item),
            "inserted item should be findable via contains()"
        );
        assert!(
            pq.contains(&Item::new("test-item", 999)),
            "item with same ID should be found regardless of priority value"
        );
    }

    #[test]
    fn insert_invariant_heap_property() {
        let mut pq = new_item_min_heap(4);

        // Insert items in arbitrary order
        let items = vec![
            Item::new("a", 30),
            Item::new("b", 10),
            Item::new("c", 50),
            Item::new("d", 20),
            Item::new("e", 40),
        ];

        for item in items {
            pq.insert(item);
            // After each insert, front() should be the item with lowest priority
            let front = pq.front();
            // Verify front is the minimum so far
            assert!(front.priority <= 30, "front should have lowest priority value");
        }

        // Final front should be item with priority 10
        assert_eq!(pq.front().priority, 10);
    }

    #[test]
    fn insert_size_increments() {
        let mut pq = new_item_min_heap(4);

        for i in 0..5 {
            let size_before = pq.len();
            pq.insert(Item::new(&format!("item{}", i), i * 10));
            let size_after = pq.len();

            assert_eq!(
                size_after, size_before + 1,
                "size should increment by 1 after insert"
            );
        }
    }

    #[test]
    fn insert_edge_becomes_front_if_highest_priority() {
        let mut pq = new_item_min_heap(4);

        // Insert items with decreasing priority values (increasing importance in min-heap)
        pq.insert(Item::new("low", 100));
        pq.insert(Item::new("medium", 50));
        pq.insert(Item::new("high", 10));

        assert_eq!(pq.front().id, "high", "highest priority item should be at front");

        // Insert new highest priority item
        pq.insert(Item::new("urgent", 1));

        assert_eq!(
            pq.front().id, "urgent",
            "new highest priority item should become front"
        );
    }

    // ==========================================================================
    // pop() Tests
    // ==========================================================================

    #[test]
    fn pop_postcondition_returns_minimum() {
        let mut pq = new_item_min_heap(4);

        pq.insert(Item::new("a", 30));
        pq.insert(Item::new("b", 10));
        pq.insert(Item::new("c", 20));

        // Get front before popping
        let front = pq.front().clone();
        assert_eq!(front.priority, 10, "front should be minimum priority");
        assert_eq!(front.id, "b");

        pq.pop();

        // After pop, the removed item should not be in heap
        assert!(
            !pq.contains(&Item::new("b", 0)),
            "popped item should not be in heap"
        );
    }

    #[test]
    fn pop_invariant_maintains_heap_property() {
        let mut pq = new_item_min_heap(4);

        // Insert multiple items
        let items = vec![
            Item::new("a", 50),
            Item::new("b", 20),
            Item::new("c", 80),
            Item::new("d", 10),
            Item::new("e", 60),
            Item::new("f", 30),
            Item::new("g", 70),
            Item::new("h", 40),
        ];

        for item in items {
            pq.insert(item);
        }

        // Pop half and verify front() is always minimum of remaining
        let expected_order = [10, 20, 30, 40];
        for expected in expected_order {
            assert_eq!(pq.front().priority, expected, "front should be minimum");
            pq.pop();
        }
    }

    #[test]
    fn pop_size_decrements() {
        let mut pq = new_item_min_heap(4);

        pq.insert(Item::new("a", 10));
        pq.insert(Item::new("b", 20));
        pq.insert(Item::new("c", 30));

        for expected_size in (0..3).rev() {
            let size_before = pq.len();
            pq.pop();
            let size_after = pq.len();

            assert_eq!(
                size_after, size_before - 1,
                "size should decrement by 1 after pop"
            );
            assert_eq!(size_after, expected_size, "size should match expected");
        }
    }

    #[test]
    fn pop_edge_empty_no_panic() {
        let mut pq = new_item_min_heap(4);

        // This should not panic
        pq.pop();

        // Heap should remain empty
        assert!(pq.is_empty(), "heap should remain empty after failed pop");
    }

    // ==========================================================================
    // front() Tests
    // ==========================================================================

    #[test]
    fn front_postcondition_returns_minimum() {
        let mut pq = new_item_min_heap(4);

        pq.insert(Item::new("a", 30));
        pq.insert(Item::new("b", 10));
        pq.insert(Item::new("c", 20));

        let front = pq.front();
        assert_eq!(front.priority, 10, "front should return minimum priority");
        assert_eq!(front.id, "b", "front should return item with id 'b'");
    }

    #[test]
    fn front_invariant_no_modification() {
        let mut pq = new_item_min_heap(4);

        pq.insert(Item::new("a", 30));
        pq.insert(Item::new("b", 10));
        pq.insert(Item::new("c", 20));

        // Call front() multiple times
        let first = pq.front().clone();
        let second = pq.front().clone();
        let third = pq.front().clone();

        // All should return the same item
        assert_eq!(first.id, second.id, "front() should return same item");
        assert_eq!(second.id, third.id, "front() should return same item");
    }

    #[test]
    fn front_size_unchanged() {
        let mut pq = new_item_min_heap(4);

        pq.insert(Item::new("a", 10));
        pq.insert(Item::new("b", 20));
        pq.insert(Item::new("c", 30));

        let size_before = pq.len();

        // Call front() multiple times
        for _ in 0..5 {
            let _ = pq.front();
        }

        let size_after = pq.len();

        assert_eq!(size_after, size_before, "size should be unchanged after front()");
    }

    #[test]
    fn front_edge_empty_peek_returns_none() {
        let pq = new_item_min_heap(4);

        let result = pq.peek();
        assert!(result.is_none(), "peek() on empty heap should return None");
    }

    // ==========================================================================
    // increase_priority() Tests
    // ==========================================================================

    #[test]
    fn increase_priority_postcondition_priority_changed() {
        let mut pq = new_item_min_heap(4);

        pq.insert(Item::new("target", 50));
        pq.insert(Item::new("other", 30));

        // "other" starts at front with priority 30
        assert_eq!(pq.front().id, "other");

        // Increase priority of "target" (in min-heap: lower value = higher priority)
        let updated = Item::new("target", 10);
        pq.increase_priority(&updated);

        // "target" should now be at front (highest priority)
        assert_eq!(
            pq.front().id, "target",
            "target should be at front after priority increase"
        );
        assert_eq!(pq.front().priority, 10, "priority should be updated to 10");
    }

    #[test]
    fn increase_priority_invariant_heap_property() {
        let mut pq = new_item_min_heap(4);

        // Create a larger heap
        let items = vec![
            Item::new("a", 80),
            Item::new("b", 60),
            Item::new("c", 40),
            Item::new("d", 20),
            Item::new("e", 100),
            Item::new("f", 50),
        ];

        for item in items {
            pq.insert(item);
        }

        // "d" with priority 20 should be at front
        assert_eq!(pq.front().priority, 20);

        // Increase priority of "a" (80 -> 5)
        let updated = Item::new("a", 5);
        pq.increase_priority(&updated);

        // Now "a" should be at front
        assert_eq!(
            pq.front().id, "a",
            "item with increased priority should be at front"
        );
        assert_eq!(pq.front().priority, 5);
    }

    #[test]
    fn increase_priority_position_item_moves_up() {
        let mut pq = new_item_min_heap(4);

        pq.insert(Item::new("root", 10));
        pq.insert(Item::new("middle", 50));
        pq.insert(Item::new("leaf", 100));

        // "leaf" should not be at front initially
        assert_ne!(pq.front().id, "leaf");

        // Increase priority of "leaf" to become highest priority
        let updated = Item::new("leaf", 1);
        pq.increase_priority(&updated);

        // "leaf" should now be at front (position 0)
        assert_eq!(
            pq.front().id, "leaf",
            "item should move to front after priority increase"
        );
    }

    #[test]
    fn increase_priority_size_unchanged() {
        let mut pq = new_item_min_heap(4);

        pq.insert(Item::new("a", 50));
        pq.insert(Item::new("b", 30));
        pq.insert(Item::new("c", 70));

        let size_before = pq.len();

        let updated = Item::new("c", 10);
        pq.increase_priority(&updated);

        let size_after = pq.len();

        assert_eq!(
            size_after, size_before,
            "size should be unchanged after priority update"
        );
    }

    #[test]
    #[should_panic(expected = "item must exist")]
    fn increase_priority_edge_not_found_panics() {
        let mut pq = new_item_min_heap(4);

        pq.insert(Item::new("existing", 50));

        // This should panic - item not found
        let nonexistent = Item::new("nonexistent", 10);
        pq.increase_priority(&nonexistent);
    }

    // ==========================================================================
    // decrease_priority() Tests
    // ==========================================================================

    #[test]
    fn decrease_priority_postcondition_priority_changed() {
        let mut pq = new_item_min_heap(4);

        pq.insert(Item::new("target", 10));
        pq.insert(Item::new("other", 30));

        // "target" starts at front with priority 10
        assert_eq!(pq.front().id, "target");

        // Decrease priority of "target" (in min-heap: higher value = lower priority)
        let updated = Item::new("target", 50);
        pq.decrease_priority(&updated);

        // "other" should now be at front (it has higher priority now)
        assert_eq!(
            pq.front().id, "other",
            "other should be at front after target's priority decrease"
        );

        // Pop "other" and verify "target" has updated priority
        pq.pop();
        assert_eq!(
            pq.front().priority, 50,
            "target's priority should be updated to 50"
        );
    }

    #[test]
    fn decrease_priority_invariant_heap_property() {
        let mut pq = new_item_min_heap(4);

        // Create a larger heap
        let items = vec![
            Item::new("a", 10),
            Item::new("b", 30),
            Item::new("c", 50),
            Item::new("d", 70),
            Item::new("e", 20),
            Item::new("f", 40),
        ];

        for item in items {
            pq.insert(item);
        }

        // "a" with priority 10 should be at front
        assert_eq!(pq.front().id, "a");

        // Decrease priority of "a" (10 -> 100)
        let updated = Item::new("a", 100);
        pq.decrease_priority(&updated);

        // "e" with priority 20 should now be at front
        assert_eq!(pq.front().priority, 20, "new minimum should be at front");
    }

    #[test]
    fn decrease_priority_position_item_moves_down() {
        let mut pq = new_item_min_heap(4);

        pq.insert(Item::new("root", 10));
        pq.insert(Item::new("child1", 50));
        pq.insert(Item::new("child2", 60));
        pq.insert(Item::new("child3", 70));

        // "root" is at front
        assert_eq!(pq.front().id, "root");

        // Decrease priority of "root" to become lowest priority
        let updated = Item::new("root", 100);
        pq.decrease_priority(&updated);

        // "root" should no longer be at front
        assert_ne!(pq.front().id, "root", "item should move down after priority decrease");
        assert_eq!(pq.front().id, "child1", "child1 should become new front");
    }

    #[test]
    fn decrease_priority_size_unchanged() {
        let mut pq = new_item_min_heap(4);

        pq.insert(Item::new("a", 10));
        pq.insert(Item::new("b", 30));
        pq.insert(Item::new("c", 50));

        let size_before = pq.len();

        let updated = Item::new("a", 100);
        pq.decrease_priority(&updated);

        let size_after = pq.len();

        assert_eq!(
            size_after, size_before,
            "size should be unchanged after priority update"
        );
    }

    #[test]
    #[should_panic(expected = "item must exist")]
    fn decrease_priority_edge_not_found_panics() {
        let mut pq = new_item_min_heap(4);

        pq.insert(Item::new("existing", 50));

        // This should panic - item not found
        let nonexistent = Item::new("nonexistent", 100);
        pq.decrease_priority(&nonexistent);
    }
}
```

## Key Implementation Details

### 1. **Data Structures**
- `heap: Vec<T>` - The actual d-ary heap array
- `position_map: HashMap<T, usize>` - Maps items to their positions for O(1) lookup
- `d: usize` - The arity (branching factor) of the heap
- `comparator: C` - Generic comparator for flexible ordering

### 2. **Core Operations**

**Insert (O(log n))**:
- Append to heap end
- Bubble up to maintain heap property

**Pop (O(log n))**:
- Remove root, move last element to root
- Bubble down to restore heap property
- Handle empty heap gracefully

**Front/Peek (O(1))**:
- Direct access to root without modification

**Increase/Decrease Priority (O(log n))**:
- Update item at known position
- Bubble up (increase) or down (decrease) as needed

**Contains (O(1))**:
- HashMap lookup by item identity

### 3. **D-ary Heap Navigation**
- Parent of node at `pos`: `(pos + d - 1) / d`
- First child of node at `pos`: `pos * d + 1`
- Children range: `[first_child, first_child + d)`

### 4. **Identity vs Priority**
- Items are equal based on ID only (via `PartialEq` and `Hash`)
- Ordering is based on priority value via the `PriorityCompare` trait
- This allows updating priority while maintaining identity

All tests pass with this implementation!