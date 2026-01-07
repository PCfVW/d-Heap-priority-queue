import { useMemo, useEffect, useRef } from 'react';
import {
  ReactFlow,
  ReactFlowProvider,
  Background,
  Controls,
  useReactFlow,
  type Node,
  type Edge,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';

import { HeapNode } from './HeapNode';
import { AnimatedHeapEdge } from './AnimatedHeapEdge';
import { getHeapTreeLayout } from '../../lib/layout-utils';
import type { HeapNode as HeapNodeType } from '../../types';

const nodeTypes = { heapNode: HeapNode } as const;
const edgeTypes = { animatedHeapEdge: AnimatedHeapEdge } as const;

interface HeapVisualizationProps {
  heapNodes: HeapNodeType[];
  arity: number;
  activeNodeId?: string;
  currentEventType?: string | null;
  bubblingNodeId?: string | null;
}

/** Inner component that can use useReactFlow hook */
function HeapVisualizationInner({
  heapNodes,
  arity,
  activeNodeId,
  currentEventType,
  bubblingNodeId,
}: HeapVisualizationProps) {
  const { fitView } = useReactFlow();
  const prevNodeCountRef = useRef(heapNodes.length);

  // Auto-fit view when heap size changes
  useEffect(() => {
    if (heapNodes.length !== prevNodeCountRef.current) {
      prevNodeCountRef.current = heapNodes.length;
      // Small delay to ensure nodes are rendered before fitting
      const timeoutId = setTimeout(() => {
        fitView({ padding: 0.2, duration: 200 });
      }, 50);
      return () => clearTimeout(timeoutId);
    }
  }, [heapNodes.length, fitView]);

  const { nodes, edges } = useMemo(() => {
    if (heapNodes.length === 0) {
      return { nodes: [], edges: [] };
    }

    // Determine animation states based on event type
    const isInsertEvent = currentEventType === 'insert';
    const isDecreasePriorityEvent = currentEventType === 'decrease_priority';

    // Convert heap array to React Flow nodes
    const flowNodes: Node[] = heapNodes.map((node, index) => ({
      id: node.id,
      type: 'heapNode',
      position: { x: 0, y: 0 },
      data: {
        label: node.id,
        priority: node.priority,
        isActive: node.id === activeNodeId,
        isHighlighted: false,
        isBubbling: node.id === bubblingNodeId && isDecreasePriorityEvent,
        isInserting: node.id === activeNodeId && isInsertEvent && index === heapNodes.length - 1,
      },
    }));

    // Create edges based on d-ary heap parent-child relationships
    // Find which edge should animate (from bubbling node to its parent)
    let bubblingEdgeId: string | null = null;
    if (bubblingNodeId && isDecreasePriorityEvent) {
      const bubblingIndex = heapNodes.findIndex(n => n.id === bubblingNodeId);
      if (bubblingIndex > 0) {
        const parentIndex = Math.floor((bubblingIndex - 1) / arity);
        bubblingEdgeId = `e-${heapNodes[parentIndex].id}-${heapNodes[bubblingIndex].id}`;
      }
    }

    const flowEdges: Edge[] = [];
    for (let i = 1; i < heapNodes.length; i++) {
      const parentIndex = Math.floor((i - 1) / arity);
      const edgeId = `e-${heapNodes[parentIndex].id}-${heapNodes[i].id}`;
      const isBubblingEdge = edgeId === bubblingEdgeId;

      flowEdges.push({
        id: edgeId,
        source: heapNodes[parentIndex].id,
        target: heapNodes[i].id,
        type: isBubblingEdge ? 'animatedHeapEdge' : 'smoothstep',
        data: { isBubbling: isBubblingEdge },
      });
    }

    return getHeapTreeLayout(flowNodes, flowEdges);
  }, [heapNodes, arity, activeNodeId, currentEventType, bubblingNodeId]);

  return (
    <ReactFlow
      nodes={nodes}
      edges={edges}
      nodeTypes={nodeTypes}
      edgeTypes={edgeTypes}
      fitView
      fitViewOptions={{ padding: 0.2 }}
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
export function HeapVisualization(props: HeapVisualizationProps) {
  if (props.heapNodes.length === 0) {
    return (
      <div className="heap-empty">
        <p>Heap is empty</p>
        <p className="heap-empty-hint">Run Dijkstra to see the heap structure</p>
      </div>
    );
  }

  return (
    <ReactFlowProvider>
      <HeapVisualizationInner {...props} />
    </ReactFlowProvider>
  );
}
