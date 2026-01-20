# Rust API Conformance Analysis

Generated: 2026-01-19

## Reference API (from README.md)

The reference Rust implementation provides:
- Constructor: `new(d)` with panic on invalid arity
- Core: `insert`, `pop`, `front`, `increase_priority`, `decrease_priority`, `contains`, `len`, `is_empty`, `d`, `clear`, `to_string`
- Extended: `peek` (returns `Option<&T>`), `increase_priority_by_index`
- Type: `PriorityQueue<K: Hash + Eq + Clone>` with configurable comparator
- Naming: `snake_case` convention
- Error handling: Panic-based with descriptive messages

## Legend

- ✅ = Present and correct
- ⚠️ = Present but different signature/behavior
- ❌ = Missing

## Core Methods

| API Element | Reference | Struct-Claude | Struct-Mistral | Doc-Claude | Doc-Mistral |
|-------------|-----------|---------------|----------------|------------|-------------|
| **Constructor** | `new(d)` panics | ✅ `new(d)` panics | ✅ `new(d)` panics | ⚠️ `new(d) -> Result` | ⚠️ `new(d)` asserts |
| **insert** | `insert(T)` | ⚠️ `insert(Item<K>) -> Result` | ⚠️ `insert(Item<K>) -> Result` | ⚠️ `insert(T) -> Result` | ⚠️ `insert(T, P)` |
| **pop** | `pop() -> ()` | ⚠️ `pop() -> Option` | ⚠️ `pop() -> Option` | ⚠️ `pop() -> Result` | ✅ `pop() -> Option` |
| **front** | `front() -> &T` (panics) | ⚠️ `front() -> Option<&Item>` | ⚠️ `front() -> Option<&Item>` | ⚠️ `front() -> Result<&T>` | ⚠️ `front() -> Option<&(T,P)>` |
| **increase_priority** | `increase_priority(T)` | ✅ | ✅ | ✅ | ⚠️ `(&T, P)` separate args |
| **decrease_priority** | `decrease_priority(T)` | ✅ | ✅ | ✅ | ⚠️ `(&T, P)` separate args |
| **contains** | `contains(&T) -> bool` | ✅ | ✅ | ✅ | ✅ |
| **len** | `len() -> usize` | ✅ | ✅ | ✅ | ✅ |
| **is_empty** | `is_empty() -> bool` | ✅ | ✅ | ✅ | ✅ |
| **d()** | `d() -> usize` | ❌ | ❌ | ❌ | ❌ |
| **clear()** | `clear(opt_d)` | ❌ | ❌ | ❌ | ❌ |
| **to_string()** | `to_string() -> String` | ❌ | ❌ | ❌ | ❌ |

## Extended Methods

| API Element | Reference | Struct-Claude | Struct-Mistral | Doc-Claude | Doc-Mistral |
|-------------|-----------|---------------|----------------|------------|-------------|
| **peek** | `peek() -> Option<&T>` | ❌ | ❌ | ❌ | ❌ |
| **increase_priority_by_index** | Yes | ❌ | ❌ | ❌ | ❌ |

## Type System

| API Element | Reference | Struct-Claude | Struct-Mistral | Doc-Claude | Doc-Mistral |
|-------------|-----------|---------------|----------------|------------|-------------|
| **Type Name** | `PriorityQueue` | ✅ `PriorityQueue` | ✅ `PriorityQueue` | ⚠️ `DaryHeap` | ⚠️ `DaryHeap` |
| **Generics** | `<K: Hash+Eq+Clone>` | ✅ | ✅ | ⚠️ `<T: Priority>` trait | ⚠️ `<T, P>` two params |
| **Item Type** | User-defined T | ⚠️ Built-in `Item<K>` | ⚠️ Built-in `Item<K>` | ✅ User trait `Priority` | ✅ Tuple `(T, P)` |
| **Comparator** | Configurable | ❌ Hardcoded min | ❌ Hardcoded min | ❌ Via trait method | ❌ Via `Ord` |
| **Error Type** | Panics | ⚠️ `Result<(), &str>` | ⚠️ `Result<(), &str>` | ⚠️ Custom `HeapError` enum | ⚠️ Panics/asserts |

## Additional Features

| Feature | Struct-Claude | Struct-Mistral | Doc-Claude | Doc-Mistral |
|---------|---------------|----------------|------------|-------------|
| **Unit tests** | ✅ Yes | ❌ No | ✅ Yes | ✅ Yes |
| **main() example** | ❌ No | ❌ No | ✅ Yes | ❌ No |
| **Custom error type** | ❌ | ❌ | ✅ `HeapError` enum | ❌ |
| **Priority trait** | ❌ | ❌ | ✅ Custom trait | ❌ |

## Conformance Score (out of 17 checkpoints)

| Implementation | ✅ Correct | ⚠️ Partial | ❌ Missing | Score |
|----------------|-----------|-----------|-----------|-------|
| **Struct-Claude** | 8 | 4 | 5 | 8/17 (47%) |
| **Struct-Mistral** | 8 | 4 | 5 | 8/17 (47%) |
| **Doc-Claude** | 6 | 7 | 4 | 6/17 (35%) |
| **Doc-Mistral** | 5 | 6 | 6 | 5/17 (29%) |

## Key Observations

1. **Struct-guided scores higher (47%)** than Doc-guided (29-35%) for Rust

2. **Type name alignment**: Struct-guided correctly uses `PriorityQueue`, Doc-guided uses `DaryHeap`

3. **Generics preserved**: Struct-guided maintains `<K: Hash + Eq + Clone>` bounds correctly

4. **Error handling divergence**:
   - Reference uses panics
   - Struct-guided uses `Result<(), &'static str>`
   - Doc-Claude creates custom `HeapError` enum (more sophisticated)
   - Doc-Mistral uses asserts/panics (closest to reference)

5. **Doc-guided shows more creativity**:
   - Claude created a `Priority` trait for abstraction
   - Mistral used separate `(T, P)` tuple representation
   - Both approaches are valid but don't match reference API

6. **Struct-guided produced near-identical code** between Claude and Mistral - strong evidence that type signatures guide consistent output

7. **All missing the same core features**: `d()`, `clear()`, `to_string()`, `peek()`, `increase_priority_by_index()`

## Qualitative Differences

**Struct-guided advantages:**
- Correct type name (`PriorityQueue`)
- Consistent generic bounds
- Nearly identical output between models
- Closer to reference API structure

**Doc-guided advantages:**
- More sophisticated error handling (Claude's `HeapError` enum)
- Trait-based abstraction (Claude's `Priority` trait)
- More flexible item representation (Mistral's tuple approach)
- But: deviates significantly from reference API
