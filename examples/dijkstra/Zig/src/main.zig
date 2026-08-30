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
    profile_sections: bool = false,
    timer_overhead: bool = false,
};

/// Back-to-back calls to `std.Io.Clock.Timestamp.now` timed as a batch and
/// divided down for one `--timer-overhead` trial — timing a single call
/// directly would be dominated by the very timer resolution being measured.
const timer_overhead_iterations: u32 = 1_000_000;
/// Independent trials, so the aggregator can report median/IQR the same way
/// it does for every other measurement in this project (methodology.md's
/// "Reported summary statistics").
const timer_overhead_trials: u32 = 10;

fn parseArgs(allocator: std.mem.Allocator, process_args: std.process.Args) !CliArgs {
    var args = CliArgs{ .graph = try allocator.dupe(u8, "small") };
    // Zig 0.16 removed the freestanding `argsWithAllocator` helper; the
    // args vector is now handed to `main` via `std.process.Init.Minimal`
    // and iterated through `Args.Iterator.initAllocator` (cross-platform,
    // needed on Windows/WASI; POSIX's allocator-free `Iterator.init` would
    // also work but this project targets Windows first).
    var it = try std.process.Args.Iterator.initAllocator(process_args, allocator);
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
        } else if (std.mem.eql(u8, arg, "--profile-sections")) {
            args.profile_sections = true;
        } else if (std.mem.eql(u8, arg, "--timer-overhead")) {
            args.timer_overhead = true;
        } else {
            std.debug.print("unknown argument: {s}\n", .{arg});
            std.debug.print("usage: zig build run -- [--graph=NAME] [--source=ID] [--target=ID] [--quiet] [--stats] [--arity=D] [--warmup=K] [--repetitions=N] [--json] [--env-file=PATH] [--report-rss] [--profile-sections] [--timer-overhead]\n", .{});
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
///
/// `io` is the process-wide `std.Io` implementation (Zig 0.16 threads it
/// explicitly through every filesystem call instead of hanging I/O off
/// `std.fs` directly).
fn loadGraph(name: []const u8, io: std.Io, allocator: std.mem.Allocator) !Graph {
    const filename = try std.fmt.allocPrint(allocator, "{s}.json", .{name});
    const candidates = [_][]const u8{
        try std.fs.path.join(allocator, &[_][]const u8{ "..", "graphs", filename }),
        try std.fs.path.join(allocator, &[_][]const u8{ "examples", "dijkstra", "graphs", filename }),
    };

    for (candidates) |path| {
        if (std.Io.Dir.cwd().openFile(io, path, .{})) |file| {
            defer file.close(io);
            const file_size = try file.length(io);
            const buffer = try allocator.alloc(u8, file_size);
            _ = try file.readPositionalAll(io, buffer, 0);
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
    io: std.Io,
) !void {
    const stdout = std.Io.File.stdout();
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
        try stdout.writeStreamingAll(io, line);
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
        // Zig 0.16 removed `std.time.nanoTimestamp`; monotonic timing now
        // goes through the process-wide `Io` implementation.
        const start = std.Io.Clock.Timestamp.now(io, .awake);
        var r = try dijkstra_mod.dijkstra(graph, source, d, allocator);
        const elapsed = start.untilNow(io).raw.nanoseconds;
        r.distances.deinit();
        r.predecessors.deinit();
        const wall_time_us = @as(f64, @floatFromInt(elapsed)) / 1000.0;
        const head = try std.fmt.bufPrint(
            &buf,
            "{{\"schema_version\":1,\"language\":\"Zig\",\"graph\":\"{s}\",\"arity\":{d},\"source\":\"{s}\",\"target\":\"{s}\",\"rep\":{d},\"wall_time_us\":{d}",
            .{ graph_name, d, source, target, rep, wall_time_us },
        );
        try stdout.writeStreamingAll(io, head);
        if (env_raw.len > 0) {
            try stdout.writeStreamingAll(io, ",\"env\":");
            try stdout.writeStreamingAll(io, env_raw);
        }
        try stdout.writeStreamingAll(io, "}\n");
    }
}

pub fn main(init: std.process.Init) !void {
    // Zig 0.16: the full `Init` (rather than `Init.Minimal`) gives us `.gpa`
    // (leak-checked in Debug builds by the runtime, same selection logic
    // `std.start` uses for a zero-param `main`) and `.io` (the process-wide
    // `std.Io` implementation every filesystem call now threads explicitly).
    // No manual allocator setup/teardown needed — the runtime owns both.
    const allocator = init.gpa;
    const io = init.io;

    const args = try parseArgs(allocator, init.minimal.args);
    defer allocator.free(args.graph);
    defer if (args.source) |s| allocator.free(s);
    defer if (args.target) |t| allocator.free(t);
    defer if (args.env_file) |e| allocator.free(e);

    // --timer-overhead: no graph/arity involved, handled before the
    // (otherwise-unconditional) graph load below.
    if (args.timer_overhead) {
        const stdout = std.Io.File.stdout();
        var buf: [256]u8 = undefined;
        var trial: u32 = 1;
        while (trial <= timer_overhead_trials) : (trial += 1) {
            const start = std.Io.Clock.Timestamp.now(io, .awake);
            var i: u32 = 0;
            while (i < timer_overhead_iterations) : (i += 1) {
                _ = std.Io.Clock.Timestamp.now(io, .awake);
            }
            const total_ns = start.untilNow(io).raw.nanoseconds;
            const per_call_us = @as(f64, @floatFromInt(total_ns)) / 1000.0 / @as(f64, @floatFromInt(timer_overhead_iterations));
            const line = try std.fmt.bufPrint(
                &buf,
                "{{\"schema_version\":1,\"language\":\"Zig\",\"trial\":{d},\"iterations\":{d},\"per_call_us\":{d},\"per_bracket_us\":{d}}}\n",
                .{ trial, timer_overhead_iterations, per_call_us, per_call_us * 2.0 },
            );
            try stdout.writeStreamingAll(io, line);
        }
        return;
    }

    var graph_arena = std.heap.ArenaAllocator.init(allocator);
    defer graph_arena.deinit();
    const graph = try loadGraph(args.graph, io, graph_arena.allocator());

    const source = args.source orelse if (std.mem.eql(u8, args.graph, "small")) "A" else graph.vertices[0];
    const target = args.target orelse if (std.mem.eql(u8, args.graph, "small")) "F" else graph.vertices[graph.vertices.len - 1];

    // Load env file (allocator-owned bytes; trimmed of trailing whitespace).
    var env_buf: []u8 = &.{};
    defer if (env_buf.len > 0) allocator.free(env_buf);
    var env_raw: []const u8 = "";
    if (args.env_file) |path| {
        const file = try std.Io.Dir.cwd().openFile(io, path, .{});
        defer file.close(io);
        const file_size = try file.length(io);
        env_buf = try allocator.alloc(u8, file_size);
        _ = try file.readPositionalAll(io, env_buf, 0);
        env_raw = std.mem.trimEnd(u8, env_buf, " \t\r\n");
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
        const stdout = std.Io.File.stdout();
        var buf: [256]u8 = undefined;
        const line = try std.fmt.bufPrint(
            &buf,
            "{{\"schema_version\":1,\"language\":\"Zig\",\"graph\":\"{s}\",\"arity\":{d},\"peak_rss_kb\":{d}}}\n",
            .{ args.graph, d, peak },
        );
        try stdout.writeStreamingAll(io, line);
        return;
    }

    // --profile-sections: per-section timers, emit one JSON line per timed
    // call (Phase 3.5 of methodology.md). Same K=2/N=10 warmup-then-repeat
    // protocol as --json wall-time below, reused here after the first real
    // Phase 3.5 run showed a single un-iterated call is too noisy to
    // attribute cross-language gaps.
    if (args.profile_sections) {
        if (args.arity == 0) {
            std.debug.print("error: --profile-sections requires --arity=<d>\n", .{});
            std.process.exit(1);
        }
        const d: usize = @as(usize, args.arity);
        const stdout = std.Io.File.stdout();
        var buf: [576]u8 = undefined;

        var i: u32 = 0;
        while (i < args.warmup) : (i += 1) {
            var r = try dijkstra_mod.dijkstraProfiled(graph, source, d, allocator, io);
            r.result.distances.deinit();
            r.result.predecessors.deinit();
        }
        var rep: u32 = 1;
        while (rep <= args.repetitions) : (rep += 1) {
            var r = try dijkstra_mod.dijkstraProfiled(graph, source, d, allocator, io);
            r.result.distances.deinit();
            r.result.predecessors.deinit();
            const t = r.timings;
            const line = try std.fmt.bufPrint(
                &buf,
                "{{\"schema_version\":1,\"language\":\"Zig\",\"graph\":\"{s}\",\"arity\":{d},\"rep\":{d},\"pop_count\":{d},\"relax_count\":{d},\"section_us\":{{\"setup\":{d},\"pop\":{d},\"relax\":{d},\"inc_pri\":{d},\"total\":{d}}}}}\n",
                .{ args.graph, d, rep, t.pop_count, t.relax_count, t.setup_us, t.pop_us, t.relax_us, t.inc_pri_us, t.total() },
            );
            try stdout.writeStreamingAll(io, line);
        }
        return;
    }

    // Compute arities once for both --json and human modes.
    const default_arities = [_]usize{ 2, 4, 8 };
    const single_arity = [_]usize{@as(usize, args.arity)};
    const arities: []const usize = if (args.arity > 0) &single_arity else &default_arities;

    // --json mode
    if (args.json) {
        for (arities) |d| {
            try runJSON(graph, source, target, d, args.graph, args.stats, args.warmup, args.repetitions, env_raw, allocator, io);
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

        const start = std.Io.Clock.Timestamp.now(io, .awake);
        var result: DijkstraResult = undefined;
        var stats: ?d_heap.ComparisonStats = null;
        if (args.stats) {
            const r = try dijkstra_mod.dijkstraInstrumented(graph, source, d, allocator);
            result = r.result;
            stats = r.stats;
        } else {
            result = try dijkstra_mod.dijkstra(graph, source, d, allocator);
        }
        const elapsed = start.untilNow(io).raw.nanoseconds;

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
