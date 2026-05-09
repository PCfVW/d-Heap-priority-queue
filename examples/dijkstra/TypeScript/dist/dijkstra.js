// dijkstra.ts - Dijkstra's shortest path algorithm implementation
import { PriorityQueue, minBy, instrumentComparator } from 'd-ary-heap';
/**
 * Dijkstra's shortest path algorithm using a d-ary heap priority queue.
 *
 * @param graph - The input graph with vertices and weighted edges
 * @param source - The source vertex to find shortest paths from
 * @param d - The arity of the heap (default: 4 for optimal performance)
 * @returns Object with distances and predecessors for path reconstruction
 */
export function dijkstra(graph, source, d = 4) {
    const pq = new PriorityQueue({
        d,
        comparator: minBy((v) => v.distance),
        keyExtractor: (v) => v.id
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
export function dijkstraInstrumented(graph, source, d = 4) {
    const cmp = instrumentComparator(minBy((v) => v.distance));
    const pq = new PriorityQueue({
        d,
        comparator: cmp,
        keyExtractor: (v) => v.id,
        onBeforeOperation: (op) => cmp.startOperation(op),
        onAfterOperation: () => cmp.endOperation(),
    });
    const result = runDijkstra(graph, source, pq);
    return { result, stats: cmp.stats };
}
/**
 * Shared algorithm body — parameterised over an already-constructed `PriorityQueue`.
 * Both `dijkstra` and `dijkstraInstrumented` delegate here.
 */
function runDijkstra(graph, source, pq) {
    // Build adjacency list for efficient neighbor lookup
    const adjacency = new Map();
    for (const vertex of graph.vertices) {
        adjacency.set(vertex, []);
    }
    for (const edge of graph.edges) {
        adjacency.get(edge.from)?.push({ to: edge.to, weight: edge.weight });
    }
    // Initialize distances and predecessors
    const distances = {};
    const predecessors = {};
    // Set initial distances and add to priority queue
    for (const vertex of graph.vertices) {
        const distance = vertex === source ? 0 : Infinity;
        distances[vertex] = distance;
        predecessors[vertex] = null;
        pq.insert({ id: vertex, distance });
    }
    // Main algorithm loop
    while (!pq.isEmpty()) {
        const current = pq.pop();
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
export function reconstructPath(predecessors, source, target) {
    if (predecessors[target] === null && target !== source) {
        return null; // No path exists
    }
    const path = [];
    let current = target;
    while (current !== null) {
        path.unshift(current);
        current = predecessors[current];
    }
    return path[0] === source ? path : null;
}
//# sourceMappingURL=dijkstra.js.map