import { readFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';
import { dijkstra, reconstructPath } from './dijkstra.js';
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
function loadGraph() {
    const graphPath = join(__dirname, '../../graphs/small.json');
    const graphData = readFileSync(graphPath, 'utf-8');
    return JSON.parse(graphData);
}
function formatResults(distances, source) {
    console.log(`Shortest paths from vertex ${source}:`);
    console.log('================================');
    for (const [vertex, distance] of Object.entries(distances)) {
        const distanceStr = distance === Infinity ? '∞' : distance.toString();
        console.log(`${source} → ${vertex}: ${distanceStr}`);
    }
}
function main() {
    const graph = loadGraph();
    const source = 'A';
    const target = 'F';
    console.log('Dijkstra\'s Algorithm Example');
    console.log('Network Flows (Ahuja, Magnanti, Orlin) - Figure 4.7');
    console.log(`Finding shortest path from ${source} to ${target}\n`);
    // Test with different heap arities
    const arities = [2, 4, 8];
    for (const d of arities) {
        console.log(`--- Using ${d}-ary heap ---`);
        const start = performance.now();
        const result = dijkstra(graph, source, d);
        const end = performance.now();
        formatResults(result.distances, source);
        const path = reconstructPath(result.predecessors, source, target);
        const pathStr = path ? path.join(' → ') : 'No path found';
        console.log(`\nShortest path from ${source} to ${target}: ${pathStr}`);
        console.log(`Path cost: ${result.distances[target]}`);
        console.log(`Execution time: ${(end - start).toFixed(3)}ms\n`);
    }
}
// Run if this file is executed directly
main();
//# sourceMappingURL=index.js.map