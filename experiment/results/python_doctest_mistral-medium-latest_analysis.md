# Python Doctest Experiment Results - Mistral Medium Latest

## Configuration
- Model: mistral-medium-latest
- Temperature: 0
- Max tokens: 8192

## Results

| Metric | Value |
|--------|-------|
| Doctests in prompt | 50 |
| Doctests in output | 50 |
| Preservation ratio | **100%** |
| All tests passing | **Yes** |

## Token Usage
- Prompt tokens: 1,460
- Completion tokens: 2,139
- Total tokens: 3,599

## Interpretation

**100% PRESERVATION**: Mistral Medium preserves ALL doctests exactly as provided in the prompt.

This confirms that doctest preservation is **not Claude-specific** — it's a consistent behavior across:
- Claude Sonnet 4
- Claude Haiku/Sonnet 4.5
- Mistral Medium
- EssentialAI RNJ-1

## Code Quality Comparison

| Aspect | Mistral Medium | Claude Sonnet 4 | RNJ-1 |
|--------|----------------|-----------------|-------|
| Lines | 219 | 253 | 199 |
| Doctests | 50 | 50 | 50 |
| Type hints | Complete | Complete | Partial |
| Error handling | Explicit exceptions | Explicit exceptions | Silent returns |
| Helper methods | `_parent`, `_children`, `_bubble_up`, `_bubble_down` | Same | `_bubble_up`, `_bubble_down`, `_swap` |
| Arity validation | Yes (`d < 2` raises) | No | No |

## Conclusion

Mistral Medium achieves **100% doctest preservation** with a robust implementation that includes:
- Full type hints
- Explicit error handling
- Input validation (arity check)
- Clean helper method structure

The doctest preservation phenomenon is now confirmed across **4 different models** from **3 different providers**.
