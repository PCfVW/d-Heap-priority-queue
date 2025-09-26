use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

/// Unified type alias for cross-language consistency
pub type Position = usize;

/// Trait to abstract the notion of "higher priority" between two items.
/// If `higher_priority(a, b)` returns true, then `a` should be placed before `b` in the queue.
pub trait PriorityCompare<T> {
    fn higher_priority(&self, a: &T, b: &T) -> bool;
}

/// A generic d-ary heap priority queue with O(1) lookup of an item's position.
///
/// - `T`: item type. Must implement `Eq + Hash + Clone` so it can be looked up and stored.
/// - `C`: comparator that defines priority order.
pub struct PriorityQueue<T, C>
where
    T: Eq + Hash + Clone,
{
    container: Vec<T>,
    positions: HashMap<T, Position>,
    comparator: C,
    depth: usize,
}

impl<T, C> PriorityQueue<T, C>
where
    T: Eq + Hash + Clone,
    C: PriorityCompare<T>,
{
    /// Create a new empty priority queue with arity `d` and comparator `comparator`.
    pub fn new(d: usize, comparator: C) -> Self {
        assert!(d > 0, "heap depth (d) must be > 0");
        Self { container: Vec::new(), positions: HashMap::new(), comparator, depth: d }
    }

    /// Create a new priority queue with arity `d`, comparator `comparator`, inserting the first item `t`.
    pub fn with_first(d: usize, comparator: C, t: T) -> Self {
        assert!(d > 0, "heap depth (d) must be > 0");
        let mut container = Vec::with_capacity(1);
        container.push(t.clone());
        let mut positions = HashMap::with_capacity(1);
        positions.insert(t, 0);
        Self { container, positions, comparator, depth: d }
    }

    #[inline]
    pub fn d(&self) -> usize { self.depth }

    #[inline]
    pub fn len(&self) -> usize { self.container.len() }

    #[inline]
    pub fn is_empty(&self) -> bool { self.container.is_empty() }

    /// Clear the queue, optionally resetting the arity `d`.
    pub fn clear(&mut self, d: Option<usize>) {
        self.container.clear();
        self.positions.clear();
        if let Some(new_d) = d {
            assert!(new_d > 0, "heap depth (d) must be > 0");
            self.depth = new_d;
        }
    }

    /// Return a reference to the highest-priority item. Panics if empty.
    pub fn front(&self) -> &T {
        self.container.first().expect("front() called on empty priority queue")
    }

    /// Safe alternative to `front()`.
    pub fn peek(&self) -> Option<&T> { self.container.first() }

    /// Insert item `t` into the queue according to its priority.
    pub fn insert(&mut self, t: T) {
        self.container.push(t.clone());
        let i = self.container.len() - 1;
        self.positions.insert(t, i);
        self.move_up(i);
    }

    /// Increase the priority of the item at index `i` (move it up if needed).
    pub fn increase_priority_by_index(&mut self, i: usize) {
        assert!(i < self.container.len());
        self.move_up(i);
    }

    /// Increase the priority of item `t` currently in the queue. The identity of `t` is
    /// determined by its Eq/Hash implementation.
    pub fn increase_priority(&mut self, updated_item: &T) {
        let &i = self.positions
            .get(updated_item)
            .expect("item must exist in the queue to increase priority");

        // Update positions: remove old key (equal identity) and insert the new (updated) item
        // Then update the container slot.
        // Since T: Eq+Hash, using the provided identity.
        let old_key = self.container[i].clone();
        self.positions.remove(&old_key);
        self.positions.insert(updated_item.clone(), i);
        self.container[i] = updated_item.clone();

        // Move up after priority increase
        self.move_up(i);
    }

    /// Decrease the priority value of item `t` currently in the queue. The identity of `t` is
    /// determined by its Eq/Hash implementation.
    pub fn decrease_priority(&mut self, updated_item: &T) {
        let &i = self.positions
            .get(updated_item)
            .expect("item must exist in the queue to decrease priority");

        // Update positions: remove old key (equal identity) and insert the new (updated) item
        // Then update the container slot.
        // Since T: Eq+Hash, using the provided identity.
        let old_key = self.container[i].clone();
        self.positions.remove(&old_key);
        self.positions.insert(updated_item.clone(), i);
        self.container[i] = updated_item.clone();

        // After priority update, the item may need to move up or down to maintain heap property
        // We need to check both directions since we don't know if priority actually decreased
        self.move_up(i);
        self.move_down(i);
    }

    /// Remove the highest-priority item from the queue. No-op if empty.
    pub fn pop(&mut self) {
        if self.container.is_empty() { return; }
        let last = self.container.len() - 1;
        self.swap(0, last);
        let removed = self.container.pop().unwrap();
        self.positions.remove(&removed);
        if !self.container.is_empty() {
            self.move_down(0);
        }
    }

    /// Produce a string rendering of the queue contents in array layout.
    /// Unified method name for cross-language consistency.
    pub fn to_string(&self) -> String
    where
        T: Display,
    {
        let mut s = String::from("{");
        for (idx, item) in self.container.iter().enumerate() {
            if idx > 0 { s.push_str(", "); }
            s.push_str(&format!("{}", item));
        }
        s.push('}');
        s
    }

    #[inline]
    fn parent(&self, i: usize) -> usize {
        assert!(i > 0 && self.depth > 0);
        (i - 1) / self.depth
    }

    fn best_child_position(&self, i: usize) -> usize {
        let n = self.container.len();
        let left = i * self.depth + 1;
        if left >= n { return left; }
        let right = ((i + 1) * self.depth).min(n - 1);
        let mut best = left;
        for p in (left + 1)..=right {
            if self.comparator.higher_priority(&self.container[p], &self.container[best]) {
                best = p;
            }
        }
        best
    }

    fn swap(&mut self, i: usize, j: usize) {
        if i == j { return; }
        self.container.swap(i, j);
        let ti = self.container[i].clone();
        let tj = self.container[j].clone();
        self.positions.insert(ti, i);
        self.positions.insert(tj, j);
    }

    fn move_up(&mut self, mut i: usize) {
        while i > 0 {
            let p = self.parent(i);
            if self.comparator.higher_priority(&self.container[i], &self.container[p]) {
                self.swap(i, p);
                i = p;
            } else {
                break;
            }
        }
    }

    fn move_down(&mut self, mut i: usize) {
        let n = self.container.len();
        loop {
            let first_child = i * self.depth + 1;
            if first_child >= n { break; }
            let best = self.best_child_position(i);
            if self.comparator.higher_priority(&self.container[best], &self.container[i]) {
                self.swap(i, best);
                i = best;
            } else {
                break;
            }
        }
    }
}

/// Convenience comparator implementing min-heap behavior using a key extractor.
pub struct MinBy<F>(pub F);
impl<T, F, K> PriorityCompare<T> for MinBy<F>
where
    F: Fn(&T) -> K,
    K: Ord,
{
    #[inline]
    fn higher_priority(&self, a: &T, b: &T) -> bool { (self.0)(a) < (self.0)(b) }
}

/// Convenience comparator implementing max-heap behavior using a key extractor.
pub struct MaxBy<F>(pub F);
impl<T, F, K> PriorityCompare<T> for MaxBy<F>
where
    F: Fn(&T) -> K,
    K: Ord,
{
    #[inline]
    fn higher_priority(&self, a: &T, b: &T) -> bool { (self.0)(a) > (self.0)(b) }
}
