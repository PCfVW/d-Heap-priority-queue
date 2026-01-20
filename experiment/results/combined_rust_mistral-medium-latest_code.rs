use std::collections::HashMap;
use std::hash::Hash;

/// Trait for comparing priorities of items
pub trait PriorityCompare<T> {
    fn compare(&self, a: &T, b: &T) -> std::cmp::Ordering;
}

/// Min-heap comparator (lower values have higher priority)
pub struct MinBy<F>(pub F);

impl<T, F> PriorityCompare<T> for MinBy<F>
where
    F: Fn(&T) -> i32,
{
    fn compare(&self, a: &T, b: &T) -> std::cmp::Ordering {
        let a_priority = (self.0)(a);
        let b_priority = (self.0)(b);
        a_priority.cmp(&b_priority)
    }
}

/// D-ary heap priority queue
pub struct PriorityQueue<T, C> {
    heap: Vec<T>,
    position_map: HashMap<T, usize>,
    d: usize,
    comparator: C,
}

impl<T, C> PriorityQueue<T, C>
where
    T: Eq + Hash + Clone,
    C: PriorityCompare<T>,
{
    /// Create a new priority queue with the specified arity
    pub fn new(d: usize, comparator: C) -> Self {
        assert!(d >= 2, "arity must be at least 2");
        PriorityQueue {
            heap: Vec::new(),
            position_map: HashMap::new(),
            d,
            comparator,
        }
    }

    /// Add an item to the queue
    pub fn insert(&mut self, item: T) {
        assert!(
            !self.position_map.contains_key(&item),
            "item with same identity already exists"
        );

        self.heap.push(item.clone());
        let index = self.heap.len() - 1;
        self.position_map.insert(item, index);
        self.sift_up(index);
    }

    /// Remove and return the item with highest priority
    pub fn pop(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }

        let last_index = self.heap.len() - 1;
        self.swap(0, last_index);
        let item = self.heap.pop().unwrap();
        self.position_map.remove(&item);

        if !self.heap.is_empty() {
            self.sift_down(0);
        }

        Some(item)
    }

    /// Return the item with highest priority without removing it
    pub fn front(&self) -> &T {
        self.heap
            .first()
            .expect("cannot call front() on empty priority queue")
    }

    /// Return the item with highest priority without removing it (Option version)
    pub fn peek(&self) -> Option<&T> {
        self.heap.first()
    }

    /// Update an existing item to have higher priority (lower priority value)
    pub fn increase_priority(&mut self, item: &T) {
        let index = *self
            .position_map
            .get(item)
            .expect("item must exist to increase priority");

        // Update the item in the heap
        let old_item = self.heap[index].clone();
        self.heap[index] = item.clone();
        self.position_map.remove(&old_item);
        self.position_map.insert(item.clone(), index);

        self.sift_up(index);
    }

    /// Update an existing item to have lower priority (higher priority value)
    pub fn decrease_priority(&mut self, item: &T) {
        let index = *self
            .position_map
            .get(item)
            .expect("item must exist to decrease priority");

        // Update the item in the heap
        let old_item = self.heap[index].clone();
        self.heap[index] = item.clone();
        self.position_map.remove(&old_item);
        self.position_map.insert(item.clone(), index);

        self.sift_down(index);
    }

    /// Check if an item with the given identity exists in the queue
    pub fn contains(&self, item: &T) -> bool {
        self.position_map.contains_key(item)
    }

    /// Return the number of items in the queue
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Return whether the queue contains no items
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Restore heap property by moving an item up toward the root
    fn sift_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent_index = (index - 1) / self.d;
            if self.comparator.compare(&self.heap[index], &self.heap[parent_index])
                == std::cmp::Ordering::Less
            {
                self.swap(index, parent_index);
                index = parent_index;
            } else {
                break;
            }
        }
    }

    /// Restore heap property by moving an item down toward the leaves
    fn sift_down(&mut self, mut index: usize) {
        loop {
            let first_child_index = index * self.d + 1;
            if first_child_index >= self.heap.len() {
                break;
            }

            // Find the child with highest priority (lowest value)
            let last_child_index = std::cmp::min(
                first_child_index + self.d - 1,
                self.heap.len() - 1,
            );
            let mut min_child_index = first_child_index;

            for child_index in first_child_index..=last_child_index {
                if self.comparator.compare(&self.heap[child_index], &self.heap[min_child_index])
                    == std::cmp::Ordering::Less
                {
                    min_child_index = child_index;
                }
            }

            // If the child has higher priority, swap and continue
            if self.comparator.compare(&self.heap[min_child_index], &self.heap[index])
                == std::cmp::Ordering::Less
            {
                self.swap(index, min_child_index);
                index = min_child_index;
            } else {
                break;
            }
        }
    }

    /// Swap two items in the heap and update their positions in the map
    fn swap(&mut self, i: usize, j: usize) {
        let item_i = self.heap[i].clone();
        let item_j = self.heap[j].clone();

        self.heap.swap(i, j);
        self.position_map.insert(item_i, j);
        self.position_map.insert(item_j, i);
    }
}