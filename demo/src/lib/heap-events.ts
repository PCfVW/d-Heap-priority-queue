import type { VertexState } from '../types';

/** Types of algorithm events for visualization */
export type AlgorithmEventType =
  | 'init'
  | 'insert'
  | 'pop'
  | 'visit_neighbor'
  | 'relax_edge'
  | 'decrease_priority'
  | 'skip_processed'
  | 'complete';

/** Base event structure */
interface BaseEvent {
  type: AlgorithmEventType;
  step: number;
  description: string;
}

/** Initialization event - algorithm starts */
export interface InitEvent extends BaseEvent {
  type: 'init';
  sourceVertex: string;
  vertices: string[];
}

/** Insert event - vertex added to heap */
export interface InsertEvent extends BaseEvent {
  type: 'insert';
  vertexId: string;
  priority: number;
  heapSnapshot: HeapSnapshot;
  /** Number of comparisons performed during this insert */
  comparisons: number;
}

/** Pop event - minimum vertex extracted */
export interface PopEvent extends BaseEvent {
  type: 'pop';
  vertexId: string;
  distance: number;
  heapSnapshot: HeapSnapshot;
  /** Number of comparisons performed during this pop (extract-min) */
  comparisons: number;
}

/** Visit neighbor event - checking an edge */
export interface VisitNeighborEvent extends BaseEvent {
  type: 'visit_neighbor';
  fromVertex: string;
  toVertex: string;
  edgeWeight: number;
  currentDistance: number;
  newDistance: number;
}

/** Relax edge event - found shorter path */
export interface RelaxEdgeEvent extends BaseEvent {
  type: 'relax_edge';
  fromVertex: string;
  toVertex: string;
  oldDistance: number;
  newDistance: number;
}

/** Decrease priority event - heap update */
export interface DecreasePriorityEvent extends BaseEvent {
  type: 'decrease_priority';
  vertexId: string;
  oldPriority: number;
  newPriority: number;
  heapSnapshot: HeapSnapshot;
  /** Number of comparisons performed during this decrease-priority */
  comparisons: number;
}

/** Skip processed event - vertex already processed */
export interface SkipProcessedEvent extends BaseEvent {
  type: 'skip_processed';
  vertexId: string;
}

/** Complete event - algorithm finished */
export interface CompleteEvent extends BaseEvent {
  type: 'complete';
  distances: Record<string, number>;
  predecessors: Record<string, string | null>;
  shortestPath: string[] | null;
}

/** Union of all event types */
export type AlgorithmEvent =
  | InitEvent
  | InsertEvent
  | PopEvent
  | VisitNeighborEvent
  | RelaxEdgeEvent
  | DecreasePriorityEvent
  | SkipProcessedEvent
  | CompleteEvent;

/** Snapshot of heap state at a point in time */
export interface HeapSnapshot {
  nodes: Array<{ id: string; priority: number }>;
  arity: number;
}

/** Operation counts (same across arities for same graph) */
export interface OperationStats {
  inserts: number;
  pops: number;
  decreasePriority: number;
}

/** Comparison counts (differ by arity - this is what matters!) */
export interface ComparisonCounts {
  insert: number;
  pop: number;
  decreasePriority: number;
  total: number;
}

/** Complete visualization state at any step */
export interface VisualizationState {
  step: number;
  heapNodes: Array<{ id: string; priority: number; index: number }>;
  vertexStates: Map<string, { distance: number; state: VertexState; predecessor: string | null }>;
  activeVertex: string | null;
  activeEdge: { from: string; to: string } | null;
  highlightedVertices: Set<string>;
  relaxedEdges: Set<string>;
  currentOperation: string;
  /** Operation counts (same across arities) */
  stats: OperationStats;
  /** Comparison counts (differ by arity - the key differentiator!) */
  comparisons: ComparisonCounts;
}
