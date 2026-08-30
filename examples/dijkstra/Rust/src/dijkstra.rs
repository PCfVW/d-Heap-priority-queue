//! dijkstra.rs - Dijkstra's shortest path algorithm implementation

use crate::types::{DijkstraResult, Graph, Vertex};
use d_ary_heap::{ComparisonStats, MinBy, PriorityCompare, PriorityQueue, StatsCollector};
use std::collections::HashMap;
use std::time::Instant;

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
    let mut pq = PriorityQueue::new(d, MinBy(|v: &Vertex| v.distance)).unwrap();
    dijkstra_with_pq(graph, source, &mut pq)
}

/// Like [`dijkstra`], but constructs an instrumented heap and returns its
/// `ComparisonStats` alongside the result. Use this when you want
/// per-operation comparison counts (e.g., for the `--stats` example flag).
pub fn dijkstra_instrumented(
    graph: &Graph,
    source: &str,
    d: usize,
) -> (DijkstraResult, ComparisonStats) {
    let mut pq = PriorityQueue::with_stats(d, MinBy(|v: &Vertex| v.distance)).unwrap();
    let result = dijkstra_with_pq(graph, source, &mut pq);
    (result, pq.stats().clone())
}

/// Wall-clock time spent in each of Dijkstra's four algorithmic sections, as
/// measured by [`dijkstra_profiled`] via `std::time::Instant`. All fields are
/// microseconds.
///
/// `total_us()` is the sum of the four sections — a distinct number from the
/// Phase 3 wall-time median (see `benchmarks/methodology.md`, "Phase 3.5"):
/// this struct measures one un-iterated call including timer overhead, not a
/// warmup + N-repetition steady state.
///
/// Cross-language equivalents:
/// - C++: `SectionTimings` (dijkstra.h)
/// - Go: `SectionTimings` (dijkstra.go)
/// - Zig: `SectionTimings` (dijkstra.zig)
/// - TypeScript: `SectionTimings` (dijkstra.ts)
#[derive(Debug, Clone, Copy, Default)]
pub struct SectionTimings {
    /// Adjacency-list build + distances/predecessors init + the initial
    /// `insert` call per vertex. One-shot, before the main loop.
    pub setup_us: f64,
    /// Sum of every `pq.pop()` call inside the main loop. Heap-only.
    pub pop_us: f64,
    /// Sum of the per-edge distance computation, comparison against
    /// `distances`, and (when improved) the writes to `distances` and
    /// `predecessors`. Excludes the heap `contains`/`increase_priority`
    /// pair, timed separately as `inc_pri_us`.
    pub relax_us: f64,
    /// Sum of the `contains` check and conditional `increase_priority` call,
    /// timed only on edges where `relax_us` found a strictly shorter path.
    pub inc_pri_us: f64,
}

impl SectionTimings {
    /// Sum of all four sections. See the struct-level doc comment for why
    /// this differs from the Phase 3 wall-time median.
    #[must_use]
    pub fn total_us(&self) -> f64 {
        self.setup_us + self.pop_us + self.relax_us + self.inc_pri_us
    }
}

/// Like [`dijkstra`], but instruments the four Phase 3.5 sections (`setup`,
/// `pop`, `relax`, `inc_pri`) with `std::time::Instant` and returns their
/// timings alongside the result. Used only by the `--profile-sections`
/// example flag — a dedicated function rather than threading instrumentation
/// through [`dijkstra_with_pq`], so the Phase 3 wall-time/`--stats` hot paths
/// carry no extra branching from this instrumentation.
///
/// The section boundaries are the cross-language-consistent split documented
/// in `benchmarks/methodology.md` ("Phase 3.5: per-section timing"): `relax`
/// covers the edge-weight comparison and (when improved) the
/// `distances`/`predecessors` writes; `inc_pri` covers the heap `contains`
/// lookup and conditional `increase_priority` call that follows an improved
/// edge — split via an explicit `improved` boolean rather than re-evaluating
/// the comparison after `distances` has already been mutated. The
/// stale-entry and unreachable-vertex `continue` checks between `pop` and
/// the neighbor loop are not separately timed — a small, symmetric gap
/// explicitly licensed by methodology.md's "mutually exclusive and (within
/// rounding) exhaustive" framing.
pub fn dijkstra_profiled(graph: &Graph, source: &str, d: usize) -> (DijkstraResult, SectionTimings) {
    let mut pq = PriorityQueue::new(d, MinBy(|v: &Vertex| v.distance)).unwrap();
    let mut timings = SectionTimings::default();

    let setup_start = Instant::now();
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
    timings.setup_us = setup_start.elapsed().as_secs_f64() * 1_000_000.0;

    // Main algorithm loop
    while !pq.is_empty() {
        let pop_start = Instant::now();
        let current = pq.pop().unwrap();
        timings.pop_us += pop_start.elapsed().as_secs_f64() * 1_000_000.0;

        // Stale-entry / unreachable-vertex skip checks: not separately timed
        // (see the function doc comment) — sub-100ns, symmetric across
        // languages, folded into methodology.md's "within rounding" caveat.
        if current.distance > *distances.get(&current.id).unwrap() {
            continue;
        }
        if current.distance == INFINITY {
            continue;
        }

        if let Some(neighbors) = adjacency.get(&current.id) {
            for (neighbor_id, weight) in neighbors {
                let relax_start = Instant::now();
                let new_distance = current.distance + weight;
                let improved = new_distance < *distances.get(neighbor_id).unwrap();
                if improved {
                    distances.insert(neighbor_id.clone(), new_distance);
                    predecessors.insert(neighbor_id.clone(), Some(current.id.clone()));
                }
                timings.relax_us += relax_start.elapsed().as_secs_f64() * 1_000_000.0;

                if improved {
                    let inc_pri_start = Instant::now();
                    // dummy distance for contains check — see the identical
                    // pattern in `dijkstra_with_pq` above.
                    let neighbor_vertex = Vertex {
                        id: neighbor_id.clone(),
                        distance: 0,
                    };
                    if pq.contains(&neighbor_vertex) {
                        pq.increase_priority(&Vertex {
                            id: neighbor_id.clone(),
                            distance: new_distance,
                        })
                        .unwrap();
                    }
                    timings.inc_pri_us += inc_pri_start.elapsed().as_secs_f64() * 1_000_000.0;
                }
            }
        }
    }

    (
        DijkstraResult {
            distances,
            predecessors,
        },
        timings,
    )
}

/// Generic algorithm body: parameterised over both the comparator type `C` and
/// the stats type `S`. Both `dijkstra` and `dijkstra_instrumented` delegate
/// here; monomorphization specializes each call site, so the default-stats
/// path inlines the empty `NoOpStats` methods to nothing.
fn dijkstra_with_pq<C, S>(
    graph: &Graph,
    source: &str,
    pq: &mut PriorityQueue<Vertex, C, S>,
) -> DijkstraResult
where
    C: PriorityCompare<Vertex>,
    S: StatsCollector,
{
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
