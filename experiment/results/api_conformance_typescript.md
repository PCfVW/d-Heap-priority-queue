# TypeScript API Conformance Analysis

Generated: 2026-01-19

## Reference API (from README.md)

The reference TypeScript implementation provides:
- Constructor: `PriorityQueue(d, keyFn, priorityFn, comparator)` throws on invalid arity
- Core: `insert`, `pop`, `front`, `increasePriority`, `decreasePriority`, `contains`, `len`, `isEmpty`, `d`, `clear`, `toString`
- Extended: `peek`, `containsKey`, `toArray`, `insertMany`, `popMany`, `size` property, `[Symbol.iterator]()`
- Type: `PriorityQueue<T, K>` with configurable key/priority extractors and comparator
- Naming: `camelCase` convention with `snake_case` aliases
- Error handling: Exceptions

## Legend

- ✅ = Present and correct
- ⚠️ = Present but different signature/behavior
- ❌ = Missing

## Core Methods

| API Element | Reference | Struct-Claude | Struct-Mistral | Doc-Claude | Doc-Mistral |
|-------------|-----------|---------------|----------------|------------|-------------|
| **Constructor** | `(d, keyFn, priorityFn, comparator)` | ✅ | ✅ | ⚠️ `(d)` only | ⚠️ `(d)` only |
| **insert** | `insert(T)` throws | ✅ | ✅ | ⚠️ `insert(PriorityItem)` | ⚠️ `insert(HeapItem)` |
| **pop** | `pop() -> T \| undefined` | ✅ | ✅ | ⚠️ `-> PriorityItem \| null` | ⚠️ `-> HeapItem \| null` |
| **front** | `front() -> T \| undefined` | ✅ | ✅ | ⚠️ `-> PriorityItem \| null` | ⚠️ `-> HeapItem \| null` |
| **increasePriority** | `increasePriority(T)` | ✅ | ✅ | ✅ | ✅ |
| **decreasePriority** | `decreasePriority(T)` | ✅ | ✅ | ✅ | ✅ |
| **contains** | `contains(T) -> boolean` | ✅ | ✅ | ✅ | ✅ |
| **len** | `len() -> number` | ✅ | ✅ | ✅ | ✅ |
| **isEmpty** | `isEmpty() -> boolean` | ✅ | ✅ | ✅ | ✅ |
| **d()** | `d() -> number` | ❌ | ❌ | ❌ | ❌ |
| **clear()** | `clear(newD?)` | ❌ | ❌ | ❌ | ❌ |
| **toString()** | `toString() -> string` | ❌ | ❌ | ❌ | ❌ |

## Extended Methods

| API Element | Reference | Struct-Claude | Struct-Mistral | Doc-Claude | Doc-Mistral |
|-------------|-----------|---------------|----------------|------------|-------------|
| **peek** | `peek() -> T \| undefined` | ❌ | ❌ | ❌ | ❌ |
| **containsKey** | `containsKey(K) -> boolean` | ❌ | ❌ | ❌ | ❌ |
| **toArray** | `toArray() -> T[]` | ❌ | ❌ | ❌ | ❌ |
| **insertMany** | `insertMany(T[])` | ❌ | ❌ | ❌ | ❌ |
| **popMany** | `popMany(n) -> T[]` | ❌ | ❌ | ❌ | ❌ |
| **size property** | `get size()` | ❌ | ❌ | ❌ | ❌ |
| **[Symbol.iterator]** | Iterable | ❌ | ❌ | ❌ | ❌ |

## Type System

| API Element | Reference | Struct-Claude | Struct-Mistral | Doc-Claude | Doc-Mistral |
|-------------|-----------|---------------|----------------|------------|-------------|
| **Type Name** | `PriorityQueue` | ✅ | ✅ | ⚠️ `DaryHeapPriorityQueue` | ⚠️ `DaryHeap` |
| **Generics** | `<T, K>` | ✅ | ✅ | ⚠️ `<T>` only | ❌ None (Identity type) |
| **keyFn** | Configurable | ✅ | ✅ | ❌ Fixed `.identity` | ❌ Fixed `.identity` |
| **priorityFn** | Configurable | ✅ | ✅ | ❌ Fixed `.priority` | ❌ Fixed `.priority` |
| **comparator** | Configurable | ✅ | ✅ | ❌ Hardcoded min | ❌ Hardcoded min |
| **Item interface** | User-defined T | ✅ Flexible | ✅ Flexible | ⚠️ `PriorityItem<T>` | ⚠️ `HeapItem` |

## Conformance Score (out of 19 checkpoints)

| Implementation | ✅ Correct | ⚠️ Partial | ❌ Missing | Score |
|----------------|-----------|-----------|-----------|-------|
| **Struct-Claude** | 12 | 0 | 7 | 12/19 (63%) |
| **Struct-Mistral** | 12 | 0 | 7 | 12/19 (63%) |
| **Doc-Claude** | 7 | 5 | 7 | 7/19 (37%) |
| **Doc-Mistral** | 6 | 5 | 8 | 6/19 (32%) |

## Key Observations

1. **Struct-guided strongly outperforms Doc-guided** (63% vs 32-37%)

2. **Constructor is the key differentiator**:
   - Struct-guided: Full `(d, keyFn, priorityFn, comparator)` matching reference exactly
   - Doc-guided: Simple `(d)` constructor with hardcoded item structure

3. **Type name alignment**:
   - Struct-guided: Correct `PriorityQueue<T, K>`
   - Doc-Claude: `DaryHeapPriorityQueue<T>` (verbose but descriptive)
   - Doc-Mistral: `DaryHeap` (too generic)

4. **Flexibility**:
   - Struct-guided: Fully configurable key/priority extraction and comparison
   - Doc-guided: Fixed `{identity, priority}` item structure

5. **Return type conventions**:
   - Struct-guided: Uses `undefined` (JavaScript idiomatic)
   - Doc-guided: Uses `null` (more traditional)

6. **Struct-guided produces identical API structure** between Claude and Mistral

7. **All missing same features**: `d()`, `clear()`, `toString()`, `peek()`, `containsKey()`, `toArray()`, bulk operations, `size` property, iterator

## Qualitative Differences

**Struct-guided advantages:**
- Full constructor with configurable functions (matches reference exactly)
- Correct type name
- Proper generic typing `<T, K>`
- Maximum flexibility for item types
- Consistent output between models

**Doc-guided characteristics:**
- Simpler API (fixed item structure)
- More opinionated (less configuration needed)
- Uses `null` instead of `undefined`
- Doc-Claude includes verification/debug methods
- More variation between models
