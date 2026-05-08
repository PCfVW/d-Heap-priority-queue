//! Dijkstra's Algorithm Example
//!
//! Demonstrates Dijkstra's shortest path algorithm using d-ary heap priority queues.

const std = @import("std");
const dijkstra_mod = @import("dijkstra.zig");
const types = @import("types.zig");

const Graph = types.Graph;
const Edge = types.Edge;
const INFINITY = dijkstra_mod.INFINITY;

const CliArgs = struct {
    graph: []const u8 = "small",
    source: ?[]const u8 = null,
    target: ?[]const u8 = null,
    quiet: bool = false,
};

fn parseArgs(allocator: std.mem.Allocator) !CliArgs {
    var args = CliArgs{};
    var it = try std.process.argsWithAllocator(allocator);
    defer it.deinit();
    _ = it.next(); // skip program name

    while (it.next()) |arg| {
        if (std.mem.startsWith(u8, arg, "--graph=")) {
            args.graph = try allocator.dupe(u8, arg["--graph=".len..]);
        } else if (std.mem.startsWith(u8, arg, "--source=")) {
            args.source = try allocator.dupe(u8, arg["--source=".len..]);
        } else if (std.mem.startsWith(u8, arg, "--target=")) {
            args.target = try allocator.dupe(u8, arg["--target=".len..]);
        } else if (std.mem.eql(u8, arg, "--quiet")) {
            args.quiet = true;
        } else {
            std.debug.print("unknown argument: {s}\n", .{arg});
            std.debug.print("usage: zig build run -- [--graph=NAME] [--source=ID] [--target=ID] [--quiet]\n", .{});
            std.process.exit(2);
        }
    }
    return args;
}

fn loadGraph(name: []const u8, allocator: std.mem.Allocator) !Graph {
    const filename = try std.fmt.allocPrint(allocator, "{s}.json", .{name});
    defer allocator.free(filename);

    const candidates = [_][]const u8{
        try std.fs.path.join(allocator, &[_][]const u8{ "..", "graphs", filename }),
        try std.fs.path.join(allocator, &[_][]const u8{ "examples", "dijkstra", "graphs", filename }),
    };
    defer for (candidates) |c| allocator.free(c);

    for (candidates) |path| {
        if (std.fs.cwd().openFile(path, .{})) |file| {
            defer file.close();
            return loadGraphFromFile(file, allocator);
        } else |err| switch (err) {
            error.FileNotFound => continue,
            else => return err,
        }
    }
    std.debug.print(
        "graph file not found for --graph={s} (looked in ../graphs/ and examples/dijkstra/graphs/)\n",
        .{name},
    );
    return error.FileNotFound;
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

    var vertices = std.ArrayList([]const u8).empty;
    defer vertices.deinit(allocator);

    var it = distances.keyIterator();
    while (it.next()) |vertex| {
        try vertices.append(allocator, vertex.*);
    }

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

    const args = try parseArgs(allocator);
    defer if (!std.mem.eql(u8, args.graph, "small")) allocator.free(args.graph);
    defer if (args.source) |s| allocator.free(s);
    defer if (args.target) |t| allocator.free(t);

    const graph = try loadGraph(args.graph, allocator);
    defer {
        for (graph.vertices) |v| allocator.free(v);
        allocator.free(graph.vertices);
        for (graph.edges) |e| {
            allocator.free(e.from);
            allocator.free(e.to);
        }
        allocator.free(graph.edges);
    }

    const source = args.source orelse if (std.mem.eql(u8, args.graph, "small")) "A" else graph.vertices[0];
    const target = args.target orelse if (std.mem.eql(u8, args.graph, "small")) "F" else graph.vertices[graph.vertices.len - 1];

    std.debug.print("Dijkstra's Algorithm Example\n", .{});
    if (std.mem.eql(u8, args.graph, "small")) {
        std.debug.print("Network Flows (Ahuja, Magnanti, Orlin) - Figure 4.7\n", .{});
    } else {
        std.debug.print("graph: {s} (|V|={d}, |E|={d})\n", .{ args.graph, graph.vertices.len, graph.edges.len });
    }
    std.debug.print("Finding shortest path from {s} to {s}\n\n", .{ source, target });

    const arities = [_]usize{ 2, 4, 8 };
    for (arities) |d| {
        std.debug.print("--- Using {d}-ary heap ---\n", .{d});

        const start = std.time.nanoTimestamp();
        var result = try dijkstra_mod.dijkstra(graph, source, d, allocator);
        const elapsed = std.time.nanoTimestamp() - start;

        defer result.distances.deinit();
        defer result.predecessors.deinit();

        if (!args.quiet) try formatResults(result.distances, source, allocator);

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

        if (result.distances.get(target)) |path_cost| {
            std.debug.print("Path cost: {d}\n", .{path_cost});
        }

        const elapsed_us = @as(f64, @floatFromInt(elapsed)) / 1000.0;
        std.debug.print("Execution time: {d:.1}µs\n\n", .{elapsed_us});
    }
}
