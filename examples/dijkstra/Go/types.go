// Package main provides Dijkstra's shortest path algorithm example.
package main

// Graph represents a weighted directed graph.
type Graph struct {
	Vertices []string `json:"vertices"`
	Edges    []Edge   `json:"edges"`
}

// Edge represents a weighted directed edge.
type Edge struct {
	From   string `json:"from"`
	To     string `json:"to"`
	Weight int    `json:"weight"`
}

// Vertex represents a vertex with its current distance from the source.
// Used as the item type in the priority queue.
type Vertex struct {
	ID       string
	Distance int
}

// DijkstraResult contains the output of Dijkstra's algorithm.
type DijkstraResult struct {
	// Distances maps each vertex to its shortest distance from the source.
	Distances map[string]int
	// Predecessors maps each vertex to its predecessor in the shortest path.
	// nil value means no predecessor (source or unreachable).
	Predecessors map[string]*string
}
