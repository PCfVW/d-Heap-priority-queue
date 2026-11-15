# d-ary-heap (TypeScript package) — Coding Conventions

This document describes the [Amphigraphic coding](https://github.com/PCfVW/Amphigraphic-Strict) conventions used in the `TypeScript/` subdirectory of the [Priority Queues](https://github.com/PCfVW/d-Heap-priority-queue) project. It is a scoped subset of [Terse — Strict TypeScript for AI-Assisted Development](https://github.com/PCfVW/Amphigraphic-Strict/tree/master/Terse), restricted to what applies to a generic, synchronous data-structure package: no `async`/`await` narrowing concerns, no enums (none exist here to begin with).

See the sibling `Rust/CONVENTIONS.md`, `Go/CONVENTIONS.md`, `Cpp/CONVENTIONS.md`, and `zig/CONVENTIONS.md` for the same project's other conventions files; this file follows the same structure so a reader who knows one can navigate the other.

## Trigger Checklist

**Before writing any line of code, check which triggers apply.**

| You are about to... | Check these rules |
|---|---|
| Write `: any` or `<any>` | [No Type Escape Hatches](#no-type-escape-hatches) — banned, `eslint` errors on it |
| Write `value!` (non-null assertion) | [Justified Non-Null Assertions](#justified-non-null-assertions) — not banned, but requires an `// eslint-disable-line ... -- INDEX:` comment |
| Write `value as T` | [Justified Type Assertions](#justified-type-assertions) — not banned, but requires an `// ASSERT:` comment |
| Declare a new exported `function` | [Explicit Return Types](#explicit-return-types) |
| Index into an array/object with a computed key | [Safe Index Access](#safe-index-access) — already enforced by `tsconfig.json` |
| Narrow a nullable value before an `await` | [Narrowing Preservation](#narrowing-preservation) — no `await` in this package today; kept for future async code |
| Add a `enum` | [Union Types Over Enums](#union-types-over-enums) |
| Add a test | [Testing conventions](#testing-conventions) |

---

## When Writing Types

### No Type Escape Hatches

`any` is banned outright, enforced by `eslint.config.js`:

```js
'@typescript-eslint/no-explicit-any': 'error',
```

`: any`/`<any>` do not appear anywhere in `src/` today (verified before turning this rule from its previous `'warn'` to `'error'`; a sibling line also had a typo, `explicit-function-return-types` — plural, not a real rule name — that ESLint silently ignored, fixed to `explicit-function-return-type` as part of adopting this document). Use a type guard instead of a cast:

```typescript
// BANNED
function process(data: any): any { ... }

// Terse: type guards
function isUser(value: unknown): value is User {
  return typeof value === 'object' && value !== null && 'id' in value;
}
```

### Justified Non-Null Assertions

Terse Rule 1 bans non-null assertions (`value!`) outright, on the reasoning that a `!` usually papers over a case the type checker correctly flagged as possibly-`null`/`undefined`. This package does **not** adopt a hard ban: `noUncheckedIndexedAccess` (see [Safe Index Access](#safe-index-access) below) makes every `container[i]` read `T | undefined`, and the hot-path heap operations (`swap`, `moveUp`, `moveDown`, `bestChildPosition`, `pop`, `insertMany`) index at positions whose validity is proven by the algorithm's own loop bounds, not by a redundant runtime check on every access — exactly the tension Rust's `Rust/CONVENTIONS.md` `INDEX:` annotation exists to resolve, applied here to TypeScript's structurally-identical problem.

`@typescript-eslint/no-non-null-assertion` is set to `'warn'`, and every `!` in `src/` must carry a trailing `// eslint-disable-line @typescript-eslint/no-non-null-assertion -- INDEX: <why this index is provably in bounds>` comment. The `INDEX:` justification is the same annotation name and shape as Rust's, so the convention transfers directly for anyone reading both codebases; written as the directive's `--` description, it also silences the warning for that line only:

```typescript
private swap(i: number, j: number): void {
  const container = this.container;
  const temp = container[i]!; // eslint-disable-line @typescript-eslint/no-non-null-assertion -- INDEX: i, j are always caller-supplied valid heap positions
  container[i] = container[j]!; // eslint-disable-line @typescript-eslint/no-non-null-assertion -- INDEX: see above
  container[j] = temp;
  ...
}
```

So `npm run lint` is silent while every assertion is justified, and a new `!` without the directive shows up as a warning (in CI too, as an annotation), instead of drowning among the justified ones. All 13 non-null assertions in `src/PriorityQueue.ts` (on 10 lines) carry the directive as of this document — a new one without it should be treated as a review-blocking gap, not accepted on the assumption that the pattern is "obviously fine because the rest of the file does it." If a future case genuinely can't prove the index in bounds, don't reach for `!` — check explicitly (Terse's original intent) instead of annotating around a real bug.

### Justified Type Assertions

Unlike `any` and `!`, `as T` type assertions are not banned by ESLint in this package — one exists today for a structurally-legitimate reason (see below), and a blanket lint ban would need to special-case it anyway. Instead, every `as` assertion must carry an inline `// ASSERT: <why the structural type can't be inferred here>` comment, mirroring the `Rust/CONVENTIONS.md` `CAST:`/`INDEX:`/`BORROW:` annotation pattern and the `Cpp/CONVENTIONS.md` `// COPY:`/`// MAP_ACCESS:` markers — declared, not automatically enforced; a reviewer checks for it.

The reference example, `src/instrumentation.ts`:

```typescript
const instrumented = ((a: T, b: T): boolean => {
  ...
  return comparator(a, b);
}) as InstrumentedComparator<T>; // ASSERT: the `stats`/`startOperation`/`endOperation` members are attached imperatively below; no structural type can express "this plain function will gain these properties before it's returned"
```

This is a legitimate use: `InstrumentedComparator<T>` extends a plain function type with extra properties attached via `Object.defineProperty` *after* the function literal is created — no assignment-time structural type could describe that without the assertion. A new `as` should meet the same bar (the value's true shape is provably correct but not expressible to the type checker at that point) — if a type guard or a differently-ordered construction can eliminate the assertion instead, prefer that.

---

## When Writing Functions

### Explicit Return Types

Exported functions declare their return type explicitly rather than relying on inference — `instrumentComparator<T>(comparator: Comparator<T>): InstrumentedComparator<T>` is the pattern to match. `tsconfig.json` does not currently set `noImplicitAny`-adjacent return-type enforcement beyond `strict: true`, and the ESLint rule for this (`@typescript-eslint/explicit-function-return-type`) is explicitly turned `'off'` today — this is a documented convention enforced by review, not yet by lint, for exported functions specifically (internal helpers and callbacks are exempt; inferring `(a, b) => boolean` for a short arrow callback is fine).

### Safe Index Access

Already enforced at the compiler level — `tsconfig.json` sets `"noUncheckedIndexedAccess": true`, so `items[0]` types as `T | undefined` rather than `T`, and `exactOptionalPropertyTypes: true` keeps `undefined` and "property absent" distinct. No further action needed here; this is called out so a future contributor doesn't assume it's missing and re-derive it.

### Narrowing Preservation

Not currently exercised — this package has no `async`/`await` code (the heap and its Dijkstra example are synchronous). If async code is introduced, capture a narrowed nullable value into a `const` *before* the `await` that would otherwise widen it back, per Terse Rule 4:

```typescript
if (user == null) return;
const validUser = user; // NARROWED
await someOperation();
console.log(validUser.name); // still safe
```

---

## When Writing Enums

### Union Types Over Enums

No `enum` exists in this package's public API today (`OperationType` and similar discriminators are string-literal unions or plain string keys) — keep new discriminated values as union types (`type Status = 'active' | 'pending'`) rather than introducing a TypeScript `enum`, which has runtime-object overhead and less predictable emitted code than a union type.

---

## Testing Conventions

### Where tests live

`TypeScript/tests/` (Vitest, `npm test` → `vitest run`), separate from `src/`. Note that ESLint's `files` glob in `eslint.config.js` currently scopes to `src/**/*.ts` only — tests are not linted under the rules above; this is an existing, deliberate scope limit, not something this pass changes.

### What to assert

For every modifier method, assert at minimum:
1. The observable result (returned value, thrown error for `front()` on empty).
2. The heap-property invariant after the call.
3. The position-map invariant.

---

## Lint Floor (informative)

**Active floor**: `tsconfig.json`'s `strict: true` plus `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`, `noUnusedLocals`, `noUnusedParameters`, `noImplicitReturns`, and `noFallthroughCasesInSwitch` are all already turned on — this package's compiler-level floor was already close to Terse before this document existed. `eslint.config.js` is the newer, lighter piece: `no-explicit-any` is `'error'` (see [No Type Escape Hatches](#no-type-escape-hatches)); `no-non-null-assertion` is `'warn'`, deliberately not `'error'`, because ESLint has no "allow with a justifying comment" mode and 13 sites in `src/PriorityQueue.ts` are legitimately annotated `// INDEX:` uses rather than bugs (see [Justified Non-Null Assertions](#justified-non-null-assertions)) — enforcement of "every `!` has an `// INDEX:` comment" is by code review, the same honest declared-vs-enforced gap as the `CAST:`/`INDEX:`/`BORROW:` annotations in `Rust/CONVENTIONS.md`. Everything else in the rules block is `@eslint/js` + `typescript-eslint` recommended defaults.

Run `npm run lint` and `npm run typecheck` before committing — both are fast (no network, no build step) and catch the Terse-validated failure class (implicit-`any`) plus surface any new, unannotated non-null assertion as a warning worth a second look.
