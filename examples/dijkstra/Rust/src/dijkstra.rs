//! dijkstra.rs - Dijkstra's shortest path algorithm implementation

use crate::types::{DijkstraResult, Graph, Vertex};
use d_ary_heap::{MinBy, PriorityQueue};
use std::collections::HashMap;

/// Infinity represents an unreachable distance.
pub const INFINITY: i32 = i32::MAX;

/// Dijkstra's shortest path algorithm using a d-ary heap priority queue.
///
/// Finds the shortest paths from a source vertex to all other vertices in a weighted
/// graph with non-negative edge weights.
///
/// # Arguments
///
/// * `graph` - The input graph with vertices and weighted edges
/// * `source` - The source vertex to find shortest paths from
/// * `d` - The arity of the heap (typically 4 for optimal performance)
///
/// # Returns
///
/// A `DijkstraResult` containing distances and predecessors for path reconstruction.
pub fn dijkstra(graph: &Graph, source: &str, d: usize) -> DijkstraResult {
    // Build adjacency list for efficient neighbor lookup
    let mut adjacency: HashMap<String, Vec<(String, i32)>> = HashMap::new();
    for vertex in &graph.vertices {
        adjacency.insert(vertex.clone(), Vec::new());
    }
    for edge in &graph.edges {
        adjacency
            .get_mut(&edge.from)
            .unwrap()
            .push((edge.to.clone(), edge.weight));
    }

    // Initialize distances and predecessors
    let mut distances: HashMap<String, i32> = HashMap::new();
    let mut predecessors: HashMap<String, Option<String>> = HashMap::new();

    // Create priority queue with min-heap by distance
    let mut pq = PriorityQueue::new(d, MinBy(|v: &Vertex| v.distance)).unwrap();

    // Set initial distances and add to priority queue
    for vertex in &graph.vertices {
        let distance = if vertex == source { 0 } else { INFINITY };
        distances.insert(vertex.clone(), distance);
        predecessors.insert(vertex.clone(), None);
        pq.insert(Vertex {
            id: vertex.clone(),
            distance,
        });
    }

    // Main algorithm loop
    while !pq.is_empty() {
        let current = pq.pop().unwrap();

        // Skip if we've already found a shorter path
        if current.distance > *distances.get(&current.id).unwrap() {
            continue;
        }

        // Skip if current distance is infinity (unreachable)
        if current.distance == INFINITY {
            continue;
        }

        // Check all neighbors
        if let Some(neighbors) = adjacency.get(&current.id) {
            for (neighbor_id, weight) in neighbors {
                let new_distance = current.distance + weight;

                if new_distance < *distances.get(neighbor_id).unwrap() {
                    distances.insert(neighbor_id.clone(), new_distance);
                    predecessors.insert(neighbor_id.clone(), Some(current.id.clone()));

                    // Update priority in queue
                    // In a min-heap, decreasing distance = increasing priority (more important)
                    let neighbor_vertex = Vertex {
                        id: neighbor_id.clone(),
                        distance: 0, // dummy value for contains check
                    };
                    if pq.contains(&neighbor_vertex) {
                        pq.increase_priority(&Vertex {
                            id: neighbor_id.clone(),
                            distance: new_distance,
                        })
                        .unwrap();
                    }
                }
            }
        }
    }

    DijkstraResult {
        distances,
        predecessors,
    }
}

/// Reconstructs the shortest path from source to target using predecessors.
///
/// Builds the path by following predecessor links backwards from the target,
/// then reversing the result.
///
/// # Arguments
///
/// * `predecessors` - Predecessor map from dijkstra result
/// * `source` - Source vertex
/// * `target` - Target vertex
///
/// # Returns
///
/// A vector of vertex IDs representing the path, or `None` if no path exists.
pub fn reconstruct_path(
    predecessors: &HashMap<String, Option<String>>,
    source: &str,
    target: &str,
) -> Option<Vec<String>> {
    if predecessors.get(target)?.is_none() && target != source {
        return None; // No path exists
    }

    let mut path = Vec::new();
    let mut current = Some(target.to_string());

    while let Some(vertex) = current {
        path.push(vertex.clone());
        current = predecessors.get(&vertex)?.clone();
    }

    path.reverse();

    if !path.is_empty() && path[0] == source {
        Some(path)
    } else {
        None
    }
}
