# d-ary-heap (Zig package) — Coding Conventions

This document describes the [Amphigraphic coding](https://github.com/PCfVW/Amphigraphic-Strict) conventions used in the `zig/` subdirectory of the [Priority Queues](https://github.com/PCfVW/d-Heap-priority-queue) project. It is a scoped subset of [Gizmo — Strict Zig for AI-Assisted Development](https://github.com/PCfVW/Amphigraphic-Strict/tree/master/Gizmo), restricted to what applies to a generic, single-allocator data-structure library: no `anytype`-parameterized public API, no packed/wire-format structs, no complex `comptime` computation.

See the sibling `Rust/CONVENTIONS.md`, `Go/CONVENTIONS.md`, and `Cpp/CONVENTIONS.md` for the same project's other conventions files; this file follows the same structure so a reader who knows one can navigate the other.

## Trigger Checklist

**Before writing any line of code, check which triggers apply.**

| You are about to... | Check these rules |
|---|---|
| Write a function that allocates or frees memory | [Document Allocator Ownership](#document-allocator-ownership) |
| Write `catch` | [Explicit Error Handling](#explicit-error-handling) |
| Write `switch` over an error set | [Exhaustive Error Switches](#exhaustive-error-switches) |
| Write `.?` | [Explicit Optional Handling](#explicit-optional-handling) — prefer `orelse` |
| Write `anytype` | [Document `anytype` Constraints](#document-anytype-constraints) — not currently used in the public API; document if introduced |
| Write `comptime` | [Limit `comptime` to Type Construction](#limit-comptime-to-type-construction) |
| Write `defer` | [One `defer` Per Resource](#one-defer-per-resource) |
| Write raw pointer arithmetic (`[*]T`) | [Prefer Slices](#prefer-slices-over-pointer-arithmetic) |
| Write `unreachable` | [`unreachable` Only for Provably Impossible States](#unreachable-only-for-provably-impossible-states) |
| Write `packed struct` | [Avoid Packed Structs Unless Required](#avoid-packed-structs-unless-required) |
| Add a test | [Testing conventions](#testing-conventions) |

---

## When Writing Allocating Functions

### Document Allocator Ownership

Every allocating type already stores its allocator explicitly and documents it:

```zig
/// Allocator used for all dynamic memory allocations
allocator: std.mem.Allocator,
```

`init`/`initCapacity` take the allocator as an explicit parameter (never a hidden global), and `deinit` frees through the same stored allocator (`self.container.deinit(self.allocator)`). Any new allocating method must follow this shape: take or reuse `self.allocator`, never a different one, and document who calls `deinit`/`free` on what it returns (caller-owned vs. heap-owned) directly in the method's doc comment.

---

## When Writing Error Handling

### Explicit Error Handling

`try` propagates; `catch` is only used with either an explicit fallback block or explicit propagation, never a silent discard. The library's dominant pattern is `orelse { return error.X; }` at the call site of a `HashMap.getPtr`/`.get` lookup:

```zig
const index_ptr = self.positions.getPtr(updated_item) orelse {
    return error.ItemNotFound;
};
```

If a future call site needs a *default value* instead of propagation, use the Gizmo `catch |err| blk { ... }` form with a `// FALLBACK:` comment explaining why silently defaulting is correct there — don't reach for a bare `catch 0`-style discard.

### Exhaustive Error Switches

This library does not currently `switch` over an error set (errors are propagated with `try`/`orelse return error.X`, not branched on internally). If a future method needs to handle specific error variants differently, the `switch` must list every variant of the error set explicitly — no `else => {}` catch-all that could silently swallow a case a caller needed to see.

### Explicit Optional Handling

Optional-typed lookups use `orelse` with an explicit fallback, never a chained `.?`:

```zig
const new_index = self.positions.get(updated_item) orelse return error.ItemNotFound;
```

There are no `.?` chains anywhere in the heap implementation — keep it that way. A chain like `a.?.b.?.c` hides which link can actually be null; one `orelse` per optional, each with its own fallback or propagation, keeps every null path visible.

---

## When Writing Generic and Comptime Code

### Document `anytype` Constraints

The public heap API (`DHeap`/`DHeapWithStats`) takes concrete `comptime` type parameters (`comptime T: type, comptime Context: type, comptime ComparatorType: type`), not `anytype` — so this rule is not currently exercised. If a future helper takes an `anytype` parameter (e.g. a generic formatter callback), document its required shape in the doc comment (`TYPE CONSTRAINT: value must have fn ...`) before merging it — do not leave callers to infer the contract from the implementation.

### Limit `comptime` to Type Construction

Every `comptime` parameter in this codebase shapes a *type* (`DHeap(comptime T: type, comptime Context: type, comptime ComparatorType: type, comptime collect_stats: bool)`), not a runtime computation performed at compile time. `collect_stats: bool` is the one comptime *value* (not type) parameter, and it's used exactly as Gizmo Rule 6 intends: to select which of two field types (`ComparisonStats` vs. `void`) a struct gets, and to comptime-eliminate the stats-bracketing branches — not to run an algorithm at compile time. Keep new `comptime` parameters to this same "pick a type / pick a struct shape" role.

---

## When Writing Cleanup and Data Layout

### One `defer` Per Resource

`defer` is used for allocator cleanup (`self.container.deinit(self.allocator)`) and stats bracketing (`defer if (collect_stats) self.stats.endOperation();`), each with exactly one resource per `defer` statement — never multiple cleanups bundled into a single `defer` block, so LIFO order stays obvious from reading top to bottom. When a method needs more than one `defer`, keep each on its own line next to the acquisition it undoes, matching Gizmo's `CLEANUP[n]:` numbering convention if the order isn't otherwise self-evident from proximity.

### Prefer Slices Over Pointer Arithmetic

The container is a `std.ArrayList(T)` (soon-to-be `std.ArrayListUnmanaged` per Zig 0.15's API split), accessed via `.items` (a slice) and iterated with `for (items) |item|` — there is no raw `[*]T` pointer arithmetic anywhere in the heap implementation. Keep new code slice-first; reach for pointer arithmetic only when interfacing with an external/C API that requires it, and mark that boundary explicitly.

### `unreachable` Only for Provably Impossible States

`unreachable` does not currently appear in `d_heap.zig`'s public operations — every "this shouldn't happen" path returns an explicit `error.X` instead (e.g. `orelse return error.ItemNotFound`), which is the Gizmo-preferred shape for anything reachable from untrusted/caller-controlled input. Reserve `unreachable` for states genuinely impossible after an established precondition (e.g. `std.debug.assert` just above it establishes the fact), never as a stand-in for "I don't expect this branch to be hit."

### Avoid Packed Structs Unless Required

No `packed struct` appears in this codebase — the heap's internal types are ordinary structs. Introduce `packed struct` only for an external/wire-format requirement (matching a binary layout defined outside this codebase), and document the requirement with a `// C INTEROP:`-style comment at the declaration, per Gizmo.

---

## Testing Conventions

### Where tests live

- `zig/src/tests/heap_tests.zig` — public-API test suite for the un-instrumented heap.
- `zig/src/tests/instrumentation_tests.zig` — the `collect_stats=true` suite, wired into `zig build test` as a second compile unit (see `zig/src/tests/test_runner.zig`), including the lockstep pop-order check against the non-instrumented heap.

### What to assert

For every modifier method, assert at minimum:
1. The observable result (returned value or propagated error).
2. The heap-property invariant after the call.
3. The position-map invariant (`positions.get(item)` matches the item's actual index).

### Comptime-friendly assertions

Prefer a comptime-evaluable assertion over a runtime one when the property is a compile-time fact — e.g. `@sizeOf(DHeapItem) < @sizeOf(InstrumentedDHeap(Item, ...))` in `instrumentation_tests.zig`, which mirrors Rust's `size_of::<...>()` equality assertions and C++'s `static_assert`s for the same zero-bytes-elision property.

---

## Lint Floor (informative)

**Active floor**: Zig has no separate linter ecosystem equivalent to `clippy`/`golangci-lint`/`clang-tidy` — the compiler's own safety checks are the floor, and unlike Go/C++'s "none automated" state (see `Go/CONVENTIONS.md`, `Cpp/CONVENTIONS.md`), this one is genuinely active today, just built into the toolchain rather than a separate linter invocation. `zig build test` runs under Zig's default safety-checked mode (bounds checks, overflow checks, `unreachable` traps at runtime in `Debug`/`ReleaseSafe`). Production builds should use `zig build -Doptimize=ReleaseSafe` (keeps safety checks) rather than `ReleaseFast`/`ReleaseSmall`, unless a specific benchmark run has already established the safety checks aren't the bottleneck.
