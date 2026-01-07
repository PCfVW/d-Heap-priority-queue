interface OperationCounterProps {
  inserts: number;
  pops: number;
  decreasePriority: number;
  currentStep: number;
  totalSteps: number;
}

export function OperationCounter({
  inserts,
  pops,
  decreasePriority,
  currentStep,
  totalSteps,
}: OperationCounterProps) {
  return (
    <div className="operation-counter">
      <div className="step-indicator">
        Step {currentStep} / {totalSteps}
      </div>
      <div className="op-stats">
        <span className="op-stat">
          <span className="op-label">Inserts:</span>
          <span className="op-value">{inserts}</span>
        </span>
        <span className="op-stat">
          <span className="op-label">Pops:</span>
          <span className="op-value">{pops}</span>
        </span>
        <span className="op-stat">
          <span className="op-label">Decrease:</span>
          <span className="op-value">{decreasePriority}</span>
        </span>
      </div>
    </div>
  );
}
