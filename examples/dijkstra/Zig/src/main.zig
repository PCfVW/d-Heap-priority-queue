//! Dijkstra's Algorithm Example
//!
//! Demonstrates Dijkstra's shortest path algorithm using d-ary heap priority queues.

const std = @import("std");
const dijkstra_mod = @import("dijkstra.zig");
const types = @import("types.zig");

const Graph = types.Graph;
const INFINITY = dijkstra_mod.INFINITY;

const CliArgs = struct {
    // `graph` is always allocator-owned (default duped in parseArgs) so the
    // caller can free it unconditionally — no string-comparison guard needed.
    graph: []const u8,
    source: ?[]const u8 = null,
    target: ?[]const u8 = null,
    quiet: bool = false,
};

fn parseArgs(allocator: std.mem.Allocator) !CliArgs {
    var args = CliArgs{ .graph = try allocator.dupe(u8, "small") };
    var it = try std.process.argsWithAllocator(allocator);
    defer it.deinit();
    _ = it.next(); // skip program name

    while (it.next()) |arg| {
        if (std.mem.startsWith(u8, arg, "--graph=")) {
            allocator.free(args.graph);
            args.graph = try allocator.dupe(u8, arg["--graph=".len..]);
        } else if (std.mem.startsWith(u8, arg, "--source=")) {
            if (args.source) |s| allocator.free(s);
            args.source = try allocator.dupe(u8, arg["--source=".len..]);
        } else if (std.mem.startsWith(u8, arg, "--target=")) {
            if (args.target) |t| allocator.free(t);
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

/// Load a named graph using `allocator`. All allocations (path strings, file
/// buffer, parsed JSON content, and the Graph's slices) live until `allocator`
/// is freed, so callers should pass an ArenaAllocator and free it wholesale
/// once the Graph is no longer needed. parseFromSliceLeaky deserializes
/// directly into Graph (whose fields match the JSON schema) — no per-string
/// dupe loop, no separate cleanup.
fn loadGraph(name: []const u8, allocator: std.mem.Allocator) !Graph {
    const filename = try std.fmt.allocPrint(allocator, "{s}.json", .{name});
    const candidates = [_][]const u8{
        try std.fs.path.join(allocator, &[_][]const u8{ "..", "graphs", filename }),
        try std.fs.path.join(allocator, &[_][]const u8{ "examples", "dijkstra", "graphs", filename }),
    };

    for (candidates) |path| {
        if (std.fs.cwd().openFile(path, .{})) |file| {
            defer file.close();
            const file_size = try file.getEndPos();
            const buffer = try allocator.alloc(u8, file_size);
            _ = try file.readAll(buffer);
            return try std.json.parseFromSliceLeaky(Graph, allocator, buffer, .{});
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
    defer allocator.free(args.graph);
    defer if (args.source) |s| allocator.free(s);
    defer if (args.target) |t| allocator.free(t);

    var graph_arena = std.heap.ArenaAllocator.init(allocator);
    defer graph_arena.deinit();
    const graph = try loadGraph(args.graph, graph_arena.allocator());

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
