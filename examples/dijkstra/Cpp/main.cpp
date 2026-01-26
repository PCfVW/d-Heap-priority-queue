// main.cpp - Dijkstra's Algorithm Example
//
// Demonstrates Dijkstra's shortest path algorithm using d-ary heap priority queues.
//
// Note on JSON: Unlike TypeScript, Go, Rust, and Zig, the C++ standard library does not
// include JSON support (not in C++23, nor in the upcoming C++26). While third-party
// libraries like nlohmann/json exist, we embed the graph data directly here to keep
// this pedagogical example self-contained and dependency-free. The graph data matches
// exactly what is in ../graphs/small.json.

#include "dijkstra.h"

#include <iostream>
#include <chrono>
#include <algorithm>

/// Load the example graph from Network Flows (Ahuja, Magnanti, Orlin) - Figure 4.7
///
/// The graph is embedded directly rather than loaded from JSON because C++ has no
/// standard library JSON support. The data matches ../graphs/small.json exactly.
Graph load_graph() {
    Graph graph;
    graph.vertices = {"A", "B", "C", "D", "E", "F"};
    graph.edges = {
        {"A", "B", 6},
        {"A", "C", 4},
        {"B", "C", 2},
        {"B", "D", 2},
        {"C", "D", 1},
        {"C", "E", 2},
        {"D", "F", 7},
        {"E", "D", 1},
        {"E", "F", 3}
    };
    return graph;
}

void format_results(const std::unordered_map<std::string, int>& distances, const std::string& source) {
    std::cout << "Shortest paths from vertex " << source << ":\n";
    std::cout << "================================\n";

    // Sort vertices for consistent output
    std::vector<std::string> vertices;
    for (const auto& [vertex, _] : distances) {
        vertices.push_back(vertex);
    }
    std::sort(vertices.begin(), vertices.end());

    for (const auto& vertex : vertices) {
        auto it = distances.find(vertex);
        std::cout << source << " \xe2\x86\x92 " << vertex << ": ";
        if (it->second == INFINITY_DISTANCE) {
            std::cout << "inf";
        } else {
            std::cout << it->second;
        }
        std::cout << "\n";
    }
}

int main() {
    Graph graph = load_graph();
    std::string source = "A";
    std::string target = "F";

    std::cout << "Dijkstra's Algorithm Example\n";
    std::cout << "Network Flows (Ahuja, Magnanti, Orlin) - Figure 4.7\n";
    std::cout << "Finding shortest path from " << source << " to " << target << "\n\n";

    // Test with different heap arities
    std::vector<size_t> arities = {2, 4, 8};

    for (size_t d : arities) {
        std::cout << "--- Using " << d << "-ary heap ---\n";

        auto start = std::chrono::high_resolution_clock::now();
        DijkstraResult result = dijkstra(graph, source, d);
        auto elapsed = std::chrono::high_resolution_clock::now() - start;

        format_results(result.distances, source);

        auto path = reconstruct_path(result.predecessors, source, target);
        std::cout << "\nShortest path from " << source << " to " << target << ": ";
        if (path.has_value()) {
            for (size_t i = 0; i < path->size(); ++i) {
                if (i > 0) std::cout << " \xe2\x86\x92 ";
                std::cout << (*path)[i];
            }
        } else {
            std::cout << "No path found";
        }
        std::cout << "\n";

        std::cout << "Path cost: " << result.distances.at(target) << "\n";

        auto elapsed_us = std::chrono::duration_cast<std::chrono::microseconds>(elapsed).count();
        std::cout << "Execution time: " << elapsed_us << "us\n\n";
    }

    return 0;
}
