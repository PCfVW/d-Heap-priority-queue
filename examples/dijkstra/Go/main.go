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

	dheap "github.com/PCfVW/d-Heap-priority-queue/Go/v2/src"
)

// WallTimeRecord is one JSONL line emitted per timed dijkstra call in --json mode.
type WallTimeRecord struct {
	SchemaVersion int             `json:"schema_version"`
	Language      string          `json:"language"`
	Graph         string          `json:"graph"`
	Arity         int             `json:"arity"`
	Source        string          `json:"source"`
	Target        string          `json:"target"`
	Rep           int             `json:"rep"`
	WallTimeUs    float64         `json:"wall_time_us"`
	Env           json.RawMessage `json:"env,omitempty"`
}

// StatsRecord is the single JSON object emitted by --json --stats per arity.
type StatsRecord struct {
	SchemaVersion    int        `json:"schema_version"`
	Language         string     `json:"language"`
	Graph            string     `json:"graph"`
	Arity            int        `json:"arity"`
	ComparisonCounts CompCounts `json:"comparison_counts"`
}

// CompCounts mirrors the six fields the Rust harness emits; Dijkstra leaves
// decrease_priority and update_priority at 0, but they're emitted for schema
// uniformity. The aggregator's cross-language invariant is `total` only.
type CompCounts struct {
	Insert           int64 `json:"insert"`
	Pop              int64 `json:"pop"`
	DecreasePriority int64 `json:"decrease_priority"`
	IncreasePriority int64 `json:"increase_priority"`
	UpdatePriority   int64 `json:"update_priority"`
	Total            int64 `json:"total"`
}

// RssRecord is the single JSON object emitted by --report-rss.
type RssRecord struct {
	SchemaVersion int    `json:"schema_version"`
	Language      string `json:"language"`
	Graph         string `json:"graph"`
	Arity         int    `json:"arity"`
	PeakRssKb     uint64 `json:"peak_rss_kb"`
}

// SectionUs mirrors the four Phase 3.5 section fields plus their sum.
type SectionUs struct {
	Setup  float64 `json:"setup"`
	Pop    float64 `json:"pop"`
	Relax  float64 `json:"relax"`
	IncPri float64 `json:"inc_pri"`
	Total  float64 `json:"total"`
}

// ProfileRecord is one JSONL line emitted per timed --profile-sections call.
type ProfileRecord struct {
	SchemaVersion int       `json:"schema_version"`
	Language      string    `json:"language"`
	Graph         string    `json:"graph"`
	Arity         int       `json:"arity"`
	Rep           int       `json:"rep"`
	SectionUs     SectionUs `json:"section_us"`
}

// timerOverheadIterations back-to-back calls to time.Now() are timed as a
// batch and divided down for one --timer-overhead trial — timing a single
// call directly would be dominated by the very timer resolution being
// measured.
const timerOverheadIterations = 1_000_000

// timerOverheadTrials independent trials, so the aggregator can report
// median/IQR the same way it does for every other measurement in this
// project (methodology.md's "Reported summary statistics").
const timerOverheadTrials = 10

// TimerOverheadRecord is one JSONL line emitted per --timer-overhead trial.
type TimerOverheadRecord struct {
	SchemaVersion int     `json:"schema_version"`
	Language      string  `json:"language"`
	Trial         int     `json:"trial"`
	Iterations    int     `json:"iterations"`
	PerCallUs     float64 `json:"per_call_us"`
	PerBracketUs  float64 `json:"per_bracket_us"`
}

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
	graphName := flag.String("graph", "small", "graph name (small | medium_sparse | medium_dense | medium_grid | large_sparse | large_dense | large_grid | huge_dense)")
	sourceFlag := flag.String("source", "", "source vertex ID (defaults to A for small, v0 otherwise)")
	targetFlag := flag.String("target", "", "target vertex ID (defaults to F for small, last vertex otherwise)")
	quiet := flag.Bool("quiet", false, "suppress per-vertex distance output")
	statsFlag := flag.Bool("stats", false, "enable comparison-count instrumentation and print per-arity buckets")
	arityFlag := flag.Int("arity", 0, "run only one specific arity (0 = default [2,4,8])")
	warmupFlag := flag.Int("warmup", 0, "number of un-timed warmup runs before timed repetitions (--json mode only)")
	repetitionsFlag := flag.Int("repetitions", 1, "number of timed repetitions per arity (--json mode only)")
	jsonFlag := flag.Bool("json", false, "emit JSONL records to stdout instead of human-readable output")
	envFileFlag := flag.String("env-file", "", "path to env-Go.json; contents are inlined into each wall-time record")
	reportRssFlag := flag.Bool("report-rss", false, "run dijkstra once and emit a single peak-RSS JSON record (requires --arity)")
	profileSectionsFlag := flag.Bool("profile-sections", false, "run dijkstra with per-section timers, reusing --warmup/--repetitions (requires --arity)")
	timerOverheadFlag := flag.Bool("timer-overhead", false, "measure this process's own timer-call overhead (graph/arity-independent)")
	flag.Parse()

	// --timer-overhead: no graph/arity involved, handled before the
	// (otherwise-unconditional) graph load below.
	if *timerOverheadFlag {
		for trial := 1; trial <= timerOverheadTrials; trial++ {
			start := time.Now()
			for i := 0; i < timerOverheadIterations; i++ {
				_ = time.Now()
			}
			elapsed := time.Since(start)
			totalUs := float64(elapsed.Nanoseconds()) / 1000.0
			perCallUs := totalUs / float64(timerOverheadIterations)
			rec := TimerOverheadRecord{
				SchemaVersion: 1,
				Language:      "Go",
				Trial:         trial,
				Iterations:    timerOverheadIterations,
				PerCallUs:     perCallUs,
				PerBracketUs:  perCallUs * 2,
			}
			data, _ := json.Marshal(rec)
			fmt.Println(string(data))
		}
		return
	}

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

	var arities []int
	if *arityFlag > 0 {
		arities = []int{*arityFlag}
	} else {
		arities = []int{2, 4, 8}
	}

	var envRaw json.RawMessage
	if *envFileFlag != "" {
		data, err := os.ReadFile(*envFileFlag)
		if err != nil {
			fmt.Fprintf(os.Stderr, "error: failed to read --env-file: %v\n", err)
			os.Exit(1)
		}
		var probe interface{}
		if err := json.Unmarshal(data, &probe); err != nil {
			fmt.Fprintf(os.Stderr, "error: --env-file is not valid JSON: %v\n", err)
			os.Exit(1)
		}
		envRaw = data
	}

	if *reportRssFlag {
		if *arityFlag == 0 {
			fmt.Fprintln(os.Stderr, "error: --report-rss requires --arity=<d>")
			os.Exit(1)
		}
		d := *arityFlag
		_ = Dijkstra(graph, source, d)
		peak, _ := peakRssKb()
		rec := RssRecord{
			SchemaVersion: 1,
			Language:      "Go",
			Graph:         *graphName,
			Arity:         d,
			PeakRssKb:     peak,
		}
		data, _ := json.Marshal(rec)
		fmt.Println(string(data))
		return
	}

	if *profileSectionsFlag {
		if *arityFlag == 0 {
			fmt.Fprintln(os.Stderr, "error: --profile-sections requires --arity=<d>")
			os.Exit(1)
		}
		d := *arityFlag
		// Same K=2/N=10 warmup-then-repeat protocol as the wall-time --json
		// path below, reused here after the first real Phase 3.5 run showed
		// a single un-iterated call is too noisy to attribute cross-language
		// gaps (see benchmarks/methodology.md, "Phase 3.5").
		for i := 0; i < *warmupFlag; i++ {
			_, _ = DijkstraProfiled(graph, source, d)
		}
		for rep := 1; rep <= *repetitionsFlag; rep++ {
			_, timings := DijkstraProfiled(graph, source, d)
			rec := ProfileRecord{
				SchemaVersion: 1,
				Language:      "Go",
				Graph:         *graphName,
				Arity:         d,
				Rep:           rep,
				SectionUs: SectionUs{
					Setup:  timings.SetupUs,
					Pop:    timings.PopUs,
					Relax:  timings.RelaxUs,
					IncPri: timings.IncPriUs,
					Total:  timings.TotalUs(),
				},
			}
			data, _ := json.Marshal(rec)
			fmt.Println(string(data))
		}
		return
	}

	if *jsonFlag {
		for _, d := range arities {
			runJSON(graph, source, target, d, *graphName, *statsFlag, *warmupFlag, *repetitionsFlag, envRaw)
		}
		return
	}

	fmt.Println("Dijkstra's Algorithm Example")
	if *graphName == "small" {
		fmt.Println("Network Flows (Ahuja, Magnanti, Orlin) - Figure 4.7")
	} else {
		fmt.Printf("graph: %s (|V|=%d, |E|=%d)\n", *graphName, len(graph.Vertices), len(graph.Edges))
	}
	fmt.Printf("Finding shortest path from %s to %s\n\n", source, target)

	for _, d := range arities {
		fmt.Printf("--- Using %d-ary heap ---\n", d)

		var result DijkstraResult
		var stats *dheap.Stats

		start := time.Now()
		if *statsFlag {
			result, stats = DijkstraInstrumented(graph, source, d)
		} else {
			result = Dijkstra(graph, source, d)
		}
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
		fmt.Printf("Execution time: %.1fµs\n", elapsedUs)

		if stats != nil {
			fmt.Printf("Comparison counts: insert=%d, pop=%d, decrease_priority=%d, increase_priority=%d, update_priority=%d, total=%d\n",
				stats.Insert, stats.Pop, stats.DecreasePriority,
				stats.IncreasePriority, stats.UpdatePriority, stats.Total())
		}

		fmt.Println()
	}
}

func runJSON(graph Graph, source, target string, d int, graphName string, statsFlag bool, warmup, repetitions int, envRaw json.RawMessage) {
	if statsFlag {
		_, stats := DijkstraInstrumented(graph, source, d)
		rec := StatsRecord{
			SchemaVersion: 1,
			Language:      "Go",
			Graph:         graphName,
			Arity:         d,
			ComparisonCounts: CompCounts{
				Insert:           int64(stats.Insert),
				Pop:              int64(stats.Pop),
				DecreasePriority: int64(stats.DecreasePriority),
				IncreasePriority: int64(stats.IncreasePriority),
				UpdatePriority:   int64(stats.UpdatePriority),
				Total:            int64(stats.Total()),
			},
		}
		data, _ := json.Marshal(rec)
		fmt.Println(string(data))
		return
	}

	for i := 0; i < warmup; i++ {
		_ = Dijkstra(graph, source, d)
	}
	for rep := 1; rep <= repetitions; rep++ {
		start := time.Now()
		_ = Dijkstra(graph, source, d)
		elapsed := time.Since(start)
		wallTimeUs := float64(elapsed.Nanoseconds()) / 1000.0
		rec := WallTimeRecord{
			SchemaVersion: 1,
			Language:      "Go",
			Graph:         graphName,
			Arity:         d,
			Source:        source,
			Target:        target,
			Rep:           rep,
			WallTimeUs:    wallTimeUs,
			Env:           envRaw,
		}
		data, _ := json.Marshal(rec)
		fmt.Println(string(data))
	}
}
