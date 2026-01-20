use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct DaryHeap<T> {
    heap: Vec<T>,
    position_map: HashMap<T, usize>,
    d: usize,
}

pub trait Priority {
    type Identity: Hash + Eq + Clone;
    
    fn identity(&self) -> Self::Identity;
    fn priority(&self) -> i32;
}

#[derive(Debug)]
pub enum HeapError {
    Empty,
    ItemNotFound,
    ItemAlreadyExists,
    InvalidArity,
}

impl<T> DaryHeap<T>
where
    T: Priority + Clone + Debug,
{
    /// Create a new d-ary heap with the specified arity
    pub fn new(d: usize) -> Result<Self, HeapError> {
        if d < 2 {
            return Err(HeapError::InvalidArity);
        }
        
        Ok(DaryHeap {
            heap: Vec::new(),
            position_map: HashMap::new(),
            d,
        })
    }

    /// Add an item to the queue
    pub fn insert(&mut self, item: T) -> Result<(), HeapError> {
        let identity = item.identity();
        
        // Check if item already exists
        if self.position_map.contains_key(&identity) {
            return Err(HeapError::ItemAlreadyExists);
        }
        
        // Add to end of heap
        let index = self.heap.len();
        self.heap.push(item);
        self.position_map.insert(identity, index);
        
        // Restore heap property by sifting up
        self.sift_up(index);
        
        Ok(())
    }

    /// Remove and return the item with highest priority
    pub fn pop(&mut self) -> Result<T, HeapError> {
        if self.heap.is_empty() {
            return Err(HeapError::Empty);
        }
        
        let root = self.heap[0].clone();
        let root_identity = root.identity();
        
        // Remove from position map
        self.position_map.remove(&root_identity);
        
        if self.heap.len() == 1 {
            self.heap.pop();
            return Ok(root);
        }
        
        // Move last element to root
        let last = self.heap.pop().unwrap();
        let last_identity = last.identity();
        
        self.heap[0] = last;
        self.position_map.insert(last_identity, 0);
        
        // Restore heap property by sifting down
        self.sift_down(0);
        
        Ok(root)
    }

    /// Return the item with highest priority without removing it
    pub fn front(&self) -> Result<&T, HeapError> {
        if self.heap.is_empty() {
            Err(HeapError::Empty)
        } else {
            Ok(&self.heap[0])
        }
    }

    /// Update an existing item to have higher priority (lower priority value)
    pub fn increase_priority(&mut self, item: T) -> Result<(), HeapError> {
        let identity = item.identity();
        
        let &index = self.position_map.get(&identity)
            .ok_or(HeapError::ItemNotFound)?;
        
        // Update the item
        self.heap[index] = item;
        
        // Since priority increased (value decreased), sift up
        self.sift_up(index);
        
        Ok(())
    }

    /// Update an existing item to have lower priority (higher priority value)
    pub fn decrease_priority(&mut self, item: T) -> Result<(), HeapError> {
        let identity = item.identity();
        
        let &index = self.position_map.get(&identity)
            .ok_or(HeapError::ItemNotFound)?;
        
        // Update the item
        self.heap[index] = item;
        
        // Since priority decreased (value increased), sift down
        self.sift_down(index);
        
        Ok(())
    }

    /// Check if an item with the given identity exists in the queue
    pub fn contains(&self, item: &T) -> bool {
        self.position_map.contains_key(&item.identity())
    }

    /// Return the number of items in the queue
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Return whether the queue contains no items
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Get parent index for a given index
    fn parent_index(&self, index: usize) -> Option<usize> {
        if index == 0 {
            None
        } else {
            Some((index - 1) / self.d)
        }
    }

    /// Get the range of child indices for a given index
    fn child_range(&self, index: usize) -> (usize, usize) {
        let first_child = self.d * index + 1;
        let last_child = std::cmp::min(self.d * index + self.d, self.heap.len() - 1);
        (first_child, last_child)
    }

    /// Restore heap property by moving an item up toward the root
    fn sift_up(&mut self, mut index: usize) {
        while let Some(parent_idx) = self.parent_index(index) {
            // If current item has higher priority (lower value) than parent
            if self.heap[index].priority() < self.heap[parent_idx].priority() {
                self.swap(index, parent_idx);
                index = parent_idx;
            } else {
                break;
            }
        }
    }

    /// Restore heap property by moving an item down toward the leaves
    fn sift_down(&mut self, mut index: usize) {
        loop {
            let (first_child, last_child) = self.child_range(index);
            
            // If no children, we're done
            if first_child >= self.heap.len() {
                break;
            }
            
            // Find child with highest priority (lowest priority value)
            let mut best_child = first_child;
            for child_idx in first_child..=last_child {
                if child_idx < self.heap.len() && 
                   self.heap[child_idx].priority() < self.heap[best_child].priority() {
                    best_child = child_idx;
                }
            }
            
            // If best child has higher priority than current item, swap
            if best_child < self.heap.len() && 
               self.heap[best_child].priority() < self.heap[index].priority() {
                self.swap(index, best_child);
                index = best_child;
            } else {
                break;
            }
        }
    }

    /// Swap two elements in the heap and update position map
    fn swap(&mut self, i: usize, j: usize) {
        let identity_i = self.heap[i].identity();
        let identity_j = self.heap[j].identity();
        
        self.heap.swap(i, j);
        
        self.position_map.insert(identity_i, j);
        self.position_map.insert(identity_j, i);
    }
}

// Example implementation of Priority trait
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Task {
    id: u32,
    priority: i32,
    name: String,
}

impl Task {
    pub fn new(id: u32, priority: i32, name: String) -> Self {
        Task { id, priority, name }
    }
}

impl Priority for Task {
    type Identity = u32;
    
    fn identity(&self) -> Self::Identity {
        self.id
    }
    
    fn priority(&self) -> i32 {
        self.priority
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut heap = DaryHeap::new(3).unwrap();
        
        assert!(heap.is_empty());
        assert_eq!(heap.len(), 0);
        
        // Insert items
        heap.insert(Task::new(1, 10, "Low priority".to_string())).unwrap();
        heap.insert(Task::new(2, 5, "Medium priority".to_string())).unwrap();
        heap.insert(Task::new(3, 1, "High priority".to_string())).unwrap();
        
        assert_eq!(heap.len(), 3);
        assert!(!heap.is_empty());
        
        // Check front (highest priority = lowest value)
        let front = heap.front().unwrap();
        assert_eq!(front.id, 3);
        assert_eq!(front.priority, 1);
        
        // Pop items in priority order
        let item1 = heap.pop().unwrap();
        assert_eq!(item1.id, 3);
        assert_eq!(item1.priority, 1);
        
        let item2 = heap.pop().unwrap();
        assert_eq!(item2.id, 2);
        assert_eq!(item2.priority, 5);
        
        let item3 = heap.pop().unwrap();
        assert_eq!(item3.id, 1);
        assert_eq!(item3.priority, 10);
        
        assert!(heap.is_empty());
    }

    #[test]
    fn test_contains() {
        let mut heap = DaryHeap::new(2).unwrap();
        
        let task1 = Task::new(1, 10, "Task 1".to_string());
        let task2 = Task::new(2, 5, "Task 2".to_string());
        
        assert!(!heap.contains(&task1));
        
        heap.insert(task1.clone()).unwrap();
        assert!(heap.contains(&task1));
        assert!(!heap.contains(&task2));
        
        heap.insert(task2.clone()).unwrap();
        assert!(heap.contains(&task1));
        assert!(heap.contains(&task2));
    }

    #[test]
    fn test_priority_updates() {
        let mut heap = DaryHeap::new(2).unwrap();
        
        heap.insert(Task::new(1, 10, "Task 1".to_string())).unwrap();
        heap.insert(Task::new(2, 20, "Task 2".to_string())).unwrap();
        heap.insert(Task::new(3, 15, "Task 3".to_string())).unwrap();
        
        // Initially, task 1 should be at front (priority 10)
        assert_eq!(heap.front().unwrap().id, 1);
        
        // Increase priority of task 2 (make it more important)
        heap.increase_priority(Task::new(2, 5, "Task 2 updated".to_string())).unwrap();
        
        // Now task 2 should be at front
        assert_eq!(heap.front().unwrap().id, 2);
        assert_eq!(heap.front().unwrap().priority, 5);
        
        // Decrease priority of task 2 (make it less important)
        heap.decrease_priority(Task::new(2, 25, "Task 2 updated again".to_string())).unwrap();
        
        // Now task 1 should be at front again
        assert_eq!(heap.front().unwrap().id, 1);
    }

    #[test]
    fn test_errors() {
        let mut heap = DaryHeap::new(2).unwrap();
        
        // Test empty heap errors
        assert!(matches!(heap.pop(), Err(HeapError::Empty)));
        assert!(matches!(heap.front(), Err(HeapError::Empty)));
        
        // Test duplicate insertion
        let task = Task::new(1, 10, "Task".to_string());
        heap.insert(task.clone()).unwrap();
        assert!(matches!(heap.insert(task), Err(HeapError::ItemAlreadyExists)));
        
        // Test item not found
        let missing_task = Task::new(999, 1, "Missing".to_string());
        assert!(matches!(heap.increase_priority(missing_task.clone()), Err(HeapError::ItemNotFound)));
        assert!(matches!(heap.decrease_priority(missing_task), Err(HeapError::ItemNotFound)));
        
        // Test invalid arity
        assert!(matches!(DaryHeap::<Task>::new(1), Err(HeapError::InvalidArity)));
    }

    #[test]
    fn test_large_heap() {
        let mut heap = DaryHeap::new(4).unwrap();
        
        // Insert many items
        for i in 0..100 {
            heap.insert(Task::new(i, (i as i32) * 2, format!("Task {}", i))).unwrap();
        }
        
        assert_eq!(heap.len(), 100);
        
        // Pop all items - they should come out in priority order
        let mut last_priority = i32::MIN;
        while !heap.is_empty() {
            let item = heap.pop().unwrap();
            assert!(item.priority >= last_priority);
            last_priority = item.priority;
        }
    }
}

fn main() {
    // Example usage
    let mut heap = DaryHeap::new(3).unwrap();
    
    // Insert some tasks
    heap.insert(Task::new(1, 10, "Low priority task".to_string())).unwrap();
    heap.insert(Task::new(2, 5, "Medium priority task".to_string())).unwrap();
    heap.insert(Task::new(3, 1, "High priority task".to_string())).unwrap();
    heap.insert(Task::new(4, 15, "Very low priority task".to_string())).unwrap();
    
    println!("Heap size: {}", heap.len());
    println!("Front item: {:?}", heap.front().unwrap());
    
    // Process tasks in priority order
    while !heap.is_empty() {
        let task = heap.pop().unwrap();
        println!("Processing: {} (priority: {})", task.name, task.priority);
    }
}