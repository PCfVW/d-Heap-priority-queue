# Roadmap

This document outlines the planned evolution of the d-Heap Priority Queue project.

## Philosophy

Each milestone follows a deliberate sequence:

1. **Validate before expanding** — Prove existing implementations work in real algorithms before adding new languages
2. **Fill ecosystem gaps** — Prioritize languages where quality d-ary heap libraries don't exist
3. **Teach by example** — Demonstrate *why* d-ary heaps matter through canonical use cases

---

## Current Status

**v2.6.0** (released 2026-05-11) — Instrumentation & Benchmarks across all five languages. Comparison-count instrumentation (zero-cost when disabled) + cross-language Dijkstra benchmark harness (255 result files across 24 (graph, arity) cells × 5 languages; 5-way `total` invariant verified byte-for-byte). Published to [crates.io](https://crates.io/crates/d-ary-heap) and [npm](https://www.npmjs.com/package/d-ary-heap); Go module surfaced via `go/v2.6.0` on [pkg.go.dev](https://pkg.go.dev/github.com/PCfVW/d-Heap-priority-queue/Go/v2).

---

## v2.3.0 — Go Implementation ✅

> *Why Go? Why now?*

Research revealed the only existing Go d-ary heap library lacks `decrease_priority` and O(1) item lookup—the very features that make d-ary heaps useful for graph algorithms.

Adding Go after the TypeScript example (rather than before) means:
- **The example pattern exists** — Dijkstra structure is already defined
- **Architecture is validated** — `go.work` setup tested with a real use case
- **Different audience reached** — Go developers ≠ TypeScript developers

### Deliverables

- [x] `Go/` — Full implementation with API parity
- [x] `go.work` — Workspace configuration for local development
- [x] `examples/dijkstra/Go/` — Dijkstra implementation
- [x] Published on [pkg.go.dev](https://pkg.go.dev/github.com/PCfVW/d-Heap-priority-queue/Go/v2)

---

## v2.2.0 — Examples Infrastructure + TypeScript Dijkstra ✅

> *Why start here?*

Dijkstra's shortest path algorithm is the canonical reason d-ary heaps exist. It performs many more `decrease_priority` operations than `pop` operations on dense graphs. A 4-ary heap reduces `decrease_priority` from O(log₂n) to O(log₄n)—a measurable win.

Starting with TypeScript enables:
- **Fast iteration** — No compilation step; instant feedback
- **NPM momentum** — The TypeScript library is already published; users need a usage example
- **Visualization groundwork** — A future React-based visualization can import this example directly

### Deliverables

- [x] `examples/` directory structure
- [x] `examples/dijkstra/README.md` — Algorithm explanation and complexity analysis
- [x] `examples/dijkstra/graphs/small.json` — Shared test graph (used by all languages)
- [x] `examples/dijkstra/TypeScript/` — Working implementation
- [x] `benchmarks/` directory scaffold (populated in v2.4.0)

---

## v2.4.0 — React Flow Visualization ✅

> *Why visualization first?*

A live, interactive demo is more compelling than static benchmarks. Users can *see* why d=4 creates a shallower tree than d=2, watch `decrease_priority` bubble up in real-time, and understand the algorithm intuitively.

### React Flow Demo

- **Heap tree visualization** — See the d-ary structure as nodes are inserted/removed
- **Dijkstra step-through** — Animated shortest path on the sample graph
- **Arity toggle** — Switch between d=2, d=4, d=8 to compare tree depths
- **Operation counter** — Track inserts, pops, and decrease_priority calls
- **Race Mode** — Compare all three arities simultaneously

### Deliverables

- [x] `demo/` — React Flow visualization app
- [x] TypeScript instrumentation — Comparison counting for performance analysis
- [x] Updated README with demo link

---

## v2.5.0 — API Completeness + Complete Dijkstra Examples ✅

> *Why API completeness and examples together?*

API completeness enables accurate benchmarking and ensures all five implementations provide identical functionality. Complete Dijkstra examples in all five languages validate the unified API in a real algorithm.

### What Was Delivered

All five implementations now have **identical core APIs**:

| Feature | C++ | Go | Rust | Zig | TypeScript |
|---------|-----|-----|------|-----|------------|
| `updatePriority()` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `getPosition()` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `*ByIndex` methods | ✅ | ✅ | ✅ | ✅ | ✅ |
| Bulk operations | ✅ | ✅ | ✅ | ✅ | ✅ |
| `toArray()` | ✅ | ✅ | ✅ | ✅ | ✅ |

**Priority Update Semantics** standardized across all languages:
- `increasePriority()`: Only `moveUp` — O(log_d n)
- `decreasePriority()`: Only `moveDown` — O(d × log_d n)
- `updatePriority()`: Both directions — O((d+1) × log_d n)

**Language-Specific Enhancements:**
- **C++**: C++23 `std::expected<T, Error>`, safe variants (`try_*`), factory functions
- **Rust**: `Result<T, Error>`, `Display` trait, comprehensive doctests
- **Go**: Idiomatic error handling, `Position` type alias, `fmt.Stringer`
- **Zig**: Error unions, snake_case aliases, fixed `decreasePriority()` semantics
- **TypeScript**: Fixed `decreasePriority()` to match instrumentation

**Test Coverage:**
- C++: 61 tests | Rust: 97 tests | Go: 57 tests | Zig: 54 tests | TypeScript: 57 tests

### Complete Dijkstra Examples (5 Languages)

Same algorithm, same graph, five languages—validates API parity:

- [x] `examples/dijkstra/TypeScript/` — Dijkstra in TypeScript
- [x] `examples/dijkstra/Go/` — Dijkstra in Go
- [x] `examples/dijkstra/Rust/` — Dijkstra in Rust
- [x] `examples/dijkstra/Zig/` — Dijkstra in Zig
- [x] `examples/dijkstra/Cpp/` — Dijkstra in C++ (graph embedded; no standard JSON support)

All implementations:
- Share the same test graph (Network Flows Figure 4.7)
- Dynamically sort output vertices alphabetically
- Compare performance across d=2, d=4, d=8 arities

---

## v2.6.0 — Instrumentation & Benchmarks ✅

> *Why instrumentation now?*

With API parity and Dijkstra examples complete in all five languages, we can now add cross-language instrumentation for performance analysis and benchmarking.

### Phase 1: Test Graphs, Generator, Grammar Spec

Larger graphs for benchmarking, plus the tooling and specification needed to generate them reproducibly. The C++ example stays dependency-free by parsing a constrained JSON subset with a small hand-rolled parser.

#### Tooling

- [x] `benchmarks/scripts/graphgen/` — Rust binary crate (`petgraph` + seeded `rand`) that reads `benchmarks/graphs.toml` and emits canonical JSON. Provides `generate` and `verify` subcommands. Standalone crate (no root workspace).
- [x] `benchmarks/graphs.toml` — single source of truth for graph specifications: `{name, kind, n | rows×cols, seed, weight_range, target_edges?}` per graph.

#### Specification

- [x] `examples/dijkstra/graphs/GRAMMAR.md` — ISO/IEC 14977 EBNF grammar for the graph file format (a constrained, deterministic subset of RFC 8259 JSON), plus a byte-level canonical-output spec.

#### Test graphs (committed to git)

Density buckets match Phase 3 dense/sparse analysis. Edge counts target sparse `|E|≈2|V|`, dense `|E|≈|V|^1.5`. Erdős–Rényi graphs are post-processed to be connected (spanning-tree pre-pass).

- [x] `examples/dijkstra/graphs/medium_sparse.json` — n=100, |E|=200
- [x] `examples/dijkstra/graphs/medium_dense.json` — n=100, |E|=1000
- [x] `examples/dijkstra/graphs/medium_grid.json` — 10×10 lattice (|E|=360)
- [x] `examples/dijkstra/graphs/large_sparse.json` — n=1000, |E|=2000
- [x] `examples/dijkstra/graphs/large_dense.json` — n=1000, |E|=31623
- [x] `examples/dijkstra/graphs/large_grid.json` — 32×32 lattice (1024 vertices, |E|=3968)
- [x] `examples/dijkstra/graphs/huge_dense.json` — n=2500, |E|=125000 (~6 MB, added post-Phase-1 to give the indicative-timings table a discrimination-rich row)

#### Loader extension

All five Dijkstra examples gain a `--graph=<name>` flag (default `small`) and an explicit `--quiet` flag for large graphs.

- [x] TypeScript, Go, Rust, Zig — extend existing JSON loaders for new graph names
- [x] C++ — new `graph_parser.h` (~150 LOC, validates against GRAMMAR.md, fixture-tested with 18/18 cases passing). Preserves the dependency-free `small` graph path; routes other graphs through the file parser.

### Phase 2: Cross-Language Instrumentation

TypeScript instrumentation shipped in v2.4.0 (the `instrumentComparator` / `onBeforeOperation` / `onAfterOperation` API). Phase 2 extends the pattern to the remaining four languages, each using its idiomatic zero-cost mechanism:

| Language | Mechanism | Overhead When Disabled |
|----------|-----------|------------------------|
| Go | Nil stats pointer | ~1 cycle (nil check) |
| Rust | Generic over `StatsCollector` trait | Zero (monomorphization) |
| C++ | Template policy class | Zero (inlining) |
| Zig | Comptime bool parameter | Zero (branch elimination) |

- [x] Go — `Options.Stats *Stats` (commit `5490423`)
- [x] C++ — `TStats = NoOpStats` template parameter (commit `983df98`)
- [x] Rust — `S = NoOpStats` generic (commit `3669d0b`)
- [x] Zig — `comptime collect_stats: bool` (commit `fa2d3fe`)
- [x] 5-way Phase 2 invariant — all languages produce byte-for-byte identical `total` comparison counts on shared benchmarks (verified across 24 cells in Phase 3).

### Phase 3: Benchmark Infrastructure

*Depends on: Phase 2 complete (instrumentation available in all languages)*

- [x] `benchmarks/scripts/` — Benchmark runners for each language
- [x] `benchmarks/results/` — Comparative data (d=2 vs d=4 vs d=8)
- [x] `benchmarks/methodology.md` — Reproducible benchmark protocol
- [x] Cross-language performance comparison (C++, Go, Rust, Zig, TypeScript)
- [x] Dense vs sparse graph analysis
- [x] Memory usage profiling

---

## v2.7.0 — Automation & Per-Section Timing

> *Why now?*

The ~19 weekly npm downloads + the existing crates.io / pkg.go.dev cohorts make "ship more often" a real lever, but each manual cross-language release is heavy. CI/CD lands first to *make the next release cheaper*. Phase 3.5 (per-section timing inside Dijkstra) then ships under the new automation — closing the Phase 2 attribution gap (why is Go 2× faster than Rust on `huge_dense`?) and giving the new workflows a real release to prove themselves on.

Target cadence: 4–6 weeks per minor.

### CI/CD Workflows (lands first)

The standalone `deploy-demo.yml` is the only workflow today; everything else is manual.

- [ ] `.github/workflows/create-release.yml` — on `v*` tag push: extract the corresponding section from `CHANGELOG.md`, create a GitHub Release with formatted notes. Benefits all 5 languages (gives every ship a single canonical release page).
- [ ] `.github/workflows/publish-npm.yml` — on GitHub Release published: run TypeScript tests, then `npm publish`. Requires `NPM_TOKEN` secret. `prepublishOnly` script already builds.
- [ ] `.github/workflows/publish-crates.yml` — on GitHub Release published: run Rust tests, then `cargo publish`. Requires `CARGO_REGISTRY_TOKEN` secret.
- Go / C++ / Zig: no workflow needed. Go's pkg.go.dev is git-tag-driven (push `go/vX.Y.Z`); C++ and Zig distribute via the repo source. The `create-release.yml` page covers their release notes need.

### Phase 3.5 — Per-Section Timing (after CI/CD)

The methodology section is already drafted in [`benchmarks/methodology.md`](benchmarks/methodology.md) (drafted in the v2.6.0 doc pass). v2.7.0 ships the implementation.

- [ ] Add `--profile-sections` flag to all 5 Dijkstra example binaries
- [ ] Implement section timers around `setup` / `pop` / `relax` / `inc_pri` in each example's `dijkstra` implementation (cross-language consistency enforced by code review — the timer pairs wrap the same lines in each language)
- [ ] `benchmarks/scripts/<Lang>/run.ps1` gains a Pass 4 (profile-sections); writes `benchmarks/results/<Lang>/<graph>_d<arity>.profile.json` for all 120 cells
- [ ] `benchmarks/results/SECTIONS.md` cross-language attribution tables, focused on `huge_dense × d=8`: % time per section + absolute µs per section
- [ ] `CHANGELOG.md` entry + the answer to "where does Rust's extra 11 ms go?"

### Current workflows (reference)

| Workflow | Status | Purpose |
|----------|--------|---------|
| `deploy-demo.yml` | ✅ Active | Deploy React Flow demo to GitHub Pages |

---

## v2.8.0 — d-ary Huffman codec (TypeScript)

> *Why Huffman? Why TypeScript first?*

Huffman coding is the conceptual complement to Dijkstra: same priority queue, opposite usage profile (extract-min-heavy, no priority updates). The DNA storage angle gives `d > 2` heap arities a concrete reason to exist — **the heap arity *is* the encoding alphabet size**:

- Standard Huffman → binary alphabet → `d=2`
- Goldman (Nature 2013) ternary DNA storage → 3-letter alphabet → `d=3`
- ETQ quaternary (A/C/G/T) → 4-letter alphabet → `d=4`

TypeScript first because npm is the most engaged channel (~19/week) and the no-compile loop makes design iteration fastest. The other four languages follow in later releases once the file format / DNA mode CLI / test fixtures are settled.

### Deliverables

- [ ] `examples/huffman/TypeScript/` — encoder, decoder, CLI (`huffman --encode in.txt out.huf` / `--decode in.huf out.txt`)
- [ ] DNA modes via `--alphabet=binary|ternary|quaternary` (corresponding `d=2|3|4`)
- [ ] `examples/huffman/FORMAT.md` — file format spec (header, frequency table layout, code table, payload)
- [ ] Round-trip tests on multiple sample texts; size-ratio comparisons against `gzip` baseline
- [ ] CHANGELOG entry + `npm publish` via the v2.7.0 automation

Other 4 languages: deferred to v2.10.0+ (port mechanically once the design is locked).

---

## v2.9.0 — Huffman visualization

> *Why visualization?*

Same logic that made v2.4.0 React Flow demo the project's strongest marketing asset: making the algorithm visible converts evaluators into users. Huffman tree construction is visually rich (pairing the two lowest-frequency nodes, building up the tree bottom-up) and the DNA mode toggle is a unique hook — no other Huffman visualization on the web shows `d=3` ternary construction.

### Deliverables

- [ ] Add "Huffman" tab to `demo/` (alongside Dijkstra)
- [ ] Live frequency-table input; tree builds step-by-step as user types
- [ ] Animate priority queue extraction + internal-node merging
- [ ] DNA alphabet toggle (`d=2` binary / `d=3` ternary / `d=4` quaternary), with encoded output preview
- [ ] Deploys via existing `deploy-demo.yml` workflow

---

## Future Considerations

The following are under consideration but not yet scheduled:

| Item | Description |
|------|-------------|
| **Svelte Flow demo** | Parallel visualization using Svelte Flow—same xyflow maintainers, framework diversity mirrors 5-language approach |
| **Julia implementation** | No d-ary heap exists in the Julia ecosystem—significant gap |
| **WebAssembly** | Compile Rust to WASM for high-performance browser benchmarks (10k+ node graphs) |
| **MoonBit implementation** | AI-friendly language for code generation experiments (see `experiment/` directory) |
| **Multi-language Huffman codec** | Port the v2.8.0/v2.9.0 TypeScript Huffman codec to Go, Rust, C++, and Zig — same cross-language API parity story as Dijkstra. Likely v2.10.0+. |

### On Svelte Flow

> *Why consider a second visualization framework?*

The same reasoning that justified five heap implementations applies: **learning through diversity**. React Flow and Svelte Flow share identical APIs (both from [xyflow](https://github.com/xyflow/xyflow)), making the port tractable while teaching Svelte idioms (stores vs hooks, reactivity model, smaller bundle size).

If pursued, the folder structure would evolve:

```
demo/
├── shared/       # Framework-agnostic: types, algorithm, layout utils
├── react/        # React Flow demo (v2.4.0)
└── svelte/       # Svelte Flow demo (future)
```

**Decision criteria for committing:**
- React demo is stable and well-received
- Personal learning goal: understand Svelte's reactivity model
- Community interest (issues, requests)

---

## Contributing

Interested in helping? Open an issue to discuss.

Feedback on this roadmap is welcome—open an issue titled `[Roadmap]` with your thoughts.