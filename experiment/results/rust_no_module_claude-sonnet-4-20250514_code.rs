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

#[derive(Debug)]
pub struct DHeap {
    heap: Vec<Item>,
    positions: HashMap<u32, usize>, // Maps item number to position in heap
    d: usize, // heap arity
}

impl DHeap {
    pub fn new(d: usize) -> Self {
        assert!(d >= 2, "Heap arity must be at least 2");
        Self {
            heap: Vec::new(),
            positions: HashMap::new(),
            d,
        }
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn contains(&self, item: &Item) -> bool {
        self.positions.contains_key(&item.number)
    }

    pub fn front(&self) -> Option<&Item> {
        self.heap.first()
    }

    pub fn insert(&mut self, item: Item) {
        // If item already exists, update its priority
        if let Some(&pos) = self.positions.get(&item.number) {
            let old_cost = self.heap[pos].cost;
            self.heap[pos] = item;
            if self.heap[pos].cost < old_cost {
                self.bubble_up(pos);
            } else if self.heap[pos].cost > old_cost {
                self.bubble_down(pos);
            }
            return;
        }

        // Insert new item
        let pos = self.heap.len();
        self.positions.insert(item.number, pos);
        self.heap.push(item);
        self.bubble_up(pos);
    }

    pub fn pop(&mut self) -> Option<Item> {
        if self.heap.is_empty() {
            return None;
        }

        let root = self.heap[0].clone();
        self.positions.remove(&root.number);

        if self.heap.len() == 1 {
            self.heap.pop();
            return Some(root);
        }

        // Move last element to root and bubble down
        let last = self.heap.pop().unwrap();
        self.heap[0] = last;
        self.positions.insert(self.heap[0].number, 0);
        self.bubble_down(0);

        Some(root)
    }

    pub fn increase_priority(&mut self, item: Item) {
        if let Some(&pos) = self.positions.get(&item.number) {
            self.heap[pos] = item;
            self.bubble_up(pos);
        }
    }

    pub fn decrease_priority(&mut self, item: Item) {
        if let Some(&pos) = self.positions.get(&item.number) {
            self.heap[pos] = item;
            self.bubble_down(pos);
        }
    }

    fn parent(&self, i: usize) -> Option<usize> {
        if i == 0 {
            None
        } else {
            Some((i - 1) / self.d)
        }
    }

    fn first_child(&self, i: usize) -> usize {
        self.d * i + 1
    }

    fn bubble_up(&mut self, mut pos: usize) {
        while let Some(parent_pos) = self.parent(pos) {
            if self.heap[pos].cost >= self.heap[parent_pos].cost {
                break;
            }
            self.swap(pos, parent_pos);
            pos = parent_pos;
        }
    }

    fn bubble_down(&mut self, mut pos: usize) {
        loop {
            let first_child = self.first_child(pos);
            if first_child >= self.heap.len() {
                break; // No children
            }

            // Find the child with minimum cost
            let mut min_child = first_child;
            let last_child = std::cmp::min(first_child + self.d, self.heap.len());
            
            for child in first_child + 1..last_child {
                if self.heap[child].cost < self.heap[min_child].cost {
                    min_child = child;
                }
            }

            // If current node is already smaller than or equal to minimum child, we're done
            if self.heap[pos].cost <= self.heap[min_child].cost {
                break;
            }

            self.swap(pos, min_child);
            pos = min_child;
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.positions.insert(self.heap[i].number, j);
        self.positions.insert(self.heap[j].number, i);
        self.heap.swap(i, j);
    }
}

// =============================================================================
// insert() Tests - TOP LEVEL (no mod wrapper)
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
// pop() Tests - TOP LEVEL (no mod wrapper)
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
// front() Tests - TOP LEVEL (no mod wrapper)
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
// increase_priority() Tests - TOP LEVEL (no mod wrapper)
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
// decrease_priority() Tests - TOP LEVEL (no mod wrapper)
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