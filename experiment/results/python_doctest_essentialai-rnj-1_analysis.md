# Python Doctest Experiment Results - EssentialAI RNJ-1

## Configuration
- Model: essentialai/rnj-1 (via LM Studio)
- Temperature: 0
- Max tokens: 8192

## Results

| Metric | Value |
|--------|-------|
| Doctests in prompt | 50 |
| Doctests in output | 50 |
| Preservation ratio | **100%** |
| All tests passing | **Yes** |

## Method-by-Method Analysis

| Method | Doctests |
|--------|----------|
| Item (class) | 3 |
| Item.__eq__ | 2 |
| DHeap (class) | 3 |
| DHeap.__init__ | 2 |
| DHeap.insert | 4 |
| DHeap.pop | 7 |
| DHeap.front | 5 |
| DHeap.increase_priority | 6 |
| DHeap.decrease_priority | 6 |
| DHeap.contains | 4 |
| DHeap.__len__ | 4 |
| DHeap.is_empty | 4 |
| **Total** | **50** |

## Interpretation

**100% PRESERVATION**: RNJ-1 preserves ALL doctests exactly as provided in the prompt.

This is significant because:
1. RNJ-1 was previously excluded from the experiment because it only generated Python regardless of requested language
2. For Python specifically, RNJ-1 demonstrates the same 100% doctest preservation as Claude Sonnet/Haiku

## Code Quality

The generated implementation:
- ✅ Correctly implements d-ary heap with configurable arity
- ✅ Uses position_map dict for O(1) contains() lookup
- ✅ Properly implements _bubble_up and _bubble_down
- ✅ All 50 doctests pass
- ✅ Clean, readable code structure

## Raw Metrics
- Input tokens: 1,376
- Output tokens: 1,473
- Total tokens: 2,849

## Conclusion

RNJ-1 successfully generates Python code with **100% doctest preservation**, matching the behavior of Claude Sonnet/Haiku 4.5. While RNJ-1 cannot be used for multi-language experiments (it only outputs Python), it demonstrates that the doctest preservation phenomenon is not unique to Claude models.
