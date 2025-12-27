use d_ary_heap::{PriorityQueue, MinBy, MaxBy};
use std::hash::{Hash, Hasher};
use std::fmt;

#[derive(Clone, Debug)]
struct Item { id: u32, cost: u32 }

impl PartialEq for Item { fn eq(&self, other: &Self) -> bool { self.id == other.id } }
impl Eq for Item {}
impl Hash for Item { fn hash<H: Hasher>(&self, state: &mut H) { self.id.hash(state) } }
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Item(id: {}, cost: {})", self.id, self.cost)
    }
}

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

#[test]
fn unified_api_methods() {
    let mut pq: PriorityQueue<Item, MinBy<_>> = PriorityQueue::new(3, MinBy(|x: &Item| x.cost));

    // Test len() method (unified API)
    assert_eq!(pq.len(), 0);

    // Test is_empty() method (unified API)
    assert!(pq.is_empty());

    // Test d() method (unified API)
    assert_eq!(pq.d(), 3);

    // Insert some items
    pq.insert(Item { id: 1, cost: 10 });
    pq.insert(Item { id: 2, cost: 5 });
    pq.insert(Item { id: 3, cost: 15 });

    // Test len() after insertions
    assert_eq!(pq.len(), 3);

    // Test is_empty() after insertions
    assert!(!pq.is_empty());

    // Test to_string() method (unified API for cross-language consistency)
    let output = pq.to_string();
    assert!(output.starts_with('{'));
    assert!(output.ends_with('}'));
    assert!(output.contains("Item"));

    // Test Display trait implementation (Rust-idiomatic)
    let display_output = format!("{}", pq);
    assert_eq!(output, display_output);  // Both should produce identical output
}

#[test]
fn position_type_alias() {
    use d_ary_heap::Position;

    let mut pq: PriorityQueue<Item, MinBy<_>> = PriorityQueue::new(2, MinBy(|x: &Item| x.cost));
    pq.insert(Item { id: 1, cost: 10 });

    // Test that Position type alias works
    let pos: Position = 0;
    pq.increase_priority_by_index(pos);

    assert_eq!(pq.len(), 1);
}

#[test]
fn parameter_naming_consistency() {
    let mut pq: PriorityQueue<Item, MinBy<_>> = PriorityQueue::new(2, MinBy(|x: &Item| x.cost));
    pq.insert(Item { id: 1, cost: 10 });

    // Test updated parameter name (updated_item instead of t_with_new_higher_priority)
    let updated_item = Item { id: 1, cost: 5 };
    pq.increase_priority(&updated_item);

    assert_eq!(pq.front().cost, 5);
}
