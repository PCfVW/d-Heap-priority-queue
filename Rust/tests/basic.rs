use priority_queue::{PriorityQueue, MinBy, MaxBy};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
struct Item { id: u32, cost: u32 }

impl PartialEq for Item { fn eq(&self, other: &Self) -> bool { self.id == other.id } }
impl Eq for Item {}
impl Hash for Item { fn hash<H: Hasher>(&self, state: &mut H) { self.id.hash(state) } }

#[test]
fn min_heap_ordering() {
    let mut pq: PriorityQueue<Item, MinBy<_>> = PriorityQueue::new(3, MinBy(|x: &Item| x.cost));
    for n in [20, 5, 22, 16, 18, 17, 12, 9] {
        pq.insert(Item { id: n, cost: n });
    }
    let mut last = 0;
    let mut first = true;
    while !pq.is_empty() {
        let top = pq.front().clone();
        if !first { assert!(top.cost >= last); } else { first = false; }
        last = top.cost;
        pq.pop();
    }
}

#[test]
fn max_heap_ordering() {
    let mut pq: PriorityQueue<Item, MaxBy<_>> = PriorityQueue::new(4, MaxBy(|x: &Item| x.cost));
    for n in [20, 5, 22, 16, 18, 17, 12, 9] {
        pq.insert(Item { id: n, cost: n });
    }
    let mut last = 0;
    let mut first = true;
    while !pq.is_empty() {
        let top = pq.front().clone();
        if !first { assert!(top.cost <= last); } else { first = false; }
        last = top.cost;
        pq.pop();
    }
}

#[test]
fn increase_priority_moves_up() {
    let mut pq: PriorityQueue<Item, MinBy<_>> = PriorityQueue::new(3, MinBy(|x: &Item| x.cost));
    pq.insert(Item { id: 1, cost: 10 });
    pq.insert(Item { id: 2, cost: 9 });
    pq.insert(Item { id: 3, cost: 8 });
    // Update id=1 to become the top
    pq.increase_priority(&Item { id: 1, cost: 1 });
    assert_eq!(pq.front().id, 1);
}
