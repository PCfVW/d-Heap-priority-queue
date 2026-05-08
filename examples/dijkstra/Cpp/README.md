# Dijkstra's Algorithm - C++ Implementation

This directory contains a C++ implementation of Dijkstra's shortest path algorithm using a d-ary heap priority queue.

## Files

- `types.h` - Type definitions (Graph, Edge, Vertex, DijkstraResult)
- `dijkstra.h` - Algorithm implementation (dijkstra, reconstruct_path)
- `graph_parser.h` - Strict parser for the GRAMMAR.md JSON subset (no third-party deps)
- `main.cpp` - Example driver program
- `test_graph_parser.cpp` - Fixture tests for the parser
- `CMakeLists.txt` - CMake build configuration

## Building

The implementation requires C++23.

```bash
cd examples/dijkstra/Cpp
cmake -B build
cmake --build build --config Release
```

## Running

```bash
./build/Release/dijkstra                              # default: --graph=small (embedded)
./build/Release/dijkstra --graph=medium_sparse        # load via graph_parser.h
./build/Release/dijkstra --graph=large_grid --quiet   # suppress per-vertex output
./build/Release/test_graph_parser examples/dijkstra/graphs   # run parser fixture tests
```

Available graph names: `small`, `medium_sparse`, `medium_dense`, `medium_grid`, `large_sparse`, `large_dense`, `large_grid`. See [`../graphs/GRAMMAR.md`](../graphs/GRAMMAR.md) for the format spec.

## Expected Output

```
Dijkstra's Algorithm Example
Network Flows (Ahuja, Magnanti, Orlin) - Figure 4.7
Finding shortest path from A to F

--- Using 2-ary heap ---
Shortest paths from vertex A:
================================
A -> A: 0
A -> B: 6
A -> C: 4
A -> D: 5
A -> E: 6
A -> F: 9

Shortest path from A to F: A -> C -> E -> F
Path cost: 9
Execution time: ...us

--- Using 4-ary heap ---
...

--- Using 8-ary heap ---
...
```

## Implementation Notes

- Uses the `PriorityQueue.h` header from the main C++ implementation
- The Vertex type uses ID-based equality and hashing (distance is ignored for lookups)
- Tests heap arities 2, 4, and 8 to demonstrate d-ary heap flexibility

## Why a hand-rolled parser instead of nlohmann/json?

C++ has no standard-library JSON support, and the other four implementations (TypeScript, Go, Rust, Zig) get JSON parsing for free from their standard libraries. To keep this C++ example dependency-free while still loading the same graph files as its siblings, the project defines a constrained, deterministic JSON subset in [`../graphs/GRAMMAR.md`](../graphs/GRAMMAR.md) — fixed key order, ASCII-only vertex IDs, integer weights — and parses it with [`graph_parser.h`](graph_parser.h) (~150 lines, no external deps, fixture-tested).

The `--graph=small` path additionally keeps its data embedded in `main.cpp`, so the default tutorial invocation needs no file I/O at all.

## Cross-Language Alignment

This implementation follows the same structure as the TypeScript, Go, Rust, and Zig versions:

1. **Types**: Graph, Edge, Vertex (with ID-based identity), DijkstraResult
2. **Algorithm**: Builds adjacency list, uses priority queue with `increase_priority()`
3. **Output**: Consistent formatting with other implementations
