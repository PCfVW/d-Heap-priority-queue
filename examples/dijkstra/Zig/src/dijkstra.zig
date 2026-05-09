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
