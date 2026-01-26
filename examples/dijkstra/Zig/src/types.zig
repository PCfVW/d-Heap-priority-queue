//! Type definitions for the Dijkstra example.

const std = @import("std");

/// Graph represents a weighted directed graph.
pub const Graph = struct {
    vertices: []const []const u8,
    edges: []const Edge,
};

/// Edge represents a weighted directed edge.
pub const Edge = struct {
    from: []const u8,
    to: []const u8,
    weight: i32,
};

/// Vertex represents a vertex with its current distance from the source.
///
/// Used as the item type in the priority queue. The priority queue uses
/// the vertex ID for lookup (via `contains()` and `increasePriority()`),
/// so equality and hashing are based only on the `id` field, not `distance`.
/// This allows updating a vertex's priority by providing a new distance value.
pub const Vertex = struct {
    id: []const u8,
    distance: i32,

    /// Hash function for HashMap lookup - based only on ID.
    pub fn hash(self: Vertex) u64 {
        return std.hash_map.hashString(self.id);
    }

    /// Equality check for HashMap - based only on ID.
    pub fn eql(a: Vertex, b: Vertex) bool {
        return std.mem.eql(u8, a.id, b.id);
    }
};

/// DijkstraResult contains the output of Dijkstra's algorithm.
pub const DijkstraResult = struct {
    /// Distances maps each vertex to its shortest distance from the source.
    distances: std.StringHashMap(i32),
    /// Predecessors maps each vertex to its predecessor in the shortest path.
    /// null value means no predecessor (source or unreachable).
    predecessors: std.StringHashMap(?[]const u8),
};
