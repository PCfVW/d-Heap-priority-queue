use std::collections::HashMap;
use std::fmt::Debug;

/// A d-ary heap priority queue.
pub struct PriorityQueue<T, C>
where
    T: Eq + Hash + Clone,
    C: PriorityCompare<T>,
{
    heap: Vec<T>,
    priority_map: HashMap<T, usize>,
    arity: usize,
    compare: C,
}

/// A trait for comparing priorities of items in the priority queue.
pub trait PriorityCompare<T>
where
    T: Eq + Hash + Clone,
{
    fn compare(&self, a: &T, b: &T) -> std::cmp::Ordering;
}

/// A struct that implements the `PriorityCompare` trait for a min-heap.
pub struct MinBy<F>
where
    F: Fn(&T) -> i32,
{
    pub(crate) f: F,
}

impl<T, F> PriorityCompare<T> for MinBy<F>
where
    T: Eq + Hash + Clone,
    F: Fn(&T) -> i32,
{
    fn compare(&self, a: &T, b: &T) -> std::cmp::Ordering {
        self.f(a).cmp(&self.f(b))
    }
}

impl<T, C> PriorityQueue<T, C>
where
    T: Eq + Hash + Clone,
    C: PriorityCompare<T>,
{
    /// Creates a new d-ary heap priority queue.
    pub fn new(arity: usize, compare: C) -> Self {
        PriorityQueue {
            heap: Vec::new(),
            priority_map: HashMap::new(),
            arity,
            compare,
        }
    }

    /// Inserts an item into the priority queue.
    pub fn insert(&mut self, item: T) {
        let index = self.heap.len();
        self.heap.push(item.clone());
        self.priority_map.insert(item, index);
        self.bubble_up(index);
    }

    /// Removes and returns the item with the highest priority (lowest value).
    pub fn pop(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }

        let root = self.heap.swap_remove(0);
        self.priority_map.remove(&root);

        if !self.heap.is_empty() {
            let last = self.heap.pop().unwrap();
            self.heap.insert(0, last);
            self.priority_map.insert(last, 0);
            self.bubble_down(0);
        }

        Some(root)
    }

    /// Returns the item with the highest priority (lowest value) without removing it.
    pub fn front(&self) -> Option<&T> {
        self.heap.first()
    }

    /// Updates an existing item to have higher priority (lower value).
    pub fn increase_priority(&mut self, item: &T) {
        let index = self.priority_map.get(item).expect("item must exist");
        self.heap[*index] = item.clone();
        self.bubble_up(*index);
    }

    /// Updates an existing item to have lower priority (higher value).
    pub fn decrease_priority(&mut self, item: &T) {
        let index = self.priority_map.get(item).expect("item must exist");
        self.heap[*index] = item.clone();
        self.bubble_down(*index);
    }

    /// Checks if an item with the given identity exists in the priority queue.
    pub fn contains(&self, item: &T) -> bool {
        self.priority_map.contains_key(item)
    }

    /// Returns the number of items in the priority queue.
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Returns whether the priority queue is empty.
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    fn bubble_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / self.arity;
            if self.compare.compare(&self.heap[index], &self.heap[parent]) == std::cmp::Ordering::Less {
                self.heap.swap(index, parent);
                self.priority_map.insert(self.heap[index].clone(), index);
                self.priority_map.insert(self.heap[parent].clone(), parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    fn bubble_down(&mut self, mut index: usize) {
        while index < self.heap.len() {
            let mut min_child = index;
            for i in 1..=self.arity {
                let child = index * self.arity + i;
                if child < self.heap.len()
                    && self.compare.compare(&self.heap[child], &self.heap[min_child]) == std::cmp::Ordering::Less
                {
                    min_child = child;
                }
            }
            if min_child != index {
                self.heap.swap(index, min_child);
                self.priority_map.insert(self.heap[index].clone(), index);
                self.priority_map.insert(self.heap[min_child].clone(), min_child);
                index = min_child;
            } else {
                break;
            }
        }
    }

    /// Returns the item with the highest priority (lowest value) without removing it.
    pub fn peek(&self) -> Option<&T> {
        self.front()
    }
}

impl<T, C> Debug for PriorityQueue<T, C>
where
    T: Eq + Hash + Clone + Debug,
    C: PriorityCompare<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PriorityQueue")
            .field("heap", &self.heap)
            .field("priority_map", &self.priority_map)
            .field("arity", &self.arity)
            .finish()
    }
}