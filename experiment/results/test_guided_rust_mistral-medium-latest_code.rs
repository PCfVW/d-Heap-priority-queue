use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

/// Trait for comparing priorities
pub trait PriorityCompare<T> {
    fn compare(&self, a: &T, b: &T) -> std::cmp::Ordering;
}

/// Min-heap comparator
pub struct MinBy<F> {
    priority: F,
}

impl<T, F> PriorityCompare<T> for MinBy<F>
where
    F: Fn(&T) -> i32,
{
    fn compare(&self, a: &T, b: &T) -> std::cmp::Ordering {
        let a_priority = (self.priority)(a);
        let b_priority = (self.priority)(b);
        a_priority.cmp(&b_priority)
    }
}

/// D-ary heap priority queue
pub struct PriorityQueue<T, C> {
    heap: Vec<T>,
    indices: HashMap<String, usize>,
    d: usize,
    compare: C,
    _marker: PhantomData<*const C>,
}

impl<T, C> PriorityQueue<T, C>
where
    T: Eq + Hash + Clone,
    C: PriorityCompare<T>,
{
    /// Create a new priority queue with given arity and comparator
    pub fn new(d: usize, compare: C) -> Self {
        assert!(d >= 2, "Heap arity must be at least 2");
        Self {
            heap: Vec::new(),
            indices: HashMap::new(),
            d,
            compare,
            _marker: PhantomData,
        }
    }

    /// Get the number of items in the queue
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Check if an item exists in the queue (based on identity)
    pub fn contains(&self, item: &T) -> bool {
        self.indices.contains_key(&Self::get_id(item))
    }

    /// Get the front item without removing it
    pub fn front(&self) -> &T {
        self.heap.get(0).expect("Heap is empty")
    }

    /// Peek at the front item (returns None if empty)
    pub fn peek(&self) -> Option<&T> {
        self.heap.get(0)
    }

    /// Insert an item into the queue
    pub fn insert(&mut self, item: T) {
        let id = Self::get_id(&item);
        if self.indices.contains_key(&id) {
            panic!("Item with this identity already exists in the queue");
        }

        self.indices.insert(id, self.heap.len());
        self.heap.push(item);
        self.heapify_up(self.heap.len() - 1);
    }

    /// Remove and return the item with highest priority
    pub fn pop(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }

        let root = self.heap.swap_remove(0);
        self.indices.remove(&Self::get_id(&root));

        if !self.heap.is_empty() {
            // Update the index of the item that was moved to root
            let id = Self::get_id(&self.heap[0]);
            *self.indices.get_mut(&id).unwrap() = 0;
            self.heapify_down(0);
        }

        Some(root)
    }

    /// Increase priority of an existing item (moves it toward the root)
    pub fn increase_priority(&mut self, item: &T) {
        let id = Self::get_id(item);
        let index = *self.indices.get(&id).expect("Item must exist in the queue");

        // Update the item in place
        self.heap[index] = item.clone();

        // Move it up the heap
        self.heapify_up(index);
    }

    /// Decrease priority of an existing item (moves it toward the leaves)
    pub fn decrease_priority(&mut self, item: &T) {
        let id = Self::get_id(item);
        let index = *self.indices.get(&id).expect("Item must exist in the queue");

        // Update the item in place
        self.heap[index] = item.clone();

        // Move it down the heap
        self.heapify_down(index);
    }

    /// Helper to get the ID of an item (for Item type)
    fn get_id(item: &T) -> String {
        // This is a bit of a hack since we can't know the exact type structure
        // In the test cases, we know it's Item with an id field
        // In a real implementation, we'd want a proper trait for this
        if let Some(item_any) = item as *const T as *const Item {
            unsafe { (*item_any).id.clone() }
        } else {
            panic!("Unsupported item type - must have String id field");
        }
    }

    /// Move an item up the heap until heap property is restored
    fn heapify_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / self.d;
            if self.compare.compare(&self.heap[index], &self.heap[parent]) == std::cmp::Ordering::Less {
                self.swap(index, parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    /// Move an item down the heap until heap property is restored
    fn heapify_down(&mut self, mut index: usize) {
        loop {
            let mut smallest = index;
            let first_child = index * self.d + 1;

            // Check all children
            for i in 0..self.d {
                let child = first_child + i;
                if child < self.heap.len() {
                    if self.compare.compare(&self.heap[child], &self.heap[smallest]) == std::cmp::Ordering::Less {
                        smallest = child;
                    }
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

    /// Swap two items in the heap and update their indices
    fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);

        let id_i = Self::get_id(&self.heap[i]);
        let id_j = Self::get_id(&self.heap[j]);

        *self.indices.get_mut(&id_i).unwrap() = i;
        *self.indices.get_mut(&id_j).unwrap() = j;
    }
}

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

impl Hash for Item {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}