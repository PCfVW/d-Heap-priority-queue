# Python Doctest Experiment Results - Devstral 2512

## Configuration
- Model: devstral-2512 (Mistral's coding-focused model)
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
- Completion tokens: 2,017
- Total tokens: 3,477

## Interpretation

**100% PRESERVATION**: Devstral preserves ALL doctests exactly as provided in the prompt.

This confirms that doctest preservation is consistent across:
- Claude Sonnet 4 (Anthropic)
- Claude Haiku/Sonnet 4.5 (Anthropic)
- Mistral Medium (Mistral)
- **Devstral 2512** (Mistral - coding model)
- EssentialAI RNJ-1

## Comparison: Devstral vs Mistral Medium

Both Mistral models produce very similar code:

| Aspect | Devstral 2512 | Mistral Medium |
|--------|---------------|----------------|
| Lines | 218 | 219 |
| Doctests | 50 | 50 |
| Has `_swap` helper | ✅ Yes | ❌ No (inline swap) |
| Arity validation | ✅ Yes | ✅ Yes |
| Error handling | Explicit | Explicit |

**Key difference**: Devstral uses a separate `_swap` helper method, making the code slightly more modular.

## Conclusion

Devstral (Mistral's coding-focused model) achieves **100% doctest preservation** with a clean, well-structured implementation. The doctest preservation phenomenon is now confirmed across **5 different models** from **3 different providers**.
