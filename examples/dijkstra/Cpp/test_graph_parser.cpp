// test_graph_parser.cpp - Fixture-based tests for graph_parser.h.
//
// Verifies that the parser:
//   * Accepts the canonical small.json format and a synthetic medium fixture
//   * Round-trips structural data (vertex count, edge fields)
//   * Rejects out-of-order keys, non-ASCII vertex IDs, and float weights
//
// Run from build directory after `cmake --build`:
//   ./test_graph_parser <path-to-graphs-dir>

#include "graph_parser.h"
#include "types.h"

#include <iostream>
#include <stdexcept>
#include <string>

namespace {

int failures = 0;

void check(bool condition, const std::string& label) {
    if (!condition) {
        std::cerr << "FAIL: " << label << "\n";
        ++failures;
    } else {
        std::cout << "  ok: " << label << "\n";
    }
}

void expect_throws(const std::string& input, const std::string& label) {
    try {
        graph_parser::parse(input);
        std::cerr << "FAIL: " << label << " (expected exception, got none)\n";
        ++failures;
    } catch (const graph_parser::ParseError& e) {
        std::cout << "  ok: " << label << " (rejected: " << e.what() << ")\n";
    } catch (const std::exception& e) {
        std::cerr << "FAIL: " << label << " (expected ParseError, got: " << e.what() << ")\n";
        ++failures;
    }
}

void test_valid_small() {
    std::cout << "[valid] small.json (textbook 6-vertex graph)\n";
    const std::string text = R"({
  "vertices": ["A", "B", "C", "D", "E", "F"],
  "edges": [
    {"from": "A", "to": "B", "weight": 6},
    {"from": "A", "to": "C", "weight": 4},
    {"from": "B", "to": "C", "weight": 2},
    {"from": "B", "to": "D", "weight": 2},
    {"from": "C", "to": "D", "weight": 1},
    {"from": "C", "to": "E", "weight": 2},
    {"from": "D", "to": "F", "weight": 7},
    {"from": "E", "to": "D", "weight": 1},
    {"from": "E", "to": "F", "weight": 3}
  ]
}
)";
    Graph g = graph_parser::parse(text);
    check(g.vertices.size() == 6, "6 vertices");
    check(g.edges.size() == 9, "9 edges");
    check(g.vertices[0] == "A" && g.vertices[5] == "F", "vertex order preserved");
    check(g.edges[0].from == "A" && g.edges[0].to == "B" && g.edges[0].weight == 6, "first edge A->B(6)");
    check(g.edges[8].from == "E" && g.edges[8].to == "F" && g.edges[8].weight == 3, "last edge E->F(3)");
}

void test_valid_negative_weight() {
    std::cout << "[valid] negative integer weight\n";
    const std::string text = R"({
  "vertices": ["v0", "v1"],
  "edges": [{"from": "v0", "to": "v1", "weight": -42}]
}
)";
    Graph g = graph_parser::parse(text);
    check(g.edges.size() == 1 && g.edges[0].weight == -42, "negative weight parsed");
}

void test_valid_empty_arrays() {
    std::cout << "[valid] empty vertices and edges arrays\n";
    const std::string text = R"({"vertices": [], "edges": []})";
    Graph g = graph_parser::parse(text);
    check(g.vertices.empty() && g.edges.empty(), "empty graph parses");
}

void test_invalid_key_order() {
    expect_throws(R"({"edges": [], "vertices": []})",
                  "[invalid] keys in wrong order (edges before vertices)");
}

void test_invalid_unicode_vertex_id() {
    // The character "café" has a non-ASCII byte sequence; bytes outside
    // [A-Za-z0-9_] must be rejected by the strict parser.
    expect_throws(std::string("{\"vertices\": [\"caf\xc3\xa9\"], \"edges\": []}"),
                  "[invalid] non-ASCII vertex ID");
}

void test_invalid_float_weight() {
    expect_throws(R"({"vertices": ["a", "b"], "edges": [{"from": "a", "to": "b", "weight": 1.5}]})",
                  "[invalid] float weight");
}

void test_invalid_missing_field() {
    expect_throws(R"({"vertices": ["a", "b"], "edges": [{"from": "a", "to": "b"}]})",
                  "[invalid] edge missing weight field");
}

void test_invalid_extra_content() {
    expect_throws(R"({"vertices": [], "edges": []} junk)",
                  "[invalid] trailing content after graph");
}

} // namespace

int main(int argc, char* argv[]) {
    test_valid_small();
    test_valid_negative_weight();
    test_valid_empty_arrays();
    test_invalid_key_order();
    test_invalid_unicode_vertex_id();
    test_invalid_float_weight();
    test_invalid_missing_field();
    test_invalid_extra_content();

    // If a graphs directory was passed, also try parsing every committed
    // .json file. This catches regressions in the parser against real
    // generator output.
    if (argc > 1) {
        const std::string base = argv[1];
        const char* names[] = {
            "small", "medium_sparse", "medium_dense", "medium_grid",
            "large_sparse", "large_dense", "large_grid"
        };
        std::cout << "\n[fixtures] parsing committed graphs from " << base << "\n";
        for (const char* name : names) {
            const std::string path = base + "/" + name + ".json";
            try {
                Graph g = graph_parser::parse_file(path);
                std::cout << "  ok: " << name << " (|V|=" << g.vertices.size()
                          << ", |E|=" << g.edges.size() << ")\n";
            } catch (const std::exception& e) {
                std::cerr << "FAIL: " << name << " - " << e.what() << "\n";
                ++failures;
            }
        }
    }

    if (failures > 0) {
        std::cerr << "\n" << failures << " test(s) failed\n";
        return 1;
    }
    std::cout << "\nAll tests passed\n";
    return 0;
}
