import { useState, useMemo, useEffect } from 'react';

import { DualPanel, ControlPanel, RaceMode } from './components/layout';
import { HeapVisualization } from './components/heap';
import { GraphVisualization } from './components/graph';
import { OperationCounter, DepthMeter } from './components/stats';
import { loadGraph, type GraphSize } from './lib/graph-loader';
import { useDijkstraRunner } from './hooks';

import './App.css';
import './styles/animations.css';

function App() {
  const [arity, setArity] = useState(4);
  const [isRaceMode, setIsRaceMode] = useState(false);
  const [playbackSpeed, setPlaybackSpeed] = useState(400);
  const [graphSize, setGraphSize] = useState<GraphSize>('small');
  const [pausedAtStep, setPausedAtStep] = useState<number | null>(null);

  // Load the graph based on size selection
  const graph = useMemo(() => loadGraph(graphSize), [graphSize]);

  // Source and target for Dijkstra (depends on graph)
  const { source, target } = useMemo(() => {
    if (graphSize === 'small') return { source: 'A', target: 'F' };
    if (graphSize === 'medium') return { source: 'A', target: 'J' };
    return { source: 'A', target: 'O' }; // large
  }, [graphSize]);

  // Use the Dijkstra runner hook
  const {
    currentStep,
    totalSteps,
    isPlaying,
    isComplete,
    step,
    play,
    pause,
    reset,
    goToStep,
    heapNodes,
    vertexData,
    activeVertexId,
    activeEdge,
    relaxedEdges,
    animatedPathEdges,
    stats,
    currentOperation,
    currentEventType,
    bubblingNodeId,
  } = useDijkstraRunner({
    graph,
    source,
    target,
    arity,
    playbackSpeed,
  });

  const handlePlay = () => {
    if (isPlaying) {
      // Pausing - record where we paused
      setPausedAtStep(currentStep);
      pause();
    } else {
      // Resuming - clear the pause marker (it will reappear on next pause)
      setPausedAtStep(null);
      play();
    }
  };

  // Clear pause marker on reset
  const handleReset = () => {
    setPausedAtStep(null);
    reset();
  };

  // Keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Ignore if user is typing in an input
      if (e.target instanceof HTMLInputElement) return;

      switch (e.key) {
        case ' ': // Space - Play/Pause
          e.preventDefault();
          handlePlay();
          break;
        case 'ArrowRight': // Right arrow - Step forward
          if (!isPlaying) step();
          break;
        case 'ArrowLeft': // Left arrow - Step backward
          if (!isPlaying && currentStep > -1) goToStep(currentStep - 1);
          break;
        case 'r': // R - Reset
        case 'R':
          handleReset();
          break;
        case '1': // 1 - Arity 2
          setArity(2);
          break;
        case '2': // 2 - Arity 4
          setArity(4);
          break;
        case '3': // 3 - Arity 8
          setArity(8);
          break;
        case 's': // S - Small graph
        case 'S':
          setGraphSize('small');
          break;
        case 'm': // M - Medium graph
        case 'M':
          setGraphSize('medium');
          break;
        case 'l': // L - Large graph
        case 'L':
          setGraphSize('large');
          break;
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isPlaying, currentStep, step, handleReset, goToStep, handlePlay]);

  // Show race mode if enabled
  if (isRaceMode) {
    return <RaceMode onExit={() => setIsRaceMode(false)} />;
  }

  return (
    <div className="app">
      <header className="app-header">
        <h1>d-ary Heap Priority Queue Visualization</h1>
        <p>Dijkstra's Shortest Path: {source} → {target}</p>
      </header>

      <main>
      <DualPanel
        leftTitle={`Heap Structure (d=${arity})`}
        rightTitle="Graph"
        leftPanel={
          <HeapVisualization
            heapNodes={heapNodes}
            arity={arity}
            activeNodeId={activeVertexId ?? undefined}
            currentEventType={currentEventType}
            bubblingNodeId={bubblingNodeId}
          />
        }
        rightPanel={
          <GraphVisualization
            graph={graph}
            vertexData={vertexData}
            activeVertexId={activeVertexId}
            activeEdge={activeEdge}
            relaxedEdges={relaxedEdges}
            pathEdges={animatedPathEdges.size > 0 ? animatedPathEdges : undefined}
            currentEventType={currentEventType}
          />
        }
      />

      <div className="operation-display">
        <span className="current-operation">{currentOperation}</span>
      </div>

      <ControlPanel
        arity={arity}
        onArityChange={setArity}
        onStep={step}
        onPlay={handlePlay}
        onReset={handleReset}
        isPlaying={isPlaying}
        isComplete={isComplete}
        pausedAtStep={pausedAtStep}
        onRaceMode={() => setIsRaceMode(true)}
        playbackSpeed={playbackSpeed}
        onSpeedChange={setPlaybackSpeed}
        currentStep={currentStep}
        totalSteps={totalSteps}
        onSeek={goToStep}
        graphSize={graphSize}
        onGraphSizeChange={setGraphSize}
      />

      <div className="stats-row">
        <OperationCounter
          inserts={stats.inserts}
          pops={stats.pops}
          decreasePriority={stats.decreasePriority}
          currentStep={Math.max(0, currentStep + 1)}
          totalSteps={totalSteps + 1}
        />
        <DepthMeter heapSize={heapNodes.length} arity={arity} />
      </div>
      </main>
    </div>
  );
}

export default App;
