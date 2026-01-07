import { useState, useCallback, useRef, useEffect, useMemo } from 'react';
import type { Graph, VertexData } from '../types';
import type { AlgorithmEvent, VisualizationState, ComparisonCounts } from '../lib/heap-events';
import { runDijkstraWithEvents, buildStateAtStep } from '../lib/dijkstra-visualizer';

interface UseDijkstraRunnerOptions {
  graph: Graph;
  source: string;
  target: string;
  arity: number;
  playbackSpeed?: number; // ms between steps during auto-play
}

interface UseDijkstraRunnerReturn {
  // State
  currentStep: number;
  totalSteps: number;
  isPlaying: boolean;
  isComplete: boolean;
  visualizationState: VisualizationState | null;
  events: AlgorithmEvent[];

  // Controls
  step: () => void;
  play: () => void;
  pause: () => void;
  reset: () => void;
  goToStep: (step: number) => void;

  // Derived data for visualization
  heapNodes: Array<{ id: string; priority: number; index: number }>;
  vertexData: Map<string, VertexData>;
  activeVertexId: string | null;
  activeEdge: { from: string; to: string } | null;
  relaxedEdges: Set<string>;
  shortestPath: string[] | null;
  animatedPathEdges: Set<string>; // For sequential path reveal animation
  /** Operation counts (same across arities for same graph) */
  stats: { inserts: number; pops: number; decreasePriority: number };
  /** Comparison counts (differ by arity - the key differentiator!) */
  comparisons: ComparisonCounts;
  currentOperation: string;
  currentEventType: string | null;
  bubblingNodeId: string | null;
}

export function useDijkstraRunner({
  graph,
  source,
  target,
  arity,
  playbackSpeed = 500,
}: UseDijkstraRunnerOptions): UseDijkstraRunnerReturn {
  const [events, setEvents] = useState<AlgorithmEvent[]>([]);
  const [currentStep, setCurrentStep] = useState(-1);
  const [isPlaying, setIsPlaying] = useState(false);
  const [visualizationState, setVisualizationState] = useState<VisualizationState | null>(null);
  const [pathAnimationIndex, setPathAnimationIndex] = useState(-1);

  const playIntervalRef = useRef<number | null>(null);
  const pathAnimationRef = useRef<number | null>(null);
  const pathAnimationStartedRef = useRef(false);

  // Generate events when arity changes
  useEffect(() => {
    const newEvents = runDijkstraWithEvents(graph, source, target, arity);
    setEvents(newEvents);
    setCurrentStep(-1);
    setIsPlaying(false);
    setVisualizationState(null);
    setPathAnimationIndex(-1);
    pathAnimationStartedRef.current = false;

    // Clear any playing interval
    if (playIntervalRef.current) {
      clearInterval(playIntervalRef.current);
      playIntervalRef.current = null;
    }
    if (pathAnimationRef.current) {
      clearInterval(pathAnimationRef.current);
      pathAnimationRef.current = null;
    }
  }, [graph, source, target, arity]);

  // Update visualization state when step changes
  useEffect(() => {
    if (currentStep >= 0 && events.length > 0) {
      const state = buildStateAtStep(events, currentStep, graph, arity);
      setVisualizationState(state);
    } else if (currentStep === -1) {
      // Initial state before algorithm starts
      const initialVertexData = new Map<string, { distance: number; state: 'unvisited' | 'in_queue' | 'processed'; predecessor: string | null }>();
      for (const v of graph.vertices) {
        initialVertexData.set(v, {
          distance: v === source ? 0 : Infinity,
          state: 'unvisited',
          predecessor: null,
        });
      }
      setVisualizationState({
        step: -1,
        heapNodes: [],
        vertexStates: initialVertexData,
        activeVertex: null,
        activeEdge: null,
        highlightedVertices: new Set(),
        relaxedEdges: new Set(),
        currentOperation: 'Ready to start',
        stats: { inserts: 0, pops: 0, decreasePriority: 0 },
        comparisons: { insert: 0, pop: 0, decreasePriority: 0, total: 0 },
      });
    }
  }, [currentStep, events, graph, arity, source]);

  const totalSteps = events.length - 1;
  const isComplete = currentStep >= totalSteps;

  // Step forward
  const step = useCallback(() => {
    if (currentStep < totalSteps) {
      setCurrentStep((prev) => prev + 1);
    }
  }, [currentStep, totalSteps]);

  // Play/auto-advance
  const play = useCallback(() => {
    if (isComplete) {
      // If complete, reset and play
      setCurrentStep(0);
    }
    setIsPlaying(true);
  }, [isComplete]);

  // Pause
  const pause = useCallback(() => {
    setIsPlaying(false);
  }, []);

  // Reset
  const reset = useCallback(() => {
    setIsPlaying(false);
    setCurrentStep(-1);
    setPathAnimationIndex(-1);
    pathAnimationStartedRef.current = false;
    if (playIntervalRef.current) {
      clearInterval(playIntervalRef.current);
      playIntervalRef.current = null;
    }
    if (pathAnimationRef.current) {
      clearInterval(pathAnimationRef.current);
      pathAnimationRef.current = null;
    }
  }, []);

  // Go to specific step
  const goToStep = useCallback(
    (targetStep: number) => {
      const clampedStep = Math.max(-1, Math.min(targetStep, totalSteps));
      setCurrentStep(clampedStep);
    },
    [totalSteps]
  );

  // Auto-play effect
  useEffect(() => {
    if (isPlaying && !isComplete) {
      playIntervalRef.current = window.setInterval(() => {
        setCurrentStep((prev) => {
          if (prev >= totalSteps) {
            setIsPlaying(false);
            return prev;
          }
          return prev + 1;
        });
      }, playbackSpeed);
    } else {
      if (playIntervalRef.current) {
        clearInterval(playIntervalRef.current);
        playIntervalRef.current = null;
      }
    }

    return () => {
      if (playIntervalRef.current) {
        clearInterval(playIntervalRef.current);
      }
    };
  }, [isPlaying, isComplete, totalSteps, playbackSpeed]);

  // Extract shortest path from complete event
  const shortestPath =
    events.length > 0 && events[events.length - 1].type === 'complete'
      ? (events[events.length - 1] as { shortestPath: string[] | null }).shortestPath
      : null;

  // Trigger path discovery animation when algorithm completes
  useEffect(() => {
    if (isComplete && shortestPath && shortestPath.length > 1 && !pathAnimationStartedRef.current) {
      // Mark animation as started to prevent re-triggering
      pathAnimationStartedRef.current = true;

      // Start animating path edges sequentially
      setPathAnimationIndex(0);
      const pathEdgeCount = shortestPath.length - 1;

      pathAnimationRef.current = window.setInterval(() => {
        setPathAnimationIndex((prev) => {
          if (prev >= pathEdgeCount - 1) {
            if (pathAnimationRef.current) {
              clearInterval(pathAnimationRef.current);
              pathAnimationRef.current = null;
            }
            return prev;
          }
          return prev + 1;
        });
      }, 300); // Reveal each edge every 300ms
    }

    return () => {
      if (pathAnimationRef.current) {
        clearInterval(pathAnimationRef.current);
      }
    };
  }, [isComplete, shortestPath]);

  // Build animated path edges set (only include edges up to current animation index)
  const animatedPathEdges = useMemo(() => {
    const edges = new Set<string>();
    if (shortestPath && shortestPath.length > 1 && pathAnimationIndex >= 0) {
      for (let i = 0; i <= pathAnimationIndex && i < shortestPath.length - 1; i++) {
        edges.add(`${shortestPath[i]}-${shortestPath[i + 1]}`);
      }
    }
    return edges;
  }, [shortestPath, pathAnimationIndex]);

  // Convert to VertexData map for compatibility
  const vertexData = new Map<string, VertexData>();
  if (visualizationState) {
    visualizationState.vertexStates.forEach((state, id) => {
      vertexData.set(id, {
        id,
        distance: state.distance,
        state: state.state,
        predecessor: state.predecessor,
      });
    });
  }

  // Get current event type for animation triggers
  const currentEvent = currentStep >= 0 && currentStep < events.length ? events[currentStep] : null;

  // Get bubbling node ID for decrease_priority animation
  const bubblingNodeId = currentEvent?.type === 'decrease_priority'
    ? (currentEvent as { vertexId: string }).vertexId
    : null;

  return {
    currentStep,
    totalSteps,
    isPlaying,
    isComplete,
    visualizationState,
    events,
    step,
    play,
    pause,
    reset,
    goToStep,
    heapNodes: visualizationState?.heapNodes ?? [],
    vertexData,
    activeVertexId: visualizationState?.activeVertex ?? null,
    activeEdge: visualizationState?.activeEdge ?? null,
    relaxedEdges: visualizationState?.relaxedEdges ?? new Set(),
    shortestPath: isComplete ? shortestPath : null,
    animatedPathEdges,
    stats: visualizationState?.stats ?? { inserts: 0, pops: 0, decreasePriority: 0 },
    comparisons: visualizationState?.comparisons ?? { insert: 0, pop: 0, decreasePriority: 0, total: 0 },
    currentOperation: visualizationState?.currentOperation ?? '',
    currentEventType: currentEvent?.type ?? null,
    bubblingNodeId,
  };
}
