# Roadmap

This document outlines the planned evolution of the d-Heap Priority Queue project.

## Philosophy

Each milestone follows a deliberate sequence:

1. **Validate before expanding** — Prove existing implementations work in real algorithms before adding new languages
2. **Fill ecosystem gaps** — Prioritize languages where quality d-ary heap libraries don't exist
3. **Teach by example** — Demonstrate *why* d-ary heaps matter through canonical use cases

---

## Current Status

**v2.2.0** — Core implementations complete for C++, Rust, Zig, and TypeScript.

---

## v2.2.0 — Examples Infrastructure + TypeScript Dijkstra

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

## v2.3.0 — Go Implementation

> *Why Go? Why now?*

Research revealed the only existing Go d-ary heap library lacks `decrease_priority` and O(1) item lookup—the very features that make d-ary heaps useful for graph algorithms.

Adding Go after the TypeScript example (rather than before) means:
- **The example pattern exists** — Dijkstra structure is already defined
- **Architecture is validated** — `go.work` setup tested with a real use case
- **Different audience reached** — Go developers ≠ TypeScript developers

### Deliverables

- [ ] `Go/` — Full implementation with API parity
- [ ] `go.work` — Workspace configuration for local development
- [ ] `examples/dijkstra/Go/` — Dijkstra implementation
- [ ] Published on [pkg.go.dev](https://pkg.go.dev)

---

## v2.4.0 — Complete Dijkstra Examples

> *Why wait for the other languages?*

By this point, both the example structure and a new language integration have been battle-tested. Completing the remaining examples becomes straightforward.

Same algorithm, same graph, five languages—ideal for:
- **Learning** — Compare idiomatic patterns across languages
- **Benchmarking** — Measure d=2 vs d=4 vs d=8 performance
- **Credibility** — Proves the unified API works everywhere

### Deliverables

- [ ] `examples/dijkstra/Cpp/`
- [ ] `examples/dijkstra/Rust/`
- [ ] `examples/dijkstra/Zig/`
- [ ] `benchmarks/scripts/` — Benchmark runners for each language
- [ ] `benchmarks/results/` — Comparative data (d=2 vs d=4 vs d=8)
- [ ] Updated README with complete examples matrix

---

## Future Considerations

The following are under consideration but not yet scheduled:

| Item | Description |
|------|-------------|
| **Julia implementation** | No d-ary heap exists in the Julia ecosystem—significant gap |
| **React Flow visualization** | Interactive step-through of Dijkstra with animated heap operations |
| **Additional examples** | Prim's MST, A* search, event-driven simulation |
| **Performance benchmarks** | Formal benchmarks with reproducible methodology |

---

## Contributing

Interested in helping? Open an issue to discuss.

Feedback on this roadmap is welcome—open an issue titled `[Roadmap]` with your thoughts.