# Dijkstra's Shortest Path Algorithm

This directory contains implementations of Dijkstra's shortest path algorithm using d-ary heap priority queues across multiple programming languages.

## Example Graph

The implementations use the example from **Figure 4.7, page 110** of Ahuja, Magnanti, and Orlin's *Network Flows* (1993). This is a classic textbook example that demonstrates Dijkstra's algorithm on a 6-vertex **directed graph**.

**Graph structure:**

```mermaid
graph LR
    A -->|6| B
    A -->|4| C
    B -->|2| C
    B -->|2| D
    C -->|1| D
    C -->|2| E
    D -->|7| F
    E -->|1| D
    E -->|3| F
    
    style A fill:#000000,stroke:#ffffff,stroke-width:2px,color:#ffffff
    style B fill:#000000,stroke:#ffffff,stroke-width:2px,color:#ffffff
    style C fill:#000000,stroke:#ffffff,stroke-width:2px,color:#ffffff
    style D fill:#000000,stroke:#ffffff,stroke-width:2px,color:#ffffff
    style E fill:#000000,stroke:#ffffff,stroke-width:2px,color:#ffffff
    style F fill:#000000,stroke:#ffffff,stroke-width:2px,color:#ffffff
    
    linkStyle 0 stroke:#ffffff,stroke-width:2px,color:#ff0000
    linkStyle 1 stroke:#ffffff,stroke-width:2px,color:#ff0000
    linkStyle 2 stroke:#ffffff,stroke-width:2px,color:#ff0000
    linkStyle 3 stroke:#ffffff,stroke-width:2px,color:#ff0000
    linkStyle 4 stroke:#ffffff,stroke-width:2px,color:#ff0000
    linkStyle 5 stroke:#ffffff,stroke-width:2px,color:#ff0000
    linkStyle 6 stroke:#ffffff,stroke-width:2px,color:#ff0000
    linkStyle 7 stroke:#ffffff,stroke-width:2px,color:#ff0000
    linkStyle 8 stroke:#ffffff,stroke-width:2px,color:#ff0000
```

**All edges:**
- A → B (6), A → C (4)
- B → C (2), B → D (2)  
- C → D (1), C → E (2)
- D → F (7)
- E → D (1), E → F (3)

**Goal:** Find the shortest path from vertex A to vertex F.

## Why d-ary Heaps for Dijkstra?

Dijkstra's algorithm is the canonical use case for d-ary heaps. The algorithm performs many more `decrease_priority` operations than `pop` operations, especially on dense graphs. A 4-ary heap provides measurable performance improvements:

- **Binary heap (d=2)**: `decrease_priority` is O(log₂n)
- **4-ary heap (d=4)**: `decrease_priority` is O(log₄n) ≈ O(log₂n / 2)

## Algorithm Overview

Dijkstra's algorithm finds the shortest path from a source vertex to all other vertices in a weighted graph with non-negative edge weights.

### Steps:
1. Initialize distances: source = 0, all others = ∞
2. Add all vertices to priority queue (min-heap by distance)
3. While queue is not empty:
   - Extract vertex with minimum distance
   - For each neighbor, if new path is shorter:
     - Update distance
     - Decrease priority in queue

### Complexity Analysis

| Operation | Binary Heap (d=2) | 4-ary Heap (d=4) | Improvement |
|-----------|-------------------|-------------------|-------------|
| Insert | O(log₂n) | O(log₄n) | ~2x faster |
| Extract-min | O(log₂n) | O(4·log₄n) | ~2x slower |
| Decrease-key | O(log₂n) | O(log₄n) | ~2x faster |

**Overall**: Since Dijkstra performs O(E) decrease-key operations vs O(V) extract-min operations, and E >> V in dense graphs, the 4-ary heap wins significantly.

## Expected Results

**Shortest path from A to F: cost = 9**
- Path: A → C → E → F
- Breakdown: A→C (4) + C→E (2) + E→F (3) = 9

**All shortest distances from A:**
- A → A: 0
- A → B: 6  
- A → C: 4
- A → D: 5 (via A→C→E→D)
- A → E: 6 (via A→C→E)
- A → F: 9 (via A→C→E→F)

## Implementation Notes

Each language implementation:
- Uses the same graph format (`graphs/small.json`)
- Implements identical algorithm logic
- Demonstrates idiomatic patterns for that language
- Includes basic testing and output formatting

## Running Examples

Currently available:
- `TypeScript/` - Node.js with npm/yarn ✅
- `Go/` - Go modules ✅

**Coming in future versions** (see [ROADMAP.md](https://github.com/PCfVW/d-Heap-priority-queue/blob/master/ROADMAP.md)):
- `Cpp/` - C++ with standard compiler
- `Rust/` - Cargo build system
- `Zig/` - Zig build system

### Running the Go Example

```bash
cd examples/dijkstra/Go
go run .
```