# Zig API Conformance Analysis

Generated: 2026-01-19

## Reference API (from README.md)

The reference Zig implementation provides:
- Constructor: `init(d, allocator)` returns error on invalid arity, `initCapacity(d, capacity, allocator)`
- Destructor: `deinit()`
- Core: `insert`, `pop`, `front`, `increasePriority`, `decreasePriority`, `contains`, `len`, `isEmpty`, `d`, `clear`, `toString`/`to_string`
- Extended: `peek` (alias for front), `insertMany`, `popMany`, `toArray`
- Type: `DHeap(T, Context, Comparator)` with comptime generics
- Item: `Item` struct with `number` (identity) and `cost` (priority)
- Naming: `camelCase` convention with `snake_case` aliases
- Error handling: Error unions (`!void`, `error.ItemNotFound`)

## Legend

- ✅ = Present and correct
- ⚠️ = Present but different signature/behavior
- ❌ = Missing

## Core Methods

| API Element | Reference | Struct-Claude | Struct-Mistral | Doc-Claude | Doc-Mistral |
|-------------|-----------|---------------|----------------|------------|-------------|
| **init** | `init(d, allocator) !Self` | ⚠️ No arity check | ✅ | ✅ | ✅ |
| **initCapacity** | `initCapacity(d, cap, alloc)` | ❌ | ❌ | ❌ | ❌ |
| **deinit** | `deinit()` | ✅ | ✅ | ✅ | ✅ |
| **insert** | `insert(Item) !void` | ✅ | ✅ | ⚠️ `insert(T, i32)` | ⚠️ `insert([]const u8)` |
| **pop** | `pop() !?Item` | ✅ | ✅ | ⚠️ `pop() Error!Item` | ⚠️ `pop() ?[]const u8` |
| **front** | `front() ?Item` | ✅ | ✅ | ⚠️ `front() Error!Item` | ✅ |
| **increasePriority** | `increasePriority(Item) !void` | ✅ | ✅ | ⚠️ `(T, i32)` | ⚠️ `([]const u8)` |
| **decreasePriority** | `decreasePriority(Item) !void` | ✅ | ✅ | ⚠️ `(T, i32)` | ⚠️ `([]const u8)` |
| **contains** | `contains(Item) bool` | ✅ | ✅ | ⚠️ `contains(T)` | ⚠️ `contains([]const u8)` |
| **len** | `len() usize` | ✅ | ✅ | ✅ | ✅ |
| **isEmpty** | `isEmpty() bool` | ✅ | ✅ | ⚠️ `is_empty()` | ✅ |
| **d()** | `d() usize` | ❌ | ❌ | ❌ | ❌ |
| **clear()** | `clear(new_depth?)` | ❌ | ❌ | ❌ | ❌ |
| **toString()** | `toString() / to_string()` | ❌ | ❌ | ❌ | ❌ |

## Extended Methods

| API Element | Reference | Struct-Claude | Struct-Mistral | Doc-Claude | Doc-Mistral |
|-------------|-----------|---------------|----------------|------------|-------------|
| **peek** | Alias for front | ❌ | ❌ | ❌ | ❌ |
| **insertMany** | Bulk insert with heapify | ❌ | ❌ | ❌ | ❌ |
| **popMany** | Bulk pop | ❌ | ❌ | ❌ | ❌ |
| **toArray** | Export to slice | ❌ | ❌ | ❌ | ❌ |

## Type System

| API Element | Reference | Struct-Claude | Struct-Mistral | Doc-Claude | Doc-Mistral |
|-------------|-----------|---------------|----------------|------------|-------------|
| **Type Name** | `DHeap` | ✅ `DHeap` | ✅ `DHeap` | ⚠️ `DaryHeap` | ⚠️ `DaryHeap` |
| **Comptime generic** | `DHeap(T, Ctx, Cmp)` | ⚠️ `DHeap(compareFn)` | ⚠️ `DHeap(compareFn)` | ⚠️ `DaryHeap(T)` | ❌ Not generic |
| **Item struct** | External `Item` | ✅ External | ✅ External | ⚠️ Internal `Item` | ❌ Uses `[]const u8` |
| **ItemContext** | Hash/Eq context | ✅ | ✅ | ⚠️ AutoHash | ⚠️ Function ptrs |
| **Error type** | Error union | ✅ | ✅ Custom errors | ✅ Custom `Error` | ✅ Custom errors |

## Additional Features

| Feature | Struct-Claude | Struct-Mistral | Doc-Claude | Doc-Mistral |
|---------|---------------|----------------|------------|-------------|
| **Unit tests** | ✅ Yes | ❌ No | ✅ Extensive | ❌ No |
| **Verification helpers** | ❌ | ❌ | ✅ `verify_heap_property` | ❌ |
| **Comptime comparator** | ✅ | ✅ | ❌ Hardcoded | ⚠️ Runtime fn ptr |
| **MinByCost helper** | ✅ | ✅ | ❌ | ❌ |

## Conformance Score (out of 18 checkpoints)

| Implementation | ✅ Correct | ⚠️ Partial | ❌ Missing | Score |
|----------------|-----------|-----------|-----------|-------|
| **Struct-Claude** | 11 | 2 | 5 | 11/18 (61%) |
| **Struct-Mistral** | 12 | 2 | 4 | 12/18 (67%) |
| **Doc-Claude** | 6 | 8 | 4 | 6/18 (33%) |
| **Doc-Mistral** | 5 | 7 | 6 | 5/18 (28%) |

## Key Observations

1. **Struct-guided strongly outperforms Doc-guided** (61-67% vs 28-33%)

2. **Type name alignment**:
   - Struct-guided: Correct `DHeap` with comptime comparator
   - Doc-guided: `DaryHeap` (different name), varying generic approaches

3. **Item struct alignment**:
   - Struct-guided: External `Item` with `number`/`cost` fields (matches reference exactly)
   - Doc-Claude: Internal `Item` struct with `data`/`priority`
   - Doc-Mistral: Uses raw `[]const u8` slices (completely different approach)

4. **Comptime vs runtime**:
   - Struct-guided: Proper comptime comparator (`DHeap(compareFn)`)
   - Doc-Mistral: Runtime function pointers (less Zig-idiomatic)

5. **Error handling**:
   - All implementations use error unions (Zig-idiomatic)
   - Custom error types vary in naming

6. **Doc-Claude is most comprehensive** in terms of tests and verification, but diverges from reference API

7. **Struct-guided produces very consistent Item/DHeap structure** between Claude and Mistral

8. **All missing same features**: `d()`, `clear()`, `toString()`, `peek()`, bulk operations, `initCapacity()`

## Qualitative Differences

**Struct-guided advantages:**
- Correct type name (`DHeap`)
- External `Item` struct matching reference
- Comptime comparator (Zig-idiomatic)
- `MinByCost` helper function provided
- Very consistent output between models

**Doc-guided characteristics:**
- More variation in approaches
- Doc-Claude has extensive test coverage
- Doc-Claude provides heap verification helpers
- Doc-Mistral uses runtime function pointers (less idiomatic)
- Different item representations (internal struct vs raw slices)

**Zig-specific notes:**
- Struct-guided correctly uses `std.HashMap` with custom `ItemContext`
- Both properly handle allocator patterns
- Error handling follows Zig conventions in all cases
