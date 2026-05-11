//! Dijkstra's Algorithm Example
//!
//! Demonstrates Dijkstra's shortest path algorithm using d-ary heap priority queues.

const std = @import("std");
const builtin = @import("builtin");
const d_heap = @import("d-heap");
const dijkstra_mod = @import("dijkstra.zig");
const types = @import("types.zig");

const Graph = types.Graph;
const DijkstraResult = types.DijkstraResult;
const INFINITY = dijkstra_mod.INFINITY;

const rss_impl = if (builtin.os.tag == .windows)
    @import("rss_windows.zig")
else
    @import("rss_other.zig");

const CliArgs = struct {
    // `graph` is always allocator-owned (default duped in parseArgs) so the
    // caller can free it unconditionally — no string-comparison guard needed.
    graph: []const u8,
    source: ?[]const u8 = null,
    target: ?[]const u8 = null,
    quiet: bool = false,
    stats: bool = false,
    arity: u32 = 0, // 0 = default [2, 4, 8]
    warmup: u32 = 0,
    repetitions: u32 = 1,
    json: bool = false,
    env_file: ?[]const u8 = null,
    report_rss: bool = false,
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
        } else if (std.mem.eql(u8, arg, "--stats")) {
            args.stats = true;
        } else if (std.mem.startsWith(u8, arg, "--arity=")) {
            args.arity = try std.fmt.parseInt(u32, arg["--arity=".len..], 10);
        } else if (std.mem.startsWith(u8, arg, "--warmup=")) {
            args.warmup = try std.fmt.parseInt(u32, arg["--warmup=".len..], 10);
        } else if (std.mem.startsWith(u8, arg, "--repetitions=")) {
            args.repetitions = try std.fmt.parseInt(u32, arg["--repetitions=".len..], 10);
        } else if (std.mem.eql(u8, arg, "--json")) {
            args.json = true;
        } else if (std.mem.startsWith(u8, arg, "--env-file=")) {
            if (args.env_file) |e| allocator.free(e);
            args.env_file = try allocator.dupe(u8, arg["--env-file=".len..]);
        } else if (std.mem.eql(u8, arg, "--report-rss")) {
            args.report_rss = true;
        } else {
            std.debug.print("unknown argument: {s}\n", .{arg});
            std.debug.print("usage: zig build run -- [--graph=NAME] [--source=ID] [--target=ID] [--quiet] [--stats] [--arity=D] [--warmup=K] [--repetitions=N] [--json] [--env-file=PATH] [--report-rss]\n", .{});
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

fn runJSON(
    graph: Graph,
    source: []const u8,
    target: []const u8,
    d: usize,
    graph_name: []const u8,
    stats_flag: bool,
    warmup: u32,
    repetitions: u32,
    env_raw: []const u8,
    allocator: std.mem.Allocator,
) !void {
    const stdout = std.fs.File.stdout();
    var buf: [1024]u8 = undefined;

    if (stats_flag) {
        const r = try dijkstra_mod.dijkstraInstrumented(graph, source, d, allocator);
        var result = r.result;
        defer result.distances.deinit();
        defer result.predecessors.deinit();
        const s = r.stats;
        const line = try std.fmt.bufPrint(
            &buf,
            "{{\"schema_version\":1,\"language\":\"Zig\",\"graph\":\"{s}\",\"arity\":{d},\"comparison_counts\":{{\"insert\":{d},\"pop\":{d},\"decrease_priority\":{d},\"increase_priority\":{d},\"update_priority\":{d},\"total\":{d}}}}}\n",
            .{ graph_name, d, s.insert_count, s.pop_count, s.decrease_priority_count, s.increase_priority_count, s.update_priority_count, s.total() },
        );
        try stdout.writeAll(line);
        return;
    }

    var i: u32 = 0;
    while (i < warmup) : (i += 1) {
        var r = try dijkstra_mod.dijkstra(graph, source, d, allocator);
        r.distances.deinit();
        r.predecessors.deinit();
    }
    var rep: u32 = 1;
    while (rep <= repetitions) : (rep += 1) {
        const start = std.time.nanoTimestamp();
        var r = try dijkstra_mod.dijkstra(graph, source, d, allocator);
        const elapsed = std.time.nanoTimestamp() - start;
        r.distances.deinit();
        r.predecessors.deinit();
        const wall_time_us = @as(f64, @floatFromInt(elapsed)) / 1000.0;
        const head = try std.fmt.bufPrint(
            &buf,
            "{{\"schema_version\":1,\"language\":\"Zig\",\"graph\":\"{s}\",\"arity\":{d},\"source\":\"{s}\",\"target\":\"{s}\",\"rep\":{d},\"wall_time_us\":{d}",
            .{ graph_name, d, source, target, rep, wall_time_us },
        );
        try stdout.writeAll(head);
        if (env_raw.len > 0) {
            try stdout.writeAll(",\"env\":");
            try stdout.writeAll(env_raw);
        }
        try stdout.writeAll("}\n");
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
    defer if (args.env_file) |e| allocator.free(e);

    var graph_arena = std.heap.ArenaAllocator.init(allocator);
    defer graph_arena.deinit();
    const graph = try loadGraph(args.graph, graph_arena.allocator());

    const source = args.source orelse if (std.mem.eql(u8, args.graph, "small")) "A" else graph.vertices[0];
    const target = args.target orelse if (std.mem.eql(u8, args.graph, "small")) "F" else graph.vertices[graph.vertices.len - 1];

    // Load env file (allocator-owned bytes; trimmed of trailing whitespace).
    var env_buf: []u8 = &.{};
    defer if (env_buf.len > 0) allocator.free(env_buf);
    var env_raw: []const u8 = "";
    if (args.env_file) |path| {
        const file = try std.fs.cwd().openFile(path, .{});
        defer file.close();
        const file_size = try file.getEndPos();
        env_buf = try allocator.alloc(u8, file_size);
        _ = try file.readAll(env_buf);
        env_raw = std.mem.trimRight(u8, env_buf, " \t\r\n");
    }

    // --report-rss: single dijkstra call, emit one peak-RSS record.
    if (args.report_rss) {
        if (args.arity == 0) {
            std.debug.print("error: --report-rss requires --arity=<d>\n", .{});
            std.process.exit(1);
        }
        const d: usize = @as(usize, args.arity);
        var r = try dijkstra_mod.dijkstra(graph, source, d, allocator);
        r.distances.deinit();
        r.predecessors.deinit();
        const peak = rss_impl.peakRssKb() orelse 0;
        const stdout = std.fs.File.stdout();
        var buf: [256]u8 = undefined;
        const line = try std.fmt.bufPrint(
            &buf,
            "{{\"schema_version\":1,\"language\":\"Zig\",\"graph\":\"{s}\",\"arity\":{d},\"peak_rss_kb\":{d}}}\n",
            .{ args.graph, d, peak },
        );
        try stdout.writeAll(line);
        return;
    }

    // Compute arities once for both --json and human modes.
    const default_arities = [_]usize{ 2, 4, 8 };
    const single_arity = [_]usize{@as(usize, args.arity)};
    const arities: []const usize = if (args.arity > 0) &single_arity else &default_arities;

    // --json mode
    if (args.json) {
        for (arities) |d| {
            try runJSON(graph, source, target, d, args.graph, args.stats, args.warmup, args.repetitions, env_raw, allocator);
        }
        return;
    }

    // Human mode (existing logic)
    std.debug.print("Dijkstra's Algorithm Example\n", .{});
    if (std.mem.eql(u8, args.graph, "small")) {
        std.debug.print("Network Flows (Ahuja, Magnanti, Orlin) - Figure 4.7\n", .{});
    } else {
        std.debug.print("graph: {s} (|V|={d}, |E|={d})\n", .{ args.graph, graph.vertices.len, graph.edges.len });
    }
    std.debug.print("Finding shortest path from {s} to {s}\n\n", .{ source, target });

    for (arities) |d| {
        std.debug.print("--- Using {d}-ary heap ---\n", .{d});

        const start = std.time.nanoTimestamp();
        var result: DijkstraResult = undefined;
        var stats: ?d_heap.ComparisonStats = null;
        if (args.stats) {
            const r = try dijkstra_mod.dijkstraInstrumented(graph, source, d, allocator);
            result = r.result;
            stats = r.stats;
        } else {
            result = try dijkstra_mod.dijkstra(graph, source, d, allocator);
        }
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
        std.debug.print("Execution time: {d:.1}µs\n", .{elapsed_us});

        if (stats) |s| {
            std.debug.print(
                "Comparison counts: insert={d}, pop={d}, decrease_priority={d}, increase_priority={d}, update_priority={d}, total={d}\n",
                .{ s.insert_count, s.pop_count, s.decrease_priority_count, s.increase_priority_count, s.update_priority_count, s.total() },
            );
        }

        std.debug.print("\n", .{});
    }
}
