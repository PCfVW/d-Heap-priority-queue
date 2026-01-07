import dagre from 'dagre';
import type { Node, Edge } from '@xyflow/react';

const NODE_WIDTH = 80;
const NODE_HEIGHT = 50;

/** Apply dagre layout to nodes and edges */
function applyDagreLayout(
  nodes: Node[],
  edges: Edge[],
  nodesep: number,
  ranksep: number
): { nodes: Node[]; edges: Edge[] } {
  const graph = new dagre.graphlib.Graph();
  graph.setDefaultEdgeLabel(() => ({}));
  graph.setGraph({ rankdir: 'TB', nodesep, ranksep });

  nodes.forEach((node) => {
    graph.setNode(node.id, { width: NODE_WIDTH, height: NODE_HEIGHT });
  });

  edges.forEach((edge) => {
    graph.setEdge(edge.source, edge.target);
  });

  dagre.layout(graph);

  const layoutedNodes = nodes.map((node) => {
    const pos = graph.node(node.id);
    return {
      ...node,
      position: {
        x: pos.x - NODE_WIDTH / 2,
        y: pos.y - NODE_HEIGHT / 2,
      },
    };
  });

  return { nodes: layoutedNodes, edges };
}

/** Layout for heap tree visualization */
export function getHeapTreeLayout(nodes: Node[], edges: Edge[]) {
  return applyDagreLayout(nodes, edges, 50, 80);
}

/** Layout for graph visualization */
export function getGraphLayout(nodes: Node[], edges: Edge[]) {
  return applyDagreLayout(nodes, edges, 100, 100);
}
