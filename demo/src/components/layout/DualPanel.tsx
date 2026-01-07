import type { ReactNode } from 'react';

interface DualPanelProps {
  leftPanel: ReactNode;
  rightPanel: ReactNode;
  leftTitle: string;
  rightTitle: string;
}

export function DualPanel({ leftPanel, rightPanel, leftTitle, rightTitle }: DualPanelProps) {
  return (
    <div className="dual-panel">
      <div className="panel left-panel">
        <div className="panel-header">{leftTitle}</div>
        <div className="panel-content">{leftPanel}</div>
      </div>
      <div className="panel right-panel">
        <div className="panel-header">{rightTitle}</div>
        <div className="panel-content">{rightPanel}</div>
      </div>
    </div>
  );
}
