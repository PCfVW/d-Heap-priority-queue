import { useState, useEffect, useCallback, useRef, useMemo } from 'react';
import { HeapVisualization } from '../heap';
import { DepthMeter, ArityComparison } from '../stats';
import { useDijkstraRunner } from '../../hooks';
import { loadGraph, getGraphEndpoints, type GraphSize, type GraphDensity } from '../../lib/graph-loader';

interface RaceModeProps {
  onExit: () => void;
}

export function RaceMode({ onExit }: RaceModeProps) {
  const [isRacing, setIsRacing] = useState(false);
  const [raceComplete, setRaceComplete] = useState(false);
  const [graphSize, setGraphSize] = useState<GraphSize>('small');
  const [graphDensity, setGraphDensity] = useState<GraphDensity>('sparse');
  const playbackSpeed = 300;

  // Load graph based on size and density
  const graph = useMemo(() => loadGraph(graphSize, graphDensity), [graphSize, graphDensity]);
  const { source, target } = useMemo(() => getGraphEndpoints(graphSize, graphDensity), [graphSize, graphDensity]);

  // Create runners for each arity
  const runner2 = useDijkstraRunner({ graph, source, target, arity: 2, playbackSpeed });
  const runner4 = useDijkstraRunner({ graph, source, target, arity: 4, playbackSpeed });
  const runner8 = useDijkstraRunner({ graph, source, target, arity: 8, playbackSpeed });

  const runners = [
    { arity: 2, runner: runner2 },
    { arity: 4, runner: runner4 },
    { arity: 8, runner: runner8 },
  ];

  // Check if all runners are complete
  const allComplete = runner2.isComplete && runner4.isComplete && runner8.isComplete;

  // Track race completion
  const raceStartedRef = useRef(false);
  useEffect(() => {
    if (isRacing && allComplete && raceStartedRef.current) {
      setIsRacing(false);
      setRaceComplete(true);
    }
  }, [isRacing, allComplete]);

  const startRace = useCallback(() => {
    runner2.reset();
    runner4.reset();
    runner8.reset();
    setRaceComplete(false);
    raceStartedRef.current = true;
    // Small delay to ensure reset is complete
    setTimeout(() => {
      runner2.play();
      runner4.play();
      runner8.play();
      setIsRacing(true);
    }, 50);
  }, [runner2, runner4, runner8]);

  const pauseRace = useCallback(() => {
    runner2.pause();
    runner4.pause();
    runner8.pause();
    setIsRacing(false);
  }, [runner2, runner4, runner8]);

  const resetRace = useCallback(() => {
    runner2.reset();
    runner4.reset();
    runner8.reset();
    setIsRacing(false);
    setRaceComplete(false);
    raceStartedRef.current = false;
  }, [runner2, runner4, runner8]);

  // Build comparison stats map (operation counts - same across arities)
  const allArityStats = new Map<number, { inserts: number; pops: number; decreasePriority: number }>();
  allArityStats.set(2, runner2.stats);
  allArityStats.set(4, runner4.stats);
  allArityStats.set(8, runner8.stats);

  // Build comparison counts map (differ by arity - the key differentiator!)
  const allArityComparisons = new Map<number, { insert: number; pop: number; decreasePriority: number; total: number }>();
  allArityComparisons.set(2, runner2.comparisons);
  allArityComparisons.set(4, runner4.comparisons);
  allArityComparisons.set(8, runner8.comparisons);

  // Find the winner based on COMPARISON COUNTS (not operation counts!)
  // This is what actually differs between arities and determines performance.
  const getWinnerInfo = () => {
    if (!raceComplete) return { winner: null, isTie: false, tiedArities: [] as number[] };
    // Winner is the one with fewest total comparisons
    const totals = runners.map(({ arity, runner }) => ({
      arity,
      total: runner.comparisons.total,
    }));
    const minTotal = Math.min(...totals.map((t) => t.total));
    const winners = totals.filter((t) => t.total === minTotal);

    if (winners.length > 1) {
      // Tie - in practice, lower arity is often preferred (simpler)
      return {
        winner: winners[0]?.arity ?? null,
        isTie: true,
        tiedArities: winners.map(w => w.arity)
      };
    }
    return { winner: winners[0]?.arity ?? null, isTie: false, tiedArities: [] };
  };

  const { winner, isTie, tiedArities } = getWinnerInfo();

  // Reset race when graph changes
  useEffect(() => {
    resetRace();
  }, [graphSize, graphDensity]);

  return (
    <div className="race-mode">
      <div className="race-header">
        <h2>Arity Race Mode</h2>
        <p>
          Compare d=2, d=4, and d=8 heaps running Dijkstra: {source} → {target}
        </p>
        <div className="race-graph-options">
          <div className="race-option-group">
            <span className="race-option-label">Density:</span>
            <button
              className={`race-option-btn ${graphDensity === 'sparse' ? 'active' : ''}`}
              onClick={() => setGraphDensity('sparse')}
              disabled={isRacing}
            >
              Sparse
            </button>
            <button
              className={`race-option-btn ${graphDensity === 'dense' ? 'active' : ''}`}
              onClick={() => setGraphDensity('dense')}
              disabled={isRacing}
            >
              Dense
            </button>
            <button
              className={`race-option-btn ${graphDensity === 'extra-dense' ? 'active' : ''}`}
              onClick={() => { setGraphDensity('extra-dense'); setGraphSize('large'); }}
              disabled={isRacing}
              title="Shows d=8 advantage on decrease-key (D) vs d=2 advantage on pop (P)"
            >
              X-Dense
            </button>
          </div>
          <div className="race-option-group">
            <span className="race-option-label">Size:</span>
            <button
              className={`race-option-btn ${graphSize === 'small' ? 'active' : ''}`}
              onClick={() => setGraphSize('small')}
              disabled={isRacing || graphDensity === 'extra-dense'}
            >
              S
            </button>
            <button
              className={`race-option-btn ${graphSize === 'medium' ? 'active' : ''}`}
              onClick={() => setGraphSize('medium')}
              disabled={isRacing || graphDensity === 'extra-dense'}
            >
              M
            </button>
            <button
              className={`race-option-btn ${graphSize === 'large' ? 'active' : ''}`}
              onClick={() => setGraphSize('large')}
              disabled={isRacing || graphDensity === 'extra-dense'}
            >
              L
            </button>
          </div>
        </div>
        <div className="race-controls">
          {!isRacing && !raceComplete && (
            <button className="race-btn start" onClick={startRace}>
              Start Race
            </button>
          )}
          {isRacing && (
            <button className="race-btn pause" onClick={pauseRace}>
              Pause
            </button>
          )}
          {(isRacing || raceComplete) && (
            <button className="race-btn reset" onClick={resetRace}>
              Reset
            </button>
          )}
          <button className="race-btn exit" onClick={onExit}>
            Exit Race Mode
          </button>
        </div>
        {raceComplete && winner && (
          <div className="race-winner">
            {isTie
              ? `Tie: d=${tiedArities.join(' & d=')} heaps (${allArityComparisons.get(winner)?.total} comparisons each)`
              : `Winner: d=${winner} heap (${allArityComparisons.get(winner)?.total} comparisons)`}
          </div>
        )}
        {graphDensity === 'sparse' && (
          <p className="race-hint">Sparse graphs typically favor d=2 (fewer extract-min comparisons)</p>
        )}
        {graphDensity === 'dense' && (
          <p className="race-hint">Dense graphs typically favor d=4 or d=8 (more decrease-key operations)</p>
        )}
        {graphDensity === 'extra-dense' && (
          <p className="race-hint">Notice d=8 has fewest D comparisons (shallower tree), but d=2 wins on P (fewer children)</p>
        )}
      </div>

      <div className="race-panels">
        {runners.map(({ arity, runner }) => (
          <div
            key={arity}
            className={`race-panel ${raceComplete && (isTie ? tiedArities.includes(arity) : winner === arity) ? 'winner' : ''}`}
          >
            <div className="race-panel-header">
              <span className="race-panel-title">d={arity} Heap</span>
              <span className="race-panel-step">
                Step {Math.max(0, runner.currentStep + 1)} / {runner.totalSteps + 1}
              </span>
            </div>
            <div className="race-panel-content">
              <HeapVisualization
                heapNodes={runner.heapNodes}
                arity={arity}
                activeNodeId={runner.activeVertexId ?? undefined}
                currentEventType={runner.currentEventType}
                bubblingNodeId={runner.bubblingNodeId}
              />
            </div>
            <div className="race-panel-stats">
              <span title="Insert comparisons">I:{runner.comparisons.insert}</span>
              <span title="Pop comparisons">P:{runner.comparisons.pop}</span>
              <span title="Decrease-priority comparisons">D:{runner.comparisons.decreasePriority}</span>
              <span title="Total comparisons" className="race-panel-total">Σ:{runner.comparisons.total}</span>
            </div>
          </div>
        ))}
      </div>

      <div className="race-comparison">
        <DepthMeter heapSize={runner4.heapNodes.length} arity={4} />
        <ArityComparison
          currentArity={4}
          currentStats={runner4.stats}
          allArityStats={allArityStats}
        />
      </div>
    </div>
  );
}
