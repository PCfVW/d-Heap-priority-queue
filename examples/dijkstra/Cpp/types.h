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
