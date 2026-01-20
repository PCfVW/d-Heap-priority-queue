use std::collections::HashMap;
use std::hash::Hash;

// Item type with separate identity and priority
#[derive(Debug, Clone)]
pub struct Item {
    pub number: u32,
    pub cost: u32,
}

impl Item {
    pub fn new(number: u32, cost: u32) -> Self {
        Self { number, cost }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl Eq for Item {}

impl Hash for Item {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.number.hash(state);
    }
}

pub struct DHeap {
    heap: Vec<Item>,
    positions: HashMap<Item, usize>,
    arity: usize,
}

impl DHeap {
    pub fn new(d: usize) -> Self {
        Self {
            heap: Vec::new(),
            positions: HashMap::new(),
            arity: d.max(2), // Ensure minimum arity of 2
        }
    }

    pub fn insert(&mut self, item: Item) {
        if self.positions.contains_key(&item) {
            // Item already exists, update its priority
            if let Some(&pos) = self.positions.get(&item) {
                let old_cost = self.heap[pos].cost;
                self.heap[pos] = item.clone();
                
                if item.cost < old_cost {
                    self.heapify_up(pos);
                } else if item.cost > old_cost {
                    self.heapify_down(pos);
                }
            }
        } else {
            // New item
            let pos = self.heap.len();
            self.heap.push(item.clone());
            self.positions.insert(item, pos);
            self.heapify_up(pos);
        }
    }

    pub fn pop(&mut self) -> Option<Item> {
        if self.heap.is_empty() {
            return None;
        }

        let min_item = self.heap[0].clone();
        self.positions.remove(&min_item);

        if self.heap.len() == 1 {
            self.heap.pop();
        } else {
            let last_item = self.heap.pop().unwrap();
            self.heap[0] = last_item.clone();
            self.positions.insert(last_item, 0);
            self.heapify_down(0);
        }

        Some(min_item)
    }

    pub fn front(&self) -> Option<&Item> {
        self.heap.get(0)
    }

    pub fn increase_priority(&mut self, item: Item) {
        if let Some(&pos) = self.positions.get(&item) {
            self.heap[pos] = item.clone();
            self.positions.insert(item, pos);
            self.heapify_up(pos);
        }
    }

    pub fn decrease_priority(&mut self, item: Item) {
        if let Some(&pos) = self.positions.get(&item) {
            self.heap[pos] = item.clone();
            self.positions.insert(item, pos);
            self.heapify_down(pos);
        }
    }

    pub fn contains(&self, item: &Item) -> bool {
        self.positions.contains_key(item)
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    fn parent(&self, index: usize) -> Option<usize> {
        if index == 0 {
            None
        } else {
            Some((index - 1) / self.arity)
        }
    }

    fn children(&self, index: usize) -> Vec<usize> {
        let first_child = self.arity * index + 1;
        let mut children = Vec::new();
        
        for i in 0..self.arity {
            let child_idx = first_child + i;
            if child_idx < self.heap.len() {
                children.push(child_idx);
            } else {
                break;
            }
        }
        
        children
    }

    fn heapify_up(&mut self, mut index: usize) {
        while let Some(parent_idx) = self.parent(index) {
            if self.heap[index].cost >= self.heap[parent_idx].cost {
                break;
            }
            
            self.swap(index, parent_idx);
            index = parent_idx;
        }
    }

    fn heapify_down(&mut self, mut index: usize) {
        loop {
            let children = self.children(index);
            if children.is_empty() {
                break;
            }

            let mut min_child_idx = children[0];
            for &child_idx in &children[1..] {
                if self.heap[child_idx].cost < self.heap[min_child_idx].cost {
                    min_child_idx = child_idx;
                }
            }

            if self.heap[index].cost <= self.heap[min_child_idx].cost {
                break;
            }

            self.swap(index, min_child_idx);
            index = min_child_idx;
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
        
        // Update positions in HashMap
        let item_i = self.heap[i].clone();
        let item_j = self.heap[j].clone();
        
        self.positions.insert(item_i, i);
        self.positions.insert(item_j, j);
    }
}

// Tests in mod wrapper (but NO #[cfg(test)])
mod tests {
    use super::*;

    // =============================================================================
    // insert() Tests
    // =============================================================================

    #[test]
    fn insert_postcondition_item_findable() {
        let mut pq = DHeap::new(4);
        let item = Item::new(50, 50);
        pq.insert(item.clone());
        assert!(pq.contains(&Item::new(50, 999)));
    }

    #[test]
    fn insert_invariant_heap_property() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(30, 30));
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(50, 50));
        pq.insert(Item::new(20, 20));
        pq.insert(Item::new(40, 40));
        assert_eq!(pq.front().unwrap().cost, 10);
    }

    #[test]
    fn insert_size_increments() {
        let mut pq = DHeap::new(4);
        for i in 0..5 {
            let size_before = pq.len();
            pq.insert(Item::new(i, i * 10));
            assert_eq!(pq.len(), size_before + 1);
        }
    }

    #[test]
    fn insert_edge_becomes_front_if_highest_priority() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(100, 100));
        pq.insert(Item::new(50, 50));
        pq.insert(Item::new(10, 10));
        assert_eq!(pq.front().unwrap().cost, 10);
        pq.insert(Item::new(1, 1));
        assert_eq!(pq.front().unwrap().cost, 1);
    }

    // =============================================================================
    // pop() Tests
    // =============================================================================

    #[test]
    fn pop_postcondition_returns_minimum() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(30, 30));
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(20, 20));
        let popped = pq.pop().unwrap();
        assert_eq!(popped.cost, 10);
        assert!(!pq.contains(&Item::new(10, 0)));
    }

    #[test]
    fn pop_invariant_maintains_heap_property() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(50, 50));
        pq.insert(Item::new(20, 20));
        pq.insert(Item::new(80, 80));
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(60, 60));
        pq.insert(Item::new(30, 30));
        pq.insert(Item::new(70, 70));
        pq.insert(Item::new(40, 40));

        let expected = [10, 20, 30, 40];
        for exp in expected {
            assert_eq!(pq.front().unwrap().cost, exp);
            pq.pop();
        }
    }

    #[test]
    fn pop_size_decrements() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(20, 20));
        pq.insert(Item::new(30, 30));
        for _ in 0..3 {
            let size_before = pq.len();
            pq.pop();
            assert_eq!(pq.len(), size_before - 1);
        }
    }

    #[test]
    fn pop_edge_empty_returns_none() {
        let mut pq: DHeap = DHeap::new(4);
        assert!(pq.pop().is_none());
    }

    // =============================================================================
    // front() Tests
    // =============================================================================

    #[test]
    fn front_postcondition_returns_minimum() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(30, 30));
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(20, 20));
        assert_eq!(pq.front().unwrap().cost, 10);
    }

    #[test]
    fn front_invariant_no_modification() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(30, 30));
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(20, 20));
        let first = pq.front().unwrap().cost;
        let second = pq.front().unwrap().cost;
        let third = pq.front().unwrap().cost;
        assert_eq!(first, second);
        assert_eq!(second, third);
    }

    #[test]
    fn front_size_unchanged() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(20, 20));
        pq.insert(Item::new(30, 30));
        let size_before = pq.len();
        for _ in 0..5 {
            let _ = pq.front();
        }
        assert_eq!(pq.len(), size_before);
    }

    #[test]
    fn front_edge_empty_returns_none() {
        let pq: DHeap = DHeap::new(4);
        assert!(pq.front().is_none());
    }

    // =============================================================================
    // increase_priority() Tests
    // =============================================================================

    #[test]
    fn increase_priority_postcondition_priority_changed() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(50, 50));
        pq.insert(Item::new(30, 30));
        assert_eq!(pq.front().unwrap().cost, 30);
        pq.increase_priority(Item::new(50, 10));
        assert_eq!(pq.front().unwrap().cost, 10);
    }

    #[test]
    fn increase_priority_invariant_heap_property() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(80, 80));
        pq.insert(Item::new(60, 60));
        pq.insert(Item::new(40, 40));
        pq.insert(Item::new(20, 20));
        pq.insert(Item::new(100, 100));
        pq.insert(Item::new(50, 50));
        assert_eq!(pq.front().unwrap().cost, 20);
        pq.increase_priority(Item::new(80, 5));
        assert_eq!(pq.front().unwrap().cost, 5);
    }

    #[test]
    fn increase_priority_position_item_moves_up() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(50, 50));
        pq.insert(Item::new(100, 100));
        assert_ne!(pq.front().unwrap().number, 100);
        pq.increase_priority(Item::new(100, 1));
        assert_eq!(pq.front().unwrap().number, 100);
    }

    #[test]
    fn increase_priority_size_unchanged() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(50, 50));
        pq.insert(Item::new(30, 30));
        pq.insert(Item::new(70, 70));
        let size_before = pq.len();
        pq.increase_priority(Item::new(70, 10));
        assert_eq!(pq.len(), size_before);
    }

    // =============================================================================
    // decrease_priority() Tests
    // =============================================================================

    #[test]
    fn decrease_priority_postcondition_priority_changed() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(30, 30));
        assert_eq!(pq.front().unwrap().number, 10);
        pq.decrease_priority(Item::new(10, 50));
        assert_eq!(pq.front().unwrap().number, 30);
    }

    #[test]
    fn decrease_priority_invariant_heap_property() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(30, 30));
        pq.insert(Item::new(50, 50));
        pq.insert(Item::new(70, 70));
        pq.insert(Item::new(20, 20));
        pq.insert(Item::new(40, 40));
        assert_eq!(pq.front().unwrap().number, 10);
        pq.decrease_priority(Item::new(10, 100));
        assert_eq!(pq.front().unwrap().cost, 20);
    }

    #[test]
    fn decrease_priority_position_item_moves_down() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(50, 50));
        pq.insert(Item::new(60, 60));
        pq.insert(Item::new(70, 70));
        assert_eq!(pq.front().unwrap().number, 10);
        pq.decrease_priority(Item::new(10, 100));
        assert_eq!(pq.front().unwrap().number, 50);
    }

    #[test]
    fn decrease_priority_size_unchanged() {
        let mut pq = DHeap::new(4);
        pq.insert(Item::new(10, 10));
        pq.insert(Item::new(30, 30));
        pq.insert(Item::new(50, 50));
        let size_before = pq.len();
        pq.decrease_priority(Item::new(10, 100));
        assert_eq!(pq.len(), size_before);
    }
}