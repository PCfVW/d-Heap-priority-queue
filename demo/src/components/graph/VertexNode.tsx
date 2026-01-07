import { memo, useRef, useEffect, useState } from 'react';
import { Handle, Position, type NodeProps } from '@xyflow/react';
import type { VertexState } from '../../types';

export interface VertexNodeData {
  label: string;
  distance: number;
  state: VertexState;
  isActive?: boolean;
}

const stateColors: Record<VertexState, string> = {
  unvisited: '#9ca3af', // gray
  in_queue: '#3b82f6',  // blue
  processed: '#22c55e', // green
};

function VertexNodeComponent({ data }: NodeProps) {
  const { label, distance, state, isActive } = data as unknown as VertexNodeData;
  const prevStateRef = useRef<VertexState>(state);
  const [isTransitioning, setIsTransitioning] = useState(false);

  // Detect state changes for transition animation
  useEffect(() => {
    if (prevStateRef.current !== state) {
      setIsTransitioning(true);
      prevStateRef.current = state;
      const timer = setTimeout(() => setIsTransitioning(false), 500);
      return () => clearTimeout(timer);
    }
  }, [state]);

  const classNames = ['vertex-node'];
  if (isActive) classNames.push('active');
  if (isTransitioning) classNames.push('state-changing');

  return (
    <div
      className={classNames.join(' ')}
      style={{ borderColor: stateColors[state] }}
    >
      <Handle type="target" position={Position.Top} style={{ visibility: 'hidden' }} />
      <Handle type="target" position={Position.Left} style={{ visibility: 'hidden' }} />
      <div className="vertex-label">{label}</div>
      <div className="vertex-distance">
        {distance === Infinity ? '∞' : distance}
      </div>
      <Handle type="source" position={Position.Bottom} style={{ visibility: 'hidden' }} />
      <Handle type="source" position={Position.Right} style={{ visibility: 'hidden' }} />
    </div>
  );
}

export const VertexNode = memo(VertexNodeComponent);
