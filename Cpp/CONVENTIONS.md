# d-ary-heap (C++ header-only library) — Coding Conventions

This document describes the [Amphigraphic coding](https://github.com/PCfVW/Amphigraphic-Strict) conventions used in the `Cpp/` subdirectory of the [Priority Queues](https://github.com/PCfVW/d-Heap-priority-queue) project. It is a scoped subset of [Stoic — Strict C++ for AI-Assisted Development](https://github.com/PCfVW/Amphigraphic-Strict/tree/master/Stoic), restricted to what applies to a generic, header-only, single-file data-structure library: no networking, no packed/wire-format structs, no lock-free code.

See the sibling `Rust/CONVENTIONS.md` and `Go/CONVENTIONS.md` for the same project's other conventions files; this file follows the same structure so a reader who knows one can navigate the other.

## Trigger Checklist

**Before writing any line of code, check which triggers apply.**

| You are about to... | Check these rules |
|---|---|
| Add a new `#include` | [Header Hygiene](#header-hygiene-fallback-for-pre-c20) |
| Write `new`/`delete`, or a raw owning pointer field | [Smart Pointers Only](#smart-pointers-only) |
| Write `#define` | [No Preprocessor Macros](#no-preprocessor-macros) |
| Write `constexpr` | [`const` by Default](#const-by-default-not-constexpr) |
| Write a function with an unchecked precondition | [`// PRECONDITION:`](#preconditions-and-undefined-behavior) |
| Write code that is UB when misused (e.g. calling `.front()` on empty) | [`// UB:`](#preconditions-and-undefined-behavior) |
| Write `myMap[key]` | [`// MAP_ACCESS:`](#map_access-annotation) |
| Pass a parameter by value intentionally (not by oversight) | [`// COPY:`](#copy-annotation) |
| Declare a `typedef`/`using` with an implicit numeric bound | [`// TYPE_LIMIT:`](#type_limit-annotation) |
| Write `#if defined(...)` / `#ifdef` | [`// PLATFORM:`](#platform-annotation) |
| Add a test | [Testing conventions](#testing-conventions) |

---

## When Writing Headers and Managing Memory

### Header Hygiene (fallback for pre-C++20)

This library does not yet use C++20 modules (Stoic's preferred form) — it ships as a single header, `PriorityQueue.h`, included by consumers via `#include "PriorityQueue.h"`. Under Stoic's documented fallback for pre-module code:

- The header is self-contained: it includes everything it needs (`<vector>`, `<unordered_map>`, `<optional>`, etc. — see the `Visibility for ...` comments next to each `#include` at the top of the file) rather than relying on transitive includes from the consumer.
- `#pragma once` guards the file (not a manual `#ifndef`/`#define` guard).
- No macro leaks into consumer translation units beyond the two narrow, allowed exceptions below.

If this library ever moves to modules, this section should be rewritten to Stoic's Rule 1 form (`export module d_ary_heap;`) rather than patched in place.

### Smart Pointers Only

The library already follows this rule throughout: `container` is a `std::vector<T>` and `positions` is a `std::unordered_map<T, ItemPositionInPriorityQueue, THash, TEqual>` (both value-semantic, RAII-managed members) — there is no raw owning pointer, no `new`/`delete`, anywhere in `PriorityQueue.h`. Keep it that way: if a future feature needs heap allocation the current containers can't express, reach for `std::unique_ptr<T>` (sole ownership) before `std::shared_ptr<T>` (shared ownership, only when truly needed), and never a bare owning `T*`.

---

## When Writing Constants and Macros

### No Preprocessor Macros

Two `#define`s exist in the file, both under the **allowed exceptions** (compiler-capability feature detection), not the banned "function-like macro" or "constant macro" patterns:

```cpp
#if defined(_MSC_VER)
    #define DHEAP_NO_UNIQUE_ADDRESS [[msvc::no_unique_address]]
#else
    #define DHEAP_NO_UNIQUE_ADDRESS [[no_unique_address]]
#endif
```

This selects between two spellings of the same standard attribute depending on the compiler — there is no language-level alternative to `#define` for attribute-name selection, so it stays. Any *new* macro proposed for this file should be checked against Stoic's allowed-exceptions table (include guards, `#pragma once`, platform detection, feature detection) before being added; a function-like or constant macro should become an `inline` function/template or a `const`/`constexpr` variable instead.

### `const` by Default, Not `constexpr`

`constexpr bool INCLUDE_ASSERT = true;` is this file's one `constexpr` — it gates `assert()` calls at compile time (an `if constexpr`-style use, genuinely requiring compile-time evaluation) rather than being a speculative annotation on an ordinary constant. When adding a new named constant, reach for plain `const` first; only promote to `constexpr` when it feeds a template argument, a `static_assert`, an `if constexpr` condition, or a compile-time array bound.

---

## When Writing Function Contracts

### Preconditions and Undefined Behavior

Every function with an unchecked precondition or a UB-on-misuse path must say so with an inline `// PRECONDITION:` or `// UB:` comment at the declaration, not just in a doc comment above it (doc comments can drift out of sync with the implementation; the inline marker sits next to the code it describes).

The reference example is `front()`, which is UB on an empty queue and has no bounds check:

```cpp
// ----- Return the highest priority item of the queue. UB: calling this function on an empty container is undefined behavior.
typename std::vector<T>::const_reference front() const
{
    return container.front();
}
```

`peek()` is the documented safe alternative (`std::optional<T>`, checks `container.empty()` first) — every UB-on-misuse method in this library should have a safe sibling like this, and the UB method's doc comment should point to it (see `front()`'s Doxygen block a few lines above the declaration).

---

## When Writing Comment Markers

### `// MAP_ACCESS:` Annotation

Documents an intentional map insertion. `positions` is a `std::unordered_map<T, ItemPositionInPriorityQueue, ...>`. Any `positions[key] = ...` write is an intentional insert-or-update and should carry a trailing `// MAP_ACCESS: intentional insertion/update` comment; any *read* should go through `.find()` (as `contains()` and `get_position()` already do) rather than `operator[]`, since a lookup via `operator[]` would silently insert a default-constructed entry.

### `// COPY:` Annotation

Documents an intentional pass-by-value. Flag any parameter that is intentionally copied (not just passed by oversight instead of `const&`) with a `// COPY:` comment explaining why the copy is load-bearing (e.g. rollback semantics, or the callee needs to mutate its own copy). This library's methods overwhelmingly take by `const&` or consume by value where ownership transfers (`insert(T item)`) — a new `const T&` parameter that gets copied internally for a real reason should say so at the call site or the parameter itself.

### `// TYPE_LIMIT:` Annotation

Documents a numeric constraint on a type alias. `typedef typename std::vector<T>::size_type ItemPositionInPriorityQueue;` has no implicit numeric ceiling beyond `size_type`'s own range, so no `// TYPE_LIMIT:` is needed today. If a future type alias narrows this (e.g. a `std::uint16_t`-backed position type for a memory-constrained variant), annotate its declared bound at the `typedef`/`using` site.

### `// PLATFORM:` Annotation

Documents platform-specific code. The `DHEAP_NO_UNIQUE_ADDRESS` macro selection above is this file's one platform/compiler branch; the `#if defined(_MSC_VER)` / `#else` arms are self-explanatory enough that no extra `// PLATFORM:` comment is currently attached, but any *new* platform branch (e.g. a future `#ifdef _WIN32` for a benchmark-only RSS helper, as already exists in the Dijkstra example's `rss_windows`/`rss_other` split) must carry one, per Stoic's convention.

---

## Testing Conventions

### Where tests live

Test files sit alongside the header (`test_comprehensive.cpp`, `test_decrease_priority.cpp`, `test_instrumentation.cpp`, `test_unified_api.cpp`) — no separate `tests/` directory. Each is a standalone translation unit with its own `main()`, matching the CMake target structure in `CMakeLists.txt`.

### What to assert

For every modifier method, assert at minimum:
1. The observable result (returned value, `std::optional` state, or thrown/UB-precondition documented separately).
2. The heap-property invariant after the call.
3. The position-map invariant (`positions[item]` matches the item's index in `container`).

### Compile-time assertions

`test_instrumentation.cpp` includes `static_assert`s proving zero-bytes elision via `[[no_unique_address]]` (e.g. `sizeof(InstrumentedPriorityQueue<int>) - sizeof(PriorityQueue<int>) == sizeof(ComparisonStats)`). Prefer a `static_assert` over a runtime check whenever the property being tested is itself a compile-time fact (layout, size, type trait) — it fails at build time, before any test binary even runs.

---

## Lint Floor (informative)

**Current floor**: none automated. There is no `.clang-tidy` in this repository yet, unlike the Rust crate's wired-up `clippy::pedantic` floor (see `Rust/CONVENTIONS.md`). The rules above are enforced by code review only, until a `.clang-tidy` (paired with a CI step running it) is added.

**Recommended, not yet wired**: `clang-tidy` with `cppcoreguidelines-owning-memory`, `cppcoreguidelines-no-malloc`, `cppcoreguidelines-macro-usage`, and `modernize-make-unique`/`modernize-make-shared` would give this header the automated floor Stoic recommends. Authoring that config against the live header (to confirm zero false positives before turning it on) is worthwhile follow-up work, not done as part of this pass.
