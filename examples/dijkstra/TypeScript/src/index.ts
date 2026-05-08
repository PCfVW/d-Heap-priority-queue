// index.ts - Dijkstra's Algorithm Example
//
// Demonstrates Dijkstra's shortest path algorithm using d-ary heap priority queues.

import { readFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';
import { dijkstra, reconstructPath } from './dijkstra.js';
import type { Graph } from './types.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

interface CliArgs {
  graph: string;
  source: string | null;
  target: string | null;
  quiet: boolean;
}

function parseArgs(argv: string[]): CliArgs {
  const args: CliArgs = { graph: 'small', source: null, target: null, quiet: false };
  for (const arg of argv.slice(2)) {
    if (arg.startsWith('--graph=')) args.graph = arg.slice('--graph='.length);
    else if (arg.startsWith('--source=')) args.source = arg.slice('--source='.length);
    else if (arg.startsWith('--target=')) args.target = arg.slice('--target='.length);
    else if (arg === '--quiet') args.quiet = true;
    else {
      console.error(`unknown argument: ${arg}`);
      console.error('usage: npm start -- [--graph=NAME] [--source=ID] [--target=ID] [--quiet]');
      process.exit(2);
    }
  }
  return args;
}

function loadGraph(name: string): Graph {
  // Resolve relative to the compiled file location (dist/), going up to the
  // example root then into ../graphs/. This matches the build layout regardless
  // of where the user runs `npm start` from.
  const graphPath = join(__dirname, '../../graphs/' + name + '.json');
  const graphData = readFileSync(graphPath, 'utf-8');
  return JSON.parse(graphData) as Graph;
}

function formatResults(distances: Record<string, number>, source: string): void {
  console.log(`Shortest paths from vertex ${source}:`);
  console.log('================================');

  const vertices = Object.keys(distances).sort();
  for (const vertex of vertices) {
    const distance = distances[vertex];
    const distanceStr = distance === Infinity ? '∞' : distance.toString();
    console.log(`${source} → ${vertex}: ${distanceStr}`);
  }
}

function main(): void {
  const args = parseArgs(process.argv);
  const graph = loadGraph(args.graph);

  // Default endpoints: textbook A→F for small; v0→v(N-1) otherwise.
  const source = args.source ?? (args.graph === 'small' ? 'A' : graph.vertices[0]);
  const target = args.target ?? (args.graph === 'small' ? 'F' : graph.vertices[graph.vertices.length - 1]);

  console.log("Dijkstra's Algorithm Example");
  if (args.graph === 'small') {
    console.log('Network Flows (Ahuja, Magnanti, Orlin) - Figure 4.7');
  } else {
    console.log(`graph: ${args.graph} (|V|=${graph.vertices.length}, |E|=${graph.edges.length})`);
  }
  console.log(`Finding shortest path from ${source} to ${target}\n`);

  const arities = [2, 4, 8];
  for (const d of arities) {
    console.log(`--- Using ${d}-ary heap ---`);
    const start = performance.now();
    const result = dijkstra(graph, source, d);
    const end = performance.now();

    if (!args.quiet) formatResults(result.distances, source);

    const path = reconstructPath(result.predecessors, source, target);
    const pathStr = path ? path.join(' → ') : 'No path found';

    console.log(`\nShortest path from ${source} to ${target}: ${pathStr}`);
    console.log(`Path cost: ${result.distances[target]}`);
    console.log(`Execution time: ${(end - start).toFixed(3)}ms\n`);
  }
}

main();
