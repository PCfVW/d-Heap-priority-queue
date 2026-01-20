# Experimental Protocol: AI Code Generation for d-ary Heap Priority Queue

## Research Question

**What forms of structured assistance most improve AI-generated code correctness?**

We test the "Three-Level Hierarchy" hypothesis:
- **Level 1**: Compiler-enforced constraints (type systems)
- **Level 2**: Pattern-matchable templates (documentation, examples)
- **Level 3**: Executable feedback (tests)

## Experimental Design

### Independent Variable: Assistance Condition

| Condition | Name | Assistance Provided |
|-----------|------|---------------------|
| C1 | Baseline | Minimal natural language spec only |
| C2 | Doc-guided | Natural language spec + API documentation |
| C3 | Struct-guided | Natural language spec + type stubs/signatures |
| C4 | Test-guided | Natural language spec + test corpus |
| C5 | Combined | All of the above |

### Dependent Variables

1. **Test pass rate**: # tests passed / 22 total tests
2. **Error categories**: Classification of failures (see below)
3. **Compilation success**: Binary (compiles or not)
4. **Implementation completeness**: All 8 operations present (5 core + 3 utility)
5. **API divergence score**: How far the design deviates from the aligned API (0-24 scale)

### Languages Under Test

1. Go
2. Rust
3. C++
4. TypeScript
5. Zig

### Total Experiments

5 conditions × 5 languages = **25 experiments**

---

## Model Selection

### Primary Model

**Claude Sonnet (claude-sonnet-4-20250514)**

Rationale:
- Consistent behavior across runs
- Strong code generation capabilities
- Available via API for reproducibility

### Secondary Models (Optional Extension)

- Claude Opus (claude-opus-4-20250514) - for capability comparison
- Claude Haiku (claude-haiku-4-20250514) - for efficiency comparison

---

## Procedure

### Pre-Experiment Setup

1. Ensure test corpus passes against reference implementations (DONE - Phase 0.5)
2. Prepare all prompts (this phase)
3. Set up isolated environment for each generation

### Per-Experiment Steps

1. **Initialize**: Fresh conversation/session with model
2. **Prompt**: Send the condition-specific prompt for the target language
3. **Capture**: Save the complete generated code response
4. **Extract**: Isolate the implementation code into appropriate file(s)
5. **Compile**: Attempt to build/compile the code
6. **Test**: Run the 22-test corpus against the implementation
7. **Record**: Document results in standardized format

### Post-Experiment

1. Aggregate results across conditions and languages
2. Perform statistical analysis
3. Categorize and analyze errors
4. Draw conclusions about the Three-Level Hierarchy hypothesis

---

## Error Classification Taxonomy

### E1: Compilation/Parse Errors
- **E1.1**: Syntax errors
- **E1.2**: Type errors
- **E1.3**: Missing imports/dependencies
- **E1.4**: Undefined symbols

### E2: Runtime Errors
- **E2.1**: Null/nil pointer dereference
- **E2.2**: Index out of bounds
- **E2.3**: Stack overflow (infinite recursion)
- **E2.4**: Memory errors (use after free, leaks)

### E3: Logic Errors
- **E3.1**: Incorrect heap property maintenance
- **E3.2**: Wrong comparison direction (min vs max)
- **E3.3**: Off-by-one errors in indexing
- **E3.4**: Incorrect parent/child calculations for d-ary
- **E3.5**: Missing or incorrect position map updates
- **E3.6**: Identity vs priority confusion

### E4: API Errors
- **E4.1**: Missing required operation
- **E4.2**: Wrong function signature
- **E4.3**: Wrong return type
- **E4.4**: Missing error handling

### E5: Semantic Errors
- **E5.1**: Implemented binary heap instead of d-ary
- **E5.2**: No O(1) lookup capability
- **E5.3**: Incorrect identity/equality semantics

---

## API Divergence Dimensions

Measures how far the generated design deviates from the aligned API (documented in main README.md).
Each dimension scored 0-3: 0 = matches, 1 = minor variation, 2 = significant divergence, 3 = missing/incompatible.

| Dimension | What It Measures |
|-----------|------------------|
| **D1: Identity/Priority Separation** | Separate identity (equality) vs priority (ordering) |
| **D2: O(1) Lookup Mechanism** | Position map (hash map) for O(1) contains/updates |
| **D3: Comparator/Ordering Pattern** | Injected comparator vs hardcoded min/max |
| **D4: Generic Type Support** | Generic over T (item) and K (key) types |
| **D5: Arity Configuration** | Runtime-configurable d parameter |
| **D6: Priority Update Semantics** | Correct increase/decrease meaning + asymmetric optimization |
| **D7: Error Handling Pattern** | Language-idiomatic error handling |
| **D8: Core Operations Completeness** | All 8 required operations present |

**Total Score Range**: 0-24 (lower = closer to aligned design)

**Interpretation**:
- 0-4: Excellent alignment (model naturally arrived at similar design)
- 5-10: Good alignment (minor variations)
- 11-16: Moderate divergence (significant differences)
- 17-24: High divergence (fundamentally different approach)

---

## Prompt Guidelines

### Consistency Rules

1. **Same core requirement** across all conditions:
   > "Implement a d-ary heap priority queue with O(1) item lookup"

2. **Same operations required**:
   - `insert(item)` - Add item to queue
   - `pop()` - Remove and return highest priority item
   - `front()` - Peek at highest priority item without removal
   - `increase_priority(item)` - Increase item's priority (move up in min-heap)
   - `decrease_priority(item)` - Decrease item's priority (move down in min-heap)

3. **Same constraints**:
   - Configurable arity (d parameter)
   - Items have separate identity and priority
   - Identity determines equality; priority determines ordering
   - O(1) lookup by identity required

### What Varies by Condition

| Condition | Additional Content |
|-----------|-------------------|
| C1 Baseline | Nothing beyond core requirement |
| C2 Doc-guided | + Detailed API documentation with behavior specs |
| C3 Struct-guided | + Type definitions and function signatures |
| C4 Test-guided | + Complete test corpus (22 tests) |
| C5 Combined | + All of the above |

---

## Data Collection Format

Each experiment produces:

1. **`{condition}_{language}_prompt.md`** - Exact prompt sent
2. **`{condition}_{language}_response.md`** - Raw model response
3. **`{condition}_{language}_code.{ext}`** - Extracted implementation
4. **`{condition}_{language}_results.json`** - Structured test results

### Results JSON Schema

```json
{
  "experiment_id": "baseline_go",
  "condition": "baseline",
  "language": "go",
  "model": "claude-sonnet-4-20250514",
  "timestamp": "2025-01-19T15:00:00Z",
  "compilation": {
    "success": true,
    "errors": []
  },
  "tests": {
    "total": 22,
    "passed": 18,
    "failed": 4,
    "details": [
      {
        "name": "insert_postcondition_item_findable",
        "passed": true
      },
      {
        "name": "pop_invariant_maintains_heap_property",
        "passed": false,
        "error_category": "E3.1",
        "error_message": "heap property violated after pop"
      }
    ]
  },
  "completeness": {
    "insert": true,
    "pop": true,
    "front": true,
    "increase_priority": true,
    "decrease_priority": false
  },
  "notes": "Optional free-form observations"
}
```

---

## Statistical Analysis Plan

### Primary Analysis

1. **Pass rate by condition**: Mean and standard deviation across languages
2. **Pass rate by language**: Mean and standard deviation across conditions
3. **Interaction effects**: Condition × Language matrix
4. **API divergence by condition**: Mean divergence score across languages
5. **API divergence by dimension**: Which design aspects diverge most?

### Hypothesis Tests

- H1: Combined (C5) > Baseline (C1) - t-test or Wilcoxon
- H2: Any single assistance > Baseline - multiple comparison correction
- H3: Combined > any single assistance - assess synergy
- H4: API divergence correlates with test pass rate - correlation analysis
- H5: Struct-guided (C3) reduces divergence more than Doc-guided (C2) - tests Level 1 vs Level 2

### Visualization

1. Heatmap: Condition × Language pass rates
2. Bar chart: Mean pass rate by condition (with error bars)
3. Error category distribution by condition
4. Heatmap: Condition × Language API divergence scores
5. Radar chart: Divergence by dimension (D1-D8) per condition
6. Scatter plot: API divergence vs test pass rate

---

## Ethical Considerations

1. **Transparency**: All prompts and results will be published
2. **Reproducibility**: Exact model versions and parameters documented
3. **No cherry-picking**: All experiments run once; no retries for better results
4. **Limitations acknowledged**: Single-run experiments have variance

---

## Timeline

| Phase | Description | Status |
|-------|-------------|--------|
| 0.5 | Test corpus development | COMPLETE |
| 1.0 | Experimental protocol & prompts | COMPLETE |
| 1.1 | Baseline experiments (C1) | PENDING |
| 1.2 | Doc-guided experiments (C2) | PENDING |
| 1.3 | Struct-guided experiments (C3) | PENDING |
| 1.4 | Test-guided experiments (C4) | PENDING |
| 1.5 | Combined experiments (C5) | PENDING |
| 2.0 | Analysis & conclusions | PENDING |

---

## References

- Three-Level Hierarchy framework (mutualizing_roadmap_v3.md)
- Test corpus specifications (test-corpus/specifications/)
- Reference implementations (Go/, Rust/, Cpp/, TypeScript/, zig/)
