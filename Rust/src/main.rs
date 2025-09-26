use priority_queue::{PriorityQueue, MinBy, MaxBy, PriorityCompare};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::hash::{Hash, Hasher};

#[derive(Clone, Eq)]
struct Int {
    number: u32,
    cost: u32,
}

impl PartialEq for Int {
    fn eq(&self, other: &Self) -> bool { self.number == other.number }
}

impl Hash for Int {
    fn hash<H: Hasher>(&self, state: &mut H) { self.number.hash(state) }
}

impl Display for Int {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { write!(f, "({}, {})", self.number, self.cost) }
}

fn print_pq<T, C>(pq: &PriorityQueue<T, C>)
where
    T: Display + Eq + Hash + Clone,
    C: PriorityCompare<T>,
{
    println!("{}", pq.to_string());
}

fn main() {
    // Min-heap by cost
    let mut pq_less: PriorityQueue<Int, MinBy<_>> = PriorityQueue::new(3, MinBy(|x: &Int| x.cost));

    let input: Vec<i32> = vec![20, 5, 22, 16, 18, 17, 12, 9, 42, 27, 48, 36, 32, 13, 14, 28, 52, 10, 21, 8, 39, 29, 15, 38, 31, 41];

    for &n in &input {
        pq_less.insert(Int { number: n as u32, cost: n as u32 });
        print_pq(&pq_less);
    }

    // dynamic update
    let i1 = Int { number: 19, cost: 19 };
    pq_less.insert(i1.clone());
    print_pq(&pq_less);

    let front = pq_less.front().clone();
    println!("front: {}", front);

    // Increase priority (lower cost for min-heap)
    let i1_new = Int { number: 19, cost: 6 };
    pq_less.increase_priority(&i1_new);
    print_pq(&pq_less);

    // Verify non-decreasing order on pops
    let mut first = true;
    let mut last_cost = 0u32;
    while !pq_less.is_empty() {
        let top = pq_less.front().clone();
        if !first { assert!(top.cost >= last_cost); } else { first = false; }
        last_cost = top.cost;
        pq_less.pop();
        print_pq(&pq_less);
    }

    // Max-heap by cost
    let mut pq_greater: PriorityQueue<Int, MaxBy<_>> = PriorityQueue::new(3, MaxBy(|x: &Int| x.cost));

    for &n in &input {
        pq_greater.insert(Int { number: n as u32, cost: n as u32 });
        print_pq(&pq_greater);
    }

    let i2 = Int { number: 40, cost: 40 };
    pq_greater.insert(i2.clone());
    print_pq(&pq_greater);

    let i2_new = Int { number: 40, cost: 50 };
    pq_greater.increase_priority(&i2_new);
    print_pq(&pq_greater);

    // Verify non-increasing order on pops
    let mut first_max = true;
    let mut last_cost_max = 0u32;
    while !pq_greater.is_empty() {
        let top = pq_greater.front().clone();
        if !first_max { assert!(top.cost <= last_cost_max); } else { first_max = false; }
        last_cost_max = top.cost;
        pq_greater.pop();
        print_pq(&pq_greater);
    }
}
