import { memo, useId } from 'react';
import { BaseEdge, getSmoothStepPath, type EdgeProps } from '@xyflow/react';

export interface AnimatedHeapEdgeData {
  isBubbling?: boolean;
}

function AnimatedHeapEdgeComponent({
  id,
  sourceX,
  sourceY,
  targetX,
  targetY,
  sourcePosition,
  targetPosition,
  data,
}: EdgeProps) {
  const edgeData = data as unknown as AnimatedHeapEdgeData | undefined;
  const isBubbling = edgeData?.isBubbling ?? false;
  const uniqueId = useId();

  const [edgePath] = getSmoothStepPath({
    sourceX,
    sourceY,
    sourcePosition,
    targetX,
    targetY,
    targetPosition,
  });

  return (
    <>
      <BaseEdge
        id={id}
        path={edgePath}
        style={{
          stroke: isBubbling ? '#f59e0b' : '#1e3a5f',
          strokeWidth: isBubbling ? 3 : 2,
          transition: 'stroke 0.3s ease, stroke-width 0.3s ease',
        }}
      />
      {isBubbling && (
        <circle r="6" fill="#f59e0b">
          <animateMotion
            dur="0.5s"
            repeatCount="1"
            path={edgePath}
            keyPoints="1;0"
            keyTimes="0;1"
            calcMode="linear"
            key={`${uniqueId}-${id}`}
          />
        </circle>
      )}
    </>
  );
}

export const AnimatedHeapEdge = memo(AnimatedHeapEdgeComponent);
