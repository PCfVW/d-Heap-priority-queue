// dijkstra.go - Dijkstra's shortest path algorithm implementation

package main

import (
	"math"

	dheap "github.com/PCfVW/d-Heap-priority-queue/Go/v2/src"
)

// Infinity represents an unreachable distance.
const Infinity = math.MaxInt

// Dijkstra finds the shortest paths from a source vertex to all other vertices
// using a d-ary heap priority queue.
//
// Parameters:
//   - graph: The input graph with vertices and weighted edges
//   - source: The source vertex to find shortest paths from
//   - d: The arity of the heap (default 4 for optimal performance)
//
// Returns a DijkstraResult containing distances and predecessors for path reconstruction.
func Dijkstra(graph Graph, source string, d int) DijkstraResult {
	return dijkstraImpl(graph, source, d, nil)
}

// DijkstraInstrumented runs the algorithm with comparison-count instrumentation
// attached to the underlying priority queue. Returns the result alongside a
// pointer to the populated dheap.Stats so the caller can read per-operation
// comparison counts (Insert / Pop / IncreasePriority / DecreasePriority /
// UpdatePriority — only Insert / Pop / IncreasePriority are exercised by this
// algorithm).
func DijkstraInstrumented(graph Graph, source string, d int) (DijkstraResult, *dheap.Stats) {
	stats := &dheap.Stats{}
	result := dijkstraImpl(graph, source, d, stats)
	return result, stats
}

// dijkstraImpl is the shared algorithm body. When stats is nil, the heap runs
// with no instrumentation; when non-nil, every comparison is bucketed into the
// matching operation counter inside *stats.
func dijkstraImpl(graph Graph, source string, d int, stats *dheap.Stats) DijkstraResult {
	// Build adjacency list for efficient neighbor lookup
	adjacency := make(map[string][]struct {
		To     string
		Weight int
	})
	for _, v := range graph.Vertices {
		adjacency[v] = nil
	}
	for _, e := range graph.Edges {
		adjacency[e.From] = append(adjacency[e.From], struct {
			To     string
			Weight int
		}{To: e.To, Weight: e.Weight})
	}

	// Initialize distances and predecessors
	distances := make(map[string]int)
	predecessors := make(map[string]*string)

	// Create priority queue with min-heap by distance
	pq := dheap.New(dheap.Options[Vertex, string]{
		D:            d,
		Comparator:   dheap.MinBy(func(v Vertex) int { return v.Distance }),
		KeyExtractor: func(v Vertex) string { return v.ID },
		Stats:        stats, // nil == no instrumentation
	})

	// Set initial distances and add to priority queue
	for _, v := range graph.Vertices {
		distance := Infinity
		if v == source {
			distance = 0
		}
		distances[v] = distance
		predecessors[v] = nil
		pq.Insert(Vertex{ID: v, Distance: distance})
	}

	// Main algorithm loop
	for !pq.IsEmpty() {
		current, _ := pq.Pop()

		// Skip if we've already found a shorter path
		if current.Distance > distances[current.ID] {
			continue
		}

		// Skip if current distance is infinity (unreachable)
		if current.Distance == Infinity {
			continue
		}

		// Check all neighbors
		for _, neighbor := range adjacency[current.ID] {
			newDistance := current.Distance + neighbor.Weight

			if newDistance < distances[neighbor.To] {
				distances[neighbor.To] = newDistance
				pred := current.ID
				predecessors[neighbor.To] = &pred

				// Update priority in queue
				// In a min-heap, decreasing distance = increasing priority (more important)
				if pq.Contains(Vertex{ID: neighbor.To}) {
					pq.IncreasePriority(Vertex{ID: neighbor.To, Distance: newDistance})
				}
			}
		}
	}

	return DijkstraResult{
		Distances:    distances,
		Predecessors: predecessors,
	}
}

// ReconstructPath builds the shortest path from source to target using predecessors.
//
// Returns the path as a slice of vertex IDs, or nil if no path exists.
func ReconstructPath(predecessors map[string]*string, source, target string) []string {
	if predecessors[target] == nil && target != source {
		return nil // No path exists
	}

	var path []string
	current := &target

	for current != nil {
		path = append([]string{*current}, path...) // Prepend
		current = predecessors[*current]
	}

	if len(path) > 0 && path[0] == source {
		return path
	}
	return nil
}
