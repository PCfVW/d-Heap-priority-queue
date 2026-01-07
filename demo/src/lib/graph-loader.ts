import type { Graph } from '../types';
import smallGraph from '../data/graphs/small.json';
import mediumGraph from '../data/graphs/medium.json';
import largeGraph from '../data/graphs/large.json';
import denseSmallGraph from '../data/graphs/dense-small.json';
import denseMediumGraph from '../data/graphs/dense-medium.json';
import denseLargeGraph from '../data/graphs/dense-large.json';
import extradenseLargeGraph from '../data/graphs/extradense-large.json';

/** Available graph sizes */
export type GraphSize = 'small' | 'medium' | 'large';

/** Graph density type */
export type GraphDensity = 'sparse' | 'dense' | 'extra-dense';

/** Load a graph by size and density */
export function loadGraph(size: GraphSize, density: GraphDensity = 'sparse'): Graph {
  if (density === 'extra-dense') {
    // Extra-dense only available for large size; fall back to dense for others
    if (size === 'large') {
      return extradenseLargeGraph as Graph;
    }
    // Fall through to dense for small/medium
    density = 'dense';
  }

  if (density === 'dense') {
    switch (size) {
      case 'small':
        return denseSmallGraph as Graph;
      case 'medium':
        return denseMediumGraph as Graph;
      case 'large':
        return denseLargeGraph as Graph;
    }
  }

  switch (size) {
    case 'small':
      return smallGraph as Graph;
    case 'medium':
      return mediumGraph as Graph;
    case 'large':
      return largeGraph as Graph;
  }
}

/** Get source and target for a graph */
export function getGraphEndpoints(size: GraphSize, density: GraphDensity = 'sparse'): { source: string; target: string } {
  if (density === 'extra-dense') {
    // Extra-dense large: A to H (8 vertices, bidirectional clique for max decrease-keys)
    return { source: 'A', target: 'H' };
  }

  if (density === 'dense') {
    switch (size) {
      case 'small':
        return { source: 'A', target: 'F' };
      case 'medium':
        return { source: 'A', target: 'H' };
      case 'large':
        return { source: 'A', target: 'J' };
    }
  }

  switch (size) {
    case 'small':
      return { source: 'A', target: 'F' };
    case 'medium':
      return { source: 'A', target: 'J' };
    case 'large':
      return { source: 'A', target: 'O' };
  }
}

/** Build adjacency list from graph */
export function buildAdjacencyList(graph: Graph): Map<string, Array<{ to: string; weight: number }>> {
  const adjacency = new Map<string, Array<{ to: string; weight: number }>>();

  for (const vertex of graph.vertices) {
    adjacency.set(vertex, []);
  }

  for (const edge of graph.edges) {
    adjacency.get(edge.from)?.push({ to: edge.to, weight: edge.weight });
  }

  return adjacency;
}
