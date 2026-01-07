import { memo } from 'react';
import { Handle, Position, type NodeProps } from '@xyflow/react';

export interface HeapNodeData {
  label: string;
  priority: number;
  isActive?: boolean;
  isHighlighted?: boolean;
  isBubbling?: boolean;
  isInserting?: boolean;
}

function HeapNodeComponent({ data }: NodeProps) {
  const nodeData = data as unknown as HeapNodeData;
  const { label, priority, isActive, isHighlighted, isBubbling, isInserting } = nodeData;

  const classNames = ['heap-node'];
  if (isActive) classNames.push('active');
  if (isHighlighted) classNames.push('highlighted');
  if (isBubbling) classNames.push('bubbling');
  if (isInserting) classNames.push('inserting');

  return (
    <div className={classNames.join(' ')}>
      <Handle type="target" position={Position.Top} />
      <div className="heap-node-label">{label}</div>
      <div className="heap-node-priority">{priority === Infinity ? '∞' : priority}</div>
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
}

export const HeapNode = memo(HeapNodeComponent);
