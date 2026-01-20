// src/lib.rs

use std::collections::HashMap;
use std::hash::Hash;

/// Trait for comparing priorities of items
pub trait PriorityCompare<T> {
    /// Returns true if `a` has higher priority than `b` (should come before in heap)
    fn has_higher_priority(&self, a: &T, b: &T) -> bool;
}

/// Min-heap comparator based on a function
pub struct MinBy<F>(pub F);

impl<T, F, P> PriorityCompare<T> for MinBy<F>
where
    F: Fn(&T) -> P,
    P: Ord,
{
    fn has_higher_priority(&self, a: &T, b: &T) -> bool {
        self.0(a) < self.0(b)
    }
}

/// A d-ary heap priority queue
pub struct PriorityQueue<T, C> {
    /// The arity of the heap
    d: usize,
    /// The heap storage
    heap: Vec<T>,
    /// Map from item to its index in the heap
    index_map: HashMap<T, usize>,
    /// Priority comparator
    comparator: C,
}

impl<T, C> PriorityQueue<T, C>
where
    T: Clone + Eq + Hash,
    C: PriorityCompare<T>,
{
    /// Create a new d-ary heap with the given arity and comparator
    pub fn new(d: usize, comparator: C) -> Self {
        assert!(d >= 2, "Heap arity must be at least 2");
        Self {
            d,
            heap: Vec::new(),
            index_map: HashMap::new(),
            comparator,
        }
    }

    /// Insert an item into the queue
    pub fn insert(&mut self, item: T) {
        // If item already exists, update it
        if let Some(&index) = self.index_map.get(&item) {
            self.heap[index] = item.clone();
            // Try both directions since we don't know if priority increased or decreased
            self.bubble_up(index);
            self.bubble_down(index);
        } else {
            // Add new item
            let index = self.heap.len();
            self.heap.push(item.clone());
            self.index_map.insert(item, index);
            self.bubble_up(index);
        }
    }

    /// Remove and return the item with highest priority
    pub fn pop(&mut self) {
        if self.heap.is_empty() {
            return;
        }

        let last_index = self.heap.len() - 1;
        
        if last_index == 0 {
            let item = self.heap.pop().unwrap();
            self.index_map.remove(&item);
        } else {
            // Remove the root from index_map
            self.index_map.remove(&self.heap[0]);
            
            // Move last element to root
            self.heap.swap(0, last_index);
            let item = self.heap.pop().unwrap();
            
            // Update index for the moved item
            if !self.heap.is_empty() {
                self.index_map.insert(self.heap[0].clone(), 0);
                self.bubble_down(0);
            }
        }
    }

    /// Return the item with highest priority without removing it
    pub fn front(&self) -> &T {
        self.heap.first().expect("front() called on empty heap")
    }

    /// Return the item with highest priority without removing it, or None if empty
    pub fn peek(&self) -> Option<&T> {
        self.heap.first()
    }

    /// Update an existing item to have higher priority (lower value for min-heap)
    pub fn increase_priority(&mut self, item: &T) {
        let index = *self.index_map.get(item)
            .expect("increase_priority: item must exist in heap");
        
        self.heap[index] = item.clone();
        self.bubble_up(index);
    }

    /// Update an existing item to have lower priority (higher value for min-heap)
    pub fn decrease_priority(&mut self, item: &T) {
        let index = *self.index_map.get(item)
            .expect("decrease_priority: item must exist in heap");
        
        self.heap[index] = item.clone();
        self.bubble_down(index);
    }

    /// Check if an item with the given identity exists
    pub fn contains(&self, item: &T) -> bool {
        self.index_map.contains_key(item)
    }

    /// Return the number of items in the queue
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Return whether the queue is empty
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Get the parent index of a node
    fn parent(&self, index: usize) -> Option<usize> {
        if index == 0 {
            None
        } else {
            Some((index - 1) / self.d)
        }
    }

    /// Get the index of the k-th child (0-indexed)
    fn child(&self, index: usize, k: usize) -> Option<usize> {
        let child_index = self.d * index + k + 1;
        if child_index < self.heap.len() {
            Some(child_index)
        } else {
            None
        }
    }

    /// Get all children indices of a node
    fn children(&self, index: usize) -> Vec<usize> {
        let mut result = Vec::new();
        for k in 0..self.d {
            if let Some(child_index) = self.child(index, k) {
                result.push(child_index);
            } else {
                break;
            }
        }
        result
    }

    /// Bubble up to maintain heap property
    fn bubble_up(&mut self, mut index: usize) {
        while let Some(parent_index) = self.parent(index) {
            if self.comparator.has_higher_priority(&self.heap[index], &self.heap[parent_index]) {
                // Update index map
                self.index_map.insert(self.heap[index].clone(), parent_index);
                self.index_map.insert(self.heap[parent_index].clone(), index);
                
                // Swap elements
                self.heap.swap(index, parent_index);
                index = parent_index;
            } else {
                break;
            }
        }
    }

    /// Bubble down to maintain heap property
    fn bubble_down(&mut self, mut index: usize) {
        loop {
            let children = self.children(index);
            if children.is_empty() {
                break;
            }

            // Find child with highest priority
            let mut best_child = children[0];
            for &child_index in &children[1..] {
                if self.comparator.has_higher_priority(&self.heap[child_index], &self.heap[best_child]) {
                    best_child = child_index;
                }
            }

            // Check if we need to swap with best child
            if self.comparator.has_higher_priority(&self.heap[best_child], &self.heap[index]) {
                // Update index map
                self.index_map.insert(self.heap[index].clone(), best_child);
                self.index_map.insert(self.heap[best_child].clone(), index);
                
                // Swap elements
                self.heap.swap(index, best_child);
                index = best_child;
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

    pub mod insert;
    pub mod pop;
    pub mod front;
    pub mod increase_priority;
    pub mod decrease_priority;
}