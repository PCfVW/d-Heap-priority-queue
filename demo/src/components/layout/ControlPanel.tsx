type GraphSize = 'small' | 'medium' | 'large';

interface ControlPanelProps {
  arity: number;
  onArityChange: (arity: number) => void;
  onStep: () => void;
  onPlay: () => void;
  onReset: () => void;
  isPlaying: boolean;
  isComplete: boolean;
  pausedAtStep: number | null;
  onRaceMode?: () => void;
  playbackSpeed: number;
  onSpeedChange: (speed: number) => void;
  currentStep: number;
  totalSteps: number;
  onSeek: (step: number) => void;
  graphSize?: GraphSize;
  onGraphSizeChange?: (size: GraphSize) => void;
}

const SPEED_OPTIONS = [
  { value: 800, label: '0.5x' },
  { value: 400, label: '1x' },
  { value: 200, label: '2x' },
  { value: 100, label: '4x' },
];

export function ControlPanel({
  arity,
  onArityChange,
  onStep,
  onPlay,
  onReset,
  isPlaying,
  isComplete,
  pausedAtStep,
  onRaceMode,
  playbackSpeed,
  onSpeedChange,
  currentStep,
  totalSteps,
  onSeek,
  graphSize,
  onGraphSizeChange,
}: ControlPanelProps) {
  const speedIndex = SPEED_OPTIONS.findIndex((s) => s.value === playbackSpeed);

  // Determine play button label based on state
  const getPlayButtonLabel = () => {
    if (isPlaying) return 'Pause';
    if (isComplete) return 'Replay';
    if (currentStep > -1) return 'Resume';
    return 'Play';
  };

  return (
    <div className="control-panel">
      {graphSize && onGraphSizeChange && (
        <div className="control-group">
          <span className="control-label">Graph:</span>
          <button
            className={`graph-btn ${graphSize === 'small' ? 'active' : ''}`}
            onClick={() => onGraphSizeChange('small')}
          >
            S
          </button>
          <button
            className={`graph-btn ${graphSize === 'medium' ? 'active' : ''}`}
            onClick={() => onGraphSizeChange('medium')}
          >
            M
          </button>
          <button
            className={`graph-btn ${graphSize === 'large' ? 'active' : ''}`}
            onClick={() => onGraphSizeChange('large')}
          >
            L
          </button>
        </div>
      )}

      <div className="control-group">
        <span className="control-label">Arity:</span>
        <button
          className={`arity-btn ${arity === 2 ? 'active' : ''}`}
          onClick={() => onArityChange(2)}
        >
          d=2
        </button>
        <button
          className={`arity-btn ${arity === 4 ? 'active' : ''}`}
          onClick={() => onArityChange(4)}
        >
          d=4
        </button>
        <button
          className={`arity-btn ${arity === 8 ? 'active' : ''}`}
          onClick={() => onArityChange(8)}
        >
          d=8
        </button>
      </div>

      <div className="control-group">
        <button className="control-btn" onClick={onStep} disabled={isPlaying}>
          Step
        </button>
        <button className="control-btn play-btn" onClick={onPlay}>
          {getPlayButtonLabel()}
        </button>
        <button className="control-btn" onClick={onReset}>
          Reset
        </button>
        {onRaceMode && (
          <button className="control-btn race-mode-btn" onClick={onRaceMode}>
            Race Mode
          </button>
        )}
      </div>

      <div className="control-group speed-control">
        <label htmlFor="speed-slider" className="control-label">Speed:</label>
        <input
          id="speed-slider"
          type="range"
          min="0"
          max={SPEED_OPTIONS.length - 1}
          value={speedIndex >= 0 ? speedIndex : 1}
          onChange={(e) => onSpeedChange(SPEED_OPTIONS[parseInt(e.target.value)].value)}
          className="speed-slider"
          aria-label="Playback speed"
        />
        <span className="speed-value">
          {SPEED_OPTIONS[speedIndex >= 0 ? speedIndex : 1].label}
        </span>
      </div>

      <div className="timeline-control">
        <label htmlFor="timeline-slider" className="visually-hidden">Timeline position</label>
        <div className="timeline-slider-container">
          <input
            id="timeline-slider"
            type="range"
            min="-1"
            max={totalSteps}
            value={currentStep}
            onChange={(e) => onSeek(parseInt(e.target.value))}
            className="timeline-slider"
            aria-label="Timeline position"
            aria-valuemin={0}
            aria-valuemax={totalSteps + 1}
            aria-valuenow={Math.max(0, currentStep + 1)}
            aria-valuetext={`Step ${Math.max(0, currentStep + 1)} of ${totalSteps + 1}`}
          />
          {pausedAtStep !== null && pausedAtStep !== currentStep && !isPlaying && totalSteps > 0 && (() => {
            // Calculate position: slider range is -1 to totalSteps
            const percent = ((pausedAtStep + 1) / (totalSteps + 1)) * 100;
            return (
              <div
                className="pause-marker"
                style={{
                  // Use calc to account for slider thumb (approx 8px radius on each end)
                  left: `calc(${percent}% + ${8 - (percent / 100) * 16}px)`,
                }}
                title={`Paused at step ${pausedAtStep + 2}`}
                onClick={() => onSeek(pausedAtStep)}
              />
            );
          })()}
        </div>
        <span className="timeline-label">
          {Math.max(0, currentStep + 1)} / {totalSteps + 1}
        </span>
      </div>
    </div>
  );
}
