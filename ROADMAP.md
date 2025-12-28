# Roadmap

This document outlines the planned evolution of the d-Heap Priority Queue project.

## Philosophy

Each milestone follows a deliberate sequence:

1. **Validate before expanding** — Prove existing implementations work in real algorithms before adding new languages
2. **Fill ecosystem gaps** — Prioritize languages where quality d-ary heap libraries don't exist
3. **Teach by example** — Demonstrate *why* d-ary heaps matter through canonical use cases

---

## Current Status

**v2.3.0** — Core implementations complete for C++, Go, Rust, Zig, and TypeScript.

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
- [ ] Published on [pkg.go.dev](https://pkg.go.dev)

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

## v2.4.0 — React Flow Visualization + Complete Examples

> *Why visualization first?*

A live, interactive demo is more compelling than static benchmarks. Users can *see* why d=4 creates a shallower tree than d=2, watch `decrease_priority` bubble up in real-time, and understand the algorithm intuitively.

### React Flow Demo

- **Heap tree visualization** — See the d-ary structure as nodes are inserted/removed
- **Dijkstra step-through** — Animated shortest path on the sample graph
- **Arity toggle** — Switch between d=2, d=4, d=8 to compare tree depths
- **Operation counter** — Track inserts, pops, and decrease_priority calls

### Complete Dijkstra Examples

Same algorithm, same graph, five languages—ideal for:
- **Learning** — Compare idiomatic patterns across languages
- **Credibility** — Proves the unified API works everywhere

### Basic Benchmarks

- **Graph sizes** — Small (6 nodes), medium (~100 nodes), large (~1000 nodes)
- **Arity comparison** — d=2 vs d=4 on each graph size
- **One language** — TypeScript (fastest iteration for demo integration)

### Deliverables

- [ ] `demo/` — React Flow visualization app
- [ ] `examples/dijkstra/Cpp/`
- [ ] `examples/dijkstra/Rust/`
- [ ] `examples/dijkstra/Zig/`
- [ ] `examples/dijkstra/graphs/medium.json` — ~100 node graph
- [ ] `examples/dijkstra/graphs/large.json` — ~1000 node graph
- [ ] `benchmarks/basic/` — Simple d=2 vs d=4 comparison
- [ ] Updated README with demo link and examples matrix

---

## v2.5.0 — Extensive Benchmarks

> *Why separate from v2.4.0?*

Rigorous benchmarking requires careful methodology: multiple runs, statistical analysis, and reproducibility across machines. This deserves focused attention after the demo proves the concept.

### Deliverables

- [ ] `benchmarks/scripts/` — Benchmark runners for each language
- [ ] `benchmarks/results/` — Comparative data (d=2 vs d=4 vs d=8)
- [ ] `benchmarks/methodology.md` — Reproducible benchmark protocol
- [ ] Cross-language performance comparison (C++, Go, Rust, Zig, TypeScript)
- [ ] Dense vs sparse graph analysis
- [ ] Memory usage profiling

---

## Future Considerations

The following are under consideration but not yet scheduled:

| Item | Description |
|------|-------------|
| **Julia implementation** | No d-ary heap exists in the Julia ecosystem—significant gap |
| **Additional examples** | Prim's MST, A* search, event-driven simulation |
| **Hosted demo** | Deploy React Flow visualization to GitHub Pages or Vercel |

---

## Contributing

Interested in helping? Open an issue to discuss.

Feedback on this roadmap is welcome—open an issue titled `[Roadmap]` with your thoughts.