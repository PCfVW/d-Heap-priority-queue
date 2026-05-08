// main.cpp - Dijkstra's Algorithm Example
//
// Demonstrates Dijkstra's shortest path algorithm using d-ary heap priority queues.
//
// Note on JSON: Unlike TypeScript, Go, Rust, and Zig, the C++ standard library does not
// include JSON support (not in C++23, nor in the upcoming C++26). We avoid pulling in a
// third-party JSON library by exploiting the fact that our graph format is a tightly
// constrained subset of JSON specified in examples/dijkstra/graphs/GRAMMAR.md. The small
// textbook graph (--graph=small, default) is embedded directly to keep the pedagogical
// path self-contained and dependency-free; larger benchmark graphs are loaded via
// graph_parser.h, a ~150-line strict parser that conforms to GRAMMAR.md.

#include "dijkstra.h"
#include "graph_parser.h"

#include <algorithm>
#include <chrono>
#include <filesystem>
#include <format>
#include <fstream>
#include <iostream>
#include <optional>
#include <sstream>
#include <stdexcept>
#include <string>
#include <string_view>

/// Embedded textbook example: Network Flows (Ahuja, Magnanti, Orlin) - Figure 4.7.
///
/// Kept embedded so the default `dijkstra` invocation has no file dependencies.
Graph load_small_embedded() {
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

Graph load_graph(const std::string& name) {
    if (name == "small") return load_small_embedded();
    namespace fs = std::filesystem;
    const std::string filename = name + ".json";
    const fs::path candidates[] = {
        fs::path("..") / "graphs" / filename,
        fs::path("examples") / "dijkstra" / "graphs" / filename
    };
    for (const auto& p : candidates) {
        std::ifstream f(p);
        if (f) {
            std::stringstream ss;
            ss << f.rdbuf();
            return graph_parser::parse(ss.str());
        }
    }
    throw std::runtime_error("graph file not found for --graph=" + name
                             + " (looked in ../graphs/ and examples/dijkstra/graphs/)");
}

void format_results(const std::unordered_map<std::string, int>& distances,
                    const std::string& source) {
    std::cout << "Shortest paths from vertex " << source << ":\n";
    std::cout << "================================\n";

    std::vector<std::string> vertices;
    vertices.reserve(distances.size());
    for (const auto& [vertex, _] : distances) vertices.push_back(vertex);
    std::sort(vertices.begin(), vertices.end());

    for (const auto& vertex : vertices) {
        auto it = distances.find(vertex);
        std::cout << source << " → " << vertex << ": ";
        if (it->second == INFINITY_DISTANCE) std::cout << "∞";
        else std::cout << it->second;
        std::cout << "\n";
    }
}

struct CliArgs {
    std::string graph = "small";
    std::optional<std::string> source;
    std::optional<std::string> target;
    bool quiet = false;
};

CliArgs parse_args(int argc, char* argv[]) {
    CliArgs args;
    constexpr std::string_view graph_prefix = "--graph=";
    constexpr std::string_view source_prefix = "--source=";
    constexpr std::string_view target_prefix = "--target=";
    for (int i = 1; i < argc; ++i) {
        std::string_view arg = argv[i];
        if (arg.starts_with(graph_prefix)) {
            args.graph = std::string(arg.substr(graph_prefix.size()));
        } else if (arg.starts_with(source_prefix)) {
            args.source = std::string(arg.substr(source_prefix.size()));
        } else if (arg.starts_with(target_prefix)) {
            args.target = std::string(arg.substr(target_prefix.size()));
        } else if (arg == "--quiet") {
            args.quiet = true;
        } else {
            std::cerr << "unknown argument: " << arg << "\n"
                      << "usage: dijkstra [--graph=NAME] [--source=ID] [--target=ID] [--quiet]\n";
            std::exit(2);
        }
    }
    return args;
}

int main(int argc, char* argv[]) {
    CliArgs args = parse_args(argc, argv);

    Graph graph;
    try {
        graph = load_graph(args.graph);
    } catch (const std::exception& e) {
        std::cerr << "error: " << e.what() << "\n";
        return 1;
    }

    if (graph.vertices.empty()) {
        std::cerr << "error: graph has no vertices\n";
        return 1;
    }

    // Default endpoints: textbook A→F for `small`; v0→v(N-1) otherwise.
    const std::string source = args.source.value_or(
        args.graph == "small" ? "A" : graph.vertices.front());
    const std::string target = args.target.value_or(
        args.graph == "small" ? "F" : graph.vertices.back());

    std::cout << "Dijkstra's Algorithm Example\n";
    if (args.graph == "small") {
        std::cout << "Network Flows (Ahuja, Magnanti, Orlin) - Figure 4.7\n";
    } else {
        std::cout << "graph: " << args.graph
                  << " (|V|=" << graph.vertices.size()
                  << ", |E|=" << graph.edges.size() << ")\n";
    }
    std::cout << "Finding shortest path from " << source
              << " to " << target << "\n\n";

    std::vector<size_t> arities = {2, 4, 8};
    for (size_t d : arities) {
        std::cout << "--- Using " << d << "-ary heap ---\n";

        auto start = std::chrono::high_resolution_clock::now();
        DijkstraResult result = dijkstra(graph, source, d);
        auto elapsed = std::chrono::high_resolution_clock::now() - start;

        if (!args.quiet) format_results(result.distances, source);

        auto path = reconstruct_path(result.predecessors, source, target);
        std::cout << "\nShortest path from " << source
                  << " to " << target << ": ";
        if (path.has_value()) {
            for (size_t i = 0; i < path->size(); ++i) {
                if (i > 0) std::cout << " → ";
                std::cout << (*path)[i];
            }
        } else {
            std::cout << "No path found";
        }
        std::cout << "\n";

        auto target_dist_it = result.distances.find(target);
        if (target_dist_it != result.distances.end()) {
            std::cout << "Path cost: " << target_dist_it->second << "\n";
        }

        const double elapsed_us = std::chrono::duration<double, std::micro>(elapsed).count();
        std::cout << std::format("Execution time: {:.1f}µs\n\n", elapsed_us);
    }

    return 0;
}
