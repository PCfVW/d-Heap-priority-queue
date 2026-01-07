import { useMemo, useEffect, useRef } from 'react';
import {
  ReactFlow,
  ReactFlowProvider,
  Background,
  Controls,
  useReactFlow,
  MarkerType,
  type Node,
  type Edge,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';

import { VertexNode } from './VertexNode';
import { WeightedEdge } from './WeightedEdge';
import { getGraphLayout } from '../../lib/layout-utils';
import type { Graph, VertexData } from '../../types';

const nodeTypes = { vertexNode: VertexNode } as const;
const edgeTypes = { weightedEdge: WeightedEdge } as const;

interface GraphVisualizationProps {
  graph: Graph;
  vertexData: Map<string, VertexData>;
  activeVertexId?: string | null;
  activeEdge?: { from: string; to: string } | null;
  relaxedEdges?: Set<string>;
  pathEdges?: Set<string>;
  currentEventType?: string | null;
}

/** Inner component that can use useReactFlow hook */
function GraphVisualizationInner({
  graph,
  vertexData,
  activeVertexId,
  activeEdge,
  relaxedEdges,
  pathEdges,
  currentEventType,
}: GraphVisualizationProps) {
  const { fitView } = useReactFlow();
  const prevVertexCountRef = useRef(graph.vertices.length);
  const hasInitializedRef = useRef(false);

  // Auto-fit view when graph changes (different size selected) or on first render
  useEffect(() => {
    const vertexCount = graph.vertices.length;
    if (vertexCount !== prevVertexCountRef.current || !hasInitializedRef.current) {
      prevVertexCountRef.current = vertexCount;
      hasInitializedRef.current = true;
      // Small delay to ensure nodes are rendered before fitting
      const timeoutId = setTimeout(() => {
        fitView({ padding: 0.3, duration: 200 });
      }, 50);
      return () => clearTimeout(timeoutId);
    }
  }, [graph.vertices.length, fitView]);

  // Also fit view when algorithm starts (activeVertexId becomes set for first time)
  const wasActiveRef = useRef(false);
  useEffect(() => {
    if (activeVertexId && !wasActiveRef.current) {
      wasActiveRef.current = true;
      const timeoutId = setTimeout(() => {
        fitView({ padding: 0.3, duration: 200 });
      }, 50);
      return () => clearTimeout(timeoutId);
    }
    if (!activeVertexId) {
      wasActiveRef.current = false;
    }
  }, [activeVertexId, fitView]);
  const { nodes, edges } = useMemo(() => {
    // Build vertex nodes
    const flowNodes: Node[] = graph.vertices.map((vertexId) => {
      const data = vertexData.get(vertexId) ?? {
        id: vertexId,
        distance: Infinity,
        state: 'unvisited' as const,
        predecessor: null,
      };

      return {
        id: vertexId,
        type: 'vertexNode',
        position: { x: 0, y: 0 },
        data: {
          label: vertexId,
          distance: data.distance,
          state: data.state,
          isActive: vertexId === activeVertexId,
        },
      };
    });

    // Build edges with animation states
    const flowEdges: Edge[] = graph.edges.map((edge) => {
      const edgeId = `${edge.from}-${edge.to}`;
      const isActive = activeEdge?.from === edge.from && activeEdge?.to === edge.to;
      const isRelaxed = relaxedEdges?.has(edgeId) ?? false;
      const isOnPath = pathEdges?.has(edgeId) ?? false;

      return {
        id: edgeId,
        source: edge.from,
        target: edge.to,
        type: 'weightedEdge',
        markerEnd: { type: MarkerType.ArrowClosed, width: 15, height: 15 },
        animated: isActive && currentEventType === 'visit_neighbor',
        data: {
          weight: edge.weight,
          isActive,
          isRelaxed,
          isOnPath,
        },
      };
    });

    return getGraphLayout(flowNodes, flowEdges);
  }, [graph, vertexData, activeVertexId, activeEdge, relaxedEdges, pathEdges, currentEventType]);

  return (
    <ReactFlow
      nodes={nodes}
      edges={edges}
      nodeTypes={nodeTypes}
      edgeTypes={edgeTypes}
      fitView
      fitViewOptions={{ padding: 0.3 }}
      nodesDraggable={false}
      nodesConnectable={false}
      elementsSelectable={false}
      panOnDrag={true}
      zoomOnScroll={true}
    >
      <Background color="#e0e0e0" gap={16} />
      <Controls showInteractive={false} />
    </ReactFlow>
  );
}

/** Wrapper component that provides ReactFlow context */
export function GraphVisualization(props: GraphVisualizationProps) {
  return (
    <ReactFlowProvider>
      <GraphVisualizationInner {...props} />
    </ReactFlowProvider>
  );
}
