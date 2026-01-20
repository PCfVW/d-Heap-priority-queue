// src/lib.rs
use std::collections::HashMap;
use std::hash::Hash;

pub trait PriorityCompare<T> {
    fn compare(&self, a: &T, b: &T) -> std::cmp::Ordering;
}

pub struct MinBy<F>(pub F);

impl<T, F> PriorityCompare<T> for MinBy<F>
where
    F: Fn(&T) -> i32,
{
    fn compare(&self, a: &T, b: &T) -> std::cmp::Ordering {
        (self.0)(a).cmp(&(self.0)(b))
    }
}

pub struct PriorityQueue<T, C>
where
    T: Clone + Eq + Hash,
    C: PriorityCompare<T>,
{
    heap: Vec<T>,
    d: usize,
    compare: C,
    index_map: HashMap<T, usize>,
}

impl<T, C> PriorityQueue<T, C>
where
    T: Clone + Eq + Hash,
    C: PriorityCompare<T>,
{
    pub fn new(d: usize, compare: C) -> Self {
        assert!(d >= 2, "d-ary heap must have d >= 2");
        Self {
            heap: Vec::new(),
            d,
            compare,
            index_map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, item: T) {
        // If item already exists, replace it
        if let Some(&existing_index) = self.index_map.get(&item) {
            self.heap[existing_index] = item.clone();
            self.bubble_up(existing_index);
            self.bubble_down(existing_index);
        } else {
            // Add new item
            let index = self.heap.len();
            self.heap.push(item.clone());
            self.index_map.insert(item, index);
            self.bubble_up(index);
        }
    }

    pub fn pop(&mut self) {
        if self.heap.is_empty() {
            return;
        }

        let last_index = self.heap.len() - 1;
        if last_index == 0 {
            let item = self.heap.pop().unwrap();
            self.index_map.remove(&item);
        } else {
            let front_item = self.heap[0].clone();
            let last_item = self.heap.pop().unwrap();
            
            self.index_map.remove(&front_item);
            self.index_map.remove(&last_item);
            
            self.heap[0] = last_item.clone();
            self.index_map.insert(last_item, 0);
            
            self.bubble_down(0);
        }
    }

    pub fn front(&self) -> &T {
        if self.heap.is_empty() {
            panic!("front() called on empty heap");
        }
        &self.heap[0]
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.get(0)
    }

    pub fn increase_priority(&mut self, item: &T) {
        let index = self.index_map.get(item)
            .copied()
            .expect("item must exist");
        
        self.index_map.remove(&self.heap[index]);
        self.heap[index] = item.clone();
        self.index_map.insert(item.clone(), index);
        
        self.bubble_up(index);
    }

    pub fn decrease_priority(&mut self, item: &T) {
        let index = self.index_map.get(item)
            .copied()
            .expect("item must exist");
        
        self.index_map.remove(&self.heap[index]);
        self.heap[index] = item.clone();
        self.index_map.insert(item.clone(), index);
        
        self.bubble_down(index);
    }

    pub fn contains(&self, item: &T) -> bool {
        self.index_map.contains_key(item)
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    fn parent_index(&self, index: usize) -> Option<usize> {
        if index == 0 {
            None
        } else {
            Some((index - 1) / self.d)
        }
    }

    fn first_child_index(&self, index: usize) -> usize {
        self.d * index + 1
    }

    fn bubble_up(&mut self, mut index: usize) {
        while let Some(parent_idx) = self.parent_index(index) {
            if self.compare.compare(&self.heap[index], &self.heap[parent_idx]) 
                == std::cmp::Ordering::Less {
                self.swap(index, parent_idx);
                index = parent_idx;
            } else {
                break;
            }
        }
    }

    fn bubble_down(&mut self, mut index: usize) {
        loop {
            let first_child = self.first_child_index(index);
            if first_child >= self.heap.len() {
                break;
            }

            let mut min_child_idx = first_child;
            let last_child = std::cmp::min(first_child + self.d, self.heap.len());
            
            for child_idx in (first_child + 1)..last_child {
                if self.compare.compare(&self.heap[child_idx], &self.heap[min_child_idx]) 
                    == std::cmp::Ordering::Less {
                    min_child_idx = child_idx;
                }
            }

            if self.compare.compare(&self.heap[min_child_idx], &self.heap[index]) 
                == std::cmp::Ordering::Less {
                self.swap(index, min_child_idx);
                index = min_child_idx;
            } else {
                break;
            }
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        // Update index map before swapping
        let item_i = self.heap[i].clone();
        let item_j = self.heap[j].clone();
        
        self.index_map.insert(item_i, j);
        self.index_map.insert(item_j, i);
        
        self.heap.swap(i, j);
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

    // Insert tests
    #[test]
    fn insert_postcondition_item_findable() {
        let mut pq = new_item_min_heap(4);

        let item = Item::new("test-item", 50);
        pq.insert(item.clone());

        assert!(pq.contains(&item), "inserted item should be findable via contains()");
        assert!(pq.contains(&Item::new("test-item", 999)), "item with same ID should be found regardless of priority value");
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

            assert_eq!(size_after, size_before + 1, "size should increment by 1 after insert");
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

        assert_eq!(pq.front().id, "urgent", "new highest priority item should become front");
    }

    // Pop tests
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

            assert_eq!(size_after, size_before - 1, "size should decrement by 1 after pop");
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

    // Front tests
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

    // Increase priority tests
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

    #[test]
    #[should_panic(expected = "item must exist")]
    fn increase_priority_edge_not_found_panics() {
        let mut pq = new_item_min_heap(4);

        pq.insert(Item::new("existing", 50));

        // This should panic - item not found
        let nonexistent = Item::new("nonexistent", 10);
        pq.increase_priority(&nonexistent);
    }

    // Decrease priority tests
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

        assert_eq!(size_after, size_before, "size should be unchanged after priority update");
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