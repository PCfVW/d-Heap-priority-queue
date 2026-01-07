import { PriorityQueue, minBy, instrumentComparator } from 'd-ary-heap';
import type { Graph } from '../types';
import type {
  AlgorithmEvent,
  HeapSnapshot,
  VisualizationState,
} from './heap-events';
import { buildAdjacencyList } from './graph-loader';

interface Vertex {
  id: string;
  distance: number;
}

/**
 * Runs Dijkstra's algorithm and records all events for step-by-step visualization.
 *
 * @param graph - The input graph
 * @param source - Source vertex
 * @param target - Target vertex (for path highlighting)
 * @param arity - Heap arity (d=2, 4, or 8)
 * @returns Array of algorithm events for playback
 */
export function runDijkstraWithEvents(
  graph: Graph,
  source: string,
  target: string,
  arity: number = 4
): AlgorithmEvent[] {
  const events: AlgorithmEvent[] = [];
  let step = 0;

  const adjacency = buildAdjacencyList(graph);
  const distances: Record<string, number> = {};
  const predecessors: Record<string, string | null> = {};

  // Create instrumented comparator to track comparison counts
  // This is the key differentiator between arities!
  const comparator = instrumentComparator(minBy((v: Vertex) => v.distance));

  // Track cumulative comparisons to calculate per-operation deltas
  let lastInsertComparisons = 0;
  let lastPopComparisons = 0;
  let lastDecreasePriorityComparisons = 0;

  // Create priority queue with instrumentation hooks
  const pq = new PriorityQueue<Vertex, string>({
    d: arity,
    comparator,
    keyExtractor: (v: Vertex) => v.id,
    onBeforeOperation: (op) => {
      comparator.startOperation(op);
    },
    onAfterOperation: () => {
      comparator.endOperation();
    },
  });

  // Helper to get comparisons for the last insert operation
  const getInsertComparisons = (): number => {
    const count = comparator.stats.insert - lastInsertComparisons;
    lastInsertComparisons = comparator.stats.insert;
    return count;
  };

  // Helper to get comparisons for the last pop operation
  const getPopComparisons = (): number => {
    const count = comparator.stats.pop - lastPopComparisons;
    lastPopComparisons = comparator.stats.pop;
    return count;
  };

  // Helper to get comparisons for the last decreasePriority operation
  const getDecreasePriorityComparisons = (): number => {
    const count = comparator.stats.decreasePriority - lastDecreasePriorityComparisons;
    lastDecreasePriorityComparisons = comparator.stats.decreasePriority;
    return count;
  };

  // Helper to capture heap snapshot
  const captureHeapSnapshot = (): HeapSnapshot => {
    const nodes: Array<{ id: string; priority: number }> = [];
    // Get heap contents by iterating (we'll rebuild state from events)
    const items = pq.toArray();
    for (const item of items) {
      nodes.push({ id: item.id, priority: item.distance });
    }
    return { nodes, arity };
  };

  // Init event
  events.push({
    type: 'init',
    step: step++,
    description: `Starting Dijkstra from vertex ${source} with d=${arity} heap`,
    sourceVertex: source,
    vertices: graph.vertices,
  });

  // Initialize distances and insert all vertices
  for (const vertex of graph.vertices) {
    const distance = vertex === source ? 0 : Infinity;
    distances[vertex] = distance;
    predecessors[vertex] = null;
    pq.insert({ id: vertex, distance });
    const insertComparisons = getInsertComparisons();

    events.push({
      type: 'insert',
      step: step++,
      description: `Insert ${vertex} with distance ${distance === Infinity ? '∞' : distance}`,
      vertexId: vertex,
      priority: distance,
      heapSnapshot: captureHeapSnapshot(),
      comparisons: insertComparisons,
    });
  }

  // Main algorithm loop
  while (!pq.isEmpty()) {
    const current = pq.pop()!;
    const popComparisons = getPopComparisons();

    events.push({
      type: 'pop',
      step: step++,
      description: `Extract min: ${current.id} (distance = ${current.distance === Infinity ? '∞' : current.distance})`,
      vertexId: current.id,
      distance: current.distance,
      heapSnapshot: captureHeapSnapshot(),
      comparisons: popComparisons,
    });

    // Skip if we've already found a shorter path (stale entry)
    if (current.distance > distances[current.id]) {
      events.push({
        type: 'skip_processed',
        step: step++,
        description: `Skip ${current.id}: already processed with shorter distance`,
        vertexId: current.id,
      });
      continue;
    }

    // Check all neighbors
    const neighbors = adjacency.get(current.id) || [];
    for (const { to, weight } of neighbors) {
      const newDistance = current.distance + weight;

      events.push({
        type: 'visit_neighbor',
        step: step++,
        description: `Check edge ${current.id} → ${to} (weight ${weight}): ${current.distance} + ${weight} = ${newDistance}`,
        fromVertex: current.id,
        toVertex: to,
        edgeWeight: weight,
        currentDistance: distances[to],
        newDistance,
      });

      if (newDistance < distances[to]) {
        const oldDistance = distances[to];
        distances[to] = newDistance;
        predecessors[to] = current.id;

        events.push({
          type: 'relax_edge',
          step: step++,
          description: `Relax: ${to} distance ${oldDistance === Infinity ? '∞' : oldDistance} → ${newDistance}`,
          fromVertex: current.id,
          toVertex: to,
          oldDistance,
          newDistance,
        });

        // Update priority in queue
        if (pq.contains({ id: to, distance: 0 })) {
          pq.decreasePriority({ id: to, distance: newDistance });
          const decreaseComparisons = getDecreasePriorityComparisons();

          events.push({
            type: 'decrease_priority',
            step: step++,
            description: `Decrease priority of ${to} to ${newDistance}`,
            vertexId: to,
            oldPriority: oldDistance,
            newPriority: newDistance,
            heapSnapshot: captureHeapSnapshot(),
            comparisons: decreaseComparisons,
          });
        }
      }
    }
  }

  // Reconstruct shortest path
  const path = reconstructPath(predecessors, source, target);

  events.push({
    type: 'complete',
    step: step++,
    description: path
      ? `Complete! Shortest path ${source} → ${target}: ${path.join(' → ')} (distance: ${distances[target]})`
      : `Complete! No path from ${source} to ${target}`,
    distances,
    predecessors,
    shortestPath: path,
  });

  return events;
}

/**
 * Reconstruct path from predecessors map
 */
function reconstructPath(
  predecessors: Record<string, string | null>,
  source: string,
  target: string
): string[] | null {
  if (predecessors[target] === null && target !== source) {
    return null;
  }

  const path: string[] = [];
  let current: string | null = target;

  while (current !== null) {
    path.unshift(current);
    current = predecessors[current];
  }

  return path[0] === source ? path : null;
}

/**
 * Build visualization state from events up to a given step.
 * This allows scrubbing through the timeline.
 */
export function buildStateAtStep(
  events: AlgorithmEvent[],
  targetStep: number,
  graph: Graph,
  _arity: number
): VisualizationState {
  const vertexStates = new Map<string, { distance: number; state: 'unvisited' | 'in_queue' | 'processed'; predecessor: string | null }>();
  const heapNodes: Array<{ id: string; priority: number; index: number }> = [];
  const processedVertices = new Set<string>();
  const inQueueVertices = new Set<string>();
  const relaxedEdges = new Set<string>();

  let activeVertex: string | null = null;
  let activeEdge: { from: string; to: string } | null = null;
  let currentOperation = '';

  const stats = { inserts: 0, pops: 0, decreasePriority: 0 };
  const comparisons = { insert: 0, pop: 0, decreasePriority: 0, total: 0 };

  // Initialize all vertices
  for (const v of graph.vertices) {
    vertexStates.set(v, { distance: Infinity, state: 'unvisited', predecessor: null });
  }

  // Replay events up to targetStep
  for (const event of events) {
    if (event.step > targetStep) break;

    currentOperation = event.description;

    switch (event.type) {
      case 'init':
        // Source vertex gets distance 0
        const initState = vertexStates.get(event.sourceVertex);
        if (initState) {
          initState.distance = 0;
        }
        break;

      case 'insert':
        stats.inserts++;
        comparisons.insert += event.comparisons;
        comparisons.total += event.comparisons;
        inQueueVertices.add(event.vertexId);
        const insertState = vertexStates.get(event.vertexId);
        if (insertState) {
          insertState.distance = event.priority;
          insertState.state = 'in_queue';
        }
        // Update heap from snapshot
        updateHeapFromSnapshot(heapNodes, event.heapSnapshot);
        break;

      case 'pop':
        stats.pops++;
        comparisons.pop += event.comparisons;
        comparisons.total += event.comparisons;
        activeVertex = event.vertexId;
        processedVertices.add(event.vertexId);
        inQueueVertices.delete(event.vertexId);
        const popState = vertexStates.get(event.vertexId);
        if (popState) {
          popState.state = 'processed';
        }
        updateHeapFromSnapshot(heapNodes, event.heapSnapshot);
        break;

      case 'visit_neighbor':
        activeEdge = { from: event.fromVertex, to: event.toVertex };
        break;

      case 'relax_edge':
        relaxedEdges.add(`${event.fromVertex}-${event.toVertex}`);
        const relaxState = vertexStates.get(event.toVertex);
        if (relaxState) {
          relaxState.distance = event.newDistance;
          relaxState.predecessor = event.fromVertex;
        }
        break;

      case 'decrease_priority':
        stats.decreasePriority++;
        comparisons.decreasePriority += event.comparisons;
        comparisons.total += event.comparisons;
        updateHeapFromSnapshot(heapNodes, event.heapSnapshot);
        break;

      case 'complete':
        activeVertex = null;
        activeEdge = null;
        break;
    }
  }

  // Update vertex states based on tracking
  for (const v of graph.vertices) {
    const state = vertexStates.get(v)!;
    if (processedVertices.has(v)) {
      state.state = 'processed';
    } else if (inQueueVertices.has(v)) {
      state.state = 'in_queue';
    }
  }

  return {
    step: targetStep,
    heapNodes,
    vertexStates,
    activeVertex,
    activeEdge,
    highlightedVertices: new Set(activeVertex ? [activeVertex] : []),
    relaxedEdges,
    currentOperation,
    stats,
    comparisons,
  };
}

function updateHeapFromSnapshot(
  heapNodes: Array<{ id: string; priority: number; index: number }>,
  snapshot: HeapSnapshot
): void {
  heapNodes.length = 0;
  snapshot.nodes.forEach((node, index) => {
    heapNodes.push({ id: node.id, priority: node.priority, index });
  });
}
