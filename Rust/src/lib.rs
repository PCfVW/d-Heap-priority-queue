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

/// Error types for d-ary heap operations.
///
/// **Cross-language equivalents**:
/// - Go: `ErrEmptyQueue`, `ErrItemNotFound`
/// - Zig: `error.DepthMustBePositive`, `error.ItemNotFound`, `error.IndexOutOfBounds`
/// - TypeScript: Throws Error with messages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Arity (d) must be >= 1.
    InvalidArity,
    /// Item not found in the priority queue.
    ItemNotFound,
    /// Index is out of bounds.
    IndexOutOfBounds,
    /// Operation requires a non-empty queue.
    EmptyQueue,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::InvalidArity => write!(f, "Heap arity (d) must be >= 1"),
            Error::ItemNotFound => write!(f, "Item not found"),
            Error::IndexOutOfBounds => write!(f, "Index out of bounds"),
            Error::EmptyQueue => write!(f, "Operation called on empty priority queue"),
        }
    }
}

impl std::error::Error for Error {}

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
/// let mut heap = PriorityQueue::new(3, MinBy(|x: &i32| *x)).unwrap();
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
    /// # Errors
    ///
    /// Returns `Error::InvalidArity` if `d == 0`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy, MaxBy, Error};
    ///
    /// // Binary heap (d=2) with min-heap ordering
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();
    ///
    /// // Quaternary heap (d=4) with max-heap ordering
    /// let mut heap = PriorityQueue::new(4, MaxBy(|x: &i32| *x)).unwrap();
    ///
    /// // Invalid arity returns error
    /// assert!(PriorityQueue::new(0, MinBy(|x: &i32| *x)).is_err());
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `PriorityQueue<T>(d)`
    /// - Zig: `DHeap.init(d, comparator, allocator)` (returns `!T`)
    /// - TypeScript: `new PriorityQueue({d, comparator, keyExtractor})` (throws)
    /// - Go: `New(d, comparator)` (returns `*T, error`)
    pub fn new(d: usize, comparator: C) -> Result<Self, Error> {
        if d == 0 {
            return Err(Error::InvalidArity);
        }
        Ok(Self { container: Vec::new(), positions: HashMap::new(), comparator, depth: d })
    }

    /// Creates a new d-ary heap with specified arity, inserting the first item.
    ///
    /// # Arguments
    ///
    /// * `d` - Arity (number of children per node). Must be ≥ 1.
    /// * `comparator` - Defines priority order
    /// * `t` - First item to insert
    ///
    /// # Errors
    ///
    /// Returns `Error::InvalidArity` if `d == 0`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy};
    ///
    /// let mut heap = PriorityQueue::with_first(3, MinBy(|x: &i32| *x), 42).unwrap();
    /// assert_eq!(heap.front(), &42);
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `PriorityQueue(d, t)`
    /// - Zig: Not directly available (use `init` + `insert`)
    /// - TypeScript: `PriorityQueue.withFirst(options, item)`
    /// - Go: `WithFirst(d, comparator, item)` (returns `*T, error`)
    pub fn with_first(d: usize, comparator: C, t: T) -> Result<Self, Error> {
        if d == 0 {
            return Err(Error::InvalidArity);
        }
        let mut container = Vec::with_capacity(1);
        container.push(t.clone());
        let mut positions = HashMap::with_capacity(1);
        positions.insert(t, 0);
        Ok(Self { container, positions, comparator, depth: d })
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
    /// let heap = PriorityQueue::new(4, MinBy(|x: &i32| *x)).unwrap();
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
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();
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
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();
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
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();
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

    /// Returns the position (index) of an item in the heap, or `None` if not found.
    ///
    /// **Time Complexity**: O(1)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();
    /// heap.insert(5);
    /// heap.insert(3);
    ///
    /// // Root item (highest priority) is at position 0
    /// assert_eq!(heap.get_position(&3), Some(0));
    /// assert!(heap.get_position(&5).is_some());
    /// assert_eq!(heap.get_position(&99), None);
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `get_position(item)`
    /// - Zig: `getPosition(item)`
    /// - TypeScript: `getPosition(item)`
    /// - Go: `GetPosition(item)`
    #[inline]
    pub fn get_position(&self, item: &T) -> Option<Position> {
        self.positions.get(item).copied()
    }

    /// Clears all items from the heap, optionally changing the arity.
    ///
    /// **Time Complexity**: O(1)
    ///
    /// # Errors
    ///
    /// Returns `Error::InvalidArity` if `d` is `Some(0)`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();
    /// heap.insert(5);
    /// heap.insert(3);
    ///
    /// heap.clear(None).unwrap();
    /// assert!(heap.is_empty());
    /// assert_eq!(heap.d(), 2); // Arity preserved
    ///
    /// heap.clear(Some(4)).unwrap(); // Change arity to 4
    /// assert_eq!(heap.d(), 4);
    ///
    /// // Invalid arity returns error
    /// assert!(heap.clear(Some(0)).is_err());
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `clear(opt_d)`
    /// - Zig: `clear(new_depth?)` (returns `!void`)
    /// - TypeScript: `clear(newD?)` (throws on invalid)
    /// - Go: `Clear(d)` (returns `error`)
    pub fn clear(&mut self, d: Option<usize>) -> Result<(), Error> {
        if let Some(new_d) = d {
            if new_d == 0 {
                return Err(Error::InvalidArity);
            }
            self.depth = new_d;
        }
        self.container.clear();
        self.positions.clear();
        Ok(())
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
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();
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
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();
    /// assert_eq!(heap.peek(), None);
    ///
    /// heap.insert(5);
    /// assert_eq!(heap.peek(), Some(&5));
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `peek()`
    /// - Zig: `front()` / `peek()`
    /// - TypeScript: `peek()`
    /// - Go: `Peek()`
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
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();
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
    /// # Errors
    ///
    /// Returns `Error::IndexOutOfBounds` if index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy, Error};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();
    /// heap.insert(10);
    /// heap.insert(5);
    ///
    /// // Increase priority of item at index 1
    /// heap.increase_priority_by_index(1).unwrap();
    ///
    /// // Error on out of bounds
    /// assert_eq!(heap.increase_priority_by_index(99), Err(Error::IndexOutOfBounds));
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `increase_priority(position)`
    /// - Zig: `increasePriorityByIndex(index)` (returns `!void`)
    /// - TypeScript: `increasePriorityByIndex(index)` (throws)
    /// - Go: `IncreasePriorityByIndex(index)` (returns `error`)
    pub fn increase_priority_by_index(&mut self, i: usize) -> Result<(), Error> {
        if i >= self.container.len() {
            return Err(Error::IndexOutOfBounds);
        }
        self.move_up(i);
        Ok(())
    }

    /// Decreases priority of item at specified index (moves down if needed).
    ///
    /// **Time Complexity**: O(d · log_d n)
    ///
    /// # Errors
    ///
    /// Returns `Error::IndexOutOfBounds` if index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy, Error};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();
    /// heap.insert(10);
    /// heap.insert(5);
    ///
    /// // Decrease priority of item at index 0 (root)
    /// heap.decrease_priority_by_index(0).unwrap();
    ///
    /// // Error on out of bounds
    /// assert_eq!(heap.decrease_priority_by_index(99), Err(Error::IndexOutOfBounds));
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `decrease_priority_by_index(index)`
    /// - Zig: `decreasePriorityByIndex(index)` (returns `!void`)
    /// - TypeScript: `decreasePriorityByIndex(index)` (throws)
    /// - Go: `DecreasePriorityByIndex(index)` (returns `error`)
    pub fn decrease_priority_by_index(&mut self, i: usize) -> Result<(), Error> {
        if i >= self.container.len() {
            return Err(Error::IndexOutOfBounds);
        }
        self.move_down(i);
        Ok(())
    }

    /// Updates priority of item at specified index (moves in correct direction).
    ///
    /// Use this when you don't know whether the item's priority increased or decreased.
    /// It will check both directions to maintain heap property.
    ///
    /// **Time Complexity**: O((d+1) · log_d n) worst case
    ///
    /// # Errors
    ///
    /// Returns `Error::IndexOutOfBounds` if index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy, Error};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();
    /// heap.insert(10);
    /// heap.insert(5);
    ///
    /// // Update priority at index - direction is determined automatically
    /// heap.update_priority_by_index(0).unwrap();
    ///
    /// // Error on out of bounds
    /// assert_eq!(heap.update_priority_by_index(99), Err(Error::IndexOutOfBounds));
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `update_priority_by_index(index)`
    /// - Zig: Not available
    /// - TypeScript: Not available
    /// - Go: Not available
    pub fn update_priority_by_index(&mut self, i: usize) -> Result<(), Error> {
        if i >= self.container.len() {
            return Err(Error::IndexOutOfBounds);
        }
        self.move_up(i);
        self.move_down(i);
        Ok(())
    }

    /// Increases priority of existing item (moves toward root if needed).
    ///
    /// **Time Complexity**: O(log_d n)
    ///
    /// # Errors
    ///
    /// Returns `Error::ItemNotFound` if item is not in the heap.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy, Error};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();
    /// heap.insert(10);
    /// heap.insert(5);
    ///
    /// // The heap maintains proper ordering
    /// assert_eq!(heap.front(), &5);
    /// assert!(heap.contains(&10));
    ///
    /// // Error on non-existent item
    /// assert_eq!(heap.increase_priority(&99), Err(Error::ItemNotFound));
    /// ```
    ///
    /// **Note**: For min-heap, "increase priority" means decreasing the priority value.
    /// For max-heap, "increase priority" means increasing the priority value.
    ///
    /// **Cross-language equivalents**:
    /// - C++: `increase_priority(item)`
    /// - Zig: `increasePriority(item)` (returns `!void`)
    /// - TypeScript: `increasePriority(item)` (throws)
    /// - Go: `IncreasePriority(item)` (returns `error`)
    pub fn increase_priority(&mut self, updated_item: &T) -> Result<(), Error> {
        let &i = self.positions
            .get(updated_item)
            .ok_or(Error::ItemNotFound)?;

        // Update positions: remove old key and insert the new (updated) item.
        // Since Hash/Eq are based on identity (not priority), updated_item can be used
        // directly to remove the old entry — no need to clone the old item.
        self.positions.remove(updated_item);
        self.positions.insert(updated_item.clone(), i);
        self.container[i] = updated_item.clone();

        // Move up after priority increase
        self.move_up(i);
        Ok(())
    }

    /// Decreases priority of existing item (moves toward leaves if needed).
    ///
    /// **Important**: Only call this when you know the item's priority has decreased
    /// (become less important). For min-heap, this means the value increased.
    /// For max-heap, this means the value decreased.
    /// If unsure of the direction, use `update_priority()` instead.
    ///
    /// **Time Complexity**: O(d · log_d n)
    ///
    /// # Errors
    ///
    /// Returns `Error::ItemNotFound` if item is not in the heap.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy, Error};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();
    /// heap.insert(5);
    /// heap.insert(10);
    ///
    /// // The heap maintains proper ordering
    /// assert_eq!(heap.front(), &5);
    /// assert!(heap.contains(&10));
    ///
    /// // Error on non-existent item
    /// assert_eq!(heap.decrease_priority(&99), Err(Error::ItemNotFound));
    /// ```
    ///
    /// **Note**: For min-heap, "decrease priority" means increasing the priority value.
    /// For max-heap, "decrease priority" means decreasing the priority value.
    ///
    /// **Cross-language equivalents**:
    /// - C++: `decrease_priority(item)`
    /// - Zig: `decreasePriority(item)` (returns `!void`)
    /// - TypeScript: `decreasePriority(item)` (throws)
    /// - Go: `DecreasePriority(item)` (returns `error`)
    pub fn decrease_priority(&mut self, updated_item: &T) -> Result<(), Error> {
        let &i = self.positions
            .get(updated_item)
            .ok_or(Error::ItemNotFound)?;

        // Update positions: remove old key and insert the new (updated) item.
        // Since Hash/Eq are based on identity (not priority), updated_item can be used
        // directly to remove the old entry — no need to clone the old item.
        self.positions.remove(updated_item);
        self.positions.insert(updated_item.clone(), i);
        self.container[i] = updated_item.clone();

        // Move down after priority decrease (item became less important)
        self.move_down(i);
        Ok(())
    }

    /// Updates priority of existing item, moving it in the correct direction.
    ///
    /// Use this when you don't know whether the item's priority increased or decreased.
    /// It will check both directions to maintain heap property.
    ///
    /// **Time Complexity**: O((d+1) · log_d n) worst case
    ///
    /// # Errors
    ///
    /// Returns `Error::ItemNotFound` if item is not in the heap.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy, Error};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();
    /// heap.insert(5);
    /// heap.insert(10);
    ///
    /// // Update priority - direction is determined automatically
    /// heap.update_priority(&3).unwrap_or(()); // Would need matching item by identity
    ///
    /// // Error on non-existent item
    /// assert_eq!(heap.update_priority(&99), Err(Error::ItemNotFound));
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `update_priority(item)` / `try_update_priority(item)`
    /// - Zig: `updatePriority(item)` (returns `!void`)
    /// - TypeScript: `updatePriority(item)` (throws)
    /// - Go: `UpdatePriority(item)` (returns `error`)
    pub fn update_priority(&mut self, updated_item: &T) -> Result<(), Error> {
        let &i = self.positions
            .get(updated_item)
            .ok_or(Error::ItemNotFound)?;

        // Update positions: remove old key and insert the new (updated) item.
        self.positions.remove(updated_item);
        self.positions.insert(updated_item.clone(), i);
        self.container[i] = updated_item.clone();

        // Check both directions since we don't know if priority increased or decreased
        self.move_up(i);
        self.move_down(i);
        Ok(())
    }

    /// Removes and returns the highest-priority item from the heap.
    ///
    /// Returns `None` if the heap is empty.
    ///
    /// **Time Complexity**: O(d · log_d n)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();
    /// heap.insert(5);
    /// heap.insert(3);
    /// heap.insert(7);
    ///
    /// assert_eq!(heap.pop(), Some(3));
    /// assert_eq!(heap.pop(), Some(5));
    /// assert_eq!(heap.pop(), Some(7));
    /// assert_eq!(heap.pop(), None);
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `pop()`
    /// - Zig: `pop()` (returns `?T`)
    /// - TypeScript: `pop()` (returns `T | undefined`)
    /// - Go: `Pop()` (returns `T, bool`)
    pub fn pop(&mut self) -> Option<T> {
        if self.container.is_empty() { return None; }
        let last = self.container.len() - 1;
        self.swap(0, last);
        let removed = self.container.pop().unwrap();
        self.positions.remove(&removed);
        if !self.container.is_empty() {
            self.move_down(0);
        }
        Some(removed)
    }

    /// Returns a copy of the heap contents as a Vec.
    ///
    /// The root element (highest priority) is at index 0. The internal heap
    /// structure is preserved—this is NOT a sorted array.
    ///
    /// **Time Complexity**: O(n)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();
    /// heap.insert(5);
    /// heap.insert(3);
    /// heap.insert(7);
    ///
    /// let arr = heap.to_array();
    /// assert_eq!(arr.len(), 3);
    /// assert_eq!(arr[0], 3); // Root is highest priority (min value)
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `to_array()`
    /// - Zig: `toArray()`
    /// - TypeScript: `toArray()`
    /// - Go: `ToArray()`
    pub fn to_array(&self) -> Vec<T> {
        self.container.clone()
    }

    /// Inserts multiple items into the heap using Floyd's heapify algorithm.
    ///
    /// This is more efficient than inserting items one at a time when adding
    /// many items at once: O(n) vs O(n log n).
    ///
    /// **Time Complexity**: O(n) where n is the number of items being inserted
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();
    /// heap.insert_many(vec![5, 3, 7, 1, 9]);
    ///
    /// assert_eq!(heap.len(), 5);
    /// assert_eq!(heap.front(), &1);
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `insert_many(items)`
    /// - Zig: `insertMany(items)`
    /// - TypeScript: `insertMany(items)`
    /// - Go: `InsertMany(items)`
    pub fn insert_many(&mut self, items: impl IntoIterator<Item = T>) {
        let items: Vec<T> = items.into_iter().collect();
        if items.is_empty() {
            return;
        }

        // Add all items to container and positions
        let start_idx = self.container.len();
        for (i, item) in items.into_iter().enumerate() {
            self.positions.insert(item.clone(), start_idx + i);
            self.container.push(item);
        }

        // Floyd's heapify: sift down from the last non-leaf to the root
        // This achieves O(n) instead of O(n log n) for individual inserts
        if self.container.len() > 1 {
            let last_non_leaf = (self.container.len() - 2) / self.depth;
            for i in (0..=last_non_leaf).rev() {
                self.move_down(i);
            }
        }
    }

    /// Removes and returns multiple highest-priority items from the heap.
    ///
    /// Returns up to `count` items in priority order (highest priority first).
    /// If the heap has fewer items than requested, returns all available items.
    ///
    /// **Time Complexity**: O(count · d · log_d n)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use d_ary_heap::{PriorityQueue, MinBy};
    ///
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();
    /// heap.insert_many(vec![5, 3, 7, 1, 9]);
    ///
    /// let items = heap.pop_many(3);
    /// assert_eq!(items, vec![1, 3, 5]);
    /// assert_eq!(heap.len(), 2);
    ///
    /// // Requesting more than available returns all remaining
    /// let remaining = heap.pop_many(10);
    /// assert_eq!(remaining, vec![7, 9]);
    /// assert!(heap.is_empty());
    /// ```
    ///
    /// **Cross-language equivalents**:
    /// - C++: `pop_many(count)`
    /// - Zig: `popMany(count)`
    /// - TypeScript: `popMany(count)`
    /// - Go: `PopMany(count)`
    pub fn pop_many(&mut self, count: usize) -> Vec<T> {
        let actual_count = count.min(self.container.len());
        let mut result = Vec::with_capacity(actual_count);
        for _ in 0..actual_count {
            if let Some(item) = self.pop() {
                result.push(item);
            }
        }
        result
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
/// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();
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
    /// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();
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
/// let mut heap = PriorityQueue::new(2, MinBy(|x: &i32| *x)).unwrap();
/// heap.insert(5);
/// heap.insert(3);
/// assert_eq!(heap.front(), &3);
///
/// // Min-heap by struct field
/// #[derive(PartialEq, Eq, Hash, Clone)]
/// struct Task { priority: i32 }
/// let mut heap = PriorityQueue::new(3, MinBy(|t: &Task| t.priority)).unwrap();
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
/// let mut heap = PriorityQueue::new(2, MaxBy(|x: &i32| *x)).unwrap();
/// heap.insert(5);
/// heap.insert(3);
/// assert_eq!(heap.front(), &5);
///
/// // Max-heap by struct field
/// #[derive(PartialEq, Eq, Hash, Clone)]
/// struct Task { priority: i32 }
/// let mut heap = PriorityQueue::new(3, MaxBy(|t: &Task| t.priority)).unwrap();
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
