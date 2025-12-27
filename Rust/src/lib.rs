//! # d-ary Heap Priority Queue
//!
//! Cross-language implementation of d-ary heap (d-heap) priority queue with O(1) item lookup.
//! This implementation provides API parity with C++, Zig, and TypeScript versions.
//!
//! ## Features
//!
//! - **Configurable arity (d)**: Number of children per node (d ≥ 1)
//! - **Min/Max flexibility**: Supports both min-heap and max-heap behavior via comparators
//! - **O(1) item lookup**: Internal hash map enables efficient priority updates
//! - **Efficient operations**: O(log_d n) insert, O(d·log_d n) pop
//! - **Cross-language API**: Unified method names and behavior across implementations
//!
//! ## Cross-Language Consistency
//!
//! This Rust implementation maintains API compatibility with:
//! - **C++**: `TOOLS::PriorityQueue<T>` in `Cpp/PriorityQueue.h`
//! - **Zig**: `DHeap(T)` in `zig/src/d_heap.zig`
//! - **TypeScript**: `PriorityQueue<T>` in `TypeScript/src/PriorityQueue.ts`
//!
//! All implementations share identical time complexities and method semantics.

use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::hash::Hash;

/// Type alias for position indices, providing cross-language consistency.
///
/// **Cross-language equivalents**:
/// - C++: `TOOLS::PriorityQueue<T>::Position`
/// - Zig: `DHeap.Position`
/// - TypeScript: `Position` type alias
pub type Position = usize;

/// Trait defining priority comparison for heap ordering.
///
/// Implement this trait to define custom priority ordering.
/// Returns `true` if `a` has higher priority than `b`.
///
/// **Cross-language equivalents**:
/// - C++: `std::less<T>` / `std::greater<T>`
/// - Zig: `Comparator(T)`
/// - TypeScript: `Comparator<T>` function
///
/// # Examples
///
/// ```rust
/// use d_ary_heap::PriorityCompare;
///
/// struct MyComparator;
/// impl PriorityCompare<i32> for MyComparator {
///     fn higher_priority(&self, a: &i32, b: &i32) -> bool {
///         a < b // Min-heap: lower values have higher priority
///     }
/// }
/// ```
pub trait PriorityCompare<T> {
    /// Returns `true` if `a` should come before `b` in the heap (has higher priority).
    fn higher_priority(&self, a: &T, b: &T) -> bool;
}

/// d-ary heap priority queue with O(1) item lookup.
///
/// **Type Parameters**:
/// - `T`: Item type (must implement `Eq + Hash + Clone`)
/// - `C`: Comparator implementing `PriorityCompare<T>`
///
/// **Cross-language equivalents**:
/// - C++: `TOOLS::PriorityQueue<T, THash, TComparisonPredicate, TEqual>`
/// - Zig: `DHeap(T, HashContext(T), Comparator(T))`
/// - TypeScript: `PriorityQueue<T, K>`
///
/// # Examples
///
/// ```rust
/// use d_ary_heap::{PriorityQueue, MinBy};
///
/// // Create min-heap with arity 3
/// let mut heap = PriorityQueue::new(3, MinBy(|x: &i32| *x));
/// heap.insert(5);
/// heap.insert(3);
/// heap.insert(7);
///
/// assert_eq!(heap.front(), &3);
/// assert_eq!(heap.len(), 3);
/// ```
///
/// **Time Complexities** (n = number of items, d = arity):
/// - `new()`: O(1)
/// - `insert()`: O(log_d n)
/// - `front()`/`peek()`: O(1)
/// - `pop()`: O(d · log_d n)
/// - `increase_priority()`: O(log_d n)
/// - `decrease_priority()`: O(d · log_d n)
/// - `contains()`: O(1)
/// - `len()`/`is_empty()`/`d()`: O(1)
#[derive(Debug)]
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
    /// Creates a new empty d-ary heap with specified arity and comparator.
    ///
    /// # Arguments
    ///
    /// * `d` - Arity (number of children per node). Must be ≥ 1.
    /// * `comparator` - Defines priority order (min-heap or max-heap)
    ///
    /// # Panics
    ///
    /// Panics if `d == 0`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy, MaxBy};
    ///
    /// // Binary heap (d=2) with min-heap ordering
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x));
    /// 
    /// // Quaternary heap (d=4) with max-heap ordering
    /// let mut heap = PriorityQueue::new(4, MaxBy(|x: &i32| *x));
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `PriorityQueue<T>(d)`
    /// - Zig: `DHeap.init(d, comparator, allocator)`
    /// - TypeScript: `new PriorityQueue({d, comparator, keyExtractor})`
    pub fn new(d: usize, comparator: C) -> Self {
        assert!(d > 0, "arity (d) must be > 0");
        Self { container: Vec::new(), positions: HashMap::new(), comparator, depth: d }
    }

    /// Creates a new d-ary heap with specified arity, inserting the first item.
    ///
    /// # Arguments
    ///
    /// * `d` - Arity (number of children per node). Must be ≥ 1.
    /// * `comparator` - Defines priority order
    /// * `t` - First item to insert
    ///
    /// # Panics
    ///
    /// Panics if `d == 0`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy};
    ///
    /// let mut heap = PriorityQueue::with_first(3, MinBy(|x: &i32| *x), 42);
    /// assert_eq!(heap.front(), &42);
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `PriorityQueue(d, t)`
    /// - Zig: Not directly available (use `init` + `insert`)
    /// - TypeScript: `PriorityQueue.withFirst(options, item)`
    pub fn with_first(d: usize, comparator: C, t: T) -> Self {
        assert!(d > 0, "arity (d) must be > 0");
        let mut container = Vec::with_capacity(1);
        container.push(t.clone());
        let mut positions = HashMap::with_capacity(1);
        positions.insert(t, 0);
        Self { container, positions, comparator, depth: d }
    }

    /// Returns the arity (number of children per node) of this heap.
    ///
    /// **Time Complexity**: O(1)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy};
    ///
    /// let heap = PriorityQueue::new(4, MinBy(|x: &i32| *x));
    /// assert_eq!(heap.d(), 4);
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `d()`
    /// - Zig: `d()`
    /// - TypeScript: `d()`
    #[inline]
    pub fn d(&self) -> usize { self.depth }

    /// Returns the number of items in the heap.
    ///
    /// **Time Complexity**: O(1)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x));
    /// assert_eq!(heap.len(), 0);
    /// heap.insert(5);
    /// assert_eq!(heap.len(), 1);
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `len()`
    /// - Zig: `len()`
    /// - TypeScript: `len()`
    #[inline]
    pub fn len(&self) -> usize { self.container.len() }

    /// Returns `true` if the heap is empty.
    ///
    /// **Time Complexity**: O(1)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x));
    /// assert!(heap.is_empty());
    /// heap.insert(5);
    /// assert!(!heap.is_empty());
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `is_empty()`
    /// - Zig: `isEmpty()`
    /// - TypeScript: `isEmpty()`
    #[inline]
    pub fn is_empty(&self) -> bool { self.container.is_empty() }

    /// Checks if an item exists in the heap by identity (O(1) lookup).
    ///
    /// **Time Complexity**: O(1)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x));
    /// heap.insert(5);
    /// assert!(heap.contains(&5));
    /// assert!(!heap.contains(&10));
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `contains(item)`
    /// - Zig: `contains(item)`
    /// - TypeScript: `contains(item)`
    #[inline]
    pub fn contains(&self, item: &T) -> bool { self.positions.contains_key(item) }

    /// Clears all items from the heap, optionally changing the arity.
    ///
    /// **Time Complexity**: O(1)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x));
    /// heap.insert(5);
    /// heap.insert(3);
    /// 
    /// heap.clear(None);
    /// assert!(heap.is_empty());
    /// assert_eq!(heap.d(), 2); // Arity preserved
    /// 
    /// heap.clear(Some(4)); // Change arity to 4
    /// assert_eq!(heap.d(), 4);
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `clear(opt_d)`
    /// - Zig: `clear(new_depth?)`
    /// - TypeScript: `clear(newD?)`
    pub fn clear(&mut self, d: Option<usize>) {
        self.container.clear();
        self.positions.clear();
        if let Some(new_d) = d {
            assert!(new_d > 0, "arity (d) must be > 0");
            self.depth = new_d;
        }
    }

    /// Returns a reference to the highest-priority item.
    ///
    /// **Time Complexity**: O(1)
    ///
    /// # Panics
    ///
    /// Panics if the heap is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x));
    /// heap.insert(5);
    /// heap.insert(3);
    /// 
    /// assert_eq!(heap.front(), &3);
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `front()` (UB if empty)
    /// - Zig: `front()` (returns `null` if empty)
    /// - TypeScript: `front()` (throws if empty)
    ///
    /// **Safe alternative**: Use `peek()` instead.
    pub fn front(&self) -> &T {
        self.container.first().expect("front() called on empty priority queue")
    }

    /// Safe alternative to `front()` that returns `None` if empty.
    ///
    /// **Time Complexity**: O(1)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x));
    /// assert_eq!(heap.peek(), None);
    /// 
    /// heap.insert(5);
    /// assert_eq!(heap.peek(), Some(&5));
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: Not available (use `!is_empty()` check)
    /// - Zig: `front()` (same as `peek()`)
    /// - TypeScript: `peek()`
    pub fn peek(&self) -> Option<&T> { self.container.first() }

    /// Inserts an item into the heap according to its priority.
    ///
    /// **Time Complexity**: O(log_d n)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x));
    /// heap.insert(5);
    /// heap.insert(3);
    /// heap.insert(7);
    /// 
    /// assert_eq!(heap.front(), &3);
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `insert(item)`
    /// - Zig: `insert(item)`
    /// - TypeScript: `insert(item)`
    pub fn insert(&mut self, t: T) {
        self.container.push(t.clone());
        let i = self.container.len() - 1;
        self.positions.insert(t, i);
        self.move_up(i);
    }

    /// Increases priority of item at specified index (moves up if needed).
    ///
    /// **Time Complexity**: O(log_d n)
    ///
    /// # Panics
    ///
    /// Panics if index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x));
    /// heap.insert(10);
    /// heap.insert(5);
    /// 
    /// // Get position of item 10 (should be at index 1)
    /// let pos = 1;
    /// heap.increase_priority_by_index(pos);
    /// // Item at position 1 moves up if it has higher priority
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `increase_priority(position)`
    /// - Zig: `increasePriorityByIndex(index)`
    /// - TypeScript: `increasePriorityByIndex(index)`
    pub fn increase_priority_by_index(&mut self, i: usize) {
        assert!(i < self.container.len());
        self.move_up(i);
    }

    /// Increases priority of existing item (moves toward root if needed).
    ///
    /// **Time Complexity**: O(log_d n)
    ///
    /// # Panics
    ///
    /// Panics if item is not found in the heap.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x));
    /// heap.insert(10);
    /// heap.insert(5);
    /// 
    /// // The heap maintains proper ordering
    /// assert_eq!(heap.front(), &5);
    /// assert!(heap.contains(&10));
    /// ```
    ///
    /// **Note**: For min-heap, "increase priority" means decreasing the priority value.
    /// For max-heap, "increase priority" means increasing the priority value.
    ///
    /// **Cross-language equivalents**:
    /// - C++: `increase_priority(item)`
    /// - Zig: `increasePriority(item)`
    /// - TypeScript: `increasePriority(item)`
    pub fn increase_priority(&mut self, updated_item: &T) {
        let &i = self.positions
            .get(updated_item)
            .expect("item must exist in the queue to increase priority");

        // Update positions: remove old key and insert the new (updated) item.
        // Since Hash/Eq are based on identity (not priority), updated_item can be used
        // directly to remove the old entry — no need to clone the old item.
        self.positions.remove(updated_item);
        self.positions.insert(updated_item.clone(), i);
        self.container[i] = updated_item.clone();

        // Move up after priority increase
        self.move_up(i);
    }

    /// Decreases priority of existing item (moves toward leaves if needed).
    ///
    /// **Time Complexity**: O(d · log_d n)
    ///
    /// # Panics
    ///
    /// Panics if item is not found in the heap.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x));
    /// heap.insert(5);
    /// heap.insert(10);
    /// 
    /// // The heap maintains proper ordering
    /// assert_eq!(heap.front(), &5);
    /// assert!(heap.contains(&10));
    /// ```
    ///
    /// **Note**: For min-heap, "decrease priority" means increasing the priority value.
    /// For max-heap, "decrease priority" means decreasing the priority value.
    ///
    /// **Cross-language equivalents**:
    /// - C++: `decrease_priority(item)`
    /// - Zig: `decreasePriority(item)`
    /// - TypeScript: `decreasePriority(item)`
    pub fn decrease_priority(&mut self, updated_item: &T) {
        let &i = self.positions
            .get(updated_item)
            .expect("item must exist in the queue to decrease priority");

        // Update positions: remove old key and insert the new (updated) item.
        // Since Hash/Eq are based on identity (not priority), updated_item can be used
        // directly to remove the old entry — no need to clone the old item.
        self.positions.remove(updated_item);
        self.positions.insert(updated_item.clone(), i);
        self.container[i] = updated_item.clone();

        // After priority update, the item may need to move up or down to maintain heap property
        // We need to check both directions since we don't know if priority actually decreased
        self.move_up(i);
        self.move_down(i);
    }

    /// Removes and returns the highest-priority item from the heap.
    ///
    /// **Time Complexity**: O(d · log_d n)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x));
    /// heap.insert(5);
    /// heap.insert(3);
    /// heap.insert(7);
    /// 
    /// let mut items = Vec::new();
    /// while !heap.is_empty() {
    ///     items.push(heap.front().clone());
    ///     heap.pop();
    /// }
    /// assert_eq!(items, vec![3, 5, 7]);
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `pop()`
    /// - Zig: `pop()`
    /// - TypeScript: `pop()`
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

/// Display implementation for PriorityQueue.
///
/// Renders the queue contents in array layout: `{item1, item2, ...}`.
///
/// **Cross-language equivalents**:
/// - C++: `put(std::ostream&)`
/// - Zig: `toString()`
/// - TypeScript: `toString()`
///
/// # Examples
///
/// ```rust
/// use d_ary_heap::{PriorityQueue, MinBy};
///
/// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x));
/// heap.insert(5);
/// heap.insert(3);
/// 
/// // Uses Display trait
/// println!("{}", heap); // Output: {3, 5}
/// ```
impl<T, C> Display for PriorityQueue<T, C>
where
    T: Eq + Hash + Clone + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{{")?;
        for (idx, item) in self.container.iter().enumerate() {
            if idx > 0 { write!(f, ", ")?; }
            write!(f, "{}", item)?;
        }
        write!(f, "}}")
    }
}

/// Additional methods available when T implements Display.
impl<T, C> PriorityQueue<T, C>
where
    T: Eq + Hash + Clone + Display,
{
    /// Produces a string representation of the heap contents.
    ///
    /// **Time Complexity**: O(n)
    ///
    /// This method provides explicit string conversion for API parity with
    /// C++, Zig, and TypeScript implementations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x));
    /// heap.insert(5);
    /// heap.insert(3);
    /// 
    /// assert_eq!(heap.to_string(), "{3, 5}");
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `to_string()`
    /// - Zig: `toString()` / `to_string()`
    /// - TypeScript: `toString()` / `to_string()`
    #[inline]
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}

/// Convenience comparator for min-heap behavior.
///
/// Creates a min-heap where items with smaller key values have higher priority.
///
/// **Cross-language equivalents**:
/// - C++: `std::less<T>`
/// - Zig: `MinBy` comparator
/// - TypeScript: `minBy` helper
///
/// # Examples
///
/// ```rust
/// use d_ary_heap::{PriorityQueue, MinBy};
///
/// // Min-heap by integer value
/// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x));
/// heap.insert(5);
/// heap.insert(3);
/// assert_eq!(heap.front(), &3);
/// 
/// // Min-heap by struct field
/// #[derive(PartialEq, Eq, Hash, Clone)]
/// struct Task { priority: i32 }
/// let mut heap = PriorityQueue::new(3, MinBy(|t: &Task| t.priority));
/// ```
pub struct MinBy<F>(pub F);
impl<T, F, K> PriorityCompare<T> for MinBy<F>
where
    F: Fn(&T) -> K,
    K: Ord,
{
    #[inline]
    fn higher_priority(&self, a: &T, b: &T) -> bool { (self.0)(a) < (self.0)(b) }
}

/// Convenience comparator for max-heap behavior.
///
/// Creates a max-heap where items with larger key values have higher priority.
///
/// **Cross-language equivalents**:
/// - C++: `std::greater<T>`
/// - Zig: `MaxBy` comparator
/// - TypeScript: `maxBy` helper
///
/// # Examples
///
/// ```rust
/// use d_ary_heap::{PriorityQueue, MaxBy};
///
/// // Max-heap by integer value
/// let mut heap = PriorityQueue::new(2, MaxBy(|x: &i32| *x));
/// heap.insert(5);
/// heap.insert(3);
/// assert_eq!(heap.front(), &5);
/// 
/// // Max-heap by struct field
/// #[derive(PartialEq, Eq, Hash, Clone)]
/// struct Task { priority: i32 }
/// let mut heap = PriorityQueue::new(3, MaxBy(|t: &Task| t.priority));
/// ```
pub struct MaxBy<F>(pub F);
impl<T, F, K> PriorityCompare<T> for MaxBy<F>
where
    F: Fn(&T) -> K,
    K: Ord,
{
    #[inline]
    fn higher_priority(&self, a: &T, b: &T) -> bool { (self.0)(a) > (self.0)(b) }
}
