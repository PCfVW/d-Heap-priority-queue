# C++ API Conformance Analysis

Generated: 2026-01-19

## Reference API (from README.md)

The reference C++ implementation provides:
- Constructor: `PriorityQueue(size_t d)` throws on invalid arity
- Core: `insert`, `pop`, `front`, `increase_priority`, `decrease_priority`, `contains`, `len`, `is_empty`, `d`, `clear`, `to_string`
- Extended: `increase_priority(position)` overload, legacy `size()`, `empty()`, `getd()`, `put()`
- Type: `PriorityQueue<K, Hash, Equal>` header-only template
- Naming: `snake_case` convention
- Error handling: Assertions or throws

## Legend

- ✅ = Present and correct
- ⚠️ = Present but different signature/behavior
- ❌ = Missing

## Core Methods

| API Element | Reference | Struct-Claude | Struct-Mistral | Doc-Claude | Doc-Mistral |
|-------------|-----------|---------------|----------------|------------|-------------|
| **Constructor** | `PriorityQueue(d)` throws | ✅ | ✅ | ⚠️ `DaryHeap(d)` | ⚠️ `DaryHeap(d)` |
| **insert** | `insert(Item)` throws | ✅ | ✅ | ⚠️ `insert(T, priority)` | ⚠️ `insert(id, priority)` |
| **pop** | `pop() -> void` | ⚠️ `-> optional<Item>` | ⚠️ `-> optional<Item>` | ⚠️ `-> T` throws | ⚠️ `-> HeapItem` throws |
| **front** | `front() -> const T&` | ✅ throws | ✅ throws | ✅ throws | ✅ throws |
| **increase_priority** | `increase_priority(Item)` | ✅ | ✅ | ⚠️ `(T, priority)` | ⚠️ `(id, priority)` |
| **decrease_priority** | `decrease_priority(Item)` | ✅ | ✅ | ⚠️ `(T, priority)` | ⚠️ `(id, priority)` |
| **contains** | `contains(Item) -> bool` | ✅ | ✅ | ⚠️ `contains(T)` | ⚠️ `contains(id)` |
| **len** | `len() -> size_t` | ✅ | ✅ | ✅ | ⚠️ `size()` |
| **is_empty** | `is_empty() -> bool` | ✅ | ✅ | ✅ | ⚠️ `empty()` |
| **d()** | `d() -> size_t` | ❌ | ❌ | ❌ | ❌ |
| **clear()** | `clear(opt_d)` | ❌ | ❌ | ❌ | ❌ |
| **to_string()** | `to_string() -> string` | ❌ | ❌ | ❌ | ❌ |

## Extended Methods

| API Element | Reference | Struct-Claude | Struct-Mistral | Doc-Claude | Doc-Mistral |
|-------------|-----------|---------------|----------------|------------|-------------|
| **increase_priority(pos)** | Position overload | ❌ | ❌ | ❌ | ❌ |
| **size()** (legacy) | Alias for len | ❌ | ❌ | ❌ | ✅ |
| **empty()** (legacy) | Alias for is_empty | ❌ | ❌ | ❌ | ✅ |

## Type System

| API Element | Reference | Struct-Claude | Struct-Mistral | Doc-Claude | Doc-Mistral |
|-------------|-----------|---------------|----------------|------------|-------------|
| **Type Name** | `PriorityQueue` | ✅ | ✅ | ⚠️ `DaryHeap` | ⚠️ `DaryHeap` |
| **Template params** | `<K, Hash, Equal>` | ✅ | ✅ | ⚠️ `<T>` only | ⚠️ `<T, Identity, Priority>` |
| **Item struct** | `Item<K>` | ✅ | ✅ | ⚠️ Internal struct | ⚠️ `HeapItem` internal |
| **Header-only** | Yes | ✅ | ✅ | ✅ | ✅ |
| **C++17 features** | `std::optional` | ✅ | ✅ | ❌ | ❌ |

## Conformance Score (out of 17 checkpoints)

| Implementation | ✅ Correct | ⚠️ Partial | ❌ Missing | Score |
|----------------|-----------|-----------|-----------|-------|
| **Struct-Claude** | 10 | 1 | 6 | 10/17 (59%) |
| **Struct-Mistral** | 10 | 1 | 6 | 10/17 (59%) |
| **Doc-Claude** | 4 | 7 | 6 | 4/17 (24%) |
| **Doc-Mistral** | 3 | 8 | 6 | 3/17 (18%) |

## Key Observations

1. **Struct-guided strongly outperforms Doc-guided** (59% vs 18-24%)

2. **Type name alignment**: Struct-guided correctly uses `PriorityQueue`, Doc-guided uses `DaryHeap`

3. **Template structure**: Struct-guided correctly implements `<K, Hash, Equal>` template parameters matching the reference

4. **Item representation divergence**:
   - Struct-guided: External `Item<K>` struct (matches reference)
   - Doc-Claude: Internal `Item` struct with separate `(T, priority)` parameters
   - Doc-Mistral: Internal `HeapItem` with `(identity, priority)` parameters

5. **C++17 feature usage**:
   - Struct-guided uses `std::optional` for pop() (modern C++)
   - Doc-guided uses exceptions for empty pop (traditional approach)

6. **Struct-guided produces nearly identical code** between Claude and Mistral

7. **Doc-Mistral uses STL naming** (`size()`, `empty()`) instead of reference naming (`len()`, `is_empty()`)

8. **All missing same features**: `d()`, `clear()`, `to_string()`, position-based priority update

## Qualitative Differences

**Struct-guided advantages:**
- Correct type name and template parameters
- External `Item<K>` struct matching reference API
- Modern C++17 `std::optional` usage
- Highly consistent output between models

**Doc-guided characteristics:**
- More "traditional" C++ style (exceptions, internal structs)
- Separate parameters for item and priority
- Doc-Mistral uses STL-style naming conventions
- More variation between models
