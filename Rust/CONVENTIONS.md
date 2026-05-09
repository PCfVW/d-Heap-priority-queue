# d-ary-heap (Rust crate) — Coding Conventions

This document describes the [Amphigraphic coding](https://github.com/PCfVW/Amphigraphic-Strict) conventions used in the Rust subdirectory of the [Priority Queues](https://github.com/PCfVW/d-Heap-priority-queue) project. It is a tightly-scoped subset of [Grit — Strict Rust for AI-Assisted Development](https://github.com/PCfVW/Amphigraphic-Strict/tree/master/Grit), restricted to what applies to a generic data-structure library: no async, no I/O, no FFI, no ML/tensor work.

Conventions outside scope (loop fission, `# Memory` sections, PROMOTE / CONTIGUOUS, FFI, feature-gated backends, OOM-safe loaders) are deliberately omitted; consult the sibling `anamnesis`, `candle-mi`, and `hypomnesis` `CONVENTIONS.md` files if the crate ever grows in those directions.

## Trigger Checklist

**Before writing any line of code, check which triggers apply.**

| You are about to... | Check these rules |
|---|---|
| Write a `///` or `//!` comment | [Backtick hygiene](#backtick-hygiene), [field-level docs](#field-level-docs), [intra-doc link safety](#intra-doc-link-safety) |
| Write a `pub fn` or `pub const fn` | [`const fn`](#const-fn), [`#[must_use]`](#must_use-policy), [pass by value](#pass-by-value-vs-reference) |
| Write a `pub fn` returning `Result<T>` | [`# Errors` section](#errors-doc-section) |
| Write a `pub enum` | [`#[non_exhaustive]`](#non_exhaustive-policy) or [`// EXHAUSTIVE:`](#exhaustive-annotation) |
| Write an `as` cast | [`// CAST:`](#cast-annotation) |
| Write `slice[i]` or `slice[a..b]` | [`// INDEX:`](#index-annotation) |
| Write `.as_str()`, `.to_owned()` | [`// BORROW:`](#borrow-annotation) |
| Write an `unsafe` block | [`// SAFETY:`](#safety-annotation) — `unsafe` is not expected in this crate |
| Write `Box<dyn T>` or `&dyn T` | [`// TRAIT_OBJECT:`](#trait_object-annotation) |
| Write a `match` or `if let` | [Control-flow rules](#if-let-vs-match), [`// EXPLICIT:`](#explicit-annotation) if no-op arm |
| Write error strings | [Error message wording](#error-message-wording) |
| Add a test | [Testing conventions](#testing-conventions) |

---

## When Writing Doc Comments (`///`, `//!`)

### Backtick Hygiene

All identifiers, types, trait names, field names, crate names, and method names in doc comments must be wrapped in backticks so that rustdoc renders them as inline code and Clippy's `doc_markdown` lint passes.

Applies to: struct/enum/field names, method names (`fn insert`, `fn pop`), types (`Vec<T>`, `Option<T>`, `Result<T, Error>`), crate names (`d_ary_heap`), trait names (`PriorityCompare`), and acronyms that double as types (`PQ` if used).

> ✅ `/// Inserts `item` into the heap, restoring the heap invariant via `move_up`.`
> ❌ `/// Inserts item into the heap, restoring the heap invariant via move_up.`

### Intra-Doc Link Safety

Rustdoc intra-doc links must resolve under all feature-flag combinations (enforced by `#![deny(warnings)]` → `rustdoc::broken_intra_doc_links`).

Two patterns to watch:

1. **Feature-gated items** — items behind `#[cfg(feature = "...")]` (e.g., a future `serde` integration) are absent when that feature is off. Use plain backtick text, not link syntax:

   > ✅ `` /// Implemented when the `serde` feature is enabled. ``
   > ❌ `` /// Implemented by [`SerdeSupport`](crate::serde::SerdeSupport). ``

2. **Cross-module links** — items re-exported at the crate root (e.g., `Error`) are not automatically in scope inside submodules. Use explicit `crate::` paths:

   > ✅ `` /// Returns [`Error::ItemNotFound`](crate::Error::ItemNotFound) when the item is absent. ``
   > ❌ `` /// Returns [`Error::ItemNotFound`] when the item is absent. ``

### Field-Level Docs

Every field of every `pub` struct must carry a `///` doc comment describing:
1. what the field represents,
2. its unit, valid range, or invariant where applicable.

Fields of `pub(crate)` structs follow the same rule. Private fields inside a `pub(crate)` or `pub` struct must have at minimum a `//` comment if their purpose is not self-evident from the name alone — heap implementations are dense with bookkeeping (positions map, depth, comparator), so naming alone usually isn't enough.

> Example (illustrative; the live struct in `lib.rs` follows this pattern):
> ```rust
> pub struct PriorityQueue<T, C>
> where
>     T: Eq + Hash + Clone,
> {
>     /// Heap-ordered backing array; index 0 is the root (highest priority).
>     container: Vec<T>,
>     /// Item-identity → position-in-`container` map; enables O(1) lookups
>     /// for `contains` and `update_priority`. Cloned items act as their own keys.
>     positions: HashMap<T, Position>,
>     /// User-supplied priority comparator: returns `true` iff `a` has
>     /// strictly higher priority than `b`.
>     comparator: C,
>     /// Branching factor `d ≥ 1`; fixed at construction.
>     depth: usize,
> }
> ```

### `# Errors` Doc Section

All public fallible methods (`-> Result<T, Error>`) must include an `# Errors` section in their doc comment. Each bullet uses the format:

    /// # Errors
    /// Returns [`Error::ItemNotFound`] if the supplied item is not in the queue.
    /// Returns [`Error::InvalidArity`] when `d == 0`.

Rules:
- Start each bullet with `Returns` followed by the variant in rustdoc link syntax, e.g., `` [`Error::ItemNotFound`] ``.
- Follow with `if` (condition), `on` (event), or `when` (circumstance).
- Use the concrete variant name, not the generic `Error`.
- One bullet per distinct error path.

This crate is mostly infallible (heap operations don't usually fail), but the constructors and identity-based mutators (`increase_priority(&item)`) do return `Result`. Document them.

---

## When Writing Function Signatures

### `const fn`

Declare a function `const fn` when **all** of the following hold:
1. The body contains no heap allocation, I/O, or `dyn` dispatch.
2. All called functions are themselves `const fn`.
3. There are no trait-method calls that are not yet `const`.

This applies to constructors, accessors, and pure arithmetic helpers. When in doubt, annotate and let the compiler reject it — do not omit `const` preemptively.

> ✅ `pub const fn d(&self) -> usize { self.depth }`
> ❌ `pub fn d(&self) -> usize { self.depth }`

Note: trait-bounded generic methods often can't be `const` yet (the trait method itself isn't `const`). That's a language limitation, not a convention violation — leave the annotation off and add a `// EXPLICIT: trait method not yet const` if it surprises a reader.

### `#[must_use]` Policy

All public functions and methods that return a value and have no side effects must be annotated `#[must_use]`. This includes constructors (`new`, `with_first`), accessors (`len`, `is_empty`, `peek`, `front`, `get_position`, `to_array`, `d`), and pure queries (`contains`).

Without the annotation, a caller can silently discard the return value — which for these functions is always a bug, since the call has no other effect.

The `clippy::must_use_candidate` lint enforces this at `warn` level — promoted to error once the crate adopts `#![deny(warnings)]` (see [Lint Floor](#lint-floor-informative)).

### Pass by Value vs Reference

Follow these rules for function parameters:

| Type | Rule |
|---|---|
| `Copy` type ≤ 2 words (`usize`, `f32`, `bool`, small `enum`) | Pass by value |
| `Copy` type > 2 words | Pass by reference |
| Non-`Copy`, not mutated | Pass by `&T` or `&[T]` |
| Non-`Copy`, mutated | Pass by `&mut T` |
| Owned, consumed by callee | Pass by value (move semantics) |
| `&mut T` not actually mutated in body | Change to `&T` |

Never accept `&mut T` when the function body never writes through the reference; Clippy's `needless_pass_by_ref_mut` will flag it and callers lose the ability to pass shared references. Similarly, `trivially_copy_pass_by_ref` flags `&T` where `T: Copy` and is small enough to pass by value.

The heap's `insert` consumes its argument (`item: T`) — that's correct: ownership transfers into the container.

> ✅ `pub fn insert(&mut self, item: T)`
> ❌ `pub fn insert(&mut self, item: &T)` — would force callers to clone.

---

## When Writing Public Enums

### `#[non_exhaustive]` Policy

- Public enums that may gain new variants: `#[non_exhaustive]`. The crate's `Error` enum is the prime candidate — adding a new error variant should not be a breaking change.
- Internal dispatch enums matched exhaustively by this crate: `#[allow(clippy::exhaustive_enums)] // EXHAUSTIVE: <reason>`.

> ✅
> ```rust
> #[non_exhaustive]
> pub enum Error {
>     InvalidArity,
>     ItemNotFound,
>     EmptyQueue,
>     IndexOutOfBounds,
> }
> ```

---

## When Writing Expressions

These annotations are required **on or immediately before** the line where the pattern occurs. Apply them as you write the line, not in a review pass. Even though several of these patterns are rare in a generic heap, the policy stays declared so future drift is caught.

### CAST Annotation

`// CAST: <from> → <to>, <reason>` — required on every `as` cast between numeric types. Prefer `From`/`Into` for lossless conversions and `TryFrom`/`TryInto` with `?` for fallible ones. Use `as` only when truncation or wrapping is the deliberate intent.

> Example: `// CAST: usize → u64, comparison count cannot exceed u64 in any realistic run`

In this crate, `as` casts are uncommon; expect at most one or two for the `Position = usize` boundary or `u64` stat counters.

### INDEX Annotation

`// INDEX: <reason>` — required on every direct slice index (`slice[i]`, `slice[a..b]`) that cannot be replaced by an iterator. Direct indexing panics on out-of-bounds; prefer `.get(i)` with `?` or explicit error handling. Use direct indexing only when the bound is provably valid and an iterator idiom would be significantly less readable.

A heap implementation is a hot zone for `INDEX:` annotations — `move_up`, `move_down`, and `best_child_position` all index by computed positions whose bounds are proved by the heap invariant rather than by `len()` checks at the use-site. Annotate every such site.

> Example: `// INDEX: parent index is (i-1)/d which is < i ≤ container.len()-1`

### BORROW Annotation

`// BORROW: <what is converted>` — required on explicit `.as_str()`, `.as_bytes()`, `.to_owned()` conversions (Grit Rule 2).

In this crate this is rare (no string handling on the hot path); the policy is declared for future drift.

### TRAIT_OBJECT Annotation

`// TRAIT_OBJECT: <reason>` — required on every `Box<dyn Trait>` or `&dyn Trait` usage.

In this crate this is rare; the heap is generic over its element type and comparator (static dispatch). Declare the policy anyway.

---

## When Writing `unsafe`

### SAFETY Annotation

`// SAFETY: <invariants>` — required on every `unsafe` block or function (inline comment, not a doc comment).

This crate aims to be `#![forbid(unsafe_code)]`; the current `lib.rs` does not yet declare the attribute, but every implementation to date is safe Rust and adding the attribute is a near-zero-effort hardening step. The annotation policy below is included for completeness — if `unsafe` ever becomes necessary (e.g., for a `MaybeUninit`-based bulk-load fast path), follow the candle-mi pattern: single dedicated module, feature-gated, with a `// SAFETY:` comment on every block.

---

## When Writing Control Flow

### `if let` vs `match`

Use the most specific construct for the pattern at hand:

| Situation | Preferred form |
|---|---|
| Testing a single variant, no binding needed | `matches!(expr, Pat)` |
| Testing a single variant, binding needed | `if let Pat(x) = expr { … }` |
| Two or more variants with different bodies | `match expr { … }` |
| Exhaustive dispatch over an enum | `match expr { … }` (never `if let` chains) |

Never use a `match` with a single non-`_` arm and a no-op `_ => {}` where `if let` or `matches!` would be clearer (Clippy: `single_match`, `match_like_matches_macro`). Conversely, never chain three or more `if let … else if let …` arms where a `match` would be exhaustive.

> ✅ `if let Some(top) = pq.peek() { use_top(top); }`
> ✅ `matches!(error, Error::EmptyQueue | Error::ItemNotFound)`
> ❌ `match pq.peek() { Some(top) => use_top(top), None => {} }`

### EXPLICIT Annotation

`// EXPLICIT: <reason>` — required when a match arm is intentionally a no-op, or when an imperative loop is used instead of an iterator chain for a stateful computation.

Heap operations are fundamentally stateful loops (`move_up`, `move_down`); `// EXPLICIT:` is the right tool to acknowledge that an iterator would obscure the algorithm.

> Example: `// EXPLICIT: move_down carries a mutable cursor and stops on heap-property restore; .map() would hide both`

### EXHAUSTIVE Annotation

`// EXHAUSTIVE: <reason>` — required on `#[allow(clippy::exhaustive_enums)]`.

> Example: `// EXHAUSTIVE: internal direction enum; crate owns and matches all variants`

---

## When Writing Error Strings

### Error Message Wording

Error strings constructed inside `Error` variants follow two patterns:

- **External failures** (rare in this crate; mostly applicable if a future `serde`-feature path emerges): `"failed to <verb>: {e}"`
- **Validation failures** (range, lookup, invariant): `"<noun> <problem> (<context>)"`
  > Example: `Error::InvalidArity` → `"arity must be ≥ 1, got 0"`
  > Example: `Error::ItemNotFound` → `"item not present in heap"`
  > Example: `Error::IndexOutOfBounds` → `"index {i} out of bounds for heap of length {len}"`

Rules:
- Use lowercase, no trailing period.
- Include the offending value and the valid range or constraint when applicable.
- Wrap external errors with `: {e}`, not `.to_string()`.

---

## Testing Conventions

### Where tests live

- **Unit tests** for private invariants (heap-property, position-map consistency): inline `#[cfg(test)] mod tests { … }` at the bottom of the file under test.
- **Integration tests** for the public API (cross-method invariants, regression cases): `Rust/tests/*.rs`.
- **Doctests** for the docstring examples: every public method's doc-comment example should be a runnable doctest. The `cargo test` invocation already exercises them.

### What to assert

For every modifier method (`insert`, `pop`, `update_priority`, …), assert at minimum:
1. The observable result (popped value, return code).
2. The heap-property invariant after the call. A shared helper `assert_heap_property(&pq)` defined in a `tests/common/mod.rs` is the recommended placement when more than one integration test needs it; until then, a local helper inside the test file is fine.
3. The position-map invariant (every item's `positions[item]` matches its index in the container).

### Property-style tests

For non-trivial reorderings (`pop_many`, `update_priority` cycles), prefer a small property-style test: random inputs, the heap's pop-sequence must equal the sorted-input sequence by priority. Keep these tests deterministic — seed the RNG explicitly.

### `#[must_use]` enforcement in tests

Tests are not exempt from `#[must_use]`. If a test calls `pq.peek()` and ignores the result for documentation purposes, bind it to `let _ = …;` and add an `// EXPLICIT:` comment.

---

## Lint Floor (informative)

**Target floor** (the convention this document describes): `#![deny(warnings)]` and `#![forbid(unsafe_code)]` at the top of `lib.rs`, plus the standard Grit clippy floor. The current `lib.rs` does not yet declare either attribute — adopting them is a near-zero-effort hardening pass that should be tracked as a separate cleanup commit (the existing code is already safe Rust and warning-free). Once declared, any new lint introduced by a newer Clippy version that fires on existing code should be fixed at the call site, not silenced — silencing requires an `#[allow(clippy::lint_name)] // <reason>` *with* a justification in the comment.

The MSRV (minimum supported Rust version) is pinned in `Cargo.toml`. If a contribution requires a newer feature, raise the MSRV in a separate commit and note it in `CHANGELOG.md`.
