//! Comparison-count instrumentation for the priority queue (v2.6.0 Phase 2).
//!
//! Mirrors the contract from `TypeScript/src/instrumentation.ts` (v2.4.0):
//! count *comparisons* (not operations), bucketed by which heap operation
//! triggered them. Per the v2.6.0 ROADMAP, the Rust mechanism is **generic over
//! a `StatsCollector` trait**, with monomorphization providing zero overhead
//! when the default `NoOpStats` is used.
//!
//! # Usage
//!
//! ```rust
//! use d_ary_heap::{MinBy, PriorityQueue, StatsCollector};
//!
//! let mut pq = PriorityQueue::with_stats(2, MinBy(|x: &i32| *x)).unwrap();
//! pq.insert(5);
//! pq.insert(3);
//! let _ = pq.pop();
//! let stats = pq.stats();
//! println!("inserts: {}, pops: {}, total: {}",
//!     stats.insert(), stats.pop(), stats.total());
//! ```
//!
//! # Ownership note
//!
//! In Rust, the heap **owns** the stats by value. With the default
//! `NoOpStats` (a zero-sized type), the `stats` field collapses to zero bytes
//! via Rust's standard ZST layout — no attribute is needed (unlike C++'s
//! `[[no_unique_address]]`). With `ComparisonStats` attached (via the
//! `InstrumentedPriorityQueue` alias), the heap holds a small fixed-size
//! struct of `Cell<u64>` counters. This is deliberately asymmetric with Go,
//! where the user owns the storage and the heap holds a `*Stats` pointer.
//!
//! # Cross-language equivalents
//!
//! - C++: `TOOLS::ComparisonStats` and `TOOLS::OperationType` in `PriorityQueue.h`
//! - Go: `dheap.Stats` and `dheap.OperationType` in `instrumentation.go`
//! - TypeScript: `ComparisonStats` from `instrumentation.ts` (v2.4.0)
//! - Zig: comptime bool parameter (Phase 2, planned)

use std::cell::Cell;

/// Identifies which public heap method triggered a comparison.
///
/// `#[non_exhaustive]` because future operations (e.g., `Heapify`) can be added
/// without breaking exhaustive matchers in downstream code; the only exhaustive
/// matcher in this crate lives inside `count_comparison()` and stays correct
/// because it owns the enum.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OperationType {
    /// No operation in flight; comparisons attributed to `None` are dropped.
    #[default]
    None,
    /// `insert` / `insert_many`.
    Insert,
    /// `pop` (and `pop_many` via per-call delegation).
    Pop,
    /// `decrease_priority` and `decrease_priority_by_index`.
    DecreasePriority,
    /// `increase_priority` and `increase_priority_by_index`.
    IncreasePriority,
    /// `update_priority` and `update_priority_by_index`.
    UpdatePriority,
}

/// Trait the heap drives during operations.
///
/// All methods take `&self` (not `&mut self`) so the heap's `&self` query
/// methods (`best_child_position`, etc.) can update the active operation /
/// counters via interior mutability. `ComparisonStats` uses `Cell<T>` for that;
/// `NoOpStats` has nothing to mutate.
pub trait StatsCollector {
    /// Mark an operation as in-flight; subsequent `count_comparison` calls
    /// attribute to the matching bucket.
    fn start_operation(&self, op: OperationType);

    /// Mark the in-flight operation as complete; subsequent comparisons (if
    /// any) attribute to `OperationType::None` and are dropped.
    fn end_operation(&self);

    /// Increment the bucket of the currently-active operation.
    fn count_comparison(&self);

    /// Sum of all five operation buckets. With `NoOpStats` this returns 0.
    fn total(&self) -> u64;

    /// Zero all counters and reset the active-operation tag.
    fn reset(&self);
}

/// Zero-sized policy: every method is an empty no-op.
///
/// With the default `S = NoOpStats`, monomorphization specializes each call
/// site to a no-op, the `stats` field of the heap is zero bytes (Rust ZST
/// layout), and the bracketing `OpGuard`'s `Drop` body is empty — i.e. zero
/// runtime cost.
#[derive(Default, Debug, Clone, Copy)]
pub struct NoOpStats;

impl StatsCollector for NoOpStats {
    #[inline]
    fn start_operation(&self, _op: OperationType) {}
    #[inline]
    fn end_operation(&self) {}
    #[inline]
    fn count_comparison(&self) {}
    #[inline]
    fn total(&self) -> u64 {
        0
    }
    #[inline]
    fn reset(&self) {}
}

/// Real comparison-count policy. Five buckets indexed by the active
/// `OperationType`, plus the active-operation tag.
///
/// All counters are `Cell<u64>` (interior mutability), allowing the trait
/// methods to take `&self`. Hot-path cost when attached: a single `Cell::get`
/// + branch on the active op, then `Cell::set`.
#[derive(Default, Debug, Clone)]
pub struct ComparisonStats {
    insert: Cell<u64>,
    pop: Cell<u64>,
    decrease_priority: Cell<u64>,
    increase_priority: Cell<u64>,
    update_priority: Cell<u64>,
    current_op: Cell<OperationType>,
}

impl ComparisonStats {
    /// Comparisons triggered by `insert` / `insert_many`.
    #[must_use]
    pub fn insert(&self) -> u64 {
        self.insert.get()
    }
    /// Comparisons triggered by `pop` (and `pop_many` via per-call delegation).
    #[must_use]
    pub fn pop(&self) -> u64 {
        self.pop.get()
    }
    /// Comparisons triggered by `decrease_priority` / `decrease_priority_by_index`.
    #[must_use]
    pub fn decrease_priority(&self) -> u64 {
        self.decrease_priority.get()
    }
    /// Comparisons triggered by `increase_priority` / `increase_priority_by_index`.
    #[must_use]
    pub fn increase_priority(&self) -> u64 {
        self.increase_priority.get()
    }
    /// Comparisons triggered by `update_priority` / `update_priority_by_index`.
    #[must_use]
    pub fn update_priority(&self) -> u64 {
        self.update_priority.get()
    }
}

impl StatsCollector for ComparisonStats {
    fn start_operation(&self, op: OperationType) {
        self.current_op.set(op);
    }

    fn end_operation(&self) {
        self.current_op.set(OperationType::None);
    }

    fn count_comparison(&self) {
        match self.current_op.get() {
            OperationType::Insert => self.insert.set(self.insert.get() + 1),
            OperationType::Pop => self.pop.set(self.pop.get() + 1),
            OperationType::DecreasePriority => {
                self.decrease_priority.set(self.decrease_priority.get() + 1);
            }
            OperationType::IncreasePriority => {
                self.increase_priority.set(self.increase_priority.get() + 1);
            }
            OperationType::UpdatePriority => {
                self.update_priority.set(self.update_priority.get() + 1);
            }
            // INDEX: None means no operation is in flight; comparisons here are
            // unattributed and dropped (matches C++/Go semantics).
            OperationType::None => {}
        }
    }

    fn total(&self) -> u64 {
        self.insert.get()
            + self.pop.get()
            + self.decrease_priority.get()
            + self.increase_priority.get()
            + self.update_priority.get()
    }

    fn reset(&self) {
        self.insert.set(0);
        self.pop.set(0);
        self.decrease_priority.set(0);
        self.increase_priority.set(0);
        self.update_priority.set(0);
        self.current_op.set(OperationType::None);
    }
}
