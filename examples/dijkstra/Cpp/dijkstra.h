// dijkstra.h - Dijkstra's shortest path algorithm implementation
#pragma once

#include "types.h"
#include "Cpp/PriorityQueue.h"

#include <limits>
#include <algorithm>

/// Infinity represents an unreachable distance.
constexpr int INFINITY_DISTANCE = std::numeric_limits<int>::max();

/// Dijkstra's shortest path algorithm using a d-ary heap priority queue.
///
/// Finds the shortest paths from a source vertex to all other vertices in a weighted
/// graph with non-negative edge weights.
///
/// @param graph  The input graph with vertices and weighted edges
/// @param source The source vertex to find shortest paths from
/// @param d      The arity of the heap (typically 4 for optimal performance)
/// @return A DijkstraResult containing distances and predecessors for path reconstruction.
inline DijkstraResult dijkstra(const Graph& graph, const std::string& source, size_t d) {
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

    // Create priority queue with min-heap by distance
    TOOLS::PriorityQueue<Vertex, VertexHash, VertexCompare, VertexEqual> pq(d);

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
