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
 * Reconstruct the shortest path from source to target using predecessors.
 *
 * @param predecessors - Predecessor map from dijkstra result
 * @param source - Source vertex
 * @param target - Target vertex
 * @returns Array of vertices representing the path, or null if no path exists
 */
export declare function reconstructPath(predecessors: Record<string, string | null>, source: string, target: string): string[] | null;
//# sourceMappingURL=dijkstra.d.ts.map