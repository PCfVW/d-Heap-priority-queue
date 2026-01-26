//! Dijkstra's Algorithm Example
//!
//! Demonstrates Dijkstra's shortest path algorithm using d-ary heap priority queues.

const std = @import("std");
const dijkstra_mod = @import("dijkstra.zig");
const types = @import("types.zig");

const Graph = types.Graph;
const Edge = types.Edge;
const INFINITY = dijkstra_mod.INFINITY;

fn loadGraph(allocator: std.mem.Allocator) !Graph {
    // Try relative path from the example directory
    const graph_path = "../graphs/small.json";

    const file = std.fs.cwd().openFile(graph_path, .{}) catch |err| {
        // Try alternative path for when running from project root
        if (err == error.FileNotFound) {
            const alt_path = "examples/dijkstra/graphs/small.json";
            return loadGraphFromPath(alt_path, allocator);
        }
        return err;
    };
    defer file.close();

    return loadGraphFromFile(file, allocator);
}

fn loadGraphFromPath(path: []const u8, allocator: std.mem.Allocator) !Graph {
    const file = try std.fs.cwd().openFile(path, .{});
    defer file.close();
    return loadGraphFromFile(file, allocator);
}

fn loadGraphFromFile(file: std.fs.File, allocator: std.mem.Allocator) !Graph {
    const file_size = try file.getEndPos();
    const buffer = try allocator.alloc(u8, file_size);
    defer allocator.free(buffer);

    _ = try file.readAll(buffer);

    const parsed = try std.json.parseFromSlice(
        struct {
            vertices: [][]const u8,
            edges: []struct {
                from: []const u8,
                to: []const u8,
                weight: i32,
            },
        },
        allocator,
        buffer,
        .{},
    );
    defer parsed.deinit();

    // Convert to our types
    var vertices = try allocator.alloc([]const u8, parsed.value.vertices.len);
    for (parsed.value.vertices, 0..) |v, i| {
        vertices[i] = try allocator.dupe(u8, v);
    }

    var edges = try allocator.alloc(Edge, parsed.value.edges.len);
    for (parsed.value.edges, 0..) |e, i| {
        edges[i] = .{
            .from = try allocator.dupe(u8, e.from),
            .to = try allocator.dupe(u8, e.to),
            .weight = e.weight,
        };
    }

    return Graph{
        .vertices = vertices,
        .edges = edges,
    };
}

fn formatResults(distances: std.StringHashMap(i32), source: []const u8, allocator: std.mem.Allocator) !void {
    std.debug.print("Shortest paths from vertex {s}:\n", .{source});
    std.debug.print("================================\n", .{});

    // Sort vertices for consistent output
    var vertices = std.ArrayList([]const u8).empty;
    defer vertices.deinit(allocator);

    var it = distances.keyIterator();
    while (it.next()) |vertex| {
        try vertices.append(allocator, vertex.*);
    }

    // Sort vertices for consistent output
    std.mem.sort([]const u8, vertices.items, {}, struct {
        fn lessThan(_: void, a: []const u8, b: []const u8) bool {
            return std.mem.order(u8, a, b) == .lt;
        }
    }.lessThan);

    for (vertices.items) |vertex| {
        const distance = distances.get(vertex).?;
        if (distance == INFINITY) {
            std.debug.print("{s} → {s}: ∞\n", .{ source, vertex });
        } else {
            std.debug.print("{s} → {s}: {d}\n", .{ source, vertex, distance });
        }
    }
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const graph = try loadGraph(allocator);
    defer {
        for (graph.vertices) |v| allocator.free(v);
        allocator.free(graph.vertices);
        for (graph.edges) |e| {
            allocator.free(e.from);
            allocator.free(e.to);
        }
        allocator.free(graph.edges);
    }

    const source = "A";
    const target = "F";

    std.debug.print("Dijkstra's Algorithm Example\n", .{});
    std.debug.print("Network Flows (Ahuja, Magnanti, Orlin) - Figure 4.7\n", .{});
    std.debug.print("Finding shortest path from {s} to {s}\n\n", .{ source, target });

    // Test with different heap arities
    const arities = [_]usize{ 2, 4, 8 };

    for (arities) |d| {
        std.debug.print("--- Using {d}-ary heap ---\n", .{d});

        const start = std.time.nanoTimestamp();
        var result = try dijkstra_mod.dijkstra(graph, source, d, allocator);
        const elapsed = std.time.nanoTimestamp() - start;

        defer result.distances.deinit();
        defer result.predecessors.deinit();

        try formatResults(result.distances, source, allocator);

        const path = try dijkstra_mod.reconstructPath(result.predecessors, source, target, allocator);
        defer if (path) |p| allocator.free(p);

        if (path) |p| {
            std.debug.print("\nShortest path from {s} to {s}: ", .{ source, target });
            for (p, 0..) |vertex, i| {
                if (i > 0) std.debug.print(" → ", .{});
                std.debug.print("{s}", .{vertex});
            }
            std.debug.print("\n", .{});
        } else {
            std.debug.print("\nShortest path from {s} to {s}: No path found\n", .{ source, target });
        }

        const path_cost = result.distances.get(target).?;
        std.debug.print("Path cost: {d}\n", .{path_cost});

        const elapsed_us = @as(f64, @floatFromInt(elapsed)) / 1000.0;
        std.debug.print("Execution time: {d:.1}µs\n\n", .{elapsed_us});
    }
}
