import { memo } from 'react';
import { BaseEdge, EdgeLabelRenderer, getSmoothStepPath, type EdgeProps } from '@xyflow/react';

export interface WeightedEdgeData {
  weight: number;
  isActive?: boolean;
  isOnPath?: boolean;
  isRelaxed?: boolean;
}

function WeightedEdgeComponent({
  id,
  sourceX,
  sourceY,
  targetX,
  targetY,
  sourcePosition,
  targetPosition,
  data,
  markerEnd,
}: EdgeProps) {
  const edgeData = data as unknown as WeightedEdgeData | undefined;
  const weight = edgeData?.weight ?? 0;
  const isActive = edgeData?.isActive ?? false;
  const isOnPath = edgeData?.isOnPath ?? false;
  const isRelaxed = edgeData?.isRelaxed ?? false;

  const [edgePath, labelX, labelY] = getSmoothStepPath({
    sourceX,
    sourceY,
    sourcePosition,
    targetX,
    targetY,
    targetPosition,
  });

  // Determine edge appearance based on state
  let strokeColor = '#b0b0b0';
  let strokeWidth = 2;
  let className = '';

  if (isOnPath) {
    strokeColor = '#22c55e';
    strokeWidth = 4;
    className = 'edge-on-path';
  } else if (isActive) {
    strokeColor = '#f59e0b';
    strokeWidth = 3;
    className = 'edge-active';
  } else if (isRelaxed) {
    strokeColor = '#3b82f6';
    strokeWidth = 2.5;
    className = 'edge-relaxed';
  }

  return (
    <>
      <BaseEdge
        id={id}
        path={edgePath}
        markerEnd={markerEnd}
        style={{
          stroke: strokeColor,
          strokeWidth,
          transition: 'stroke 0.3s ease, stroke-width 0.3s ease',
        }}
        className={className}
      />
      <EdgeLabelRenderer>
        <div
          className={`edge-label ${isActive ? 'edge-label-active' : ''} ${isOnPath ? 'edge-label-path' : ''}`}
          style={{
            position: 'absolute',
            transform: `translate(-50%, -50%) translate(${labelX}px, ${labelY}px)`,
            pointerEvents: 'all',
          }}
        >
          {weight}
        </div>
      </EdgeLabelRenderer>
    </>
  );
}

export const WeightedEdge = memo(WeightedEdgeComponent);
