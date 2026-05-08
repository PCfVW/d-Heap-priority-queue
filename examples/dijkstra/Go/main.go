// main.go - Dijkstra's Algorithm Example
//
// Demonstrates Dijkstra's shortest path algorithm using d-ary heap priority queues.

package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"time"
)

func loadGraph(name string) (Graph, error) {
	filename := name + ".json"
	candidates := []string{
		filepath.Join("..", "graphs", filename),
		filepath.Join("examples", "dijkstra", "graphs", filename),
	}

	var data []byte
	var err error
	for _, p := range candidates {
		data, err = os.ReadFile(p)
		if err == nil {
			break
		}
	}
	if err != nil {
		return Graph{}, fmt.Errorf("graph file not found for --graph=%s (looked in ../graphs/ and examples/dijkstra/graphs/)", name)
	}

	var graph Graph
	if err := json.Unmarshal(data, &graph); err != nil {
		return Graph{}, fmt.Errorf("failed to parse graph JSON: %w", err)
	}
	return graph, nil
}

func formatResults(distances map[string]int, source string) {
	fmt.Printf("Shortest paths from vertex %s:\n", source)
	fmt.Println("================================")

	vertices := make([]string, 0, len(distances))
	for v := range distances {
		vertices = append(vertices, v)
	}
	sort.Strings(vertices)

	for _, v := range vertices {
		distance := distances[v]
		distanceStr := fmt.Sprintf("%d", distance)
		if distance == Infinity {
			distanceStr = "∞"
		}
		fmt.Printf("%s → %s: %s\n", source, v, distanceStr)
	}
}

func main() {
	graphName := flag.String("graph", "small", "graph name (small | medium_sparse | medium_dense | medium_grid | large_sparse | large_dense | large_grid)")
	sourceFlag := flag.String("source", "", "source vertex ID (defaults to A for small, v0 otherwise)")
	targetFlag := flag.String("target", "", "target vertex ID (defaults to F for small, last vertex otherwise)")
	quiet := flag.Bool("quiet", false, "suppress per-vertex distance output")
	flag.Parse()

	graph, err := loadGraph(*graphName)
	if err != nil {
		fmt.Fprintf(os.Stderr, "error: %v\n", err)
		os.Exit(1)
	}

	source := *sourceFlag
	target := *targetFlag
	if source == "" {
		if *graphName == "small" {
			source = "A"
		} else {
			source = graph.Vertices[0]
		}
	}
	if target == "" {
		if *graphName == "small" {
			target = "F"
		} else {
			target = graph.Vertices[len(graph.Vertices)-1]
		}
	}

	fmt.Println("Dijkstra's Algorithm Example")
	if *graphName == "small" {
		fmt.Println("Network Flows (Ahuja, Magnanti, Orlin) - Figure 4.7")
	} else {
		fmt.Printf("graph: %s (|V|=%d, |E|=%d)\n", *graphName, len(graph.Vertices), len(graph.Edges))
	}
	fmt.Printf("Finding shortest path from %s to %s\n\n", source, target)

	arities := []int{2, 4, 8}
	for _, d := range arities {
		fmt.Printf("--- Using %d-ary heap ---\n", d)

		start := time.Now()
		result := Dijkstra(graph, source, d)
		elapsed := time.Since(start)

		if !*quiet {
			formatResults(result.Distances, source)
		}

		path := ReconstructPath(result.Predecessors, source, target)
		var pathStr string
		if path != nil {
			pathStr = strings.Join(path, " → ")
		} else {
			pathStr = "No path found"
		}

		fmt.Printf("\nShortest path from %s to %s: %s\n", source, target, pathStr)
		fmt.Printf("Path cost: %d\n", result.Distances[target])
		elapsedUs := float64(elapsed.Nanoseconds()) / 1000.0
		fmt.Printf("Execution time: %.1fµs\n\n", elapsedUs)
	}
}
