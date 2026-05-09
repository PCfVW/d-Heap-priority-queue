/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////// test_instrumentation.cpp
///
/// Test suite for v2.6.0 Phase 2 comparison-count instrumentation.
///
/// Verifies two properties:
///   1. Zero-overhead-when-disabled is structurally true (compile-time static_asserts).
///   2. ComparisonStats correctly attributes comparisons to the operation that triggered them.
///
/// Copyright (c) 2023-2026 Eric Jacopin
///
/// Licensed under the Apache License, Version 2.0 (the "License")
///
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "PriorityQueue.h"
#include <cassert>
#include <iostream>
#include <type_traits>

using namespace TOOLS;

// =====================================================================================================================
// Compile-time structural properties: prove zero-overhead at the type level
// =====================================================================================================================

// Default PriorityQueue<int> resolves to the NoOpStats variant.
static_assert(std::is_same_v<
                  PriorityQueue<int>,
                  PriorityQueue<int, std::hash<int>, std::less<int>, std::equal_to<int>, NoOpStats>>,
              "Default 5th template argument must be NoOpStats");

// ComparisonStats version is strictly larger than the default version.
static_assert(sizeof(PriorityQueue<int>) < sizeof(InstrumentedPriorityQueue<int>),
              "InstrumentedPriorityQueue must be strictly larger than the default heap");

// [[no_unique_address]] (or [[msvc::no_unique_address]] on MSVC) must collapse the
// empty NoOpStats member to zero bytes. If it does, the size delta between the two
// instantiations equals exactly sizeof(ComparisonStats). If [[no_unique_address]]
// is silently ignored, the delta would be smaller (the NoOpStats version would
// itself carry a 1+ byte stats_ member plus padding).
static_assert(sizeof(InstrumentedPriorityQueue<int>) - sizeof(PriorityQueue<int>)
                  == sizeof(ComparisonStats),
              "[[no_unique_address]] must collapse NoOpStats to zero bytes; "
              "the size delta should equal exactly sizeof(ComparisonStats).");

// =====================================================================================================================
// Runtime behavior
// =====================================================================================================================

void test_initial_state() {
    std::cout << "Testing initial state of a fresh InstrumentedPriorityQueue..." << std::endl;
    InstrumentedPriorityQueue<int> pq(2);
    assert(pq.stats().insert == 0);
    assert(pq.stats().pop == 0);
    assert(pq.stats().decrease_priority == 0);
    assert(pq.stats().increase_priority == 0);
    assert(pq.stats().update_priority == 0);
    assert(pq.stats().total() == 0);
    std::cout << "  [OK] all five buckets and total() are zero on a fresh heap" << std::endl;
}

void test_insert_bucket() {
    std::cout << "Testing insert(): only the insert bucket grows..." << std::endl;
    InstrumentedPriorityQueue<int> pq(2);
    for (int v : {5, 3, 8, 1, 9}) pq.insert(v);

    assert(pq.stats().insert > 0);
    assert(pq.stats().pop == 0);
    assert(pq.stats().decrease_priority == 0);
    assert(pq.stats().increase_priority == 0);
    assert(pq.stats().update_priority == 0);
    assert(pq.stats().total() == pq.stats().insert);
    std::cout << "  [OK] insert=" << pq.stats().insert
              << ", others=0, total()=insert" << std::endl;
}

void test_pop_bucket() {
    std::cout << "Testing pop_front(): pop bucket grows, insert is preserved..." << std::endl;
    InstrumentedPriorityQueue<int> pq(2);
    for (int v : {5, 3, 8, 1, 9}) pq.insert(v);
    auto insert_baseline = pq.stats().insert;

    auto popped = pq.pop_front();
    assert(popped.has_value() && *popped == 1);  // 1 is the smallest (highest priority for std::less)

    assert(pq.stats().insert == insert_baseline);  // insert bucket NOT touched by pop
    assert(pq.stats().pop > 0);
    assert(pq.stats().decrease_priority == 0);
    assert(pq.stats().increase_priority == 0);
    assert(pq.stats().update_priority == 0);
    std::cout << "  [OK] pop=" << pq.stats().pop
              << ", insert preserved at " << pq.stats().insert << std::endl;
}

void test_decrease_priority_bucket() {
    std::cout << "Testing decrease_priority_by_index(): only its bucket grows..." << std::endl;
    InstrumentedPriorityQueue<int> pq(2);
    for (int v : {1, 2, 3, 4}) pq.insert(v);
    pq.stats().reset();

    // Calling decrease_priority on the root forces move_down: best_child_position
    // iterates the children (1 comparison) and move_down compares root vs best child
    // (1 comparison), so the bucket should be > 0.
    auto r = pq.decrease_priority_by_index(0);
    assert(r.has_value());

    assert(pq.stats().decrease_priority > 0);
    assert(pq.stats().insert == 0);
    assert(pq.stats().pop == 0);
    assert(pq.stats().increase_priority == 0);
    assert(pq.stats().update_priority == 0);
    std::cout << "  [OK] decrease_priority=" << pq.stats().decrease_priority
              << ", others=0" << std::endl;
}

void test_increase_priority_bucket() {
    std::cout << "Testing increase_priority_by_index(): only its bucket grows..." << std::endl;
    InstrumentedPriorityQueue<int> pq(2);
    for (int v : {1, 2, 3, 4}) pq.insert(v);
    pq.stats().reset();

    // Calling increase_priority on a leaf (index 3) forces move_up to compare with
    // its parent at least once.
    auto r = pq.increase_priority_by_index(3);
    assert(r.has_value());

    assert(pq.stats().increase_priority > 0);
    assert(pq.stats().insert == 0);
    assert(pq.stats().pop == 0);
    assert(pq.stats().decrease_priority == 0);
    assert(pq.stats().update_priority == 0);
    std::cout << "  [OK] increase_priority=" << pq.stats().increase_priority
              << ", others=0" << std::endl;
}

void test_update_priority_bucket() {
    std::cout << "Testing update_priority_by_index(): only its bucket grows..." << std::endl;
    InstrumentedPriorityQueue<int> pq(2);
    for (int v : {1, 2, 3, 4}) pq.insert(v);
    pq.stats().reset();

    // update_priority does both move_up and move_down; from the root, move_up is
    // a no-op but move_down compares with children.
    auto r = pq.update_priority_by_index(0);
    assert(r.has_value());

    assert(pq.stats().update_priority > 0);
    assert(pq.stats().insert == 0);
    assert(pq.stats().pop == 0);
    assert(pq.stats().decrease_priority == 0);
    assert(pq.stats().increase_priority == 0);
    std::cout << "  [OK] update_priority=" << pq.stats().update_priority
              << ", others=0" << std::endl;
}

void test_reset_zeros_counters_but_preserves_heap() {
    std::cout << "Testing reset(): clears counters, leaves heap state intact..." << std::endl;
    InstrumentedPriorityQueue<int> pq(2);
    for (int v : {5, 3, 8, 1, 9}) pq.insert(v);
    pq.pop_front();
    assert(pq.stats().total() > 0);

    int front_before = pq.front();
    auto size_before = pq.len();

    pq.stats().reset();

    assert(pq.stats().insert == 0);
    assert(pq.stats().pop == 0);
    assert(pq.stats().decrease_priority == 0);
    assert(pq.stats().increase_priority == 0);
    assert(pq.stats().update_priority == 0);
    assert(pq.stats().total() == 0);

    // Heap state untouched by reset().
    assert(pq.front() == front_before);
    assert(pq.len() == size_before);
    std::cout << "  [OK] all five buckets zeroed; front()=" << pq.front()
              << ", len()=" << pq.len() << " unchanged" << std::endl;
}

void test_total_equals_sum_of_buckets() {
    std::cout << "Testing total() = sum of all five buckets..." << std::endl;
    InstrumentedPriorityQueue<int> pq(2);
    for (int v : {5, 3, 8, 1, 9}) pq.insert(v);
    pq.pop_front();
    pq.pop_front();
    auto r1 = pq.decrease_priority_by_index(0); assert(r1.has_value());
    auto r2 = pq.update_priority_by_index(0);   assert(r2.has_value());
    auto r3 = pq.increase_priority_by_index(static_cast<PriorityQueue<int>::Position>(pq.len() - 1));
    assert(r3.has_value());

    const auto& s = pq.stats();
    const auto manual_sum = s.insert + s.pop + s.decrease_priority + s.increase_priority + s.update_priority;
    assert(s.total() == manual_sum);
    std::cout << "  [OK] total()=" << s.total()
              << " = insert(" << s.insert
              << ") + pop(" << s.pop
              << ") + dec(" << s.decrease_priority
              << ") + inc(" << s.increase_priority
              << ") + upd(" << s.update_priority << ")" << std::endl;
}

void test_default_heap_unchanged() {
    std::cout << "Testing default PriorityQueue<int> still works (NoOpStats path)..." << std::endl;
    // The non-instrumented heap continues to behave correctly. Its stats() accessor
    // returns the NoOpStats sentinel whose total() always returns 0.
    PriorityQueue<int> pq(2);
    pq.insert(5);
    pq.insert(3);
    pq.insert(8);

    assert(pq.stats().total() == 0);
    assert(pq.front() == 3);
    auto popped = pq.pop_front();
    assert(popped.has_value() && *popped == 3);
    std::cout << "  [OK] default heap behaves normally; stats().total() is constant 0" << std::endl;
}

void test_lockstep_pop_order() {
    std::cout << "Testing lockstep pop-order: default heap and instrumented heap agree..." << std::endl;
    // Mirrors Go's TestNilStatsHeapBehaviorUnchanged: two heaps, identical inputs,
    // one with NoOpStats and one with ComparisonStats. Instrumentation must be
    // observation-only — the popped-item order has to match exactly.
    PriorityQueue<int> default_pq(4);
    InstrumentedPriorityQueue<int> stats_pq(4);

    const int input[] = {42, 17, 99, 3, 8, 25, 61, 5, 88, 1};
    for (int v : input) {
        default_pq.insert(v);
        stats_pq.insert(v);
    }

    while (!default_pq.is_empty()) {
        auto a = default_pq.pop_front();
        auto b = stats_pq.pop_front();
        assert(a.has_value() && b.has_value());
        assert(*a == *b);
    }
    assert(stats_pq.is_empty());
    std::cout << "  [OK] both heaps emptied in the same order; instrumentation is observation-only" << std::endl;
}

int main() {
    std::cout << "=== Phase 2 comparison-count instrumentation tests ===" << std::endl;
    std::cout << "sizeof(PriorityQueue<int>)             = " << sizeof(PriorityQueue<int>) << " bytes" << std::endl;
    std::cout << "sizeof(InstrumentedPriorityQueue<int>) = " << sizeof(InstrumentedPriorityQueue<int>) << " bytes" << std::endl;
    std::cout << "sizeof(ComparisonStats)                = " << sizeof(ComparisonStats) << " bytes (delta when instrumenting)" << std::endl;
    std::cout << std::endl;

    test_initial_state();
    test_insert_bucket();
    test_pop_bucket();
    test_decrease_priority_bucket();
    test_increase_priority_bucket();
    test_update_priority_bucket();
    test_reset_zeros_counters_but_preserves_heap();
    test_total_equals_sum_of_buckets();
    test_default_heap_unchanged();
    test_lockstep_pop_order();

    std::cout << std::endl << "All instrumentation tests passed." << std::endl;
    return 0;
}
