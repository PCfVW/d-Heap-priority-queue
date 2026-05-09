//! Integration tests for v2.6.0 Phase 2 comparison-count instrumentation.
//!
//! Mirrors the runtime portions of `Cpp/test_instrumentation.cpp` and
//! `Go/src/instrumentation_test.go`. Rust gets monomorphization-based
//! zero-overhead at compile time; the size-equality check is a runtime
//! `assert_eq!` (Rust has no `static_assert` analog over `size_of`).

use d_ary_heap::{
    ComparisonStats, InstrumentedPriorityQueue, MinBy, NoOpStats, PriorityQueue, StatsCollector,
};

type IdentityMinBy = MinBy<fn(&i32) -> i32>;
type TestHeap = InstrumentedPriorityQueue<i32, IdentityMinBy>;

fn fresh_min_heap_with_stats(d: usize) -> TestHeap {
    PriorityQueue::with_stats(d, MinBy(identity_i32 as fn(&i32) -> i32)).unwrap()
}

// `identity_i32` must take `&i32` to satisfy the `Fn(&T) -> K` contract that
// `MinBy<F>` is generic over — clippy::trivially_copy_pass_by_ref doesn't
// apply here because the signature is dictated by the comparator interface.
#[allow(clippy::trivially_copy_pass_by_ref)]
fn identity_i32(x: &i32) -> i32 {
    *x
}

#[test]
fn stats_initial_state_is_zero() {
    let pq = fresh_min_heap_with_stats(2);
    let s = pq.stats();
    assert_eq!(s.insert(), 0);
    assert_eq!(s.pop(), 0);
    assert_eq!(s.decrease_priority(), 0);
    assert_eq!(s.increase_priority(), 0);
    assert_eq!(s.update_priority(), 0);
    assert_eq!(s.total(), 0);
}

#[test]
fn insert_bucket_isolation() {
    let mut pq = fresh_min_heap_with_stats(2);
    for v in [5, 3, 8, 1, 9] {
        pq.insert(v);
    }
    let s = pq.stats();
    assert!(s.insert() > 0, "insert bucket should grow during inserts");
    assert_eq!(s.pop(), 0);
    assert_eq!(s.decrease_priority(), 0);
    assert_eq!(s.increase_priority(), 0);
    assert_eq!(s.update_priority(), 0);
    assert_eq!(s.total(), s.insert());
}

#[test]
fn pop_bucket_isolation() {
    let mut pq = fresh_min_heap_with_stats(2);
    for v in [5, 3, 8, 1, 9] {
        pq.insert(v);
    }
    let insert_baseline = pq.stats().insert();

    let popped = pq.pop();
    assert_eq!(popped, Some(1));

    let s = pq.stats();
    assert_eq!(
        s.insert(),
        insert_baseline,
        "Pop must not perturb the insert bucket"
    );
    assert!(s.pop() > 0, "pop bucket should grow during pop");
    assert_eq!(s.decrease_priority(), 0);
    assert_eq!(s.increase_priority(), 0);
    assert_eq!(s.update_priority(), 0);
}

#[test]
fn decrease_priority_bucket() {
    let mut pq = fresh_min_heap_with_stats(2);
    for v in [1, 2, 3, 4] {
        pq.insert(v);
    }
    pq.stats().reset();

    pq.decrease_priority_by_index(0)
        .expect("index 0 is in bounds");

    let s = pq.stats();
    assert!(
        s.decrease_priority() > 0,
        "decrease_priority bucket should grow"
    );
    assert_eq!(s.insert(), 0);
    assert_eq!(s.pop(), 0);
    assert_eq!(s.increase_priority(), 0);
    assert_eq!(s.update_priority(), 0);
}

#[test]
fn increase_priority_bucket() {
    let mut pq = fresh_min_heap_with_stats(2);
    for v in [1, 2, 3, 4] {
        pq.insert(v);
    }
    pq.stats().reset();

    pq.increase_priority_by_index(3)
        .expect("index 3 is in bounds for a 4-element heap");

    let s = pq.stats();
    assert!(
        s.increase_priority() > 0,
        "increase_priority bucket should grow"
    );
    assert_eq!(s.insert(), 0);
    assert_eq!(s.pop(), 0);
    assert_eq!(s.decrease_priority(), 0);
    assert_eq!(s.update_priority(), 0);
}

#[test]
fn update_priority_bucket() {
    let mut pq = fresh_min_heap_with_stats(2);
    for v in [1, 2, 3, 4] {
        pq.insert(v);
    }
    pq.stats().reset();

    pq.update_priority(&1).expect("item 1 is in the heap");

    let s = pq.stats();
    assert!(
        s.update_priority() > 0,
        "update_priority bucket should grow"
    );
    assert_eq!(s.insert(), 0);
    assert_eq!(s.pop(), 0);
    assert_eq!(s.decrease_priority(), 0);
    assert_eq!(s.increase_priority(), 0);
}

#[test]
fn reset_zeros_counters_but_preserves_heap() {
    let mut pq = fresh_min_heap_with_stats(2);
    for v in [5, 3, 8, 1, 9] {
        pq.insert(v);
    }
    pq.pop();
    assert!(
        pq.stats().total() > 0,
        "expected non-zero counters before reset"
    );

    let front_before = *pq.front();
    let len_before = pq.len();

    pq.stats().reset();

    let s = pq.stats();
    assert_eq!(s.total(), 0);
    assert_eq!(s.insert(), 0);
    assert_eq!(s.pop(), 0);
    assert_eq!(s.decrease_priority(), 0);
    assert_eq!(s.increase_priority(), 0);
    assert_eq!(s.update_priority(), 0);

    // Reset is independent of heap state.
    assert_eq!(*pq.front(), front_before);
    assert_eq!(pq.len(), len_before);
}

#[test]
fn total_equals_sum_of_buckets() {
    let mut pq = fresh_min_heap_with_stats(2);
    for v in [5, 3, 8, 1, 9] {
        pq.insert(v);
    }
    pq.pop();
    pq.pop();
    pq.decrease_priority_by_index(0).unwrap();
    pq.update_priority_by_index(0).unwrap();
    let last = pq.len() - 1;
    pq.increase_priority_by_index(last).unwrap();

    let s = pq.stats();
    let manual =
        s.insert() + s.pop() + s.decrease_priority() + s.increase_priority() + s.update_priority();
    assert_eq!(s.total(), manual);
}

#[test]
fn nil_stats_lockstep_pop_order() {
    // Two heaps with identical inputs — one default (NoOpStats), one
    // instrumented. Pop sequences must match: instrumentation is observation-only.
    let mut default_pq = PriorityQueue::new(4, MinBy(identity_i32 as fn(&i32) -> i32)).unwrap();
    let mut stats_pq = fresh_min_heap_with_stats(4);

    let input = [42, 17, 99, 3, 8, 25, 61, 5, 88, 1];
    for v in input {
        default_pq.insert(v);
        stats_pq.insert(v);
    }

    while !default_pq.is_empty() {
        let a = default_pq.pop();
        let b = stats_pq.pop();
        assert_eq!(
            a, b,
            "default heap and instrumented heap diverged on pop order"
        );
    }
    assert!(stats_pq.is_empty());
}

#[test]
fn default_stats_member_is_zero_sized() {
    // Rust's analog to C++'s static_assert on the size delta. NoOpStats is a
    // unit-like struct, which is a zero-sized type; the heap's `stats` field
    // therefore takes zero bytes via Rust's standard ZST layout — no
    // `[[no_unique_address]]` attribute needed.
    assert_eq!(std::mem::size_of::<NoOpStats>(), 0);

    // The default heap and an explicitly-NoOpStats heap must have identical
    // size (proves no padding was introduced for the stats field).
    let default_size = std::mem::size_of::<PriorityQueue<i32, MinBy<fn(&i32) -> i32>>>();
    let no_op_size = std::mem::size_of::<PriorityQueue<i32, MinBy<fn(&i32) -> i32>, NoOpStats>>();
    assert_eq!(default_size, no_op_size);

    // The instrumented heap is strictly larger by exactly sizeof(ComparisonStats).
    let instrumented_size =
        std::mem::size_of::<PriorityQueue<i32, MinBy<fn(&i32) -> i32>, ComparisonStats>>();
    assert!(instrumented_size > default_size);
    assert_eq!(
        instrumented_size - default_size,
        std::mem::size_of::<ComparisonStats>(),
        "instrumented overhead must equal exactly sizeof(ComparisonStats); \
         any extra bytes would mean the NoOpStats slot took space"
    );
}

#[test]
fn nil_stats_total_and_reset_via_default_path() {
    // Sanity check: the default heap's stats() returns &NoOpStats, whose
    // total() / reset() methods are no-ops. This mirrors the cross-language
    // guarantee that `pq.stats().total()` always works.
    let mut pq = PriorityQueue::new(2, MinBy(identity_i32 as fn(&i32) -> i32)).unwrap();
    pq.insert(1);
    pq.insert(2);
    assert_eq!(pq.stats().total(), 0);
    pq.stats().reset(); // must not panic
}
