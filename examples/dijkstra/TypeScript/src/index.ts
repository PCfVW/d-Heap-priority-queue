// index.ts - Dijkstra's Algorithm Example
//
// Demonstrates Dijkstra's shortest path algorithm using d-ary heap priority queues.

import { readFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';
import { dijkstra, dijkstraInstrumented, reconstructPath } from './dijkstra.js';
import type { Graph } from './types.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

interface CliArgs {
  graph: string;
  source: string | null;
  target: string | null;
  quiet: boolean;
  stats: boolean;
  arity: number;          // 0 = default [2, 4, 8]
  warmup: number;
  repetitions: number;
  json: boolean;
  envFile: string | null;
  reportRss: boolean;
}

function parseArgs(argv: string[]): CliArgs {
  const args: CliArgs = {
    graph: 'small', source: null, target: null,
    quiet: false, stats: false,
    arity: 0, warmup: 0, repetitions: 1,
    json: false, envFile: null, reportRss: false,
  };
  for (const arg of argv.slice(2)) {
    if (arg.startsWith('--graph=')) args.graph = arg.slice('--graph='.length);
    else if (arg.startsWith('--source=')) args.source = arg.slice('--source='.length);
    else if (arg.startsWith('--target=')) args.target = arg.slice('--target='.length);
    else if (arg === '--quiet') args.quiet = true;
    else if (arg === '--stats') args.stats = true;
    else if (arg.startsWith('--arity=')) args.arity = parseInt(arg.slice('--arity='.length), 10);
    else if (arg.startsWith('--warmup=')) args.warmup = parseInt(arg.slice('--warmup='.length), 10);
    else if (arg.startsWith('--repetitions=')) args.repetitions = parseInt(arg.slice('--repetitions='.length), 10);
    else if (arg === '--json') args.json = true;
    else if (arg.startsWith('--env-file=')) args.envFile = arg.slice('--env-file='.length);
    else if (arg === '--report-rss') args.reportRss = true;
    else {
      console.error(`unknown argument: ${arg}`);
      console.error('usage: node dist/index.js [--graph=NAME] [--source=ID] [--target=ID] [--quiet] [--stats] [--arity=D] [--warmup=K] [--repetitions=N] [--json] [--env-file=PATH] [--report-rss]');
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

function runJSON(
  graph: Graph,
  source: string,
  target: string,
  d: number,
  graphName: string,
  stats: boolean,
  warmup: number,
  repetitions: number,
  env: unknown,
): void {
  if (stats) {
    const { stats: cs } = dijkstraInstrumented(graph, source, d);
    const record = {
      schema_version: 1,
      language: 'TypeScript',
      graph: graphName,
      arity: d,
      comparison_counts: {
        insert: cs.insert,
        pop: cs.pop,
        decrease_priority: cs.decreasePriority,
        increase_priority: cs.increasePriority,
        update_priority: cs.updatePriority,
        total: cs.total,
      },
    };
    console.log(JSON.stringify(record));
    return;
  }

  for (let i = 0; i < warmup; i++) {
    dijkstra(graph, source, d);
  }
  for (let rep = 1; rep <= repetitions; rep++) {
    const start = performance.now();
    dijkstra(graph, source, d);
    const end = performance.now();
    const wallTimeUs = (end - start) * 1000;
    const record = {
      schema_version: 1,
      language: 'TypeScript',
      graph: graphName,
      arity: d,
      source,
      target,
      rep,
      wall_time_us: wallTimeUs,
      ...(env !== null ? { env } : {}),
    };
    console.log(JSON.stringify(record));
  }
}

function main(): void {
  const args = parseArgs(process.argv);
  const graph = loadGraph(args.graph);

  // Default endpoints: textbook A→F for small; v0→v(N-1) otherwise.
  const source = args.source ?? (args.graph === 'small' ? 'A' : graph.vertices[0]);
  const target = args.target ?? (args.graph === 'small' ? 'F' : graph.vertices[graph.vertices.length - 1]);

  const arities = args.arity > 0 ? [args.arity] : [2, 4, 8];

  let env: unknown = null;
  if (args.envFile !== null) {
    try {
      env = JSON.parse(readFileSync(args.envFile, 'utf-8'));
    } catch (e) {
      console.error(`error: failed to read --env-file: ${(e as Error).message}`);
      process.exit(1);
    }
  }

  if (args.reportRss) {
    if (args.arity === 0) {
      console.error('error: --report-rss requires --arity=<d>');
      process.exit(1);
    }
    const d = args.arity;
    dijkstra(graph, source, d);
    const peakRssKb = process.resourceUsage().maxRSS;
    const record = {
      schema_version: 1,
      language: 'TypeScript',
      graph: args.graph,
      arity: d,
      peak_rss_kb: peakRssKb,
    };
    console.log(JSON.stringify(record));
    return;
  }

  if (args.json) {
    for (const d of arities) {
      runJSON(graph, source, target, d, args.graph, args.stats, args.warmup, args.repetitions, env);
    }
    return;
  }

  console.log("Dijkstra's Algorithm Example");
  if (args.graph === 'small') {
    console.log('Network Flows (Ahuja, Magnanti, Orlin) - Figure 4.7');
  } else {
    console.log(`graph: ${args.graph} (|V|=${graph.vertices.length}, |E|=${graph.edges.length})`);
  }
  console.log(`Finding shortest path from ${source} to ${target}\n`);

  for (const d of arities) {
    console.log(`--- Using ${d}-ary heap ---`);
    const start = performance.now();
    const { result, stats } = args.stats
      ? (() => {
          const r = dijkstraInstrumented(graph, source, d);
          return { result: r.result, stats: r.stats };
        })()
      : { result: dijkstra(graph, source, d), stats: null };
    const end = performance.now();

    if (!args.quiet) formatResults(result.distances, source);

    const path = reconstructPath(result.predecessors, source, target);
    const pathStr = path ? path.join(' → ') : 'No path found';

    console.log(`\nShortest path from ${source} to ${target}: ${pathStr}`);
    console.log(`Path cost: ${result.distances[target]}`);
    const elapsedUs = (end - start) * 1000;
    console.log(`Execution time: ${elapsedUs.toFixed(1)}µs`);

    if (stats !== null) {
      console.log(
        `Comparison counts: insert=${stats.insert}, pop=${stats.pop}, ` +
        `decrease_priority=${stats.decreasePriority}, ` +
        `increase_priority=${stats.increasePriority}, ` +
        `update_priority=${stats.updatePriority}, total=${stats.total}`
      );
    }

    console.log();
  }
}

main();
