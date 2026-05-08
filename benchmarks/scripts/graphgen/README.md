# graphgen

Reproducible graph generator for the Dijkstra examples. Reads `benchmarks/graphs.toml` and emits JSON files conforming to [`examples/dijkstra/graphs/GRAMMAR.md`](../../../examples/dijkstra/graphs/GRAMMAR.md).

## Determinism

All randomness flows from a single per-graph `seed` through `rand_chacha::ChaCha8Rng`. The crate pins `petgraph`, `rand`, and `rand_chacha` to exact versions; `Cargo.lock` is committed. Generated JSON files are committed to git so examples remain runnable without invoking this tool.

## Commands

Run from the crate directory (`benchmarks/scripts/graphgen/`):

```bash
# Generate all graphs declared in ../../graphs.toml into ../../../examples/dijkstra/graphs/
cargo run --release -- generate

# Re-emit each graph and compare byte-for-byte against the committed file.
# Non-zero exit if any drift is detected.
cargo run --release -- verify
```

## Adding a graph

1. Add a `[[graph]]` block to [`benchmarks/graphs.toml`](../../graphs.toml).
2. Run `cargo run --release -- generate`.
3. Commit the new JSON file.
4. Update [`examples/dijkstra/README.md`](../../../examples/dijkstra/README.md)'s graph corpus listing.

## Generators

| `kind` | Description |
|---|---|
| `erdos_renyi` | Directed graph with arborescence-enforced connectivity from `v0`. Sample `target_edges` directed edges, no self-loops, no duplicates. |
| `grid` | 4-connected `rows × cols` lattice. Each lattice connection emits two directed edges with the same weight. |

## Files

```
benchmarks/scripts/graphgen/
├── Cargo.toml
├── Cargo.lock          (committed)
├── README.md
└── src/
    ├── main.rs         CLI dispatcher
    ├── config.rs       TOML schema
    ├── graph.rs        in-memory graph type alias (petgraph)
    ├── emit.rs         canonical-JSON writer
    ├── verify.rs       byte-diff against committed files
    └── generators/
        ├── mod.rs
        ├── erdos_renyi.rs
        └── grid.rs
```
