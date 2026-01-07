import { useMemo } from 'react';

interface ArityStats {
  arity: number;
  inserts: number;
  pops: number;
  decreasePriority: number;
  totalOps: number;
}

interface ArityComparisonProps {
  currentArity: number;
  currentStats: { inserts: number; pops: number; decreasePriority: number };
  /** Pre-computed stats for all arities (from running algorithm with each) */
  allArityStats?: Map<number, { inserts: number; pops: number; decreasePriority: number }>;
}

export function ArityComparison({
  currentArity,
  currentStats,
  allArityStats,
}: ArityComparisonProps) {
  const stats: ArityStats[] = useMemo(() => {
    const arities = [2, 4, 8];
    return arities.map((arity) => {
      const s = arity === currentArity ? currentStats : allArityStats?.get(arity);
      if (!s) {
        return {
          arity,
          inserts: 0,
          pops: 0,
          decreasePriority: 0,
          totalOps: 0,
        };
      }
      return {
        arity,
        inserts: s.inserts,
        pops: s.pops,
        decreasePriority: s.decreasePriority,
        totalOps: s.inserts + s.pops + s.decreasePriority,
      };
    });
  }, [currentArity, currentStats, allArityStats]);

  const maxOps = Math.max(...stats.map((s) => s.totalOps), 1);

  return (
    <div className="arity-comparison">
      <div className="arity-comparison-title">Operations (same for all arities)</div>
      <div className="arity-comparison-subtitle">Comparisons differ - see panel stats above</div>
      <div className="arity-comparison-grid">
        {stats.map((s) => (
          <div
            key={s.arity}
            className={`arity-stat-card ${s.arity === currentArity ? 'active' : ''}`}
          >
            <div className="arity-stat-header">d={s.arity}</div>
            <div className="arity-stat-bar-container">
              <div
                className="arity-stat-bar"
                style={{ height: `${(s.totalOps / maxOps) * 100}%` }}
              />
            </div>
            <div className="arity-stat-details">
              <div className="arity-stat-row">
                <span>Insert:</span>
                <span>{s.inserts}</span>
              </div>
              <div className="arity-stat-row">
                <span>Pop:</span>
                <span>{s.pops}</span>
              </div>
              <div className="arity-stat-row">
                <span>Decrease:</span>
                <span>{s.decreasePriority}</span>
              </div>
              <div className="arity-stat-row total">
                <span>Total:</span>
                <span>{s.totalOps}</span>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
