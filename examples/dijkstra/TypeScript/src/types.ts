// types.ts - Type definitions for the Dijkstra example

export interface Graph {
  vertices: string[];
  edges: Edge[];
}

export interface Edge {
  from: string;
  to: string;
  weight: number;
}

export interface Vertex {
  id: string;
  distance: number;
}

export type ShortestPaths = Record<string, number>;

export interface DijkstraResult {
  distances: ShortestPaths;
  predecessors: Record<string, string | null>;
}