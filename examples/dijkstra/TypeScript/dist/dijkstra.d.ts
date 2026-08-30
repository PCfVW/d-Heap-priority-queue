import type { ComparisonStats } from 'd-ary-heap';
import type { Graph, DijkstraResult } from './types.js';
/**
 * Dijkstra's shortest path algorithm using a d-ary heap priority queue.
 *
 * @param graph - The input graph with vertices and weighted edges
 * @param source - The source vertex to find shortest paths from
 * @param d - The arity of the heap (default: 4 for optimal performance)
 * @returns Object with distances and predecessors for path reconstruction
 */
export declare function dijkstra(graph: Graph, source: string, d?: number): DijkstraResult;
/**
 * Like {@link dijkstra} but constructs an instrumented heap and returns its
 * `ComparisonStats` alongside the result. Use this when you want per-operation
 * comparison counts (e.g., for the `--stats` example flag).
 *
 * Mirrors C++ `dijkstra_with_stats`, Go `DijkstraInstrumented`, and Rust
 * `dijkstra_instrumented`.
 */
export declare function dijkstraInstrumented(graph: Graph, source: string, d?: number): {
    result: DijkstraResult;
    stats: ComparisonStats;
};
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
export declare function dijkstraProfiled(graph: Graph, source: string, d?: number): {
    result: DijkstraResult;
    timings: SectionTimings;
};
/**
 * Reconstruct the shortest path from source to target using predecessors.
 *
 * @param predecessors - Predecessor map from dijkstra result
 * @param source - Source vertex
 * @param target - Target vertex
 * @returns Array of vertices representing the path, or null if no path exists
 */
export declare function reconstructPath(predecessors: Record<string, string | null>, source: string, target: string): string[] | null;
//# sourceMappingURL=dijkstra.d.ts.map