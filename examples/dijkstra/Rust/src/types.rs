//! types.rs - Type definitions for the Dijkstra example

use serde::Deserialize;
use std::collections::HashMap;

/// Graph represents a weighted directed graph.
#[derive(Debug, Deserialize)]
pub struct Graph {
    pub vertices: Vec<String>,
    pub edges: Vec<Edge>,
}

/// Edge represents a weighted directed edge.
#[derive(Debug, Deserialize)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub weight: i32,
}

/// Vertex represents a vertex with its current distance from the source.
///
/// Used as the item type in the priority queue. The priority queue uses
/// the vertex ID for lookup (via `contains()` and `increase_priority()`),
/// so equality and hashing are based only on the `id` field, not `distance`.
/// This allows updating a vertex's priority by providing a new distance value.
#[derive(Debug, Clone)]
pub struct Vertex {
    pub id: String,
    pub distance: i32,
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Vertex {}

impl std::hash::Hash for Vertex {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// DijkstraResult contains the output of Dijkstra's algorithm.
pub struct DijkstraResult {
    /// Distances maps each vertex to its shortest distance from the source.
    pub distances: HashMap<String, i32>,
    /// Predecessors maps each vertex to its predecessor in the shortest path.
    /// None value means no predecessor (source or unreachable).
    pub predecessors: HashMap<String, Option<String>>,
}
