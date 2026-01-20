use std::collections::HashMap;
use std::hash::Hash;

/// Trait for comparing priorities in the heap
pub trait PriorityCompare<T> {
    fn compare(&self, a: &T, b: &T) -> std::cmp::Ordering;
}

/// Min-heap comparator wrapper
pub struct MinBy<F>(pub F);

impl<T, F> PriorityCompare<T> for MinBy<F>
where
    F: Fn(&T) -> i32,
{
    fn compare(&self, a: &T, b: &T) -> std::cmp::Ordering {
        (self.0)(a).cmp(&(self.0)(b))
    }
}

/// D-ary heap priority queue implementation
pub struct PriorityQueue<T, C>
where
    T: Clone + Eq + Hash,
    C: PriorityCompare<T>,
{
    /// The heap array storing items
    container: Vec<T>,
    /// Position map for O(1) lookup: item identity -> heap index
    position_map: HashMap<T, usize>,
    /// Arity of the heap (number of children per node)
    d: usize,
    /// Priority comparator
    comparator: C,
}

impl<T, C> PriorityQueue<T, C>
where
    T: Clone + Eq + Hash,
    C: PriorityCompare<T>,
{
    /// Create a new priority queue with the specified arity
    pub fn new(d: usize, comparator: C) -> Self {
        assert!(d >= 2, "arity must be at least 2");
        Self {
            container: Vec::new(),
            position_map: HashMap::new(),
            d,
            comparator,
        }
    }

    /// Add an item to the queue
    pub fn insert(&mut self, item: T) {
        assert!(!self.position_map.contains_key(&item), "item must not already exist");
        
        let index = self.container.len();
        self.container.push(item.clone());
        self.position_map.insert(item, index);
        
        self.sift_up(index);
    }

    /// Remove and return the item with highest priority
    pub fn pop(&mut self) {
        if self.container.is_empty() {
            return; // No-op for empty heap
        }

        let last_index = self.container.len() - 1;
        
        if last_index == 0 {
            // Only one element
            let item = self.container.pop().unwrap();
            self.position_map.remove(&item);
        } else {
            // Swap root with last element
            self.container.swap(0, last_index);
            self.position_map.insert(self.container[0].clone(), 0);
            
            // Remove the last element (original root)
            let removed = self.container.pop().unwrap();
            self.position_map.remove(&removed);
            
            // Restore heap property
            self.sift_down(0);
        }
    }

    /// Return the item with highest priority without removing it
    pub fn front(&self) -> &T {
        &self.container[0]
    }

    /// Return the item with highest priority without removing it (returns None if empty)
    pub fn peek(&self) -> Option<&T> {
        self.container.get(0)
    }

    /// Update an existing item to have higher priority (lower priority value)
    pub fn increase_priority(&mut self, item: &T) {
        let index = *self.position_map.get(item)
            .expect("item must exist");
        
        // Update the item in the container
        self.container[index] = item.clone();
        self.position_map.insert(item.clone(), index);
        
        // Since priority increased (lower value), item may need to move up
        self.sift_up(index);
    }

    /// Update an existing item to have lower priority (higher priority value)
    pub fn decrease_priority(&mut self, item: &T) {
        let index = *self.position_map.get(item)
            .expect("item must exist");
        
        // Update the item in the container
        self.container[index] = item.clone();
        self.position_map.insert(item.clone(), index);
        
        // Since priority decreased (higher value), item may need to move down
        self.sift_down(index);
    }

    /// Check if an item with the given identity exists in the queue
    pub fn contains(&self, item: &T) -> bool {
        self.position_map.contains_key(item)
    }

    /// Return the number of items in the queue
    pub fn len(&self) -> usize {
        self.container.len()
    }

    /// Return whether the queue contains no items
    pub fn is_empty(&self) -> bool {
        self.container.is_empty()
    }

    /// Get parent index for a given index
    fn parent_index(&self, index: usize) -> Option<usize> {
        if index == 0 {
            None
        } else {
            Some((index - 1) / self.d)
        }
    }

    /// Get first child index for a given index
    fn first_child_index(&self, index: usize) -> usize {
        self.d * index + 1
    }

    /// Restore heap property by moving an item up toward the root
    fn sift_up(&mut self, mut index: usize) {
        while let Some(parent_idx) = self.parent_index(index) {
            // If current item has higher priority than parent, swap
            if self.comparator.compare(&self.container[index], &self.container[parent_idx])
                == std::cmp::Ordering::Less
            {
                // Swap items
                self.container.swap(index, parent_idx);
                
                // Update position map
                self.position_map.insert(self.container[index].clone(), index);
                self.position_map.insert(self.container[parent_idx].clone(), parent_idx);
                
                index = parent_idx;
            } else {
                break;
            }
        }
    }

    /// Restore heap property by moving an item down toward the leaves
    fn sift_down(&mut self, mut index: usize) {
        loop {
            let first_child = self.first_child_index(index);
            
            // Find the child with highest priority (lowest value in min-heap)
            let mut highest_priority_child = None;
            
            for child_offset in 0..self.d {
                let child_index = first_child + child_offset;
                if child_index >= self.container.len() {
                    break;
                }
                
                match highest_priority_child {
                    None => highest_priority_child = Some(child_index),
                    Some(current_best) => {
                        if self.comparator.compare(&self.container[child_index], &self.container[current_best])
                            == std::cmp::Ordering::Less
                        {
                            highest_priority_child = Some(child_index);
                        }
                    }
                }
            }
            
            // If no children or current item has higher priority than best child, stop
            match highest_priority_child {
                None => break,
                Some(child_idx) => {
                    if self.comparator.compare(&self.container[child_idx], &self.container[index])
                        == std::cmp::Ordering::Less
                    {
                        // Swap with the highest priority child
                        self.container.swap(index, child_idx);
                        
                        // Update position map
                        self.position_map.insert(self.container[index].clone(), index);
                        self.position_map.insert(self.container[child_idx].clone(), child_idx);
                        
                        index = child_idx;
                    } else {
                        break;
                    }
                }
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
            Self { id: id.to_string(), priority }
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
    pub fn new_item_min_heap(d: usize) -> PriorityQueue<Item, MinBy<impl Fn(&Item) -> i32>> {
        PriorityQueue::new(d, MinBy(|i: &Item| i.priority))
    }

    /// Helper: verify heap invariant for min-heap
    pub fn verify_heap_invariant(pq: &PriorityQueue<Item, impl PriorityCompare<Item>>) -> bool {
        // We can't directly access container, so we verify via pop order
        // For test purposes, we verify that we can call front() without panic if non-empty
        if pq.is_empty() {
            return true;
        }
        // Basic sanity check: front() should not panic
        let _ = pq.front();
        true
    }

    // =============================================================================
    // insert() Tests
    // =============================================================================

    /// Test: insert_postcondition_item_findable
    /// Spec: specifications/insert.md
    /// Property: inserted item can be found via contains() after insertion
    #[test]
    fn insert_postcondition_item_findable() {
        let mut pq = new_item_min_heap(4);

        let item = Item::new("test-item", 50);
        pq.insert(item.clone());

        assert!(pq.contains(&item), "inserted item should be findable via contains()");
        assert!(pq.contains(&Item::new("test-item", 999)), "item with same ID should be found regardless of priority value");
    }

    /// Test: insert_invariant_heap_property
    /// Spec: specifications/insert.md
    /// Property: heap invariant holds after insertion (front() returns minimum)
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

    /// Test: insert_size_increments
    /// Spec: specifications/insert.md
    /// Property: heap size increases by 1 after each insertion
    #[test]
    fn insert_size_increments() {
        let mut pq = new_item_min_heap(4);

        for i in 0..5 {
            let size_before = pq.len();
            pq.insert(Item::new(&format!("item{}", i), i * 10));
            let size_after = pq.len();

            assert_eq!(size_after, size_before + 1, "size should increment by 1 after insert");
        }
    }

    /// Test: insert_edge_becomes_front_if_highest_priority
    /// Spec: specifications/insert.md
    /// Property: if inserted item has highest priority, it becomes front()
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

        assert_eq!(pq.front().id, "urgent", "new highest priority item should become front");
    }

    // =============================================================================
    // pop() Tests
    // =============================================================================

    /// Test: pop_postcondition_returns_minimum
    /// Spec: specifications/pop.md
    /// Property: pop() removes the item with lowest priority value (min-heap)
    /// Note: Rust's pop() doesn't return; use front() before pop() to get value
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
        assert!(!pq.contains(&Item::new("b", 0)), "popped item should not be in heap");
    }

    /// Test: pop_invariant_maintains_heap_property
    /// Spec: specifications/pop.md
    /// Property: after pop(), heap invariant holds (front() is minimum of remaining)
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

    /// Test: pop_size_decrements
    /// Spec: specifications/pop.md
    /// Property: size() decreases by 1 after successful pop()
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

            assert_eq!(size_after, size_before - 1, "size should decrement by 1 after pop");
            assert_eq!(size_after, expected_size, "size should match expected");
        }
    }

    /// Test: pop_edge_empty_no_panic
    /// Spec: specifications/pop.md
    /// Property: pop() on empty heap does not panic (no-op behavior in Rust impl)
    #[test]
    fn pop_edge_empty_no_panic() {
        let mut pq = new_item_min_heap(4);

        // This should not panic
        pq.pop();

        // Heap should remain empty
        assert!(pq.is_empty(), "heap should remain empty after failed pop");
    }

    // =============================================================================
    // front() Tests
    // =============================================================================

    /// Test: front_postcondition_returns_minimum
    /// Spec: specifications/front.md
    /// Property: front() returns the item with lowest priority value (min-heap) without removal
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

    /// Test: front_invariant_no_modification
    /// Spec: specifications/front.md
    /// Property: front() does not modify the heap (calling multiple times returns same result)
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

    /// Test: front_size_unchanged
    /// Spec: specifications/front.md
    /// Property: size() remains the same after front()
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

    /// Test: front_edge_empty_peek_returns_none
    /// Spec: specifications/front.md
    /// Property: peek() on empty heap returns None (front() would panic)
    #[test]
    fn front_edge_empty_peek_returns_none() {
        let pq = new_item_min_heap(4);

        let result = pq.peek();
        assert!(result.is_none(), "peek() on empty heap should return None");
    }

    // =============================================================================
    // increase_priority() Tests
    // =============================================================================

    /// Test: increase_priority_postcondition_priority_changed
    /// Spec: specifications/increase_priority.md
    /// Property: item's priority is updated to the new value
    #[test]
    fn increase_priority_postcondition_priority_changed() {
        let mut pq = new_item_min_heap(4);

        pq.insert(Item::new("target", 50));
        pq.insert(Item::new("other", 30));

        // "other" starts at front with priority 30
        assert_eq!(pq.front().id, "other");

        // Increase priority of "target" (in min-heap: lower value = higher priority)
        // Create updated item with same ID but new priority
        let updated = Item::new("target", 10);
        pq.increase_priority(&updated);

        // "target" should now be at front (highest priority)
        assert_eq!(pq.front().id, "target", "target should be at front after priority increase");
        assert_eq!(pq.front().priority, 10, "priority should be updated to 10");
    }

    /// Test: increase_priority_invariant_heap_property
    /// Spec: specifications/increase_priority.md
    /// Property: heap invariant holds after priority increase
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
        assert_eq!(pq.front().id, "a", "item with increased priority should be at front");
        assert_eq!(pq.front().priority, 5);
    }

    /// Test: increase_priority_position_item_moves_up
    /// Spec: specifications/increase_priority.md
    /// Property: item moves toward root after priority increase (becomes front if highest)
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
        assert_eq!(pq.front().id, "leaf", "item should move to front after priority increase");
    }

    /// Test: increase_priority_size_unchanged
    /// Spec: specifications/increase_priority.md
    /// Property: size() remains unchanged after priority update
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

        assert_eq!(size_after, size_before, "size should be unchanged after priority update");
    }

    /// Test: increase_priority_edge_not_found_panics
    /// Spec: specifications/increase_priority.md
    /// Property: panics if item not in heap (Rust implementation behavior)
    #[test]
    #[should_panic(expected = "item must exist")]
    fn increase_priority_edge_not_found_panics() {
        let mut pq = new_item_min_heap(4);

        pq.insert(Item::new("existing", 50));

        // This should panic - item not found
        let nonexistent = Item::new("nonexistent", 10);
        pq.increase_priority(&nonexistent);
    }

    // =============================================================================
    // decrease_priority() Tests
    // =============================================================================

    /// Test: decrease_priority_postcondition_priority_changed
    /// Spec: specifications/decrease_priority.md
    /// Property: item's priority is updated to the new value
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
        assert_eq!(pq.front().id, "other", "other should be at front after target's priority decrease");

        // Pop "other" and verify "target" has updated priority
        pq.pop();
        assert_eq!(pq.front().priority, 50, "target's priority should be updated to 50");
    }

    /// Test: decrease_priority_invariant_heap_property
    /// Spec: specifications/decrease_priority.md
    /// Property: heap invariant holds after priority decrease
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

    /// Test: decrease_priority_position_item_moves_down
    /// Spec: specifications/decrease_priority.md
    /// Property: item moves toward leaves after priority decrease (no longer front if was)
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

    /// Test: decrease_priority_size_unchanged
    /// Spec: specifications/decrease_priority.md
    /// Property: size() remains unchanged after priority update
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

        assert_eq!(size_after, size_before, "size should be unchanged after priority update");
    }

    /// Test: decrease_priority_edge_not_found_panics
    /// Spec: specifications/decrease_priority.md
    /// Property: panics if item not in heap (Rust implementation behavior)
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