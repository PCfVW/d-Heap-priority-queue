// dijkstra.h - Dijkstra's shortest path algorithm implementation
#pragma once

#include "types.h"
#include "Cpp/PriorityQueue.h"

#include <limits>
#include <algorithm>

/// Infinity represents an unreachable distance.
constexpr int INFINITY_DISTANCE = std::numeric_limits<int>::max();

/// Dijkstra's shortest path algorithm, parameterised over the priority queue type.
///
/// The body lives here so it can be invoked with either the default
/// `TOOLS::PriorityQueue<Vertex, ...>` (zero overhead) or
/// `TOOLS::InstrumentedPriorityQueue<Vertex, ...>` (Phase 2 comparison counts).
/// Both heap variants expose the same public API used below.
///
/// @tparam PQ    A heap type compatible with this algorithm's API.
/// @param graph  The input graph with vertices and weighted edges.
/// @param source The source vertex to find shortest paths from.
/// @param pq     A constructed-but-empty heap; caller picks d and any policy parameters.
/// @return A DijkstraResult containing distances and predecessors for path reconstruction.
template <typename PQ>
inline DijkstraResult dijkstra_with_pq(const Graph& graph, const std::string& source, PQ& pq) {
    // Build adjacency list for efficient neighbor lookup
    std::unordered_map<std::string, std::vector<std::pair<std::string, int>>> adjacency;
    for (const auto& vertex : graph.vertices) {
        adjacency[vertex] = {};
    }
    for (const auto& edge : graph.edges) {
        adjacency[edge.from].push_back({edge.to, edge.weight});
    }

    // Initialize distances and predecessors
    std::unordered_map<std::string, int> distances;
    std::unordered_map<std::string, std::optional<std::string>> predecessors;

    // Set initial distances and add to priority queue
    for (const auto& vertex : graph.vertices) {
        int distance = (vertex == source) ? 0 : INFINITY_DISTANCE;
        distances[vertex] = distance;
        predecessors[vertex] = std::nullopt;
        pq.insert(Vertex{vertex, distance});
    }

    // Main algorithm loop
    while (!pq.is_empty()) {
        auto current = pq.pop_front();
        if (!current) break;

        // Skip if we've already found a shorter path
        if (current->distance > distances[current->id]) {
            continue;
        }

        // Skip if current distance is infinity (unreachable)
        if (current->distance == INFINITY_DISTANCE) {
            continue;
        }

        // Check all neighbors
        for (const auto& [neighbor_id, weight] : adjacency[current->id]) {
            int new_distance = current->distance + weight;

            if (new_distance < distances[neighbor_id]) {
                distances[neighbor_id] = new_distance;
                predecessors[neighbor_id] = current->id;

                // Update priority in queue
                // In a min-heap, decreasing distance = increasing priority (more important)
                Vertex neighbor_vertex{neighbor_id, 0}; // dummy distance for contains check
                if (pq.contains(neighbor_vertex)) {
                    pq.increase_priority(Vertex{neighbor_id, new_distance});
                }
            }
        }
    }

    return DijkstraResult{distances, predecessors};
}

/// Default-heap convenience wrapper. Constructs a zero-overhead
/// `TOOLS::PriorityQueue<Vertex, ...>(d)` and delegates to `dijkstra_with_pq`.
inline DijkstraResult dijkstra(const Graph& graph, const std::string& source, size_t d) {
    TOOLS::PriorityQueue<Vertex, VertexHash, VertexCompare, VertexEqual> pq(d);
    return dijkstra_with_pq(graph, source, pq);
}

/// Reconstructs the shortest path from source to target using predecessors.
///
/// Builds the path by following predecessor links backwards from the target,
/// then reversing the result.
///
/// @param predecessors Predecessor map from dijkstra result
/// @param source       Source vertex
/// @param target       Target vertex
/// @return A vector of vertex IDs representing the path, or nullopt if no path exists.
inline std::optional<std::vector<std::string>> reconstruct_path(
    const std::unordered_map<std::string, std::optional<std::string>>& predecessors,
    const std::string& source,
    const std::string& target
) {
    auto it = predecessors.find(target);
    if (it == predecessors.end()) {
        return std::nullopt;
    }
    if (!it->second.has_value() && target != source) {
        return std::nullopt; // No path exists
    }

    std::vector<std::string> path;
    std::optional<std::string> current = target;

    while (current.has_value()) {
        path.push_back(current.value());
        auto pred_it = predecessors.find(current.value());
        if (pred_it != predecessors.end()) {
            current = pred_it->second;
        } else {
            current = std::nullopt;
        }
    }

    std::reverse(path.begin(), path.end());

    if (!path.empty() && path[0] == source) {
        return path;
    }

    return std::nullopt;
}
