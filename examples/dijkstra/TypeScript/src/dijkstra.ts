// dijkstra.ts - Dijkstra's shortest path algorithm implementation

import { PriorityQueue, minBy, instrumentComparator } from 'd-ary-heap';
import type { ComparisonStats } from 'd-ary-heap';
import type { Graph, Vertex, DijkstraResult } from './types.js';

/**
 * Dijkstra's shortest path algorithm using a d-ary heap priority queue.
 *
 * @param graph - The input graph with vertices and weighted edges
 * @param source - The source vertex to find shortest paths from
 * @param d - The arity of the heap (default: 4 for optimal performance)
 * @returns Object with distances and predecessors for path reconstruction
 */
export function dijkstra(graph: Graph, source: string, d: number = 4): DijkstraResult {
  const pq = new PriorityQueue<Vertex, string>({
    d,
    comparator: minBy((v: Vertex) => v.distance),
    keyExtractor: (v: Vertex) => v.id
  });
  return runDijkstra(graph, source, pq);
}

/**
 * Like {@link dijkstra} but constructs an instrumented heap and returns its
 * `ComparisonStats` alongside the result. Use this when you want per-operation
 * comparison counts (e.g., for the `--stats` example flag).
 *
 * Mirrors C++ `dijkstra_with_stats`, Go `DijkstraInstrumented`, and Rust
 * `dijkstra_instrumented`.
 */
export function dijkstraInstrumented(
  graph: Graph,
  source: string,
  d: number = 4
): { result: DijkstraResult; stats: ComparisonStats } {
  const cmp = instrumentComparator(minBy((v: Vertex) => v.distance));
  const pq = new PriorityQueue<Vertex, string>({
    d,
    comparator: cmp,
    keyExtractor: (v: Vertex) => v.id,
    onBeforeOperation: (op) => cmp.startOperation(op),
    onAfterOperation: () => cmp.endOperation(),
  });
  const result = runDijkstra(graph, source, pq);
  return { result, stats: cmp.stats };
}

/**
 * Wall-clock time spent in each of Dijkstra's four algorithmic sections, as
 * measured by {@link dijkstraProfiled} via `performance.now()`. All fields
 * are microseconds.
 *
 * `total` is the sum of the four sections — a distinct number from the
 * Phase 3 wall-time median (see `benchmarks/methodology.md`, "Phase 3.5"):
 * this measures one un-iterated call including timer overhead, not a
 * warmup + N-repetition steady state.
 *
 * Cross-language equivalents:
 * - C++: `SectionTimings` (dijkstra.h)
 * - Go: `SectionTimings` (dijkstra.go)
 * - Rust: `SectionTimings` (dijkstra.rs)
 * - Zig: `SectionTimings` (dijkstra.zig)
 */
export interface SectionTimings {
  /** Adjacency-list build + distances/predecessors init + the initial `insert` per vertex. One-shot, before the main loop. */
  setup: number;
  /** Sum of every `pq.pop()` call inside the main loop. Heap-only. */
  pop: number;
  /**
   * Sum of the per-edge distance computation, comparison against
   * `distances`, and (when improved) the writes to `distances` and
   * `predecessors`. Excludes the heap `contains`/`increasePriority` pair,
   * timed separately as `incPri`.
   */
  relax: number;
  /**
   * Sum of the `contains` check and conditional `increasePriority` call,
   * timed only on edges where `relax` found a strictly shorter path.
   */
  incPri: number;
  /** Sum of the four sections above. See the interface doc comment for why this differs from the Phase 3 wall-time median. */
  total: number;
}

/**
 * Like {@link dijkstra}, but instruments the four Phase 3.5 sections
 * (`setup`, `pop`, `relax`, `incPri`) with `performance.now()` and returns
 * their timings alongside the result. Used only by the `--profile-sections`
 * example flag — a dedicated function rather than threading instrumentation
 * through {@link runDijkstra}, so the Phase 3 wall-time/`--stats` hot paths
 * carry no extra branching from this instrumentation.
 *
 * The section boundaries are the cross-language-consistent split documented
 * in `benchmarks/methodology.md` ("Phase 3.5: per-section timing"): `relax`
 * covers the edge-weight comparison and (when improved) the
 * `distances`/`predecessors` writes; `incPri` covers the heap `contains`
 * lookup and conditional `increasePriority` call that follows an improved
 * edge — split via an explicit `improved` boolean rather than re-evaluating
 * the comparison after `distances` has already been mutated. The
 * stale-entry and unreachable-vertex `continue` checks between `pop` and
 * the neighbor loop are not separately timed — a small, symmetric gap
 * explicitly licensed by methodology.md's "mutually exclusive and (within
 * rounding) exhaustive" framing.
 */
export function dijkstraProfiled(
  graph: Graph,
  source: string,
  d: number = 4
): { result: DijkstraResult; timings: SectionTimings } {
  const pq = new PriorityQueue<Vertex, string>({
    d,
    comparator: minBy((v: Vertex) => v.distance),
    keyExtractor: (v: Vertex) => v.id,
  });

  const timings: SectionTimings = { setup: 0, pop: 0, relax: 0, incPri: 0, total: 0 };

  const setupStart = performance.now();

  // Build adjacency list for efficient neighbor lookup
  const adjacency = new Map<string, Array<{ to: string; weight: number }>>();
  for (const vertex of graph.vertices) {
    adjacency.set(vertex, []);
  }
  for (const edge of graph.edges) {
    adjacency.get(edge.from)?.push({ to: edge.to, weight: edge.weight });
  }

  // Initialize distances and predecessors
  const distances: Record<string, number> = {};
  const predecessors: Record<string, string | null> = {};

  // Set initial distances and add to priority queue
  for (const vertex of graph.vertices) {
    const distance = vertex === source ? 0 : Infinity;
    distances[vertex] = distance;
    predecessors[vertex] = null;
    pq.insert({ id: vertex, distance });
  }
  timings.setup = (performance.now() - setupStart) * 1000;

  // Main algorithm loop
  while (!pq.isEmpty()) {
    const popStart = performance.now();
    const current = pq.pop()!; // INDEX: the while-condition just proved the heap non-empty
    timings.pop += (performance.now() - popStart) * 1000;

    // Stale-entry / unreachable-vertex skip checks: not separately timed
    // (see the function doc comment) — sub-100ns, symmetric across
    // languages, folded into methodology.md's "within rounding" caveat.
    if (current.distance > distances[current.id]) {
      continue;
    }
    if (current.distance === Infinity) {
      continue;
    }

    const neighbors = adjacency.get(current.id) || [];
    for (const { to, weight } of neighbors) {
      const relaxStart = performance.now();
      const newDistance = current.distance + weight;
      const improved = newDistance < distances[to];
      if (improved) {
        distances[to] = newDistance;
        predecessors[to] = current.id;
      }
      timings.relax += (performance.now() - relaxStart) * 1000;

      if (improved) {
        const incPriStart = performance.now();
        if (pq.contains({ id: to, distance: 0 })) {
          pq.increasePriority({ id: to, distance: newDistance });
        }
        timings.incPri += (performance.now() - incPriStart) * 1000;
      }
    }
  }

  timings.total = timings.setup + timings.pop + timings.relax + timings.incPri;

  return { result: { distances, predecessors }, timings };
}

/**
 * Shared algorithm body — parameterised over an already-constructed `PriorityQueue`.
 * Both `dijkstra` and `dijkstraInstrumented` delegate here.
 */
function runDijkstra(
  graph: Graph,
  source: string,
  pq: PriorityQueue<Vertex, string>
): DijkstraResult {
  // Build adjacency list for efficient neighbor lookup
  const adjacency = new Map<string, Array<{ to: string; weight: number }>>();

  for (const vertex of graph.vertices) {
    adjacency.set(vertex, []);
  }

  for (const edge of graph.edges) {
    adjacency.get(edge.from)?.push({ to: edge.to, weight: edge.weight });
  }

  // Initialize distances and predecessors
  const distances: Record<string, number> = {};
  const predecessors: Record<string, string | null> = {};

  // Set initial distances and add to priority queue
  for (const vertex of graph.vertices) {
    const distance = vertex === source ? 0 : Infinity;
    distances[vertex] = distance;
    predecessors[vertex] = null;
    pq.insert({ id: vertex, distance });
  }

  // Main algorithm loop
  while (!pq.isEmpty()) {
    const current = pq.pop()!;

    // Skip if we've already found a shorter path
    if (current.distance > distances[current.id]) {
      continue;
    }

    // Skip if current distance is infinity (unreachable)
    if (current.distance === Infinity) {
      continue;
    }

    // Check all neighbors
    const neighbors = adjacency.get(current.id) || [];
    for (const { to, weight } of neighbors) {
      const newDistance = current.distance + weight;

      if (newDistance < distances[to]) {
        distances[to] = newDistance;
        predecessors[to] = current.id;

        // Update priority in queue
        // In a min-heap, decreasing distance = increasing priority (more important)
        if (pq.contains({ id: to, distance: 0 })) {
          pq.increasePriority({ id: to, distance: newDistance });
        }
      }
    }
  }

  return { distances, predecessors };
}

/**
 * Reconstruct the shortest path from source to target using predecessors.
 * 
 * @param predecessors - Predecessor map from dijkstra result
 * @param source - Source vertex
 * @param target - Target vertex
 * @returns Array of vertices representing the path, or null if no path exists
 */
export function reconstructPath(
  predecessors: Record<string, string | null>, 
  source: string, 
  target: string
): string[] | null {
  if (predecessors[target] === null && target !== source) {
    return null; // No path exists
  }

  const path: string[] = [];
  let current: string | null = target;
  
  while (current !== null) {
    path.unshift(current);
    current = predecessors[current];
  }
  
  return path[0] === source ? path : null;
}