//! Dijkstra's shortest path algorithm implementation.

const std = @import("std");
const d_heap = @import("d-heap");
const types = @import("types.zig");

const Graph = types.Graph;
const Vertex = types.Vertex;
const DijkstraResult = types.DijkstraResult;

/// Infinity represents an unreachable distance.
pub const INFINITY: i32 = std.math.maxInt(i32);

/// Min-heap comparator for Vertex: lower distance = higher priority.
fn minByDistance(a: Vertex, b: Vertex) bool {
    return a.distance < b.distance;
}

/// Neighbor represents a graph neighbor with edge weight.
const Neighbor = struct {
    to: []const u8,
    weight: i32,
};

/// Comptime helpers — both heap variants reuse the same `(Context, Comparator)`
/// pair, so we name them here once.
const VertexContext = d_heap.HashContext(Vertex);
const VertexComparator = d_heap.Comparator(Vertex);

/// `dijkstraInstrumented` returns this — the algorithm result paired with the
/// per-bucket comparison counts collected during the run. Mirrors C++'s
/// `DijkstraResultWithStats`, Go's `DijkstraInstrumented` return tuple, and
/// Rust's `(DijkstraResult, ComparisonStats)` tuple.
pub const DijkstraInstrumentedResult = struct {
    result: DijkstraResult,
    stats: d_heap.ComparisonStats,
};

/// Dijkstra's shortest path algorithm using a d-ary heap priority queue.
///
/// Finds the shortest paths from a source vertex to all other vertices in a weighted
/// graph with non-negative edge weights.
///
/// ## Arguments
///
/// * `graph` - The input graph with vertices and weighted edges
/// * `source` - The source vertex to find shortest paths from
/// * `d` - The arity of the heap (typically 4 for optimal performance)
/// * `allocator` - Memory allocator for dynamic allocations
///
/// ## Returns
///
/// A `DijkstraResult` containing distances and predecessors for path reconstruction.
pub fn dijkstra(
    graph: Graph,
    source: []const u8,
    d: usize,
    allocator: std.mem.Allocator,
) !DijkstraResult {
    const PriorityQueue = d_heap.DHeap(Vertex, VertexContext, VertexComparator);
    var pq = try PriorityQueue.init(d, .{ .cmp = minByDistance }, allocator);
    defer pq.deinit();
    return runDijkstra(PriorityQueue, graph, source, &pq, allocator);
}

/// Like `dijkstra`, but with Phase 2 comparison-count instrumentation enabled.
/// Returns the algorithm result alongside the per-operation comparison-count
/// buckets. Mirrors C++ `dijkstra_with_stats`, Go `DijkstraInstrumented`, and
/// Rust `dijkstra_instrumented`.
pub fn dijkstraInstrumented(
    graph: Graph,
    source: []const u8,
    d: usize,
    allocator: std.mem.Allocator,
) !DijkstraInstrumentedResult {
    const PriorityQueue = d_heap.InstrumentedDHeap(Vertex, VertexContext, VertexComparator);
    var pq = try PriorityQueue.init(d, .{ .cmp = minByDistance }, allocator);
    defer pq.deinit();
    const result = try runDijkstra(PriorityQueue, graph, source, &pq, allocator);
    return .{ .result = result, .stats = pq.stats };
}

/// Wall-clock time spent in each of Dijkstra's four algorithmic sections, as
/// measured by `dijkstraProfiled` via `std.Io.Clock.Timestamp` (Zig 0.16
/// removed `std.time.nanoTimestamp`; monotonic timing now goes through the
/// process-wide `Io` implementation). All fields are microseconds.
///
/// `total()` is the sum of the four sections — a distinct number from the
/// Phase 3 wall-time median (see `benchmarks/methodology.md`, "Phase 3.5"):
/// this struct measures one un-iterated call including timer overhead, not a
/// warmup + N-repetition steady state.
///
/// Cross-language equivalents:
/// - C++: `SectionTimings` (dijkstra.h)
/// - Go: `SectionTimings` (dijkstra.go)
/// - Rust: `SectionTimings` (dijkstra.rs)
/// - TypeScript: `SectionTimings` (dijkstra.ts)
pub const SectionTimings = struct {
    /// Adjacency-list build + distances/predecessors init + the initial
    /// `insert` call per vertex. One-shot, before the main loop.
    setup_us: f64 = 0,
    /// Sum of every `pq.pop()` call inside the main loop. Heap-only.
    pop_us: f64 = 0,
    /// Sum of the per-edge distance computation, comparison against
    /// `distances`, and (when improved) the writes to `distances` and
    /// `predecessors`. Excludes the heap `contains`/`increasePriority` pair,
    /// timed separately as `inc_pri_us`.
    relax_us: f64 = 0,
    /// Sum of the `contains` check and conditional `increasePriority` call,
    /// timed only on edges where `relax_us` found a strictly shorter path.
    inc_pri_us: f64 = 0,

    /// Sum of all four sections. See the struct-level doc comment for why
    /// this differs from the Phase 3 wall-time median.
    pub fn total(self: SectionTimings) f64 {
        return self.setup_us + self.pop_us + self.relax_us + self.inc_pri_us;
    }
};

/// Returns a fresh monotonic timestamp on the `.awake` clock, via the
/// process-wide `Io` implementation.
fn nowTimestamp(io: std.Io) std.Io.Clock.Timestamp {
    return std.Io.Clock.Timestamp.now(io, .awake);
}

/// Microseconds elapsed between `start` and now.
fn elapsedUs(start: std.Io.Clock.Timestamp, io: std.Io) f64 {
    const ns = start.untilNow(io).raw.nanoseconds;
    return @as(f64, @floatFromInt(ns)) / 1000.0;
}

/// Like `dijkstra`, but instruments the four Phase 3.5 sections (`setup`,
/// `pop`, `relax`, `inc_pri`) with `std.Io.Clock.Timestamp` and returns their
/// timings alongside the result. Used only by the `--profile-sections`
/// example flag — a dedicated function rather than threading instrumentation
/// through `runDijkstra`, so the Phase 3 wall-time/`--stats` hot paths carry
/// no extra branching from this instrumentation.
///
/// The section boundaries are the cross-language-consistent split documented
/// in `benchmarks/methodology.md` ("Phase 3.5: per-section timing"): `relax`
/// covers the edge-weight comparison and (when improved) the
/// `distances`/`predecessors` writes; `inc_pri` covers the heap `contains`
/// lookup and conditional `increasePriority` call that follows an improved
/// edge — split via an explicit `improved` boolean rather than re-evaluating
/// the comparison after `distances` has already been mutated. The
/// stale-entry and unreachable-vertex `continue` checks between `pop` and
/// the neighbor loop are not separately timed — a small, symmetric gap
/// explicitly licensed by methodology.md's "mutually exclusive and (within
/// rounding) exhaustive" framing.
///
/// `io` is the process-wide `std.Io` implementation the timer reads through.
pub fn dijkstraProfiled(
    graph: Graph,
    source: []const u8,
    d: usize,
    allocator: std.mem.Allocator,
    io: std.Io,
) !struct { result: DijkstraResult, timings: SectionTimings } {
    const PriorityQueue = d_heap.DHeap(Vertex, VertexContext, VertexComparator);
    var pq = try PriorityQueue.init(d, .{ .cmp = minByDistance }, allocator);
    defer pq.deinit();

    var timings = SectionTimings{};
    const setup_start = nowTimestamp(io);

    // Build adjacency list for efficient neighbor lookup
    const NeighborList = std.ArrayList(Neighbor);
    var adjacency = std.StringHashMap(NeighborList).init(allocator);
    defer {
        var it = adjacency.valueIterator();
        while (it.next()) |list| {
            list.deinit(allocator);
        }
        adjacency.deinit();
    }

    for (graph.vertices) |vertex| {
        try adjacency.put(vertex, NeighborList.empty);
    }

    for (graph.edges) |edge| {
        var list = adjacency.getPtr(edge.from).?;
        try list.append(allocator, .{ .to = edge.to, .weight = edge.weight });
    }

    // Initialize distances and predecessors
    var distances = std.StringHashMap(i32).init(allocator);
    var predecessors = std.StringHashMap(?[]const u8).init(allocator);

    // Set initial distances and add to priority queue
    for (graph.vertices) |vertex| {
        const distance = if (std.mem.eql(u8, vertex, source)) 0 else INFINITY;
        try distances.put(vertex, distance);
        try predecessors.put(vertex, null);
        try pq.insert(.{ .id = vertex, .distance = distance });
    }
    timings.setup_us = elapsedUs(setup_start, io);

    // Main algorithm loop
    while (!pq.isEmpty()) {
        const pop_start = nowTimestamp(io);
        const current = (try pq.pop()) orelse break;
        timings.pop_us += elapsedUs(pop_start, io);

        // Stale-entry / unreachable-vertex skip checks: not separately timed
        // (see the function doc comment) — sub-100ns, symmetric across
        // languages, folded into methodology.md's "within rounding" caveat.
        if (current.distance > distances.get(current.id).?) {
            continue;
        }
        if (current.distance == INFINITY) {
            continue;
        }

        if (adjacency.get(current.id)) |neighbors| {
            for (neighbors.items) |neighbor| {
                const relax_start = nowTimestamp(io);
                const new_distance = current.distance + neighbor.weight;
                const improved = new_distance < distances.get(neighbor.to).?;
                if (improved) {
                    try distances.put(neighbor.to, new_distance);
                    try predecessors.put(neighbor.to, current.id);
                }
                timings.relax_us += elapsedUs(relax_start, io);

                if (improved) {
                    const inc_pri_start = nowTimestamp(io);
                    if (pq.contains(.{ .id = neighbor.to, .distance = 0 })) {
                        try pq.increasePriority(.{ .id = neighbor.to, .distance = new_distance });
                    }
                    timings.inc_pri_us += elapsedUs(inc_pri_start, io);
                }
            }
        }
    }

    return .{
        .result = DijkstraResult{
            .distances = distances,
            .predecessors = predecessors,
        },
        .timings = timings,
    };
}

/// Shared algorithm body, generic over the priority-queue type so both
/// `dijkstra` (no stats) and `dijkstraInstrumented` (stats on) reuse it.
/// `comptime PQ: type` is monomorphised by Zig — no runtime cost.
fn runDijkstra(
    comptime PQ: type,
    graph: Graph,
    source: []const u8,
    pq: *PQ,
    allocator: std.mem.Allocator,
) !DijkstraResult {
    // Build adjacency list for efficient neighbor lookup
    const NeighborList = std.ArrayList(Neighbor);
    var adjacency = std.StringHashMap(NeighborList).init(allocator);
    defer {
        var it = adjacency.valueIterator();
        while (it.next()) |list| {
            list.deinit(allocator);
        }
        adjacency.deinit();
    }

    for (graph.vertices) |vertex| {
        try adjacency.put(vertex, NeighborList.empty);
    }

    for (graph.edges) |edge| {
        var list = adjacency.getPtr(edge.from).?;
        try list.append(allocator, .{ .to = edge.to, .weight = edge.weight });
    }

    // Initialize distances and predecessors
    var distances = std.StringHashMap(i32).init(allocator);
    var predecessors = std.StringHashMap(?[]const u8).init(allocator);

    // Set initial distances and add to priority queue
    for (graph.vertices) |vertex| {
        const distance = if (std.mem.eql(u8, vertex, source)) 0 else INFINITY;
        try distances.put(vertex, distance);
        try predecessors.put(vertex, null);
        try pq.insert(.{ .id = vertex, .distance = distance });
    }

    // Main algorithm loop
    while (!pq.isEmpty()) {
        const current = (try pq.pop()) orelse break;

        // Skip if we've already found a shorter path
        if (current.distance > distances.get(current.id).?) {
            continue;
        }

        // Skip if current distance is infinity (unreachable)
        if (current.distance == INFINITY) {
            continue;
        }

        // Check all neighbors
        if (adjacency.get(current.id)) |neighbors| {
            for (neighbors.items) |neighbor| {
                const new_distance = current.distance + neighbor.weight;

                if (new_distance < distances.get(neighbor.to).?) {
                    try distances.put(neighbor.to, new_distance);
                    try predecessors.put(neighbor.to, current.id);

                    // Update priority in queue
                    // In a min-heap, decreasing distance = increasing priority (more important)
                    if (pq.contains(.{ .id = neighbor.to, .distance = 0 })) {
                        try pq.increasePriority(.{ .id = neighbor.to, .distance = new_distance });
                    }
                }
            }
        }
    }

    return DijkstraResult{
        .distances = distances,
        .predecessors = predecessors,
    };
}

/// Reconstructs the shortest path from source to target using predecessors.
///
/// Builds the path by following predecessor links backwards from the target,
/// then reversing the result.
///
/// ## Arguments
///
/// * `predecessors` - Predecessor map from dijkstra result
/// * `source` - Source vertex
/// * `target` - Target vertex
/// * `allocator` - Memory allocator for path array
///
/// ## Returns
///
/// An array of vertex IDs representing the path, or null if no path exists.
/// Caller owns the returned slice and must free it.
pub fn reconstructPath(
    predecessors: std.StringHashMap(?[]const u8),
    source: []const u8,
    target: []const u8,
    allocator: std.mem.Allocator,
) !?[]const []const u8 {
    const pred = predecessors.get(target) orelse return null;
    if (pred == null and !std.mem.eql(u8, target, source)) {
        return null; // No path exists
    }

    var path = std.ArrayList([]const u8).empty;
    defer path.deinit(allocator);

    var current: ?[]const u8 = target;
    while (current) |vertex| {
        try path.append(allocator, vertex);
        current = predecessors.get(vertex).?;
    }

    // Reverse the path
    std.mem.reverse([]const u8, path.items);

    if (path.items.len > 0 and std.mem.eql(u8, path.items[0], source)) {
        return try path.toOwnedSlice(allocator);
    }

    return null;
}
