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
export function dijkstraProfiled(graph, source, d = 4) {
    const pq = new PriorityQueue({
        d,
        comparator: minBy((v) => v.distance),
        keyExtractor: (v) => v.id,
    });
    const timings = {
        setup: 0,
        pop: 0,
        relax: 0,
        incPri: 0,
        total: 0,
        popCount: 0,
        relaxCount: 0,
    };
    const setupStart = performance.now();
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
    timings.setup = (performance.now() - setupStart) * 1000;
    // Main algorithm loop
    while (!pq.isEmpty()) {
        const popStart = performance.now();
        const current = pq.pop(); // INDEX: the while-condition just proved the heap non-empty
        timings.pop += (performance.now() - popStart) * 1000;
        timings.popCount++;
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
            timings.relaxCount++;
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