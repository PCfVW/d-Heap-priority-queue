use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct Item<I, P> {
    pub identity: I,
    pub priority: P,
}

impl<I, P> Item<I, P> {
    pub fn new(identity: I, priority: P) -> Self {
        Self { identity, priority }
    }
}

impl<I: PartialEq, P> PartialEq for Item<I, P> {
    fn eq(&self, other: &Self) -> bool {
        self.identity == other.identity
    }
}

impl<I: Eq, P> Eq for Item<I, P> {}

impl<I: Hash, P> Hash for Item<I, P> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.identity.hash(state);
    }
}

#[derive(Debug)]
pub struct DaryHeap<I, P> {
    heap: Vec<Item<I, P>>,
    positions: HashMap<I, usize>,
    arity: usize,
}

impl<I, P> DaryHeap<I, P>
where
    I: Clone + Eq + Hash,
    P: Clone + PartialOrd,
{
    /// Create a new d-ary heap with the specified arity
    pub fn new(arity: usize) -> Self {
        assert!(arity >= 2, "Heap arity must be at least 2");
        Self {
            heap: Vec::new(),
            positions: HashMap::new(),
            arity,
        }
    }

    /// Insert an item into the heap
    pub fn insert(&mut self, item: Item<I, P>) -> Result<(), &'static str> {
        if self.positions.contains_key(&item.identity) {
            return Err("Item with this identity already exists");
        }

        let index = self.heap.len();
        self.positions.insert(item.identity.clone(), index);
        self.heap.push(item);
        self.bubble_up(index);
        Ok(())
    }

    /// Remove and return the item with highest priority (lowest value)
    pub fn pop(&mut self) -> Option<Item<I, P>> {
        if self.heap.is_empty() {
            return None;
        }

        let root = self.heap[0].clone();
        self.positions.remove(&root.identity);

        if self.heap.len() == 1 {
            self.heap.pop();
            return Some(root);
        }

        // Move last element to root and bubble down
        let last = self.heap.pop().unwrap();
        self.heap[0] = last;
        self.positions.insert(self.heap[0].identity.clone(), 0);
        self.bubble_down(0);

        Some(root)
    }

    /// Return the item with highest priority without removing it
    pub fn front(&self) -> Option<&Item<I, P>> {
        self.heap.first()
    }

    /// Update an existing item to have higher priority (lower value)
    pub fn increase_priority(&mut self, identity: &I, new_priority: P) -> Result<(), &'static str> {
        let &index = self.positions.get(identity)
            .ok_or("Item not found")?;

        if new_priority >= self.heap[index].priority {
            return Err("New priority is not higher (lower value) than current priority");
        }

        self.heap[index].priority = new_priority;
        self.bubble_up(index);
        Ok(())
    }

    /// Update an existing item to have lower priority (higher value)
    pub fn decrease_priority(&mut self, identity: &I, new_priority: P) -> Result<(), &'static str> {
        let &index = self.positions.get(identity)
            .ok_or("Item not found")?;

        if new_priority <= self.heap[index].priority {
            return Err("New priority is not lower (higher value) than current priority");
        }

        self.heap[index].priority = new_priority;
        self.bubble_down(index);
        Ok(())
    }

    /// Check if an item with the given identity exists
    pub fn contains(&self, identity: &I) -> bool {
        self.positions.contains_key(identity)
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
            Some((index - 1) / self.arity)
        }
    }

    /// Get the first child index of a node
    fn first_child(&self, index: usize) -> usize {
        self.arity * index + 1
    }

    /// Bubble up an element to maintain heap property
    fn bubble_up(&mut self, mut index: usize) {
        while let Some(parent_idx) = self.parent(index) {
            if self.heap[index].priority >= self.heap[parent_idx].priority {
                break;
            }
            self.swap(index, parent_idx);
            index = parent_idx;
        }
    }

    /// Bubble down an element to maintain heap property
    fn bubble_down(&mut self, mut index: usize) {
        loop {
            let first_child = self.first_child(index);
            if first_child >= self.heap.len() {
                break; // No children
            }

            // Find the child with minimum priority
            let mut min_child = first_child;
            let last_child = std::cmp::min(first_child + self.arity, self.heap.len());
            
            for child in first_child + 1..last_child {
                if self.heap[child].priority < self.heap[min_child].priority {
                    min_child = child;
                }
            }

            // If current node has lower or equal priority than min child, we're done
            if self.heap[index].priority <= self.heap[min_child].priority {
                break;
            }

            self.swap(index, min_child);
            index = min_child;
        }
    }

    /// Swap two elements in the heap and update their positions
    fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
        self.positions.insert(self.heap[i].identity.clone(), i);
        self.positions.insert(self.heap[j].identity.clone(), j);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut heap = DaryHeap::new(3);
        
        assert!(heap.is_empty());
        assert_eq!(heap.len(), 0);
        assert_eq!(heap.front(), None);
        assert_eq!(heap.pop(), None);

        // Insert items
        heap.insert(Item::new("a", 5)).unwrap();
        heap.insert(Item::new("b", 2)).unwrap();
        heap.insert(Item::new("c", 8)).unwrap();
        heap.insert(Item::new("d", 1)).unwrap();

        assert_eq!(heap.len(), 4);
        assert!(!heap.is_empty());
        assert!(heap.contains(&"a"));
        assert!(heap.contains(&"d"));
        assert!(!heap.contains(&"x"));

        // Check front (should be item with priority 1)
        assert_eq!(heap.front().unwrap().identity, "d");
        assert_eq!(heap.front().unwrap().priority, 1);

        // Pop items (should come out in priority order)
        assert_eq!(heap.pop().unwrap().identity, "d"); // priority 1
        assert_eq!(heap.pop().unwrap().identity, "b"); // priority 2
        assert_eq!(heap.pop().unwrap().identity, "a"); // priority 5
        assert_eq!(heap.pop().unwrap().identity, "c"); // priority 8
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_priority_updates() {
        let mut heap = DaryHeap::new(2);
        
        heap.insert(Item::new("a", 10)).unwrap();
        heap.insert(Item::new("b", 20)).unwrap();
        heap.insert(Item::new("c", 30)).unwrap();

        // Initially, "a" should be at front
        assert_eq!(heap.front().unwrap().identity, "a");

        // Increase priority of "c" (make it 5, higher priority than "a")
        heap.increase_priority(&"c", 5).unwrap();
        assert_eq!(heap.front().unwrap().identity, "c");

        // Decrease priority of "c" (make it 25, lower priority)
        heap.decrease_priority(&"c", 25).unwrap();
        assert_eq!(heap.front().unwrap().identity, "a");
    }

    #[test]
    fn test_duplicate_identity() {
        let mut heap = DaryHeap::new(2);
        
        heap.insert(Item::new("a", 10)).unwrap();
        let result = heap.insert(Item::new("a", 5));
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_priority_updates() {
        let mut heap = DaryHeap::new(2);
        
        heap.insert(Item::new("a", 10)).unwrap();

        // Try to "increase" priority with a higher value (should fail)
        assert!(heap.increase_priority(&"a", 15).is_err());
        
        // Try to "decrease" priority with a lower value (should fail)
        assert!(heap.decrease_priority(&"a", 5).is_err());
        
        // Try to update non-existent item
        assert!(heap.increase_priority(&"x", 5).is_err());
        assert!(heap.decrease_priority(&"x", 15).is_err());
    }

    #[test]
    fn test_different_arities() {
        for arity in 2..=5 {
            let mut heap = DaryHeap::new(arity);
            
            // Insert items in reverse priority order
            for i in (1..=10).rev() {
                heap.insert(Item::new(i, i)).unwrap();
            }

            // Pop all items, should come out in priority order
            for expected in 1..=10 {
                let item = heap.pop().unwrap();
                assert_eq!(item.identity, expected);
                assert_eq!(item.priority, expected);
            }
        }
    }

    #[test]
    #[should_panic(expected = "Heap arity must be at least 2")]
    fn test_invalid_arity() {
        DaryHeap::<i32, i32>::new(1);
    }
}

fn main() {
    // Example usage
    let mut heap = DaryHeap::new(3); // 3-ary heap

    // Insert some tasks with priorities
    heap.insert(Item::new("task1", 5)).unwrap();
    heap.insert(Item::new("task2", 1)).unwrap();
    heap.insert(Item::new("task3", 3)).unwrap();
    heap.insert(Item::new("task4", 2)).unwrap();

    println!("Heap size: {}", heap.len());
    println!("Contains task2: {}", heap.contains(&"task2"));

    // Process tasks in priority order
    while let Some(task) = heap.pop() {
        println!("Processing task: {} with priority: {}", task.identity, task.priority);
    }

    // Example with priority updates
    let mut heap = DaryHeap::new(2);
    heap.insert(Item::new("urgent", 10)).unwrap();
    heap.insert(Item::new("normal", 20)).unwrap();

    println!("\nBefore priority update:");
    println!("Next task: {}", heap.front().unwrap().identity);

    // Make "normal" task more urgent
    heap.increase_priority(&"normal", 5).unwrap();
    
    println!("After priority update:");
    println!("Next task: {}", heap.front().unwrap().identity);
}