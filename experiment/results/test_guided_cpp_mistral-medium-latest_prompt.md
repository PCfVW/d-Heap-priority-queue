Implement a d-ary heap priority queue in C++.

Requirements:
1. The heap arity (d) should be configurable at construction time
2. Items have two distinct properties: an identity (for equality) and a priority (for ordering)
3. Two items are equal if they have the same identity, regardless of priority
4. The queue should support O(1) lookup to check if an item exists
5. Implement a min-heap where lower priority values have higher importance

Required operations:
- insert(item): Add an item to the queue
- pop(): Remove and return the item with highest priority (lowest value)
- front(): Return the item with highest priority without removing it
- increase_priority(item): Update an existing item to have higher priority (lower value)
- decrease_priority(item): Update an existing item to have lower priority (higher value)
- contains(item): Check if an item with the given identity exists
- len(): Return the number of items in the queue
- is_empty(): Return whether the queue is empty

Your implementation must pass all of the following tests:

// Test corpus for insert() operation
// Spec: specifications/insert.md
// Part of: Amphigraphic-Strict × d-ary Heap Priority Queue research

#include <gtest/gtest.h>
#include "test_common.h"

// =============================================================================
// insert() Tests
// =============================================================================

// Test: insert_postcondition_item_findable
// Spec: specifications/insert.md
// Property: inserted item can be found via contains() after insertion
TEST(InsertTest, Postcondition_ItemFindable) {
    auto pq = std::unique_ptr<TestPriorityQueue>(newItemMinHeap(4));

    Item item("test-item", 50);
    pq->insert(item);

    EXPECT_TRUE(pq->contains(item)) << "inserted item should be findable via contains()";

    // Also test with different priority but same ID
    Item sameId("test-item", 999);
    EXPECT_TRUE(pq->contains(sameId)) << "item with same ID should be found regardless of priority value";
}

// Test: insert_invariant_heap_property
// Spec: specifications/insert.md
// Property: heap invariant holds after insertion (front() returns minimum)
TEST(InsertTest, Invariant_HeapProperty) {
    auto pq = std::unique_ptr<TestPriorityQueue>(newItemMinHeap(4));

    // Insert items in arbitrary order
    std::vector<Item> items = {
        Item("a", 30),
        Item("b", 10),
        Item("c", 50),
        Item("d", 20),
        Item("e", 40),
    };

    for (const auto& item : items) {
        pq->insert(item);
        // After each insert, front() should be the item with lowest priority
        const Item& front = pq->front();
        EXPECT_LE(front.priority, 30) << "front should have lowest priority value";
    }

    // Final front should be item with priority 10
    EXPECT_EQ(pq->front().priority, 10);
}

// Test: insert_size_increments
// Spec: specifications/insert.md
// Property: heap size increases by 1 after each insertion
TEST(InsertTest, Size_Increments) {
    auto pq = std::unique_ptr<TestPriorityQueue>(newItemMinHeap(4));

    for (int i = 0; i < 5; i++) {
        size_t sizeBefore = pq->len();
        pq->insert(Item("item" + std::to_string(i), i * 10));
        size_t sizeAfter = pq->len();

        EXPECT_EQ(sizeAfter, sizeBefore + 1) << "size should increment by 1 after insert";
    }
}

// Test: insert_edge_becomes_front_if_highest_priority
// Spec: specifications/insert.md
// Property: if inserted item has highest priority, it becomes front()
TEST(InsertTest, Edge_BecomesFrontIfHighestPriority) {
    auto pq = std::unique_ptr<TestPriorityQueue>(newItemMinHeap(4));

    // Insert items with decreasing priority values (increasing importance in min-heap)
    pq->insert(Item("low", 100));
    pq->insert(Item("medium", 50));
    pq->insert(Item("high", 10));

    EXPECT_EQ(pq->front().id, "high") << "highest priority item should be at front";

    // Insert new highest priority item
    pq->insert(Item("urgent", 1));

    EXPECT_EQ(pq->front().id, "urgent") << "new highest priority item should become front";
}


// --- pop_test.cpp ---

// Test corpus for pop() operation
// Spec: specifications/pop.md
// Part of: Amphigraphic-Strict × d-ary Heap Priority Queue research

#include <gtest/gtest.h>
#include "test_common.h"

// =============================================================================
// pop() Tests
// =============================================================================

// Test: pop_postcondition_removes_minimum
// Spec: specifications/pop.md
// Property: pop() removes the item with lowest priority value (min-heap)
// Note: C++ pop() doesn't return; use front() before pop() to get value
TEST(PopTest, Postcondition_RemovesMinimum) {
    auto pq = std::unique_ptr<TestPriorityQueue>(newItemMinHeap(4));

    pq->insert(Item("a", 30));
    pq->insert(Item("b", 10));
    pq->insert(Item("c", 20));

    // Get front before popping
    const Item& front = pq->front();
    EXPECT_EQ(front.priority, 10) << "front should be minimum priority";
    EXPECT_EQ(front.id, "b");

    pq->pop();

    // After pop, the removed item should not be in heap
    EXPECT_FALSE(pq->contains(Item("b", 0))) << "popped item should not be in heap";
}

// Test: pop_invariant_maintains_heap_property
// Spec: specifications/pop.md
// Property: after pop(), heap invariant holds (front() is minimum of remaining)
TEST(PopTest, Invariant_MaintainsHeapProperty) {
    auto pq = std::unique_ptr<TestPriorityQueue>(newItemMinHeap(4));

    // Insert multiple items
    std::vector<Item> items = {
        Item("a", 50),
        Item("b", 20),
        Item("c", 80),
        Item("d", 10),
        Item("e", 60),
        Item("f", 30),
        Item("g", 70),
        Item("h", 40),
    };

    for (const auto& item : items) {
        pq->insert(item);
    }

    // Pop half and verify front() is always minimum of remaining
    std::vector<int> expectedOrder = {10, 20, 30, 40};
    for (int expected : expectedOrder) {
        EXPECT_EQ(pq->front().priority, expected) << "front should be minimum";
        pq->pop();
    }
}

// Test: pop_size_decrements
// Spec: specifications/pop.md
// Property: size() decreases by 1 after successful pop()
TEST(PopTest, Size_Decrements) {
    auto pq = std::unique_ptr<TestPriorityQueue>(newItemMinHeap(4));

    pq->insert(Item("a", 10));
    pq->insert(Item("b", 20));
    pq->insert(Item("c", 30));

    for (int expectedSize = 2; expectedSize >= 0; expectedSize--) {
        size_t sizeBefore = pq->len();
        pq->pop();
        size_t sizeAfter = pq->len();

        EXPECT_EQ(sizeAfter, sizeBefore - 1) << "size should decrement by 1 after pop";
        EXPECT_EQ(sizeAfter, static_cast<size_t>(expectedSize)) << "size should match expected";
    }
}

// Test: pop_edge_empty_behavior
// Spec: specifications/pop.md
// Property: pop() on empty heap behavior (C++ has UB, but we test non-empty first)
// Note: C++ implementation doesn't check for empty - caller must check is_empty() first
TEST(PopTest, Edge_EmptyStartsEmpty) {
    auto pq = std::unique_ptr<TestPriorityQueue>(newItemMinHeap(4));

    // Verify starting empty
    EXPECT_TRUE(pq->is_empty()) << "heap should start empty";

    // Note: Calling pop() on empty heap is undefined behavior in C++ implementation
    // We don't test it directly to avoid UB
}


// --- front_test.cpp ---

// Test corpus for front() operation
// Spec: specifications/front.md
// Part of: Amphigraphic-Strict × d-ary Heap Priority Queue research

#include <gtest/gtest.h>
#include "test_common.h"

// =============================================================================
// front() Tests
// =============================================================================

// Test: front_postcondition_returns_minimum
// Spec: specifications/front.md
// Property: front() returns the item with lowest priority value (min-heap) without removal
TEST(FrontTest, Postcondition_ReturnsMinimum) {
    auto pq = std::unique_ptr<TestPriorityQueue>(newItemMinHeap(4));

    pq->insert(Item("a", 30));
    pq->insert(Item("b", 10));
    pq->insert(Item("c", 20));

    const Item& front = pq->front();
    EXPECT_EQ(front.priority, 10) << "front should return minimum priority";
    EXPECT_EQ(front.id, "b") << "front should return item with id 'b'";
}

// Test: front_invariant_no_modification
// Spec: specifications/front.md
// Property: front() does not modify the heap (calling multiple times returns same result)
TEST(FrontTest, Invariant_NoModification) {
    auto pq = std::unique_ptr<TestPriorityQueue>(newItemMinHeap(4));

    pq->insert(Item("a", 30));
    pq->insert(Item("b", 10));
    pq->insert(Item("c", 20));

    // Call front() multiple times
    const Item& first = pq->front();
    const Item& second = pq->front();
    const Item& third = pq->front();

    // All should return the same item
    EXPECT_EQ(first.id, second.id) << "front() should return same item";
    EXPECT_EQ(second.id, third.id) << "front() should return same item";
}

// Test: front_size_unchanged
// Spec: specifications/front.md
// Property: size() remains the same after front()
TEST(FrontTest, Size_Unchanged) {
    auto pq = std::unique_ptr<TestPriorityQueue>(newItemMinHeap(4));

    pq->insert(Item("a", 10));
    pq->insert(Item("b", 20));
    pq->insert(Item("c", 30));

    size_t sizeBefore = pq->len();

    // Call front() multiple times
    for (int i = 0; i < 5; i++) {
        pq->front();
    }

    size_t sizeAfter = pq->len();

    EXPECT_EQ(sizeAfter, sizeBefore) << "size should be unchanged after front()";
}

// Test: front_edge_empty_behavior
// Spec: specifications/front.md
// Property: front() on empty heap is UB in C++ (caller must check is_empty())
// Note: We don't test calling front() on empty as it's undefined behavior
TEST(FrontTest, Edge_EmptyCheck) {
    auto pq = std::unique_ptr<TestPriorityQueue>(newItemMinHeap(4));

    // Verify the heap starts empty
    EXPECT_TRUE(pq->is_empty()) << "heap should start empty";

    // Proper pattern: check is_empty() before calling front()
    // NEVER call front() on empty heap - it's UB
}


// --- increase_priority_test.cpp ---

// Test corpus for increase_priority() operation
// Spec: specifications/increase_priority.md
// Part of: Amphigraphic-Strict × d-ary Heap Priority Queue research

#include <gtest/gtest.h>
#include "test_common.h"

// =============================================================================
// increase_priority() Tests
// =============================================================================

// Test: increase_priority_postcondition_priority_changed
// Spec: specifications/increase_priority.md
// Property: item's priority is updated to the new value
TEST(IncreasePriorityTest, Postcondition_PriorityChanged) {
    auto pq = std::unique_ptr<TestPriorityQueue>(newItemMinHeap(4));

    pq->insert(Item("target", 50));
    pq->insert(Item("other", 30));

    // "other" starts at front with priority 30
    EXPECT_EQ(pq->front().id, "other");

    // Increase priority of "target" (in min-heap: lower value = higher priority)
    Item updated("target", 10);
    pq->increase_priority(updated);

    // "target" should now be at front (highest priority)
    EXPECT_EQ(pq->front().id, "target") << "target should be at front after priority increase";
    EXPECT_EQ(pq->front().priority, 10) << "priority should be updated to 10";
}

// Test: increase_priority_invariant_heap_property
// Spec: specifications/increase_priority.md
// Property: heap invariant holds after priority increase
TEST(IncreasePriorityTest, Invariant_HeapProperty) {
    auto pq = std::unique_ptr<TestPriorityQueue>(newItemMinHeap(4));

    // Create a larger heap
    std::vector<Item> items = {
        Item("a", 80),
        Item("b", 60),
        Item("c", 40),
        Item("d", 20),
        Item("e", 100),
        Item("f", 50),
    };

    for (const auto& item : items) {
        pq->insert(item);
    }

    // "d" with priority 20 should be at front
    EXPECT_EQ(pq->front().priority, 20);

    // Increase priority of "a" (80 -> 5)
    Item updated("a", 5);
    pq->increase_priority(updated);

    // Now "a" should be at front
    EXPECT_EQ(pq->front().id, "a") << "item with increased priority should be at front";
    EXPECT_EQ(pq->front().priority, 5);
}

// Test: increase_priority_position_item_moves_up
// Spec: specifications/increase_priority.md
// Property: item moves toward root after priority increase (becomes front if highest)
TEST(IncreasePriorityTest, Position_ItemMovesUp) {
    auto pq = std::unique_ptr<TestPriorityQueue>(newItemMinHeap(4));

    pq->insert(Item("root", 10));
    pq->insert(Item("middle", 50));
    pq->insert(Item("leaf", 100));

    // "leaf" should not be at front initially
    EXPECT_NE(pq->front().id, "leaf");

    // Increase priority of "leaf" to become highest priority
    Item updated("leaf", 1);
    pq->increase_priority(updated);

    // "leaf" should now be at front
    EXPECT_EQ(pq->front().id, "leaf") << "item should move to front after priority increase";
}

// Test: increase_priority_size_unchanged
// Spec: specifications/increase_priority.md
// Property: size() remains unchanged after priority update
TEST(IncreasePriorityTest, Size_Unchanged) {
    auto pq = std::unique_ptr<TestPriorityQueue>(newItemMinHeap(4));

    pq->insert(Item("a", 50));
    pq->insert(Item("b", 30));
    pq->insert(Item("c", 70));

    size_t sizeBefore = pq->len();

    Item updated("c", 10);
    pq->increase_priority(updated);

    size_t sizeAfter = pq->len();

    EXPECT_EQ(sizeAfter, sizeBefore) << "size should be unchanged after priority update";
}

// Test: increase_priority_edge_not_found_asserts
// Spec: specifications/increase_priority.md
// Property: asserts if item not in heap (C++ uses assert)
// Note: Testing this would trigger an assert, so we just verify item exists before updating
TEST(IncreasePriorityTest, Edge_ItemMustExist) {
    auto pq = std::unique_ptr<TestPriorityQueue>(newItemMinHeap(4));

    pq->insert(Item("existing", 50));

    // Verify item exists before attempting update
    EXPECT_TRUE(pq->contains(Item("existing", 0))) << "item must exist before increase_priority";

    // Note: Calling increase_priority on non-existent item triggers assert
    // We don't test it directly to avoid terminating the test
}


// --- decrease_priority_test.cpp ---

// Test corpus for decrease_priority() operation
// Spec: specifications/decrease_priority.md
// Part of: Amphigraphic-Strict × d-ary Heap Priority Queue research

#include <gtest/gtest.h>
#include "test_common.h"

// =============================================================================
// decrease_priority() Tests
// =============================================================================

// Test: decrease_priority_postcondition_priority_changed
// Spec: specifications/decrease_priority.md
// Property: item's priority is updated to the new value
TEST(DecreasePriorityTest, Postcondition_PriorityChanged) {
    auto pq = std::unique_ptr<TestPriorityQueue>(newItemMinHeap(4));

    pq->insert(Item("target", 10));
    pq->insert(Item("other", 30));

    // "target" starts at front with priority 10
    EXPECT_EQ(pq->front().id, "target");

    // Decrease priority of "target" (in min-heap: higher value = lower priority)
    Item updated("target", 50);
    pq->decrease_priority(updated);

    // "other" should now be at front (it has higher priority now)
    EXPECT_EQ(pq->front().id, "other") << "other should be at front after target's priority decrease";

    // Pop "other" and verify "target" has updated priority
    pq->pop();
    EXPECT_EQ(pq->front().priority, 50) << "target's priority should be updated to 50";
}

// Test: decrease_priority_invariant_heap_property
// Spec: specifications/decrease_priority.md
// Property: heap invariant holds after priority decrease
TEST(DecreasePriorityTest, Invariant_HeapProperty) {
    auto pq = std::unique_ptr<TestPriorityQueue>(newItemMinHeap(4));

    // Create a larger heap
    std::vector<Item> items = {
        Item("a", 10),
        Item("b", 30),
        Item("c", 50),
        Item("d", 70),
        Item("e", 20),
        Item("f", 40),
    };

    for (const auto& item : items) {
        pq->insert(item);
    }

    // "a" with priority 10 should be at front
    EXPECT_EQ(pq->front().id, "a");

    // Decrease priority of "a" (10 -> 100)
    Item updated("a", 100);
    pq->decrease_priority(updated);

    // "e" with priority 20 should now be at front
    EXPECT_EQ(pq->front().priority, 20) << "new minimum should be at front";
}

// Test: decrease_priority_position_item_moves_down
// Spec: specifications/decrease_priority.md
// Property: item moves toward leaves after priority decrease (no longer front if was)
TEST(DecreasePriorityTest, Position_ItemMovesDown) {
    auto pq = std::unique_ptr<TestPriorityQueue>(newItemMinHeap(4));

    pq->insert(Item("root", 10));
    pq->insert(Item("child1", 50));
    pq->insert(Item("child2", 60));
    pq->insert(Item("child3", 70));

    // "root" is at front
    EXPECT_EQ(pq->front().id, "root");

    // Decrease priority of "root" to become lowest priority
    Item updated("root", 100);
    pq->decrease_priority(updated);

    // "root" should no longer be at front
    EXPECT_NE(pq->front().id, "root") << "item should move down after priority decrease";
    EXPECT_EQ(pq->front().id, "child1") << "child1 should become new front";
}

// Test: decrease_priority_size_unchanged
// Spec: specifications/decrease_priority.md
// Property: size() remains unchanged after priority update
TEST(DecreasePriorityTest, Size_Unchanged) {
    auto pq = std::unique_ptr<TestPriorityQueue>(newItemMinHeap(4));

    pq->insert(Item("a", 10));
    pq->insert(Item("b", 30));
    pq->insert(Item("c", 50));

    size_t sizeBefore = pq->len();

    Item updated("a", 100);
    pq->decrease_priority(updated);

    size_t sizeAfter = pq->len();

    EXPECT_EQ(sizeAfter, sizeBefore) << "size should be unchanged after priority update";
}

// Test: decrease_priority_edge_not_found_asserts
// Spec: specifications/decrease_priority.md
// Property: asserts if item not in heap (C++ uses assert)
// Note: Testing this would trigger an assert, so we just verify item exists before updating
TEST(DecreasePriorityTest, Edge_ItemMustExist) {
    auto pq = std::unique_ptr<TestPriorityQueue>(newItemMinHeap(4));

    pq->insert(Item("existing", 50));

    // Verify item exists before attempting update
    EXPECT_TRUE(pq->contains(Item("existing", 0))) << "item must exist before decrease_priority";

    // Note: Calling decrease_priority on non-existent item triggers assert
    // We don't test it directly to avoid terminating the test
}


Provide a complete, working implementation that passes all tests.