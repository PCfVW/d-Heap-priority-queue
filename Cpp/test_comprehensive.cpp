/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////// test_comprehensive.cpp
///
/// Comprehensive test suite for C++ d-ary heap priority queue v2.5.0
/// Aligned with Rust, TypeScript, Go, and Zig test patterns for cross-language consistency
///
/// Copyright (c) 2023-2026 Eric Jacopin
///
/// Licensed under the Apache License, Version 2.0 (the "License")
///
/// Compile with CMake or: cl /std:c++latest /EHsc /O2 test_comprehensive.cpp /Fe:test_comprehensive.exe
///
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "PriorityQueue.h"
#include <cassert>
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <sstream>

using namespace TOOLS;

// =============================================================================
// Test Item Type (matches Rust Item struct)
// =============================================================================

struct Item {
    uint32_t id;
    uint32_t cost;

    Item(uint32_t i, uint32_t c) : id(i), cost(c) {}

    bool operator==(const Item& other) const { return id == other.id; }

    friend std::ostream& operator<<(std::ostream& os, const Item& item) {
        return os << "Item(id: " << item.id << ", cost: " << item.cost << ")";
    }
};

struct ItemHash {
    size_t operator()(const Item& item) const {
        return std::hash<uint32_t>()(item.id);
    }
};

struct ItemEqual {
    bool operator()(const Item& a, const Item& b) const {
        return a.id == b.id;
    }
};

struct MinCost {
    bool operator()(const Item& a, const Item& b) const {
        return a.cost < b.cost;
    }
};

struct MaxCost {
    bool operator()(const Item& a, const Item& b) const {
        return a.cost > b.cost;
    }
};

// Type aliases for clarity
using MinHeap = PriorityQueue<Item, ItemHash, MinCost, ItemEqual>;
using MaxHeap = PriorityQueue<Item, ItemHash, MaxCost, ItemEqual>;

// Test counter
static int tests_passed = 0;
static int tests_failed = 0;

void pass(const std::string& name) {
    std::cout << "  [PASS] " << name << std::endl;
    tests_passed++;
}

void fail(const std::string& name, const std::string& reason) {
    std::cout << "  [FAIL] " << name << ": " << reason << std::endl;
    tests_failed++;
}

// =============================================================================
// Basic Operations Tests
// =============================================================================

void test_new() {
    auto result = MinHeap::create(2);
    assert(result.has_value());
    auto& pq = *result;
    assert(pq.len() == 0);
    assert(pq.is_empty());
    assert(pq.d() == 2);
    pass("test_new");
}

void test_new_default_arity() {
    for (size_t d : {1, 2, 3, 4, 8, 16}) {
        auto result = MinHeap::create(d);
        assert(result.has_value());
        assert(result->d() == d);
    }
    pass("test_new_default_arity");
}

void test_new_invalid_arity() {
    auto result = MinHeap::create(0);
    assert(!result.has_value());
    assert(result.error() == Error::InvalidArity);
    pass("test_new_invalid_arity");
}

void test_with_first() {
    Item first(1, 10);
    MinHeap pq(3, first);
    assert(pq.len() == 1);
    assert(!pq.is_empty());
    assert(pq.front().id == 1);
    pass("test_with_first");
}

void test_len() {
    MinHeap pq(2);
    assert(pq.len() == 0);
    pq.insert(Item(1, 10));
    assert(pq.len() == 1);
    pq.insert(Item(2, 20));
    assert(pq.len() == 2);
    pass("test_len");
}

void test_is_empty() {
    MinHeap pq(2);
    assert(pq.is_empty());
    pq.insert(Item(1, 10));
    assert(!pq.is_empty());
    pq.pop();
    assert(pq.is_empty());
    pass("test_is_empty");
}

void test_d() {
    for (size_t d : {1, 2, 3, 4, 8, 16}) {
        MinHeap pq(d);
        assert(pq.d() == d);
    }
    pass("test_d");
}

// =============================================================================
// Insert and Pop Tests
// =============================================================================

void test_insert() {
    MinHeap pq(2);
    pq.insert(Item(1, 10));
    assert(pq.len() == 1);
    assert(pq.front().id == 1);
    pass("test_insert");
}

void test_insert_many() {
    MinHeap pq(2);
    pq.insert_many({Item(1, 50), Item(2, 30), Item(3, 70), Item(4, 10), Item(5, 40)});
    assert(pq.len() == 5);
    assert(pq.front().id == 4);  // Lowest cost = highest priority
    pass("test_insert_many");
}

void test_insert_many_empty() {
    MinHeap pq(2);
    std::vector<Item> empty;
    pq.insert_many(empty);
    assert(pq.is_empty());
    pass("test_insert_many_empty");
}

void test_pop() {
    MinHeap pq(2);
    pq.insert(Item(1, 30));
    pq.insert(Item(2, 10));
    pq.insert(Item(3, 20));

    auto item1 = pq.pop_front();
    assert(item1.has_value() && item1->id == 2);
    auto item2 = pq.pop_front();
    assert(item2.has_value() && item2->id == 3);
    auto item3 = pq.pop_front();
    assert(item3.has_value() && item3->id == 1);
    auto item4 = pq.pop_front();
    assert(!item4.has_value());
    pass("test_pop");
}

void test_pop_many() {
    MinHeap pq(2);
    pq.insert_many({Item(1, 50), Item(2, 10), Item(3, 30), Item(4, 20), Item(5, 40)});

    auto items = pq.pop_many(3);
    assert(items.size() == 3);
    assert(items[0].id == 2);  // cost 10
    assert(items[1].id == 4);  // cost 20
    assert(items[2].id == 3);  // cost 30
    assert(pq.len() == 2);
    pass("test_pop_many");
}

void test_pop_many_more_than_available() {
    MinHeap pq(2);
    pq.insert_many({Item(1, 10), Item(2, 20)});

    auto items = pq.pop_many(10);
    assert(items.size() == 2);
    assert(pq.is_empty());
    pass("test_pop_many_more_than_available");
}

void test_pop_empty() {
    MinHeap pq(2);
    auto item = pq.pop_front();
    assert(!item.has_value());
    pass("test_pop_empty");
}

// =============================================================================
// Front/Peek Tests
// =============================================================================

void test_front() {
    MinHeap pq(2);
    pq.insert(Item(1, 30));
    pq.insert(Item(2, 10));
    assert(pq.front().id == 2);
    pass("test_front");
}

void test_peek() {
    MinHeap pq(2);
    assert(!pq.peek().has_value());

    pq.insert(Item(1, 10));
    auto item = pq.peek();
    assert(item.has_value());
    assert(item->id == 1);
    pass("test_peek");
}

void test_peek_empty() {
    MinHeap pq(2);
    assert(!pq.peek().has_value());
    pass("test_peek_empty");
}

// =============================================================================
// Contains and GetPosition Tests
// =============================================================================

void test_contains() {
    MinHeap pq(2);
    pq.insert(Item(1, 10));

    assert(pq.contains(Item(1, 999)));  // ID matches, cost doesn't matter
    assert(!pq.contains(Item(2, 10)));
    pass("test_contains");
}

void test_contains_empty() {
    MinHeap pq(2);
    assert(!pq.contains(Item(1, 10)));
    pass("test_contains_empty");
}

void test_get_position() {
    MinHeap pq(2);
    pq.insert(Item(1, 30));
    pq.insert(Item(2, 10));
    pq.insert(Item(3, 20));

    // Root (highest priority) is at position 0
    auto pos = pq.get_position(Item(2, 0));
    assert(pos.has_value() && *pos == 0);
    assert(pq.get_position(Item(1, 0)).has_value());
    assert(pq.get_position(Item(3, 0)).has_value());
    assert(!pq.get_position(Item(99, 0)).has_value());
    pass("test_get_position");
}

void test_get_position_missing() {
    MinHeap pq(2);
    assert(!pq.get_position(Item(1, 10)).has_value());
    pass("test_get_position_missing");
}

// =============================================================================
// Priority Update Tests
// =============================================================================

void test_increase_priority() {
    MinHeap pq(2);
    pq.insert(Item(1, 30));
    pq.insert(Item(2, 10));
    pq.insert(Item(3, 20));

    // Item 1 has cost 30, increase priority by lowering cost to 5
    Item updated(1, 5);
    auto result = pq.try_increase_priority(updated);
    assert(result.has_value());
    assert(pq.front().id == 1);
    pass("test_increase_priority");
}

void test_increase_priority_not_found() {
    MinHeap pq(2);
    pq.insert(Item(1, 10));

    auto result = pq.try_increase_priority(Item(99, 5));
    assert(!result.has_value());
    assert(result.error() == Error::ItemNotFound);
    pass("test_increase_priority_not_found");
}

void test_decrease_priority() {
    MinHeap pq(2);
    pq.insert(Item(1, 10));
    pq.insert(Item(2, 20));
    pq.insert(Item(3, 30));

    // Item 1 has cost 10, decrease priority by raising cost to 50
    Item updated(1, 50);
    auto result = pq.try_decrease_priority(updated);
    assert(result.has_value());
    assert(pq.front().id == 2);  // Now item 2 (cost 20) is front
    pass("test_decrease_priority");
}

void test_decrease_priority_not_found() {
    MinHeap pq(2);
    pq.insert(Item(1, 10));

    auto result = pq.try_decrease_priority(Item(99, 50));
    assert(!result.has_value());
    assert(result.error() == Error::ItemNotFound);
    pass("test_decrease_priority_not_found");
}

void test_update_priority_moves_up() {
    MinHeap pq(2);
    pq.insert(Item(1, 30));
    pq.insert(Item(2, 10));
    pq.insert(Item(3, 20));

    // Item 3 has cost 20, update to cost 5 (moves up)
    Item updated(3, 5);
    auto result = pq.try_update_priority(updated);
    assert(result.has_value());
    assert(pq.front().id == 3);
    pass("test_update_priority_moves_up");
}

void test_update_priority_moves_down() {
    MinHeap pq(2);
    pq.insert(Item(1, 10));
    pq.insert(Item(2, 20));
    pq.insert(Item(3, 30));

    // Item 1 has cost 10, update to cost 100 (moves down)
    Item updated(1, 100);
    auto result = pq.try_update_priority(updated);
    assert(result.has_value());
    assert(pq.front().id == 2);
    pass("test_update_priority_moves_down");
}

void test_update_priority_not_found() {
    MinHeap pq(2);
    pq.insert(Item(1, 10));

    auto result = pq.try_update_priority(Item(99, 5));
    assert(!result.has_value());
    assert(result.error() == Error::ItemNotFound);
    pass("test_update_priority_not_found");
}

// =============================================================================
// By-Index Priority Update Tests
// =============================================================================

void test_increase_priority_by_index() {
    MinHeap pq(2);
    pq.insert(Item(1, 10));
    pq.insert(Item(2, 20));

    auto pos = pq.get_position(Item(2, 0));
    assert(pos.has_value());
    auto result = pq.increase_priority_by_index(*pos);
    assert(result.has_value());
    assert(pq.len() == 2);
    pass("test_increase_priority_by_index");
}

void test_increase_priority_by_index_out_of_bounds() {
    MinHeap pq(2);
    pq.insert(Item(1, 10));

    auto result = pq.increase_priority_by_index(99);
    assert(!result.has_value());
    assert(result.error() == Error::IndexOutOfBounds);
    pass("test_increase_priority_by_index_out_of_bounds");
}

void test_decrease_priority_by_index() {
    MinHeap pq(2);
    pq.insert(Item(1, 10));
    pq.insert(Item(2, 20));

    auto result = pq.decrease_priority_by_index(0);
    assert(result.has_value());
    assert(pq.len() == 2);
    pass("test_decrease_priority_by_index");
}

void test_decrease_priority_by_index_out_of_bounds() {
    MinHeap pq(2);
    pq.insert(Item(1, 10));

    auto result = pq.decrease_priority_by_index(99);
    assert(!result.has_value());
    assert(result.error() == Error::IndexOutOfBounds);
    pass("test_decrease_priority_by_index_out_of_bounds");
}

void test_update_priority_by_index() {
    MinHeap pq(2);
    pq.insert(Item(1, 10));
    pq.insert(Item(2, 20));

    auto result = pq.update_priority_by_index(0);
    assert(result.has_value());
    assert(pq.len() == 2);
    pass("test_update_priority_by_index");
}

void test_update_priority_by_index_out_of_bounds() {
    MinHeap pq(2);
    pq.insert(Item(1, 10));

    auto result = pq.update_priority_by_index(99);
    assert(!result.has_value());
    assert(result.error() == Error::IndexOutOfBounds);
    pass("test_update_priority_by_index_out_of_bounds");
}

// =============================================================================
// Min/Max Heap Tests
// =============================================================================

void test_min_heap() {
    MinHeap pq(2);

    for (uint32_t cost : {50u, 30u, 70u, 10u, 40u, 60u, 20u, 80u}) {
        pq.insert(Item(cost, cost));
    }

    uint32_t prev = 0;
    while (!pq.is_empty()) {
        auto item = pq.pop_front();
        assert(item.has_value());
        assert(item->cost >= prev);
        prev = item->cost;
    }
    pass("test_min_heap");
}

void test_max_heap() {
    MaxHeap pq(2);

    for (uint32_t cost : {50u, 30u, 70u, 10u, 40u, 60u, 20u, 80u}) {
        pq.insert(Item(cost, cost));
    }

    uint32_t prev = UINT32_MAX;
    while (!pq.is_empty()) {
        auto item = pq.pop_front();
        assert(item.has_value());
        assert(item->cost <= prev);
        prev = item->cost;
    }
    pass("test_max_heap");
}

// =============================================================================
// Different Arities Tests
// =============================================================================

void test_arity_helper(size_t d) {
    MinHeap pq(d);

    std::vector<uint32_t> costs = {50, 30, 70, 10, 40, 60, 20, 80, 90, 5};
    for (size_t i = 0; i < costs.size(); ++i) {
        pq.insert(Item(static_cast<uint32_t>(i), costs[i]));
    }

    uint32_t prev = 0;
    while (!pq.is_empty()) {
        auto item = pq.pop_front();
        assert(item.has_value());
        assert(item->cost >= prev);
        prev = item->cost;
    }
}

void test_arity_1() { test_arity_helper(1); pass("test_arity_1"); }
void test_arity_2() { test_arity_helper(2); pass("test_arity_2"); }
void test_arity_3() { test_arity_helper(3); pass("test_arity_3"); }
void test_arity_4() { test_arity_helper(4); pass("test_arity_4"); }
void test_arity_8() { test_arity_helper(8); pass("test_arity_8"); }
void test_arity_16() { test_arity_helper(16); pass("test_arity_16"); }

// =============================================================================
// Clear Tests
// =============================================================================

void test_clear() {
    MinHeap pq(2);
    pq.insert(Item(1, 10));
    pq.insert(Item(2, 20));

    auto result = pq.try_clear();
    assert(result.has_value());
    assert(pq.is_empty());
    assert(pq.d() == 2);  // Arity preserved
    pass("test_clear");
}

void test_clear_with_new_arity() {
    MinHeap pq(2);
    pq.insert(Item(1, 10));

    auto result = pq.try_clear(4);
    assert(result.has_value());
    assert(pq.is_empty());
    assert(pq.d() == 4);  // Arity changed
    pass("test_clear_with_new_arity");
}

void test_clear_invalid_arity() {
    MinHeap pq(2);

    auto result = pq.try_clear(0);
    assert(!result.has_value());
    assert(result.error() == Error::InvalidArity);
    pass("test_clear_invalid_arity");
}

// =============================================================================
// String Representation Tests
// =============================================================================

void test_to_string() {
    MinHeap pq(2);
    pq.insert(Item(1, 10));
    pq.insert(Item(2, 5));

    std::string output = pq.to_string();
    assert(output.front() == '{');
    assert(output.back() == '}');
    assert(output.find("Item") != std::string::npos);
    pass("test_to_string");
}

void test_to_string_empty() {
    MinHeap pq(2);
    assert(pq.to_string() == "{}");
    pass("test_to_string_empty");
}

void test_put_stream() {
    MinHeap pq(2);
    pq.insert(Item(1, 10));

    std::ostringstream oss;
    pq.put(oss);
    std::string stream_output = oss.str();

    std::string to_string_output = pq.to_string();
    assert(stream_output == to_string_output);
    pass("test_put_stream");
}

// =============================================================================
// to_array Tests
// =============================================================================

void test_to_array() {
    MinHeap pq(2);
    pq.insert(Item(1, 30));
    pq.insert(Item(2, 10));
    pq.insert(Item(3, 20));

    auto arr = pq.to_array();
    assert(arr.size() == 3);
    assert(arr[0].id == 2);  // Root is highest priority (lowest cost)
    pass("test_to_array");
}

void test_to_array_empty() {
    MinHeap pq(2);
    auto arr = pq.to_array();
    assert(arr.empty());
    pass("test_to_array_empty");
}

// =============================================================================
// Heap Property Maintenance Tests
// =============================================================================

void test_heap_property_maintained() {
    MinHeap pq(3);

    // Insert many items with pseudo-random costs
    for (uint32_t i = 0; i < 100; ++i) {
        pq.insert(Item(i, (i * 7 + 13) % 100));
    }

    // Verify heap property with sequential pops
    uint32_t prev = 0;
    while (!pq.is_empty()) {
        auto item = pq.pop_front();
        assert(item.has_value());
        assert(item->cost >= prev);
        prev = item->cost;
    }
    pass("test_heap_property_maintained");
}

void test_heap_property_after_updates() {
    MinHeap pq(2);

    // Insert items
    for (uint32_t i = 0; i < 50; ++i) {
        pq.insert(Item(i, i * 2));
    }

    // Perform updates
    for (uint32_t i = 0; i < 25; ++i) {
        uint32_t new_cost = (i * 3 + 7) % 100;
        (void)pq.try_update_priority(Item(i, new_cost));
    }

    // Pop some items
    for (int i = 0; i < 10; ++i) {
        pq.pop_front();
    }

    // Verify remaining items maintain heap property
    uint32_t prev = 0;
    while (!pq.is_empty()) {
        auto item = pq.pop_front();
        assert(item.has_value());
        assert(item->cost >= prev);
        prev = item->cost;
    }
    pass("test_heap_property_after_updates");
}

// =============================================================================
// Edge Cases Tests
// =============================================================================

void test_single_element() {
    MinHeap pq(2);
    pq.insert(Item(1, 10));

    assert(pq.len() == 1);
    assert(pq.front().id == 1);
    assert(pq.contains(Item(1, 0)));

    (void)pq.try_increase_priority(Item(1, 5));
    assert(pq.front().cost == 5);

    auto item = pq.pop_front();
    assert(item.has_value() && item->id == 1);
    assert(pq.is_empty());
    pass("test_single_element");
}

void test_duplicate_priorities() {
    MinHeap pq(2);

    // All items have same priority
    pq.insert(Item(1, 10));
    pq.insert(Item(2, 10));
    pq.insert(Item(3, 10));

    assert(pq.len() == 3);

    // All items should be poppable
    std::vector<uint32_t> ids;
    while (!pq.is_empty()) {
        auto item = pq.pop_front();
        ids.push_back(item->id);
    }
    assert(ids.size() == 3);
    pass("test_duplicate_priorities");
}

// =============================================================================
// Large Scale Tests
// =============================================================================

void test_large_heap() {
    MinHeap pq(4);

    // Insert 10000 items with pseudo-random costs
    for (uint32_t i = 0; i < 10000; ++i) {
        uint32_t cost = ((i * 31337 + 12345) % 5000);
        pq.insert(Item(i, cost));
    }

    assert(pq.len() == 10000);

    // Verify sorted output
    uint32_t prev = 0;
    while (!pq.is_empty()) {
        auto item = pq.pop_front();
        assert(item.has_value());
        assert(item->cost >= prev);
        prev = item->cost;
    }
    pass("test_large_heap");
}

void test_large_heap_with_updates() {
    MinHeap pq(4);

    // Insert 1000 items
    for (uint32_t i = 0; i < 1000; ++i) {
        pq.insert(Item(i, i));
    }

    // Perform 500 updates
    for (uint32_t i = 0; i < 500; ++i) {
        uint32_t new_cost = ((i * 17 + 23) % 1000);
        (void)pq.try_update_priority(Item(i, new_cost));
    }

    // Verify sorted output
    uint32_t prev = 0;
    while (!pq.is_empty()) {
        auto item = pq.pop_front();
        assert(item.has_value());
        assert(item->cost >= prev);
        prev = item->cost;
    }
    pass("test_large_heap_with_updates");
}

// =============================================================================
// Position Type Alias Test
// =============================================================================

void test_position_type_alias() {
    MinHeap pq(2);
    pq.insert(Item(1, 10));

    MinHeap::Position pos = *pq.get_position(Item(1, 0));
    assert(pos == 0);
    pass("test_position_type_alias");
}

// =============================================================================
// Error Display Test
// =============================================================================

void test_error_display() {
    assert(to_string(Error::InvalidArity) == "Heap arity (d) must be >= 1");
    assert(to_string(Error::ItemNotFound) == "Item not found");
    assert(to_string(Error::IndexOutOfBounds) == "Index out of bounds");
    assert(to_string(Error::EmptyQueue) == "Operation called on empty priority queue");
    pass("test_error_display");
}

// =============================================================================
// Primitive Type Tests (using int directly)
// =============================================================================

void test_primitive_min_heap() {
    PriorityQueue<int> pq(2);

    pq.insert(5);
    pq.insert(3);
    pq.insert(7);
    pq.insert(1);

    assert(pq.pop_front() == 1);
    assert(pq.pop_front() == 3);
    assert(pq.pop_front() == 5);
    assert(pq.pop_front() == 7);
    assert(!pq.pop_front().has_value());
    pass("test_primitive_min_heap");
}

void test_primitive_max_heap() {
    PriorityQueue<int, std::hash<int>, std::greater<int>> pq(2);

    pq.insert(5);
    pq.insert(3);
    pq.insert(7);
    pq.insert(1);

    assert(pq.pop_front() == 7);
    assert(pq.pop_front() == 5);
    assert(pq.pop_front() == 3);
    assert(pq.pop_front() == 1);
    assert(!pq.pop_front().has_value());
    pass("test_primitive_max_heap");
}

// =============================================================================
// Backward Compatibility Tests
// =============================================================================

void test_backward_compatibility_methods() {
    MinHeap pq(3);
    pq.insert(Item(1, 10));
    pq.insert(Item(2, 5));
    pq.insert(Item(3, 15));

    // Test old method names still work
    assert(pq.size() == 3);
    assert(pq.empty() == false);
    assert(pq.getd() == 3);

    // Test consistency with new names
    assert(pq.size() == pq.len());
    assert(pq.empty() == pq.is_empty());
    assert(pq.getd() == pq.d());
    pass("test_backward_compatibility_methods");
}

// =============================================================================
// Main
// =============================================================================

int main() {
    std::cout << "=== C++ d-ary Heap Priority Queue v2.5.0 Comprehensive Test Suite ===" << std::endl;
    std::cout << "    Aligned with Rust, TypeScript, Go, and Zig test patterns" << std::endl;
    std::cout << std::endl;

    try {
        // Basic Operations Tests
        std::cout << "Basic Operations Tests:" << std::endl;
        test_new();
        test_new_default_arity();
        test_new_invalid_arity();
        test_with_first();
        test_len();
        test_is_empty();
        test_d();

        // Insert and Pop Tests
        std::cout << "\nInsert and Pop Tests:" << std::endl;
        test_insert();
        test_insert_many();
        test_insert_many_empty();
        test_pop();
        test_pop_many();
        test_pop_many_more_than_available();
        test_pop_empty();

        // Front/Peek Tests
        std::cout << "\nFront/Peek Tests:" << std::endl;
        test_front();
        test_peek();
        test_peek_empty();

        // Contains and GetPosition Tests
        std::cout << "\nContains and GetPosition Tests:" << std::endl;
        test_contains();
        test_contains_empty();
        test_get_position();
        test_get_position_missing();

        // Priority Update Tests
        std::cout << "\nPriority Update Tests:" << std::endl;
        test_increase_priority();
        test_increase_priority_not_found();
        test_decrease_priority();
        test_decrease_priority_not_found();
        test_update_priority_moves_up();
        test_update_priority_moves_down();
        test_update_priority_not_found();

        // By-Index Priority Update Tests
        std::cout << "\nBy-Index Priority Update Tests:" << std::endl;
        test_increase_priority_by_index();
        test_increase_priority_by_index_out_of_bounds();
        test_decrease_priority_by_index();
        test_decrease_priority_by_index_out_of_bounds();
        test_update_priority_by_index();
        test_update_priority_by_index_out_of_bounds();

        // Min/Max Heap Tests
        std::cout << "\nMin/Max Heap Tests:" << std::endl;
        test_min_heap();
        test_max_heap();

        // Different Arities Tests
        std::cout << "\nDifferent Arities Tests:" << std::endl;
        test_arity_1();
        test_arity_2();
        test_arity_3();
        test_arity_4();
        test_arity_8();
        test_arity_16();

        // Clear Tests
        std::cout << "\nClear Tests:" << std::endl;
        test_clear();
        test_clear_with_new_arity();
        test_clear_invalid_arity();

        // String Representation Tests
        std::cout << "\nString Representation Tests:" << std::endl;
        test_to_string();
        test_to_string_empty();
        test_put_stream();

        // to_array Tests
        std::cout << "\nto_array Tests:" << std::endl;
        test_to_array();
        test_to_array_empty();

        // Heap Property Maintenance Tests
        std::cout << "\nHeap Property Maintenance Tests:" << std::endl;
        test_heap_property_maintained();
        test_heap_property_after_updates();

        // Edge Cases Tests
        std::cout << "\nEdge Cases Tests:" << std::endl;
        test_single_element();
        test_duplicate_priorities();

        // Large Scale Tests
        std::cout << "\nLarge Scale Tests:" << std::endl;
        test_large_heap();
        test_large_heap_with_updates();

        // Position Type Alias Test
        std::cout << "\nPosition Type Alias Test:" << std::endl;
        test_position_type_alias();

        // Error Display Test
        std::cout << "\nError Display Test:" << std::endl;
        test_error_display();

        // Primitive Type Tests
        std::cout << "\nPrimitive Type Tests:" << std::endl;
        test_primitive_min_heap();
        test_primitive_max_heap();

        // Backward Compatibility Tests
        std::cout << "\nBackward Compatibility Tests:" << std::endl;
        test_backward_compatibility_methods();

        std::cout << std::endl;
        std::cout << "=== Results: " << tests_passed << " passed, " << tests_failed << " failed ===" << std::endl;

        if (tests_failed == 0) {
            std::cout << "=== All tests passed! ===" << std::endl;
            return 0;
        } else {
            return 1;
        }
    } catch (const std::exception& e) {
        std::cerr << "Test failed with exception: " << e.what() << std::endl;
        return 1;
    } catch (...) {
        std::cerr << "Test failed with unknown exception" << std::endl;
        return 1;
    }
}
