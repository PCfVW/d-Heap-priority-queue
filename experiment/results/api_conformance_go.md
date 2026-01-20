# Go API Conformance Analysis

Generated: 2026-01-19

## Reference API (from README.md)

The reference Go implementation provides:
- Constructor: `New(Options[T,K]{D, Comparator, KeyExtractor})`
- Core: `Insert`, `Pop`, `Front`, `IncreasePriority`, `DecreasePriority`, `Contains`, `Len`, `IsEmpty`, `D`, `Clear`, `String`
- Extended: `Peek`, `ContainsKey`, `ToArray`, `InsertMany`, `PopMany`
- Type: `PriorityQueue[T any, K comparable]` with configurable comparator and key extractor

## Legend

- ✅ = Present and correct
- ⚠️ = Present but different signature/behavior
- ❌ = Missing

## Core Methods

| API Element | Reference | Struct-Claude | Struct-Mistral | Doc-Claude | Doc-Mistral |
|-------------|-----------|---------------|----------------|------------|-------------|
| **Constructor** | `New(Options[T,K]{...})` | ⚠️ `New[K](d)` | ⚠️ `New[K](d)` | ⚠️ `NewDaryHeap(d)` | ⚠️ `NewDaryHeap(d)` |
| **Insert** | `Insert(T)` | ⚠️ `Insert(Item[K])` | ⚠️ `Insert(Item[K])` | ⚠️ `Insert(Item)` | ⚠️ `Insert(*Item)` |
| **Pop** | `Pop() (T, bool)` | ✅ `(Item[K], bool)` | ✅ `(Item[K], bool)` | ⚠️ `(Item, error)` | ⚠️ `(*Item, error)` |
| **Front** | `Front() (T, error)` | ⚠️ `(Item[K], bool)` | ⚠️ `(Item[K], bool)` | ✅ `(Item, error)` | ✅ `(*Item, error)` |
| **IncreasePriority** | `IncreasePriority(T)` | ✅ | ✅ | ✅ | ✅ |
| **DecreasePriority** | `DecreasePriority(T)` | ✅ | ✅ | ✅ | ✅ |
| **Contains** | `Contains(T) bool` | ✅ | ✅ | ⚠️ `Contains(interface{})` | ⚠️ `Contains(*Item)` |
| **Len** | `Len() int` | ✅ | ✅ | ✅ | ✅ |
| **IsEmpty** | `IsEmpty() bool` | ✅ | ✅ | ✅ | ✅ |
| **D()** | `D() int` | ❌ | ❌ | ❌ | ❌ |
| **Clear()** | `Clear(newD...)` | ❌ | ❌ | ❌ | ❌ |
| **String()** | `String() string` | ❌ | ❌ | ✅ | ✅ |

## Extended Methods

| API Element | Reference | Struct-Claude | Struct-Mistral | Doc-Claude | Doc-Mistral |
|-------------|-----------|---------------|----------------|------------|-------------|
| **Peek** | `Peek() (T, bool)` | ❌ | ❌ | ❌ | ❌ |
| **ContainsKey** | `ContainsKey(K) bool` | ❌ | ❌ | ❌ | ❌ |
| **ToArray** | `ToArray() []T` | ❌ | ❌ | ❌ | ❌ |
| **InsertMany** | `InsertMany([]T)` | ❌ | ❌ | ❌ | ❌ |
| **PopMany** | `PopMany(n) []T` | ❌ | ❌ | ❌ | ❌ |

## Type System

| API Element | Reference | Struct-Claude | Struct-Mistral | Doc-Claude | Doc-Mistral |
|-------------|-----------|---------------|----------------|------------|-------------|
| **Type Name** | `PriorityQueue` | ✅ | ✅ | ⚠️ `DaryHeap` | ⚠️ `DaryHeap` |
| **Generics** | `[T any, K comparable]` | ⚠️ `[K comparable]` | ⚠️ `[K comparable]` | ❌ `interface{}` | ❌ `interface{}` |
| **Item separate** | User-defined T | ❌ Built-in Item | ❌ Built-in Item | ❌ Built-in Item | ❌ Built-in Item |
| **Comparator** | Configurable | ❌ Hardcoded min | ❌ Hardcoded min | ❌ Hardcoded min | ❌ Hardcoded min |
| **KeyExtractor** | Configurable | ❌ Uses Item.ID | ❌ Uses Item.ID | ❌ Uses Identity | ❌ Uses Identity |

## Conformance Score (out of 20 checkpoints)

| Implementation | ✅ Correct | ⚠️ Partial | ❌ Missing | Score |
|----------------|-----------|-----------|-----------|-------|
| **Struct-Claude** | 7 | 4 | 9 | 7/20 (35%) |
| **Struct-Mistral** | 7 | 4 | 9 | 7/20 (35%) |
| **Doc-Claude** | 7 | 6 | 7 | 7/20 (35%) |
| **Doc-Mistral** | 7 | 6 | 7 | 7/20 (35%) |

## Key Observations

1. **All implementations score equally (~35%)** - Neither condition produced significantly closer alignment to the reference API

2. **Struct-guided produced consistent naming** - Both Claude and Mistral produced nearly identical code structure with `PriorityQueue[K]`

3. **Doc-guided produced different naming** - Both used `DaryHeap` instead of `PriorityQueue`, and lost generics (used `interface{}`)

4. **All missing the same features**: D(), Clear(), Peek(), ContainsKey(), ToArray(), bulk operations, configurable comparator/key extractor

5. **Front/Pop return type divergence**: Struct-guided uses `(T, bool)`, Doc-guided uses `(T, error)` - reference uses mixed (`Front` returns error, `Pop` returns bool)

## Qualitative Differences

Despite equal scores, struct-guided shows better alignment:
- Correct type name (`PriorityQueue` vs `DaryHeap`)
- Preserves generics (`[K comparable]` vs `interface{}`)
- More consistent output between models (Claude and Mistral produced nearly identical struct-guided code)
