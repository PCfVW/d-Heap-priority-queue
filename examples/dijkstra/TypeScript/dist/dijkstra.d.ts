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
 * Reconstruct the shortest path from source to target using predecessors.
 *
 * @param predecessors - Predecessor map from dijkstra result
 * @param source - Source vertex
 * @param target - Target vertex
 * @returns Array of vertices representing the path, or null if no path exists
 */
export declare function reconstructPath(predecessors: Record<string, string | null>, source: string, target: string): string[] | null;
//# sourceMappingURL=dijkstra.d.ts.map