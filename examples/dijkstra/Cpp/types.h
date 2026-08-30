// types.h - Type definitions for the Dijkstra example
#pragma once

#include <string>
#include <vector>
#include <unordered_map>
#include <optional>
#include <functional>

/// Graph represents a weighted directed graph.
struct Graph {
    std::vector<std::string> vertices;
    std::vector<struct Edge> edges;
};

/// Edge represents a weighted directed edge.
struct Edge {
    std::string from;
    std::string to;
    int weight;
};

/// Vertex represents a vertex with its current distance from the source.
///
/// Used as the item type in the priority queue. The priority queue uses
/// the vertex ID for lookup (via `contains()` and `increase_priority()`),
/// so equality and hashing are based only on the `id` field, not `distance`.
/// This allows updating a vertex's priority by providing a new distance value.
struct Vertex {
    std::string id;
    int distance;
};

/// Hash function for Vertex - based only on ID
struct VertexHash {
    std::size_t operator()(const Vertex& v) const {
        return std::hash<std::string>{}(v.id);
    }
};

/// Equality function for Vertex - based only on ID
struct VertexEqual {
    bool operator()(const Vertex& a, const Vertex& b) const {
        return a.id == b.id;
    }
};

/// Comparator for min-heap by distance
struct VertexCompare {
    bool operator()(const Vertex& a, const Vertex& b) const {
        return a.distance < b.distance;
    }
};

/// DijkstraResult contains the output of Dijkstra's algorithm.
struct DijkstraResult {
    /// Distances maps each vertex to its shortest distance from the source.
    std::unordered_map<std::string, int> distances;
    /// Predecessors maps each vertex to its predecessor in the shortest path.
    /// nullopt value means no predecessor (source or unreachable).
    std::unordered_map<std::string, std::optional<std::string>> predecessors;
};

/// Wall-clock time spent in each of Dijkstra's four algorithmic sections, as
/// measured by `dijkstra_profiled` via `std::chrono::high_resolution_clock`.
/// All fields are microseconds.
///
/// `total()` is the sum of the four sections — a distinct number from the
/// Phase 3 wall-time median (see `benchmarks/methodology.md`, "Phase 3.5"):
/// this struct measures one un-iterated call including timer overhead, not a
/// warmup + N-repetition steady state.
///
/// Cross-language equivalents:
/// - Go: `SectionTimings` (dijkstra.go)
/// - Rust: `SectionTimings` (dijkstra.rs)
/// - Zig: `SectionTimings` (dijkstra.zig)
/// - TypeScript: `SectionTimings` (dijkstra.ts)
struct SectionTimings {
    /// Adjacency-list build + distances/predecessors init + the initial
    /// `insert` call per vertex. One-shot, before the main loop.
    double setup_us = 0.0;
    /// Sum of every `pq.pop_front()` call inside the main loop. Heap-only.
    double pop_us = 0.0;
    /// Sum of the per-edge distance computation, comparison against
    /// `distances`, and (when improved) the writes to `distances` and
    /// `predecessors`. Excludes the heap `contains`/`increase_priority`
    /// pair, timed separately as `inc_pri_us`.
    double relax_us = 0.0;
    /// Sum of the `contains` check and conditional `increase_priority`
    /// call, timed only on edges where `relax_us` found a strictly shorter
    /// path.
    double inc_pri_us = 0.0;

    /// Sum of all four sections. See the struct-level doc comment for why
    /// this differs from the Phase 3 wall-time median.
    [[nodiscard]] double total() const {
        return setup_us + pop_us + relax_us + inc_pri_us;
    }
};
