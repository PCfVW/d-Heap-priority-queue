# d-ary Heap Visualization Demo

Interactive visualization of d-ary heap priority queues and Dijkstra's shortest path algorithm using React Flow.

**[Live Demo](https://pcfvw.github.io/d-Heap-priority-queue/)**

## Quick Start

```bash
npm install
npm run dev
```

Open http://localhost:5173 in your browser.

## Features

### Visualization
- Dual-panel layout: heap tree (left) and graph (right)
- Dynamic heap tree with automatic dagre layout
- Graph visualization with weighted edges
- Real-time vertex state coloring (unvisited/in-queue/processed)
- Shortest path highlighting with sequential animation

### Controls
- **Arity toggle**: d=2, d=4, d=8 heap comparison
- **Graph size**: Small (6 nodes), Medium (10 nodes), Large (15 nodes)
- **Playback**: Step, Play/Pause, Reset
- **Speed slider**: 0.5x to 4x playback speed
- **Timeline scrubber**: Jump to any algorithm step
- **Race Mode**: Compare all three arities simultaneously

### Animations
- Node highlighting for active operations
- Bubble-up animation with SVG `<animateMotion>`
- Edge relaxation wave effects
- Smooth state transitions
- Sequential path discovery animation

### Keyboard Shortcuts
| Key | Action |
|-----|--------|
| Space | Play/Pause |
| → | Step forward |
| ← | Step backward |
| R | Reset |
| 1/2/3 | Set arity (d=2/4/8) |
| S/M/L | Set graph size |

### Statistics
- Live operation counter (inserts, pops, decrease_priority)
- Heap depth comparison meter
- Current step / total steps indicator

## Tech Stack

- React 18 + TypeScript
- Vite 7
- React Flow (@xyflow/react)
- dagre (tree layout)
- d-ary-heap (NPM package)

## Deployment

The demo is automatically deployed to GitHub Pages on push to master. See `.github/workflows/deploy-demo.yml`.

---

Part of the [d-Heap Priority Queue](https://github.com/eric-jacopin/Priority-Queues) project v2.5.0.
