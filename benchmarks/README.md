# Benchmarks

Performance comparisons for d-ary heap operations across languages and configurations.

## Status

🚧 **Coming in v2.4.0**

Benchmarks will be populated once Dijkstra examples exist for all five languages (C++, Rust, Zig, TypeScript, Go).

## Planned Comparisons

### Arity Comparison

Measure the impact of different `d` values on Dijkstra's algorithm:

| Configuration | Hypothesis |
|---------------|------------|
| d=2 (binary) | Baseline; more `decrease_priority` operations but simpler arithmetic |
| d=4 | Expected sweet spot; shallower tree, cache-friendly |
| d=8 | Diminishing returns; `pop` becomes expensive |

### Cross-Language Comparison

Same algorithm, same graph, five implementations:

| Language | Notes |
|----------|-------|
| C++ | Compiled; expected fastest |
| Rust | Compiled; comparable to C++ |
| Zig | Compiled; minimal runtime |
| Go | Compiled; GC overhead possible |
| TypeScript | JIT; expected slowest but most accessible |

### Graph Density

| Graph Type | Characteristics |
|------------|-----------------|
| Sparse | \|E\| ≈ \|V\|; `pop` dominates |
| Dense | \|E\| ≈ \|V\|²; `decrease_priority` dominates |
| Grid | Regular structure; predictable access patterns |

## Methodology

*To be defined in v2.4.0*

Principles:
- Reproducible (seeded random graphs, fixed iterations)
- Fair (warm-up runs, median of N trials)
- Documented (hardware specs, compiler versions)

## Directory Structure

```
benchmarks/
├── README.md        # This file
├── results/         # Raw benchmark data (JSON/CSV)
└── scripts/         # Benchmark runners per language
```

## Contributing

Want to help design the benchmark methodology? Open an issue titled `[Benchmarks]` with your suggestions.