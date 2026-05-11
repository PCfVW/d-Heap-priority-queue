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
#include <cctype>
#include <chrono>
#include <cstdint>
#include <filesystem>
#include <format>
#include <fstream>
#include <iostream>
#include <optional>
#include <sstream>
#include <stdexcept>
#include <string>
#include <string_view>
#include <vector>

#ifdef _WIN32
#  define WIN32_LEAN_AND_MEAN
#  define NOMINMAX
#  include <windows.h>
#  include <psapi.h>
#endif

#ifdef _WIN32
static std::uint64_t peak_rss_kb() {
    PROCESS_MEMORY_COUNTERS info{};
    info.cb = sizeof(info);
    if (GetProcessMemoryInfo(GetCurrentProcess(), &info, info.cb)) {
        return static_cast<std::uint64_t>(info.PeakWorkingSetSize) / 1024;
    }
    return 0;
}
#else
static std::uint64_t peak_rss_kb() { return 0; }
#endif

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
    bool stats = false;  // Phase 2: count comparisons via InstrumentedPriorityQueue
    // Phase 3 benchmark flags:
    std::optional<std::size_t> arity;       // nullopt = default [2, 4, 8]
    unsigned int warmup = 0;
    unsigned int repetitions = 1;
    bool json = false;
    std::optional<std::string> env_file;
    bool report_rss = false;
};

CliArgs parse_args(int argc, char* argv[]) {
    CliArgs args;
    constexpr std::string_view graph_prefix       = "--graph=";
    constexpr std::string_view source_prefix      = "--source=";
    constexpr std::string_view target_prefix      = "--target=";
    constexpr std::string_view arity_prefix       = "--arity=";
    constexpr std::string_view warmup_prefix      = "--warmup=";
    constexpr std::string_view repetitions_prefix = "--repetitions=";
    constexpr std::string_view env_file_prefix    = "--env-file=";
    for (int i = 1; i < argc; ++i) {
        std::string_view arg = argv[i];
        if (arg.starts_with(graph_prefix)) {
            args.graph = std::string(arg.substr(graph_prefix.size()));
        } else if (arg.starts_with(source_prefix)) {
            args.source = std::string(arg.substr(source_prefix.size()));
        } else if (arg.starts_with(target_prefix)) {
            args.target = std::string(arg.substr(target_prefix.size()));
        } else if (arg.starts_with(arity_prefix)) {
            args.arity = std::stoul(std::string(arg.substr(arity_prefix.size())));
        } else if (arg.starts_with(warmup_prefix)) {
            args.warmup = static_cast<unsigned int>(std::stoul(std::string(arg.substr(warmup_prefix.size()))));
        } else if (arg.starts_with(repetitions_prefix)) {
            args.repetitions = static_cast<unsigned int>(std::stoul(std::string(arg.substr(repetitions_prefix.size()))));
        } else if (arg.starts_with(env_file_prefix)) {
            args.env_file = std::string(arg.substr(env_file_prefix.size()));
        } else if (arg == "--quiet") {
            args.quiet = true;
        } else if (arg == "--stats") {
            args.stats = true;
        } else if (arg == "--json") {
            args.json = true;
        } else if (arg == "--report-rss") {
            args.report_rss = true;
        } else {
            std::cerr << "unknown argument: " << arg << "\n"
                      << "usage: dijkstra [--graph=NAME] [--source=ID] [--target=ID] [--quiet] [--stats] "
                         "[--arity=D] [--warmup=K] [--repetitions=N] [--json] [--env-file=PATH] [--report-rss]\n";
            std::exit(2);
        }
    }
    return args;
}

static void run_json(const Graph& graph, const std::string& source, const std::string& target,
                     std::size_t d, const CliArgs& args, const std::string& env_raw) {
    if (args.stats) {
        TOOLS::InstrumentedPriorityQueue<Vertex, VertexHash, VertexCompare, VertexEqual> pq(d);
        (void)dijkstra_with_pq(graph, source, pq);
        const auto s = pq.stats();
        std::cout << std::format(
            "{{\"schema_version\":1,\"language\":\"Cpp\",\"graph\":\"{}\","
            "\"arity\":{},\"comparison_counts\":{{"
            "\"insert\":{},\"pop\":{},\"decrease_priority\":{},"
            "\"increase_priority\":{},\"update_priority\":{},\"total\":{}}}}}\n",
            args.graph, d, s.insert, s.pop, s.decrease_priority,
            s.increase_priority, s.update_priority, s.total());
        return;
    }

    for (unsigned int i = 0; i < args.warmup; ++i) {
        (void)dijkstra(graph, source, d);
    }
    for (unsigned int rep = 1; rep <= args.repetitions; ++rep) {
        auto start = std::chrono::high_resolution_clock::now();
        (void)dijkstra(graph, source, d);
        auto elapsed = std::chrono::high_resolution_clock::now() - start;
        const double wall_time_us = std::chrono::duration<double, std::micro>(elapsed).count();
        std::cout << std::format(
            "{{\"schema_version\":1,\"language\":\"Cpp\",\"graph\":\"{}\","
            "\"arity\":{},\"source\":\"{}\",\"target\":\"{}\","
            "\"rep\":{},\"wall_time_us\":{}",
            args.graph, d, source, target, rep, wall_time_us);
        if (!env_raw.empty()) {
            std::cout << ",\"env\":" << env_raw;
        }
        std::cout << "}\n";
    }
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

    const std::vector<std::size_t> arities = args.arity.has_value()
        ? std::vector<std::size_t>{*args.arity}
        : std::vector<std::size_t>{2, 4, 8};

    // Load env-file (single-line compact JSON; spliced verbatim into each wall-time record).
    std::string env_raw;
    if (args.env_file) {
        std::ifstream f(*args.env_file);
        if (!f) {
            std::cerr << "error: failed to read --env-file: " << *args.env_file << "\n";
            return 1;
        }
        std::stringstream ss;
        ss << f.rdbuf();
        env_raw = ss.str();
        while (!env_raw.empty() && std::isspace(static_cast<unsigned char>(env_raw.back()))) {
            env_raw.pop_back();
        }
    }

    // --report-rss: single dijkstra call, emit one peak-RSS record.
    if (args.report_rss) {
        if (!args.arity) {
            std::cerr << "error: --report-rss requires --arity=<d>\n";
            return 1;
        }
        const std::size_t d = *args.arity;
        (void)dijkstra(graph, source, d);
        std::cout << std::format(
            "{{\"schema_version\":1,\"language\":\"Cpp\",\"graph\":\"{}\","
            "\"arity\":{},\"peak_rss_kb\":{}}}\n",
            args.graph, d, peak_rss_kb());
        return 0;
    }

    // --json mode
    if (args.json) {
        for (std::size_t d : arities) {
            run_json(graph, source, target, d, args, env_raw);
        }
        return 0;
    }

    // Human mode (existing logic)
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

    for (std::size_t d : arities) {
        std::cout << "--- Using " << d << "-ary heap ---\n";

        DijkstraResult result;
        std::optional<TOOLS::ComparisonStats> captured_stats;

        auto start = std::chrono::high_resolution_clock::now();
        if (args.stats) {
            // Phase 2: instrumented run. Build an InstrumentedPriorityQueue locally,
            // pass it to the algorithm template, and snapshot pq.stats() into
            // captured_stats so we can print after the timing/path-cost block.
            TOOLS::InstrumentedPriorityQueue<Vertex, VertexHash, VertexCompare, VertexEqual> pq(d);
            result = dijkstra_with_pq(graph, source, pq);
            captured_stats = pq.stats();
        } else {
            result = dijkstra(graph, source, d);
        }
        auto elapsed = std::chrono::high_resolution_clock::now() - start;

        if (!args.quiet) format_results(result.distances, source);

        auto path = reconstruct_path(result.predecessors, source, target);
        std::cout << "\nShortest path from " << source
                  << " to " << target << ": ";
        if (path.has_value()) {
            for (std::size_t i = 0; i < path->size(); ++i) {
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
        std::cout << std::format("Execution time: {:.1f}µs\n", elapsed_us);

        if (captured_stats) {
            const auto& s = *captured_stats;
            std::cout << std::format(
                "Comparison counts: insert={}, pop={}, decrease_priority={}, "
                "increase_priority={}, update_priority={}, total={}\n",
                s.insert, s.pop, s.decrease_priority, s.increase_priority, s.update_priority, s.total());
        }

        std::cout << "\n";
    }

    return 0;
}
