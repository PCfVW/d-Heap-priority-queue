import { PriorityQueue, minBy } from 'd-ary-heap';
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
  // Build adjacency list for efficient neighbor lookup
  const adjacency = new Map<string, Array<{ to: string; weight: number }>>();
  
  for (const vertex of graph.vertices) {
    adjacency.set(vertex, []);
  }
  
  for (const edge of graph.edges) {
    adjacency.get(edge.from)?.push({ to: edge.to, weight: edge.weight });
  }

  // Initialize distances, predecessors, and priority queue
  const distances: Record<string, number> = {};
  const predecessors: Record<string, string | null> = {};
  const pq = new PriorityQueue<Vertex, string>({
    d,
    comparator: minBy((v: Vertex) => v.distance),
    keyExtractor: (v: Vertex) => v.id
  });

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

    // Check all neighbors
    const neighbors = adjacency.get(current.id) || [];
    for (const { to, weight } of neighbors) {
      const newDistance = current.distance + weight;
      
      if (newDistance < distances[to]) {
        distances[to] = newDistance;
        predecessors[to] = current.id;
        
        // Update priority in queue (decrease key operation)
        if (pq.contains({ id: to, distance: 0 })) {
          pq.decreasePriority({ id: to, distance: newDistance });
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