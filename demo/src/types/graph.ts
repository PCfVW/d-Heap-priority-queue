/** Graph edge with weight */
export interface GraphEdge {
  from: string;
  to: string;
  weight: number;
}

/** Graph data structure from JSON files */
export interface Graph {
  vertices: string[];
  edges: GraphEdge[];
}

/** Vertex state during Dijkstra execution */
export type VertexState = 'unvisited' | 'in_queue' | 'processed';

/** Vertex with distance and state for visualization */
export interface VertexData {
  id: string;
  distance: number;
  state: VertexState;
  predecessor: string | null;
}
