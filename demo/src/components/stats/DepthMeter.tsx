interface DepthMeterProps {
  heapSize: number;
  arity: number;
}

/** Calculate the depth of a d-ary heap with n elements */
function calculateHeapDepth(n: number, d: number): number {
  if (n <= 0) return 0;
  if (n === 1) return 1;
  // Depth = floor(log_d(n * (d-1) + 1)) for d-ary heap
  return Math.floor(Math.log(n * (d - 1) + 1) / Math.log(d));
}

/** Calculate theoretical depths for comparison */
function getComparisonDepths(n: number): { d2: number; d4: number; d8: number } {
  return {
    d2: calculateHeapDepth(n, 2),
    d4: calculateHeapDepth(n, 4),
    d8: calculateHeapDepth(n, 8),
  };
}

export function DepthMeter({ heapSize, arity }: DepthMeterProps) {
  const depths = getComparisonDepths(heapSize);
  const currentDepth = calculateHeapDepth(heapSize, arity);
  const maxDepth = Math.max(depths.d2, depths.d4, depths.d8, 1);

  return (
    <div className="depth-meter">
      <div className="depth-meter-title">Heap Depth Comparison</div>
      <div className="depth-bars">
        <div className="depth-bar-row">
          <span className="depth-label">d=2</span>
          <div className="depth-bar-container">
            <div
              className={`depth-bar ${arity === 2 ? 'active' : ''}`}
              style={{ width: `${(depths.d2 / maxDepth) * 100}%` }}
            />
          </div>
          <span className="depth-value">{depths.d2}</span>
        </div>
        <div className="depth-bar-row">
          <span className="depth-label">d=4</span>
          <div className="depth-bar-container">
            <div
              className={`depth-bar ${arity === 4 ? 'active' : ''}`}
              style={{ width: `${(depths.d4 / maxDepth) * 100}%` }}
            />
          </div>
          <span className="depth-value">{depths.d4}</span>
        </div>
        <div className="depth-bar-row">
          <span className="depth-label">d=8</span>
          <div className="depth-bar-container">
            <div
              className={`depth-bar ${arity === 8 ? 'active' : ''}`}
              style={{ width: `${(depths.d8 / maxDepth) * 100}%` }}
            />
          </div>
          <span className="depth-value">{depths.d8}</span>
        </div>
      </div>
      <div className="depth-note">
        Current: depth {currentDepth} with {heapSize} nodes
      </div>
    </div>
  );
}
