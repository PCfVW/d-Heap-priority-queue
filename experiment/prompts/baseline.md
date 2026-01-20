# Condition 1: Baseline Prompt (No Assistance)

## Purpose

This prompt provides only the minimal natural language specification required to understand the task. No documentation, type hints, or tests are provided.

---

## Prompt Template

Replace `{LANGUAGE}` with: Go, Rust, C++, TypeScript, or Zig

---

### Prompt Text

```
Implement a d-ary heap priority queue in {LANGUAGE}.

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

Provide a complete, working implementation.
```

---

## Language-Specific Variations

### Go
```
Implement a d-ary heap priority queue in Go.

Requirements:
1. The heap arity (d) should be configurable at construction time
2. Items have two distinct properties: an identity (for equality) and a priority (for ordering)
3. Two items are equal if they have the same identity, regardless of priority
4. The queue should support O(1) lookup to check if an item exists
5. Implement a min-heap where lower priority values have higher importance

Required operations:
- Insert(item): Add an item to the queue
- Pop(): Remove and return the item with highest priority (lowest value)
- Front(): Return the item with highest priority without removing it
- IncreasePriority(item): Update an existing item to have higher priority (lower value)
- DecreasePriority(item): Update an existing item to have lower priority (higher value)
- Contains(item): Check if an item with the given identity exists
- Len(): Return the number of items in the queue
- IsEmpty(): Return whether the queue is empty

Provide a complete, working implementation.
```

### Rust
```
Implement a d-ary heap priority queue in Rust.

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

Provide a complete, working implementation.
```

### C++
```
Implement a d-ary heap priority queue in C++17.

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

Provide a complete, working implementation as a header-only template class.
```

### TypeScript
```
Implement a d-ary heap priority queue in TypeScript.

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
- increasePriority(item): Update an existing item to have higher priority (lower value)
- decreasePriority(item): Update an existing item to have lower priority (higher value)
- contains(item): Check if an item with the given identity exists
- len(): Return the number of items in the queue
- isEmpty(): Return whether the queue is empty

Provide a complete, working implementation with proper TypeScript types.
```

### Zig
```
Implement a d-ary heap priority queue in Zig.

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
- increasePriority(item): Update an existing item to have higher priority (lower value)
- decreasePriority(item): Update an existing item to have lower priority (higher value)
- contains(item): Check if an item with the given identity exists
- len(): Return the number of items in the queue
- isEmpty(): Return whether the queue is empty

Provide a complete, working implementation.
```

---

## Notes

- This prompt intentionally omits:
  - Specific data structure guidance (hash map for O(1) lookup)
  - Parent/child index formulas for d-ary heap
  - Edge case handling specifications
  - Type definitions or signatures
  - Example usage or tests

- The goal is to measure what the model can produce from first principles with only high-level requirements.
