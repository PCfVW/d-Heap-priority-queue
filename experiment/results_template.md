# Results Recording Template

## Experiment Identification

| Field | Value |
|-------|-------|
| Experiment ID | `{condition}_{language}` |
| Condition | C1-Baseline / C2-DocGuided / C3-StructGuided / C4-TestGuided / C5-Combined |
| Language | Go / Rust / C++ / TypeScript / Zig |
| Model | claude-sonnet-4-20250514 |
| Timestamp | YYYY-MM-DDTHH:MM:SSZ |
| Experimenter | |

---

## Prompt Delivery

- [ ] Fresh session/conversation started
- [ ] Exact prompt from prompts/{condition}.md used
- [ ] No additional hints or corrections given
- [ ] Complete response captured

**Prompt file**: `results/{condition}_{language}_prompt.md`
**Response file**: `results/{condition}_{language}_response.md`

---

## Compilation Results

| Metric | Result |
|--------|--------|
| Compiles successfully | Yes / No |
| Compilation errors | (count) |
| Warnings | (count) |

### Compilation Errors (if any)

| # | Error Category | Error Message | Location |
|---|----------------|---------------|----------|
| 1 | E1.x | | |
| 2 | | | |

**Error categories**: E1.1 Syntax, E1.2 Type, E1.3 Missing import, E1.4 Undefined symbol

---

## Test Results

| Metric | Result |
|--------|--------|
| Total tests | 22 |
| Passed | |
| Failed | |
| Pass rate | % |

### Test Details

#### insert() Tests (4 tests)

| Test Name | Pass | Error Category | Notes |
|-----------|------|----------------|-------|
| insert_postcondition_item_findable | | | |
| insert_invariant_heap_property | | | |
| insert_size_increments | | | |
| insert_edge_becomes_front_if_highest | | | |

#### pop() Tests (4 tests)

| Test Name | Pass | Error Category | Notes |
|-----------|------|----------------|-------|
| pop_postcondition_removes_minimum | | | |
| pop_invariant_maintains_heap_property | | | |
| pop_size_decrements | | | |
| pop_edge_empty_heap | | | |

#### front() Tests (4 tests)

| Test Name | Pass | Error Category | Notes |
|-----------|------|----------------|-------|
| front_postcondition_returns_minimum | | | |
| front_invariant_no_modification | | | |
| front_size_unchanged | | | |
| front_edge_empty_heap | | | |

#### increase_priority() Tests (5 tests)

| Test Name | Pass | Error Category | Notes |
|-----------|------|----------------|-------|
| increase_priority_postcondition_changed | | | |
| increase_priority_invariant_heap | | | |
| increase_priority_position_moves_up | | | |
| increase_priority_size_unchanged | | | |
| increase_priority_edge_nonexistent | | | |

#### decrease_priority() Tests (5 tests)

| Test Name | Pass | Error Category | Notes |
|-----------|------|----------------|-------|
| decrease_priority_postcondition_changed | | | |
| decrease_priority_invariant_heap | | | |
| decrease_priority_position_moves_down | | | |
| decrease_priority_size_unchanged | | | |
| decrease_priority_edge_nonexistent | | | |

---

## Implementation Completeness

| Operation | Present | Signature Correct | Notes |
|-----------|---------|-------------------|-------|
| insert | | | |
| pop | | | |
| front | | | |
| increase_priority | | | |
| decrease_priority | | | |
| contains | | | |
| len | | | |
| is_empty | | | |

---

## Error Analysis

### Error Distribution

| Category | Count | Description |
|----------|-------|-------------|
| E1 Compilation | | |
| E2 Runtime | | |
| E3 Logic | | |
| E4 API | | |
| E5 Semantic | | |

### Notable Errors

Describe the most significant errors and their likely causes:

1. **Error**:
   - **Category**:
   - **Likely cause**:
   - **Would docs help?**:
   - **Would types help?**:
   - **Would tests help?**:

2. **Error**:
   - **Category**:
   - **Likely cause**:
   - **Would docs help?**:
   - **Would types help?**:
   - **Would tests help?**:

---

## API Divergence Scorecard

This section measures how far the generated implementation diverges from the **aligned API design** documented in the main README.md. Lower scores indicate closer alignment with the reference design.

**Scoring**: 0 = Matches aligned design | 1 = Minor variation | 2 = Significant divergence | 3 = Missing/incompatible

### D1: Identity/Priority Separation

**Aligned Design**: Items have separate identity (for equality) and priority (for ordering). Two items with same identity but different priorities are equal.

| Score | Criteria |
|-------|----------|
| 0 | Separate identity and priority fields/concepts, equality based on identity only |
| 1 | Separate fields but equality includes priority, or uses callbacks for both |
| 2 | Single combined key, or priority is the identity |
| 3 | No clear distinction, or items are not comparable |

**Generated**: ___  **Score**: ___

**Notes**:

---

### D2: O(1) Lookup Mechanism

**Aligned Design**: Position map (hash map) tracking each item's index in the heap array. Enables O(1) contains() and O(1) lookup for priority updates.

| Score | Criteria |
|-------|----------|
| 0 | Hash map from identity to array index, updated on every heap operation |
| 1 | Hash map but not consistently updated, or maps to item instead of index |
| 2 | Linear search, or O(log n) tree-based lookup |
| 3 | No lookup capability, or contains() not implemented |

**Generated**: ___  **Score**: ___

**Notes**:

---

### D3: Comparator/Ordering Pattern

**Aligned Design**: Comparator injected at construction time (MinBy/MaxBy pattern). Supports both min-heap and max-heap via comparator choice.

| Score | Criteria |
|-------|----------|
| 0 | Comparator function/functor provided at construction, configurable min/max |
| 1 | Comparator as generic type parameter, or separate MinHeap/MaxHeap classes |
| 2 | Hardcoded comparison direction (min-only or max-only) |
| 3 | No comparison abstraction, or comparison logic scattered |

**Generated**: ___  **Score**: ___

**Notes**:

---

### D4: Generic Type Support

**Aligned Design**: Generic over item type T and key type K. Works with any user-defined item types.

| Score | Criteria |
|-------|----------|
| 0 | Fully generic with separate T (item) and K (key) type parameters |
| 1 | Generic over single type T with key extraction callback |
| 2 | Generic but requires specific traits/interfaces beyond Hash+Eq |
| 3 | Concrete types only, or uses `any`/`interface{}` without type safety |

**Generated**: ___  **Score**: ___

**Notes**:

---

### D5: Arity Configuration

**Aligned Design**: Arity (d) configurable at construction time as a runtime parameter. Validated to be >= 2.

| Score | Criteria |
|-------|----------|
| 0 | Runtime parameter at construction, validated, stored in struct |
| 1 | Compile-time constant or template parameter |
| 2 | Hardcoded value (e.g., always binary heap d=2) |
| 3 | No arity concept, or arity not configurable |

**Generated**: ___  **Score**: ___

**Notes**:

---

### D6: Priority Update Semantics

**Aligned Design**: `increase_priority()` = make MORE important (moves up in min-heap). `decrease_priority()` = make LESS important (moves down). Asymmetric: increase is optimized (up only), decrease is defensive (checks both directions).

| Score | Criteria |
|-------|----------|
| 0 | Correct semantics with asymmetric optimization |
| 1 | Correct semantics but symmetric (both check both directions) |
| 2 | Inverted semantics (increase = higher value), or single update_priority method |
| 3 | No priority update capability, or fundamentally broken |

**Generated**: ___  **Score**: ___

**Notes**:

---

### D7: Error Handling Pattern

**Aligned Design**: Language-idiomatic error handling. Returns errors/options for expected failures (empty pop, item not found). Panics/asserts for programmer errors (invalid arity).

| Score | Criteria |
|-------|----------|
| 0 | Idiomatic for language (errors in Go, Result in Rust, exceptions in TS, etc.) |
| 1 | Consistent but non-idiomatic (e.g., boolean returns everywhere) |
| 2 | Inconsistent error handling across operations |
| 3 | No error handling (silent failures, undefined behavior) |

**Generated**: ___  **Score**: ___

**Notes**:

---

### D8: Core Operations Completeness

**Aligned Design**: All 8 required operations: insert, pop, front, increase_priority, decrease_priority, contains, len, is_empty

| Score | Criteria |
|-------|----------|
| 0 | All 8 operations present with correct signatures |
| 1 | 7 operations (1 missing or wrong signature) |
| 2 | 5-6 operations present |
| 3 | Fewer than 5 operations, or core operations (insert/pop) missing |

**Generated**: ___  **Score**: ___

**Notes**:

---

### API Divergence Summary

| Dimension | Score (0-3) |
|-----------|-------------|
| D1: Identity/Priority Separation | |
| D2: O(1) Lookup Mechanism | |
| D3: Comparator/Ordering Pattern | |
| D4: Generic Type Support | |
| D5: Arity Configuration | |
| D6: Priority Update Semantics | |
| D7: Error Handling Pattern | |
| D8: Core Operations Completeness | |
| **Total Divergence Score** | **/24** |

**Divergence Interpretation**:
- **0-4**: Excellent alignment - model naturally arrived at similar design
- **5-10**: Good alignment - minor variations, likely still functional
- **11-16**: Moderate divergence - significant design differences, may need adaptation
- **17-24**: High divergence - fundamentally different approach

---

## Qualitative Observations

### Code Quality

- [ ] Idiomatic for the language
- [ ] Reasonable variable naming
- [ ] Appropriate error handling
- [ ] Efficient algorithm choices

### Interesting Patterns

(Note any interesting implementation choices, common mistakes, or unexpected approaches)

---

## JSON Export

Copy this structure and fill in for automated aggregation:

```json
{
  "experiment_id": "{condition}_{language}",
  "condition": "{condition}",
  "language": "{language}",
  "model": "claude-sonnet-4-20250514",
  "timestamp": "",
  "compilation": {
    "success": true,
    "error_count": 0,
    "errors": []
  },
  "tests": {
    "total": 22,
    "passed": 0,
    "failed": 0,
    "pass_rate": 0.0,
    "details": {
      "insert_postcondition_item_findable": {"passed": false, "error_category": null},
      "insert_invariant_heap_property": {"passed": false, "error_category": null},
      "insert_size_increments": {"passed": false, "error_category": null},
      "insert_edge_becomes_front_if_highest": {"passed": false, "error_category": null},
      "pop_postcondition_removes_minimum": {"passed": false, "error_category": null},
      "pop_invariant_maintains_heap_property": {"passed": false, "error_category": null},
      "pop_size_decrements": {"passed": false, "error_category": null},
      "pop_edge_empty_heap": {"passed": false, "error_category": null},
      "front_postcondition_returns_minimum": {"passed": false, "error_category": null},
      "front_invariant_no_modification": {"passed": false, "error_category": null},
      "front_size_unchanged": {"passed": false, "error_category": null},
      "front_edge_empty_heap": {"passed": false, "error_category": null},
      "increase_priority_postcondition_changed": {"passed": false, "error_category": null},
      "increase_priority_invariant_heap": {"passed": false, "error_category": null},
      "increase_priority_position_moves_up": {"passed": false, "error_category": null},
      "increase_priority_size_unchanged": {"passed": false, "error_category": null},
      "increase_priority_edge_nonexistent": {"passed": false, "error_category": null},
      "decrease_priority_postcondition_changed": {"passed": false, "error_category": null},
      "decrease_priority_invariant_heap": {"passed": false, "error_category": null},
      "decrease_priority_position_moves_down": {"passed": false, "error_category": null},
      "decrease_priority_size_unchanged": {"passed": false, "error_category": null},
      "decrease_priority_edge_nonexistent": {"passed": false, "error_category": null}
    }
  },
  "completeness": {
    "insert": false,
    "pop": false,
    "front": false,
    "increase_priority": false,
    "decrease_priority": false,
    "contains": false,
    "len": false,
    "is_empty": false
  },
  "error_distribution": {
    "E1_compilation": 0,
    "E2_runtime": 0,
    "E3_logic": 0,
    "E4_api": 0,
    "E5_semantic": 0
  },
  "api_divergence": {
    "D1_identity_priority_separation": 0,
    "D2_lookup_mechanism": 0,
    "D3_comparator_pattern": 0,
    "D4_generic_type_support": 0,
    "D5_arity_configuration": 0,
    "D6_priority_update_semantics": 0,
    "D7_error_handling_pattern": 0,
    "D8_operations_completeness": 0,
    "total_divergence": 0
  },
  "notes": ""
}
```

---

## Files Checklist

For this experiment, ensure these files are saved:

- [ ] `results/{condition}_{language}_prompt.md` - Exact prompt sent
- [ ] `results/{condition}_{language}_response.md` - Raw model response
- [ ] `results/{condition}_{language}_code.{ext}` - Extracted implementation
- [ ] `results/{condition}_{language}_results.md` - This completed template
- [ ] `results/{condition}_{language}_results.json` - JSON export above
