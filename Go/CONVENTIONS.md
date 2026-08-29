# d-ary-heap (Go module) — Coding Conventions

This document describes the [Amphigraphic coding](https://github.com/PCfVW/Amphigraphic-Strict) conventions used in the `Go/` subdirectory of the [Priority Queues](https://github.com/PCfVW/d-Heap-priority-queue) project. It is a scoped subset of [Cog — Strict Go for AI-Assisted Development](https://github.com/PCfVW/Amphigraphic-Strict/tree/master/Cog), restricted to what applies to a generic data-structure library: no concurrency, no I/O, no service-layer error wrapping chains.

Cog rules outside scope for this module (goroutine capture safety) are noted as not-yet-applicable rather than omitted outright — this is a generic collection type today, but a future concurrent-safe variant would need them. See the sibling `Rust/CONVENTIONS.md` for the same project's Rust conventions (adapted from Grit); this file follows the same structure so a reader who knows one can navigate the other.

## Trigger Checklist

**Before writing any line of code, check which triggers apply.**

| You are about to... | Check these rules |
|---|---|
| Write a `//` doc comment on an exported identifier | [Doc comment conventions](#doc-comment-conventions) |
| Write a function parameter or field typed `any` | [No Type Erasure](#no-type-erasure) — generic type *parameters* named `any` are fine; `any` as a concrete field/argument *type* is not |
| Write `_, err := ...` | [Explicit Error Handling](#explicit-error-handling) |
| Write `func f() (result int, err error)` | [No Named Returns](#no-named-returns) |
| Write a function returning `error` | [Interface Nil Safety](#interface-nil-safety) |
| Write `var s []T` | [Explicit Slice Initialization](#explicit-slice-initialization) |
| Write `go func() { ... }` | [Safe Goroutine Capture](#safe-goroutine-capture) — not currently applicable, kept for future concurrent code |
| Add a test | [Testing conventions](#testing-conventions) |

---

## When Writing Doc Comments

### Doc Comment Conventions

Every exported identifier (`type`, `func`, `var`, `const`, field of an exported struct) gets a `//` doc comment starting with the identifier's name, per standard `godoc` convention. This module additionally documents, for every exported type and the package's core operations, a **Cross-language equivalents** block mapping the Go name to its C++/Rust/Zig/TypeScript counterparts — see `Position`, `Comparator[T]`, and `PriorityQueue[T, K]` in `dheap.go` for the reference shape:

```go
// Position is the unified type alias for heap indices (cross-language consistency).
//
// Cross-language equivalents:
//   - C++: TOOLS::PriorityQueue<T>::Position
//   - Rust: d_ary_heap::Position
//   - Zig: DHeap.Position
//   - TypeScript: Position type alias
type Position = int
```

Exported methods that have a non-trivial complexity should document it directly (see the `Time Complexities` block on `PriorityQueue[T, K]`) rather than leaving it implicit.

**Naming note**: this package exposes a handful of `snake_case` aliases alongside the idiomatic `CamelCase` method (e.g. `Is_empty` next to `IsEmpty`, `Increase_priority` next to `IncreasePriority`). These are intentional, documented cross-language-parity aliases, not a style violation — don't "fix" one without checking whether the other is still referenced by the Dijkstra example or an experiment-runner script.

---

## When Writing Type Signatures

### No Type Erasure

Cog Rule 1 bans `any`/`interface{}` as a concrete parameter or return *type* (the pattern that erases the caller's real type from the callee). This module's generic declarations, e.g.:

```go
type Comparator[T any] func(a, b T) bool
type Options[T any, K comparable] struct { ... }
func New[T any, K comparable](opts Options[T, K]) *PriorityQueue[T, K]
```

use `any` only as a **generic type constraint** (meaning "no constraint," i.e. "any type is acceptable here at instantiation") — the compiler still fully monomorphizes each concrete `PriorityQueue[Foo, Bar]` instantiation, so no type information is erased at any call site. This is the Cog-compliant pattern; it looks similar to the banned pattern but is a different mechanism. The banned pattern would look like:

```go
// BANNED in Cog — data is any concrete value of unknown type at the call site
func process(data any) any { ... }
```

which does not appear, and should not appear, anywhere in this package.

---

## When Writing Error Handling

### Explicit Error Handling

Every fallible call's error must be checked or explicitly discarded with a comment explaining why. This module's fallible public methods return `error` directly (no wrapping needed internally, since there's no I/O or nested call chain to add context to):

```go
func (pq *PriorityQueue[T, K]) Front() (T, error) {
    if pq.IsEmpty() {
        return zero, ErrEmptyQueue
    }
    ...
}
```

Sentinel errors (`ErrEmptyQueue`, `ErrItemNotFound`, `ErrInvalidArity`, declared via `errors.New` at package scope) are the module's error vocabulary — callers use `errors.Is(err, dheap.ErrEmptyQueue)` to branch on which one occurred. If this package ever wraps an error from a lower layer, use `fmt.Errorf("context: %w", err)` (Cog's required wrapping form) rather than string-formatting the error away.

> ✅ `if err != nil { return zero, fmt.Errorf("dheap: %w", err) }`
> ❌ `value, _ := mightFail()` — silently discards the error

### No Named Returns

Every exported function in this package declares explicit, unnamed return types and returns explicit values — no `func f() (result int, err error) { result = 42; return }`. This is already the case throughout `dheap.go` (`Front() (T, error)`, `GetPosition(item T) (Position, bool)`, etc.) — keep new methods consistent with that shape rather than introducing named returns for convenience.

### Interface Nil Safety

`error` is an interface; a `nil`-valued concrete error type wrapped in a non-nil `error` interface is a classic Go footgun (`var e *MyError = nil; return e` produces a non-nil `error`). This package sidesteps the entire hazard by using package-level sentinel `error` values (`var ErrEmptyQueue = errors.New(...)`) instead of a custom error struct type — every fallible method returns either a literal `nil` or one of these sentinels directly, never a typed-then-boxed pointer. Keep it that way: if a future error needs structured fields (not just a message), prefer returning it as a value type (not `*T`) so a "zero" instance can't accidentally satisfy the interface as a non-nil error.

### Explicit Slice Initialization

Slices returned to callers are always explicitly sized with `make`, never left as a bare `nil` declaration that would round-trip to JSON `null` instead of `[]`:

```go
func (pq *PriorityQueue[T, K]) ToArray() []T {
    result := make([]T, len(pq.container))
    ...
}
```

`PopMany`/`InsertMany` follow the same pattern (`result := make([]T, 0, actualCount)`). Any new method returning a slice should do the same, even when the expected result is often empty.

### Safe Goroutine Capture

Not currently applicable: this package has no goroutines (the heap is not safe for concurrent use without external synchronization, and none is provided). If a future concurrent-safe wrapper is added, loop-variable capture must pass the item as an explicit argument (`go func(it Item) { ... }(item)`), not rely on Go 1.22+'s per-iteration loop variable semantics alone — this repo's `go.mod` pins `go 1.21`, which predates that change.

---

## Testing Conventions

### Where tests live

- `Go/src/dheap_test.go` — table-driven tests for the public API (construction, insert/pop/update-priority sequences, error paths via `errors.Is`).
- `Go/src/instrumentation_test.go` — the `Stats`-specific suite, including the lockstep pop-order check against the non-instrumented heap and the nil-safe `Total()`/`Reset()` regression test (mirrors the equivalent Rust/C++/Zig instrumentation test suites).

### What to assert

For every modifier method, assert at minimum:
1. The observable result (returned value, `ok`/`error`).
2. The heap-property invariant after the call.
3. The position-map invariant (`positions[key]` matches the item's actual index in `container`).

### Table-driven style

Prefer Go's idiomatic table-driven test structure (`[]struct{ name string; ... }` + `t.Run(tc.name, ...)`) for cases that share setup but vary inputs/expected outputs, rather than repeating near-identical test functions.

---

## Lint Floor (informative)

**Current floor**: `go vet` and `gofmt` (implicitly run by `go build`/`go test`) are the only checks exercised today — there is no `.golangci.yml` in this repository yet, unlike the Rust crate's wired-up `clippy::pedantic` floor (see `Rust/CONVENTIONS.md`). This is declared honestly rather than implied: the rules above are enforced by code review, not by a linter, until a `.golangci.yml` (paired with a CI lint step) is added.

**Recommended, not yet wired**: `golangci-lint` with `errcheck` (catches the exact "discarded error" pattern Cog Rule 2 bans) and `forcetypeassert` enabled would give this module the same kind of automated floor Rust already has. Adding that config against the live codebase (to confirm zero false positives before turning it on) is worthwhile follow-up work, not done as part of this pass.
