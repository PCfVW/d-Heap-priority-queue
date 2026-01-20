# Condition 4: Test-Guided Prompt (Test Corpus Assistance)

## Purpose

This prompt provides the core requirements plus the complete test corpus. This tests whether executable feedback (Level 3: tests) guides the model toward correct implementation.

---

## Prompt Template

Replace `{LANGUAGE}` with: Go, Rust, C++, TypeScript, or Zig
Replace `{TEST_CODE}` with the corresponding test file contents from test-corpus/

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

Your implementation must pass all of the following tests:

{TEST_CODE}

Provide a complete, working implementation that passes all tests.
```

---

## Test Code by Language

### Go Tests

Include all content from these files concatenated:
- test-corpus/go/insert_test.go
- test-corpus/go/pop_test.go
- test-corpus/go/front_test.go
- test-corpus/go/increase_priority_test.go
- test-corpus/go/decrease_priority_test.go

**Key test helper the implementation must support:**
```go
type Item struct {
    ID       string
    Priority int
}

// Implementation must be created via:
pq := dheap.New(dheap.Options[Item, string]{
    D:            d,
    Comparator:   dheap.MinBy(func(i Item) int { return i.Priority }),
    KeyExtractor: func(i Item) string { return i.ID },
})
```

---

### Rust Tests

Include all content from:
- test-corpus/rust/src/tests/mod.rs
- test-corpus/rust/src/tests/insert.rs
- test-corpus/rust/src/tests/pop.rs
- test-corpus/rust/src/tests/front.rs
- test-corpus/rust/src/tests/increase_priority.rs
- test-corpus/rust/src/tests/decrease_priority.rs

**Key test helper the implementation must support:**
```rust
#[derive(Debug, Clone)]
struct Item {
    id: String,
    priority: i32,
}

impl PartialEq for Item { fn eq(&self, other: &Self) -> bool { self.id == other.id } }
impl Eq for Item {}
impl Hash for Item { fn hash<H: Hasher>(&self, state: &mut H) { self.id.hash(state); } }

// Implementation must be created via:
let mut pq = PriorityQueue::new(d, MinBy(|i: &Item| i.priority));
```

---

### C++ Tests

Include all content from:
- test-corpus/cpp/test_common.h
- test-corpus/cpp/insert_test.cpp
- test-corpus/cpp/pop_test.cpp
- test-corpus/cpp/front_test.cpp
- test-corpus/cpp/increase_priority_test.cpp
- test-corpus/cpp/decrease_priority_test.cpp

**Key test helper the implementation must support:**
```cpp
struct Item {
    std::string id;
    int priority;
    Item(const std::string& id_, int priority_) : id(id_), priority(priority_) {}
    bool operator==(const Item& other) const { return id == other.id; }
};

struct ItemHash {
    std::size_t operator()(const Item& item) const {
        return std::hash<std::string>{}(item.id);
    }
};

struct ItemMinComparator {
    bool operator()(const Item& a, const Item& b) const {
        return a.priority < b.priority;
    }
};

// Type alias and usage (in namespace TOOLS):
using TestPriorityQueue = TOOLS::PriorityQueue<Item, ItemHash, ItemMinComparator>;
TestPriorityQueue* pq = new TestPriorityQueue(d);
```

---

### TypeScript Tests

Include all content from:
- test-corpus/typescript/insert.test.ts
- test-corpus/typescript/pop.test.ts
- test-corpus/typescript/front.test.ts
- test-corpus/typescript/increase_priority.test.ts
- test-corpus/typescript/decrease_priority.test.ts

**Key test helper the implementation must support:**
```typescript
interface Item {
    id: string;
    priority: number;
}

// Implementation must be created via:
const pq = new PriorityQueue<Item, string>({
    d,
    comparator: (a, b) => a.priority < b.priority,
    keyExtractor: (item) => item.id,
});
```

---

### Zig Tests

Include all content from:
- test-corpus/zig/src/corpus_tests.zig

**Key test helper the implementation must support:**
```zig
const d_heap = @import("d_heap");
const DHeapItem = d_heap.DHeapItem;
const MinByCost = d_heap.MinByCost;
const Item = d_heap.Item;

// Item has fields: number (identity) and cost (priority)
// Item.init(number, cost) creates an item
// MinByCost is a comparator: a.cost < b.cost

// Implementation must be created via:
var pq = try DHeapItem.init(d, MinByCost, allocator);
defer pq.deinit();
```

---

## Notes

- This prompt adds:
  - Complete test corpus (22 tests)
  - Implicit API through test usage patterns
  - Expected behavior through test assertions
  - Edge case handling requirements through edge tests

- The model must infer:
  - Exact API signatures from test usage
  - Internal data structure from test expectations
  - Error handling behavior from edge case tests

- This tests whether Level 3 assistance (executable feedback) is effective without explicit documentation or type definitions.

---

## Practical Considerations

When running this experiment:

1. **Test code length**: The full test corpus is substantial. Consider whether to:
   - Include all tests (comprehensive but long prompt)
   - Include representative subset (shorter but less complete signal)
   - Recommendation: Include all tests for scientific completeness

2. **API inference challenge**: The model must reverse-engineer the expected API from test usage. This is a realistic scenario (writing code to pass tests).

3. **Compilation context**: Test code includes imports and helpers that hint at the expected module structure.
