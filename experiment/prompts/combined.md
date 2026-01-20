# Condition 5: Combined Prompt (All Assistance)

## Purpose

This prompt provides all forms of assistance: documentation, type signatures, and test corpus. This represents the maximum support condition and tests whether combining all three levels produces better results than any single level.

---

## Prompt Template

Replace `{LANGUAGE}` with: Go, Rust, C++, TypeScript, or Zig
Replace `{TYPE_STUBS}` with type definitions extracted from the test corpus (see Assembly Instructions below)
Replace `{TEST_CODE}` with the test files from test_guided.md

---

### Prompt Text

```
Implement a d-ary heap priority queue in {LANGUAGE}.

## Overview

A d-ary heap is a generalization of a binary heap where each node has up to d children instead of 2. This implementation requires O(1) item lookup via a position map (hash map tracking each item's index in the heap array).

## Core Concepts

### Identity vs Priority
- **Identity**: Determines equality between items. Used as the key in the position map.
- **Priority**: Determines ordering in the heap. Lower values = higher priority (min-heap).
- Two items with the same identity but different priorities are considered equal.

### Heap Structure
- Array-based complete tree representation
- For a node at index i:
  - Parent index: (i - 1) / d
  - First child index: d * i + 1
  - Last child index: d * i + d (if it exists)
- Root is at index 0

### Position Map
- Hash map from item identity to array index
- Enables O(1) contains() and O(1) lookup for priority updates
- Must be kept synchronized with heap array at all times

## API Documentation

### Constructor
Create a new priority queue with the specified arity.
- **Parameter**: d (arity) - number of children per node, must be >= 2
- **Returns**: New empty priority queue

### insert(item)
Add an item to the queue.
- **Precondition**: Item with same identity must not already exist
- **Postcondition**: Item is in the queue and findable via contains()
- **Postcondition**: Heap property is maintained
- **Postcondition**: Size increases by 1
- **Algorithm**: Add to end of array, then sift up to restore heap property
- **Time complexity**: O(log_d n)

### pop()
Remove and return the item with highest priority (lowest priority value).
- **Precondition**: Queue is not empty
- **Postcondition**: Returned item is no longer in the queue
- **Postcondition**: Heap property is maintained
- **Postcondition**: Size decreases by 1
- **Algorithm**: Swap root with last element, remove last, sift down from root
- **Time complexity**: O(d * log_d n)
- **Edge case**: Return null/None/error if queue is empty

### front()
Return the item with highest priority without removing it.
- **Precondition**: Queue is not empty
- **Postcondition**: Queue is unchanged (same size, same items)
- **Returns**: Item at root (index 0)
- **Time complexity**: O(1)
- **Edge case**: Return null/None/error if queue is empty

### increase_priority(item)
Update an existing item to have higher priority (lower priority value).
- **Precondition**: Item with same identity must exist in queue
- **Input**: Item with the identity to find and the new (lower) priority value
- **Postcondition**: Item's priority is updated to the new value
- **Postcondition**: Heap property is maintained (item may move up)
- **Postcondition**: Size is unchanged
- **Algorithm**: Update priority at current position, then sift up
- **Time complexity**: O(log_d n)
- **Note**: "Increase priority" means making it MORE important (lower value in min-heap)

### decrease_priority(item)
Update an existing item to have lower priority (higher priority value).
- **Precondition**: Item with same identity must exist in queue
- **Input**: Item with the identity to find and the new (higher) priority value
- **Postcondition**: Item's priority is updated to the new value
- **Postcondition**: Heap property is maintained (item may move down)
- **Postcondition**: Size is unchanged
- **Algorithm**: Update priority at current position, then sift down
- **Time complexity**: O(d * log_d n)
- **Note**: "Decrease priority" means making it LESS important (higher value in min-heap)

### contains(item)
Check if an item with the given identity exists in the queue.
- **Returns**: true if item with same identity exists, false otherwise
- **Note**: Compares by identity only, not priority
- **Time complexity**: O(1) via position map lookup

### len()
Return the number of items in the queue.
- **Returns**: Non-negative integer count
- **Time complexity**: O(1)

### is_empty()
Return whether the queue contains no items.
- **Returns**: true if len() == 0, false otherwise
- **Time complexity**: O(1)

## Sift Operations

### sift_up(index)
Restore heap property by moving an item up toward the root.
- Compare item at index with its parent
- If item has higher priority (lower value) than parent, swap them
- Repeat until item is at root or parent has higher/equal priority
- Update position map after each swap

### sift_down(index)
Restore heap property by moving an item down toward the leaves.
- Find the child with highest priority (lowest value) among all children
- If that child has higher priority than the item, swap them
- Repeat until item has no children or no child has higher priority
- Update position map after each swap

## Type Definitions

{TYPE_STUBS}

## Test Corpus

Your implementation must pass all of the following tests:

{TEST_CODE}

Provide a complete, working implementation that satisfies the documentation, matches the type signatures, and passes all tests.
```

---

## Assembly Instructions

When creating the actual prompt for each language:

1. Copy the template above
2. Replace `{LANGUAGE}` with the target language name
3. Replace `{TYPE_STUBS}` with type definitions **extracted from the test corpus helper code** (see test_guided.md "Key test helper" sections for each language). This ensures compatibility between types and tests.
4. Replace `{TEST_CODE}` with the corresponding test files from the test-corpus directory

**Important**: The type stubs in struct_guided.md are idealized and simplified. For the Combined condition, use the actual types from the test corpus instead to avoid API conflicts.

---

## Expected Outcomes

This condition provides:
- **Level 1** (Compiler-enforced): Type definitions and signatures
- **Level 2** (Pattern-matchable): Detailed documentation with algorithms
- **Level 3** (Executable feedback): Complete test corpus

If the Three-Level Hierarchy hypothesis holds:
- Combined should have the highest pass rate
- The improvement should be greater than any single level alone
- Different languages may benefit differently based on their type system strength

---

## Notes

- This is the longest prompt due to combining all materials
- Context window limits may be a factor for some models
- The redundancy between docs, types, and tests provides cross-validation
- Conflicts between the three sources (if any) would be informative
