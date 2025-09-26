// test_decrease_priority.rs
//
// Comprehensive test suite for decrease_priority() method in Rust d-ary heap priority queue
//
// Copyright (c) 2023-2025 Eric Jacopin
//
// Licensed under the Apache License, Version 2.0 (the "License")

use priority_queue::{PriorityQueue, MinBy, MaxBy};

#[derive(Clone, Debug)]
struct TestItem {
    id: i32,
    priority: i32,
}

impl TestItem {
    fn new(id: i32, priority: i32) -> Self {
        Self { id, priority }
    }
}

// Implement Eq and PartialEq based only on id (identity)
impl PartialEq for TestItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for TestItem {}

// Implement Hash based only on id (identity)
impl std::hash::Hash for TestItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl std::fmt::Display for TestItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TestItem(id: {}, priority: {})", self.id, self.priority)
    }
}

#[test]
fn test_basic_decrease_functionality() {
    let mut pq: PriorityQueue<TestItem, MinBy<_>> = 
        PriorityQueue::new(3, MinBy(|x: &TestItem| x.priority));
    
    // Insert items
    pq.insert(TestItem::new(1, 10));
    pq.insert(TestItem::new(2, 5));
    pq.insert(TestItem::new(3, 15));
    
    // Verify initial state (min-heap: 5 should be at front)
    assert_eq!(pq.front().priority, 5);
    assert_eq!(pq.len(), 3);
    
    // Decrease priority of item 3 (15 -> 3, should become new front)
    let updated_item = TestItem::new(3, 3);
    pq.decrease_priority(&updated_item);
    
    // Verify item 3 is now at front
    assert_eq!(pq.front().id, 3);
    assert_eq!(pq.front().priority, 3);
    assert_eq!(pq.len(), 3);
}

#[test]
fn test_min_heap_behavior() {
    let mut pq: PriorityQueue<TestItem, MinBy<_>> = 
        PriorityQueue::new(2, MinBy(|x: &TestItem| x.priority));
    
    // Insert items in min-heap
    pq.insert(TestItem::new(1, 20));
    pq.insert(TestItem::new(2, 10));
    pq.insert(TestItem::new(3, 30));
    pq.insert(TestItem::new(4, 15));
    
    // Initial front should be item 2 (priority 10)
    assert_eq!(pq.front().id, 2);
    
    // Decrease priority of item 1 (20 -> 5, should become new front)
    let updated_item1 = TestItem::new(1, 5);
    pq.decrease_priority(&updated_item1);
    assert_eq!(pq.front().id, 1);
    assert_eq!(pq.front().priority, 5);
    
    // Decrease priority of item 3 (30 -> 25, should not affect front)
    let updated_item3 = TestItem::new(3, 25);
    pq.decrease_priority(&updated_item3);
    assert_eq!(pq.front().id, 1);  // Still item 1 at front
}

#[test]
fn test_max_heap_behavior() {
    let mut pq: PriorityQueue<TestItem, MaxBy<_>> = 
        PriorityQueue::new(2, MaxBy(|x: &TestItem| x.priority));
    
    // Insert items in max-heap
    pq.insert(TestItem::new(1, 10));
    pq.insert(TestItem::new(2, 20));
    pq.insert(TestItem::new(3, 5));
    pq.insert(TestItem::new(4, 15));
    
    // Initial front should be item 2 (priority 20)
    assert_eq!(pq.front().id, 2);
    
    // Decrease priority of item 2 (20 -> 8, should no longer be front)
    let updated_item2 = TestItem::new(2, 8);
    pq.decrease_priority(&updated_item2);
    assert_eq!(pq.front().id, 4);  // Item 4 (priority 15) should now be front
}

#[test]
fn test_edge_cases() {
    let mut pq: PriorityQueue<TestItem, MinBy<_>> = 
        PriorityQueue::new(3, MinBy(|x: &TestItem| x.priority));
    
    // Test single item
    pq.insert(TestItem::new(1, 10));
    let updated_single = TestItem::new(1, 5);
    pq.decrease_priority(&updated_single);
    assert_eq!(pq.front().priority, 5);
    assert_eq!(pq.len(), 1);
}

#[test]
#[should_panic(expected = "item must exist in the queue to decrease priority")]
fn test_item_not_found() {
    let mut pq: PriorityQueue<TestItem, MinBy<_>> = 
        PriorityQueue::new(3, MinBy(|x: &TestItem| x.priority));
    
    pq.insert(TestItem::new(1, 10));
    
    // Try to decrease priority of non-existent item
    let non_existent = TestItem::new(999, 5);
    pq.decrease_priority(&non_existent);
}

#[test]
fn test_integration_mixed_operations() {
    let mut pq: PriorityQueue<TestItem, MinBy<_>> = 
        PriorityQueue::new(3, MinBy(|x: &TestItem| x.priority));
    
    // Complex sequence of operations
    pq.insert(TestItem::new(1, 50));
    pq.insert(TestItem::new(2, 30));
    pq.insert(TestItem::new(3, 70));
    pq.insert(TestItem::new(4, 20));
    pq.insert(TestItem::new(5, 60));
    
    // Initial front should be item 4 (priority 20)
    assert_eq!(pq.front().id, 4);
    
    // Increase priority of item 1 (50 -> 10, should become new front)
    let increased_item1 = TestItem::new(1, 10);
    pq.increase_priority(&increased_item1);
    assert_eq!(pq.front().id, 1);
    
    // Decrease priority of item 2 (30 -> 40)
    let decreased_item2 = TestItem::new(2, 40);
    pq.decrease_priority(&decreased_item2);
    assert_eq!(pq.front().id, 1);  // Still item 1 at front
    
    // Pop front item
    pq.pop();
    assert_eq!(pq.front().id, 4);  // Item 4 (priority 20) should now be front
    
    // Decrease priority of current front (20 -> 45, should make item 2 the new front)
    let decreased_item4 = TestItem::new(4, 45);
    pq.decrease_priority(&decreased_item4);
    assert_eq!(pq.front().id, 2);  // Item 2 (priority 40) should now be front
}

#[test]
fn test_heap_property_maintenance() {
    let mut pq: PriorityQueue<TestItem, MinBy<_>> = 
        PriorityQueue::new(2, MinBy(|x: &TestItem| x.priority));
    
    // Insert many items
    let priorities = vec![50, 30, 70, 20, 60, 10, 80, 40];
    for (i, &priority) in priorities.iter().enumerate() {
        pq.insert(TestItem::new((i + 1) as i32, priority));
    }
    
    // Perform several decrease operations
    pq.decrease_priority(&TestItem::new(1, 55));  // 50 -> 55
    pq.decrease_priority(&TestItem::new(6, 15));  // 10 -> 15
    pq.decrease_priority(&TestItem::new(3, 75));  // 70 -> 75
    
    // Verify heap property by popping all items in order
    let mut popped_priorities = Vec::new();
    while !pq.is_empty() {
        popped_priorities.push(pq.front().priority);
        pq.pop();
    }
    
    // Verify non-decreasing order (min-heap property)
    for i in 1..popped_priorities.len() {
        assert!(popped_priorities[i] >= popped_priorities[i-1]);
    }
}

#[test]
fn test_decrease_priority_with_to_string() {
    let mut pq: PriorityQueue<TestItem, MinBy<_>> = 
        PriorityQueue::new(2, MinBy(|x: &TestItem| x.priority));
    
    pq.insert(TestItem::new(1, 10));
    pq.insert(TestItem::new(2, 5));
    
    // Test to_string works with decreased priority items
    let output = pq.to_string();
    assert!(output.contains("TestItem"));
    
    // Decrease priority and test again
    pq.decrease_priority(&TestItem::new(1, 3));
    let output_after = pq.to_string();
    assert!(output_after.contains("TestItem"));
    assert_eq!(pq.front().id, 1);
    assert_eq!(pq.front().priority, 3);
}
