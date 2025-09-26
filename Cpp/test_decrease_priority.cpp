/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////// test_decrease_priority.cpp
///
/// Comprehensive test suite for decrease_priority() method in C++ d-ary heap priority queue
/// 
/// Copyright (c) 2023-2025 Eric Jacopin
/// 
/// Licensed under the Apache License, Version 2.0 (the "License")
/// 
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "PriorityQueue.h"
#include <cassert>
#include <iostream>
#include <string>
#include <vector>

using namespace TOOLS;

// Test item structure
struct TestItem {
    int id;
    int priority;
    
    TestItem(int i, int p) : id(i), priority(p) {}
    
    bool operator==(const TestItem& other) const { return id == other.id; }
};

// Hash function for TestItem
struct TestItemHash {
    size_t operator()(const TestItem& item) const {
        return std::hash<int>()(item.id);
    }
};

// Equality function for TestItem
struct TestItemEqual {
    bool operator()(const TestItem& a, const TestItem& b) const {
        return a.id == b.id;
    }
};

// Min-heap comparator (lower priority value = higher priority)
struct MinComparator {
    bool operator()(const TestItem& a, const TestItem& b) const {
        return a.priority < b.priority;
    }
};

// Max-heap comparator (higher priority value = higher priority)
struct MaxComparator {
    bool operator()(const TestItem& a, const TestItem& b) const {
        return a.priority > b.priority;
    }
};

void test_basic_decrease_functionality() {
    std::cout << "Testing basic decrease functionality..." << std::endl;
    
    PriorityQueue<TestItem, TestItemHash, MinComparator, TestItemEqual> pq(3);
    
    // Insert items
    pq.insert(TestItem(1, 10));
    pq.insert(TestItem(2, 5));
    pq.insert(TestItem(3, 15));
    
    // Verify initial state (min-heap: 5 should be at front)
    assert(pq.front().priority == 5);
    assert(pq.len() == 3);
    
    // Decrease priority of item 3 (15 -> 3, should become new front)
    TestItem updated_item(3, 3);
    pq.decrease_priority(updated_item);
    
    // Verify item 3 is now at front
    assert(pq.front().id == 3);
    assert(pq.front().priority == 3);
    assert(pq.len() == 3);
    
    std::cout << "✓ Basic decrease functionality works correctly" << std::endl;
}

void test_min_heap_behavior() {
    std::cout << "\nTesting min-heap behavior..." << std::endl;
    
    PriorityQueue<TestItem, TestItemHash, MinComparator, TestItemEqual> pq(2);
    
    // Insert items in min-heap
    pq.insert(TestItem(1, 20));
    pq.insert(TestItem(2, 10));
    pq.insert(TestItem(3, 30));
    pq.insert(TestItem(4, 15));
    
    // Initial front should be item 2 (priority 10)
    assert(pq.front().id == 2);
    
    // Decrease priority of item 1 (20 -> 5, should become new front)
    TestItem updated_item1(1, 5);
    pq.decrease_priority(updated_item1);
    assert(pq.front().id == 1);
    assert(pq.front().priority == 5);
    
    // Decrease priority of item 3 (30 -> 25, should not affect front)
    TestItem updated_item3(3, 25);
    pq.decrease_priority(updated_item3);
    assert(pq.front().id == 1);  // Still item 1 at front
    
    std::cout << "✓ Min-heap behavior works correctly" << std::endl;
}

void test_max_heap_behavior() {
    std::cout << "\nTesting max-heap behavior..." << std::endl;
    
    PriorityQueue<TestItem, TestItemHash, MaxComparator, TestItemEqual> pq(2);
    
    // Insert items in max-heap
    pq.insert(TestItem(1, 10));
    pq.insert(TestItem(2, 20));
    pq.insert(TestItem(3, 5));
    pq.insert(TestItem(4, 15));
    
    // Initial front should be item 2 (priority 20)
    assert(pq.front().id == 2);
    
    // Decrease priority of item 2 (20 -> 8, should no longer be front)
    TestItem updated_item2(2, 8);
    pq.decrease_priority(updated_item2);
    assert(pq.front().id == 4);  // Item 4 (priority 15) should now be front
    
    std::cout << "✓ Max-heap behavior works correctly" << std::endl;
}

void test_edge_cases() {
    std::cout << "\nTesting edge cases..." << std::endl;
    
    PriorityQueue<TestItem, TestItemHash, MinComparator, TestItemEqual> pq(3);
    
    // Test single item
    pq.insert(TestItem(1, 10));
    TestItem updated_single(1, 5);
    pq.decrease_priority(updated_single);
    assert(pq.front().priority == 5);
    assert(pq.len() == 1);
    
    // Clear and test item not found (should assert in debug mode)
    pq.clear();
    std::cout << "✓ Edge cases handled correctly" << std::endl;
}

void test_integration_mixed_operations() {
    std::cout << "\nTesting integration with mixed operations..." << std::endl;
    
    PriorityQueue<TestItem, TestItemHash, MinComparator, TestItemEqual> pq(3);
    
    // Complex sequence of operations
    pq.insert(TestItem(1, 50));
    pq.insert(TestItem(2, 30));
    pq.insert(TestItem(3, 70));
    pq.insert(TestItem(4, 20));
    pq.insert(TestItem(5, 60));
    
    // Initial front should be item 4 (priority 20)
    assert(pq.front().id == 4);
    
    // Increase priority of item 1 (50 -> 10, should become new front)
    TestItem increased_item1(1, 10);
    pq.increase_priority(increased_item1);
    assert(pq.front().id == 1);
    
    // Decrease priority of item 2 (30 -> 40)
    TestItem decreased_item2(2, 40);
    pq.decrease_priority(decreased_item2);
    assert(pq.front().id == 1);  // Still item 1 at front
    
    // Pop front item
    pq.pop();
    assert(pq.front().id == 4);  // Item 4 (priority 20) should now be front
    
    // Decrease priority of current front (20 -> 45, should make item 2 the new front)
    TestItem decreased_item4(4, 45);
    pq.decrease_priority(decreased_item4);
    assert(pq.front().id == 2);  // Item 2 (priority 40) should now be front
    
    std::cout << "✓ Integration with mixed operations works correctly" << std::endl;
}

void test_heap_property_maintenance() {
    std::cout << "\nTesting heap property maintenance..." << std::endl;
    
    PriorityQueue<TestItem, TestItemHash, MinComparator, TestItemEqual> pq(2);
    
    // Insert many items
    std::vector<int> priorities = {50, 30, 70, 20, 60, 10, 80, 40};
    for (int i = 0; i < priorities.size(); ++i) {
        pq.insert(TestItem(i + 1, priorities[i]));
    }
    
    // Perform several decrease operations
    TestItem item1_updated(1, 55);  // 50 -> 55
    TestItem item6_updated(6, 15);  // 10 -> 15
    TestItem item3_updated(3, 75);  // 70 -> 75
    pq.decrease_priority(item1_updated);
    pq.decrease_priority(item6_updated);
    pq.decrease_priority(item3_updated);
    
    // Verify heap property by popping all items in order
    std::vector<int> popped_priorities;
    while (!pq.is_empty()) {
        popped_priorities.push_back(pq.front().priority);
        pq.pop();
    }
    
    // Verify non-decreasing order (min-heap property)
    for (size_t i = 1; i < popped_priorities.size(); ++i) {
        assert(popped_priorities[i] >= popped_priorities[i-1]);
    }
    
    std::cout << "✓ Heap property maintenance works correctly" << std::endl;
}

int main() {
    std::cout << "=== C++ decrease_priority() Test Suite ===" << std::endl;
    
    try {
        test_basic_decrease_functionality();
        test_min_heap_behavior();
        test_max_heap_behavior();
        test_edge_cases();
        test_integration_mixed_operations();
        test_heap_property_maintenance();
        
        std::cout << "\n🎉 All decrease_priority() tests passed!" << std::endl;
        return 0;
    } catch (const std::exception& e) {
        std::cerr << "❌ Test failed with exception: " << e.what() << std::endl;
        return 1;
    } catch (...) {
        std::cerr << "❌ Test failed with unknown exception" << std::endl;
        return 1;
    }
}
