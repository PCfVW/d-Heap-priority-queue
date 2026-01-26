# Dijkstra's Algorithm - C++ Implementation

This directory contains a C++ implementation of Dijkstra's shortest path algorithm using a d-ary heap priority queue.

## Files

- `types.h` - Type definitions (Graph, Edge, Vertex, DijkstraResult)
- `dijkstra.h` - Algorithm implementation (dijkstra, reconstruct_path)
- `main.cpp` - Example driver program
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
./build/Release/dijkstra
```

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

## Why No JSON Loading?

Unlike the TypeScript, Go, Rust, and Zig implementations that load from `../graphs/small.json`,
this C++ version embeds the graph data directly. This is because:

1. **No standard library JSON support**: C++ does not include JSON parsing in its standard
   library (not in C++23, nor in the upcoming C++26)
2. **Pedagogical simplicity**: Adding a third-party library (e.g., nlohmann/json) would
   introduce external dependencies, complicating the build for a simple example
3. **Self-contained**: The embedded data keeps the example dependency-free and easy to compile

The graph data in `load_graph()` matches `../graphs/small.json` exactly

## Cross-Language Alignment

This implementation follows the same structure as the TypeScript, Go, Rust, and Zig versions:

1. **Types**: Graph, Edge, Vertex (with ID-based identity), DijkstraResult
2. **Algorithm**: Builds adjacency list, uses priority queue with `increase_priority()`
3. **Output**: Consistent formatting with other implementations
