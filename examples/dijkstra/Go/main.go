package main

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"
)

func loadGraph() (Graph, error) {
	// Get the directory of the current executable or source file
	graphPath := filepath.Join("..", "graphs", "small.json")

	data, err := os.ReadFile(graphPath)
	if err != nil {
		// Try alternative path for when running from project root
		graphPath = filepath.Join("examples", "dijkstra", "graphs", "small.json")
		data, err = os.ReadFile(graphPath)
		if err != nil {
			return Graph{}, fmt.Errorf("failed to read graph file: %w", err)
		}
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

	// Sort vertices for consistent output
	vertices := []string{"A", "B", "C", "D", "E", "F"}
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
	graph, err := loadGraph()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}

	source := "A"
	target := "F"

	fmt.Println("Dijkstra's Algorithm Example")
	fmt.Println("Network Flows (Ahuja, Magnanti, Orlin) - Figure 4.7")
	fmt.Printf("Finding shortest path from %s to %s\n\n", source, target)

	// Test with different heap arities
	arities := []int{2, 4, 8}

	for _, d := range arities {
		fmt.Printf("--- Using %d-ary heap ---\n", d)

		start := time.Now()
		result := Dijkstra(graph, source, d)
		elapsed := time.Since(start)

		formatResults(result.Distances, source)

		path := ReconstructPath(result.Predecessors, source, target)
		var pathStr string
		if path != nil {
			pathStr = strings.Join(path, " → ")
		} else {
			pathStr = "No path found"
		}

		fmt.Printf("\nShortest path from %s to %s: %s\n", source, target, pathStr)
		fmt.Printf("Path cost: %d\n", result.Distances[target])
		fmt.Printf("Execution time: %v\n\n", elapsed)
	}
}
