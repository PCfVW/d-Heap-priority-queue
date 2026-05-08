// graph_parser.h - Strict parser for the Dijkstra example graph format.
//
// Parses the constrained JSON subset specified in
// examples/dijkstra/graphs/GRAMMAR.md. Validates strictly: rejects out-of-order
// keys, non-ASCII characters in vertex IDs, float weights, missing fields, and
// any trailing content. Throws ParseError with line/column context on failure.
//
// This parser preserves the C++ example's "self-contained, dependency-free"
// pedagogical pitch (see main.cpp). It does NOT implement RFC 8259 JSON in
// general — only the subset our generator produces and our schema requires.
#pragma once

#include "types.h"

#include <charconv>
#include <fstream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <string_view>
#include <system_error>

namespace graph_parser {

class ParseError : public std::runtime_error {
public:
    ParseError(const std::string& msg, std::size_t line, std::size_t col)
        : std::runtime_error("graph parse error at line " + std::to_string(line)
                             + ", column " + std::to_string(col) + ": " + msg) {}
};

class Parser {
public:
    explicit Parser(std::string text) : text_(std::move(text)) {}

    Graph parse() {
        Graph g;
        skip_ws();
        expect('{');
        expect_key("vertices");
        parse_vertex_array(g.vertices);
        expect(',');
        expect_key("edges");
        parse_edge_array(g.edges);
        expect('}');
        skip_ws();
        if (pos_ < text_.size()) {
            throw error("unexpected trailing content");
        }
        return g;
    }

private:
    std::string text_;
    std::size_t pos_ = 0;

    [[nodiscard]] ParseError error(const std::string& msg) const {
        std::size_t line = 1, col = 1;
        for (std::size_t i = 0; i < pos_; ++i) {
            if (text_[i] == '\n') { ++line; col = 1; } else { ++col; }
        }
        return ParseError(msg, line, col);
    }

    [[nodiscard]] bool eof() const { return pos_ >= text_.size(); }

    void skip_ws() {
        while (!eof()) {
            char c = text_[pos_];
            if (c == ' ' || c == '\t' || c == '\n' || c == '\r') ++pos_;
            else break;
        }
    }

    void expect(char c) {
        skip_ws();
        if (eof() || text_[pos_] != c) {
            throw error(std::string("expected '") + c + "'");
        }
        ++pos_;
    }

    void expect_key(std::string_view key) {
        std::string parsed = parse_string();
        if (parsed != key) {
            throw error("expected key \"" + std::string(key) + "\", got \"" + parsed + "\"");
        }
        expect(':');
    }

    static bool is_id_char(char c) {
        return (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z')
            || (c >= '0' && c <= '9') || c == '_';
    }

    static bool is_digit(char c) { return c >= '0' && c <= '9'; }

    std::string parse_string() {
        skip_ws();
        if (eof() || text_[pos_] != '"') throw error("expected '\"'");
        ++pos_;
        std::string out;
        while (!eof() && text_[pos_] != '"') {
            char c = text_[pos_];
            if (!is_id_char(c)) {
                throw error("invalid character in identifier (only [A-Za-z0-9_] allowed)");
            }
            out.push_back(c);
            ++pos_;
        }
        if (eof()) throw error("unterminated string");
        if (out.empty()) throw error("empty identifier");
        ++pos_;
        return out;
    }

    int parse_int() {
        skip_ws();
        const std::size_t start = pos_;
        if (!eof() && text_[pos_] == '-') ++pos_;
        const std::size_t digits_start = pos_;
        while (!eof() && is_digit(text_[pos_])) ++pos_;
        if (pos_ == digits_start) throw error("expected integer");
        if (!eof() && (text_[pos_] == '.' || text_[pos_] == 'e' || text_[pos_] == 'E')) {
            throw error("only integer weights are allowed (no decimal or exponent)");
        }
        int value = 0;
        const auto fc = std::from_chars(text_.data() + start, text_.data() + pos_, value);
        if (fc.ec == std::errc::result_out_of_range) throw error("integer out of range");
        if (fc.ec != std::errc{}) throw error("invalid integer");
        return value;
    }

    void parse_vertex_array(std::vector<std::string>& vertices) {
        expect('[');
        skip_ws();
        if (!eof() && text_[pos_] == ']') { ++pos_; return; }
        while (true) {
            vertices.push_back(parse_string());
            skip_ws();
            if (!eof() && text_[pos_] == ',') { ++pos_; continue; }
            break;
        }
        expect(']');
    }

    void parse_edge_array(std::vector<Edge>& edges) {
        expect('[');
        skip_ws();
        if (!eof() && text_[pos_] == ']') { ++pos_; return; }
        while (true) {
            edges.push_back(parse_edge());
            skip_ws();
            if (!eof() && text_[pos_] == ',') { ++pos_; continue; }
            break;
        }
        expect(']');
    }

    Edge parse_edge() {
        Edge e;
        expect('{');
        expect_key("from");   e.from   = parse_string();   expect(',');
        expect_key("to");     e.to     = parse_string();   expect(',');
        expect_key("weight"); e.weight = parse_int();
        expect('}');
        return e;
    }
};

inline Graph parse(const std::string& text) {
    return Parser(text).parse();
}

inline Graph parse_file(const std::string& path) {
    std::ifstream f(path);
    if (!f) throw std::runtime_error("could not open graph file: " + path);
    std::stringstream ss;
    ss << f.rdbuf();
    return parse(ss.str());
}

} // namespace graph_parser
