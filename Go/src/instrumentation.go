package dheap

// Comparison-count instrumentation for the priority queue (v2.6.0 Phase 2).
//
// Mirrors the contract from TypeScript/src/instrumentation.ts (v2.4.0): count
// *comparisons* (not operations), bucketed by which heap operation triggered
// them. Per the v2.6.0 ROADMAP, the Go mechanism is a nil pointer in the heap:
// pq.stats *Stats is nil by default — overhead in that path is a single
// well-predicted nil check per comparison.
//
// Ownership note: in Go, the *user* owns the Stats storage (typically a
// stack-local var dheap.Stats); the heap stores a *Stats pointer that may be
// nil. Read access is via pq.Stats() or via the original variable; Total() and
// Reset() are nil-safe so pq.Stats().Total() always works regardless of
// whether instrumentation was attached. This is deliberately asymmetric with
// C++, where the heap owns the stats by value via [[no_unique_address]] — a
// design choice driven by each language's idioms and zero-overhead mechanism.
//
// Usage:
//
//	var stats dheap.Stats
//	pq := dheap.New(dheap.Options[Vertex, string]{
//	    D:            4,
//	    Comparator:   dheap.MinBy(func(v Vertex) int { return v.Distance }),
//	    KeyExtractor: func(v Vertex) string { return v.ID },
//	    Stats:        &stats,
//	})
//	// ... use pq ...
//	fmt.Printf("inserts=%d, pops=%d, total=%d\n", stats.Insert, stats.Pop, stats.Total())
//
// Cross-language equivalents:
//   - TypeScript: ComparisonStats from instrumentation.ts (v2.4.0)
//   - C++: TOOLS::ComparisonStats from PriorityQueue.h (v2.6.0)
//   - Rust: StatsCollector trait (Phase 2, planned)
//   - Zig: comptime bool parameter (Phase 2, planned)

// OperationType identifies which public heap method triggered a comparison.
type OperationType int

const (
	OpNone OperationType = iota
	OpInsert
	OpPop
	OpDecreasePriority
	OpIncreasePriority
	OpUpdatePriority
)

// Stats holds per-operation comparison counts. Pass a *Stats via
// Options.Stats to opt in. Counters are public so callers can read them
// directly; Total() and Reset() are convenience methods.
//
// Cross-language equivalents:
//   - C++: TOOLS::ComparisonStats
//   - TypeScript: ComparisonStats
type Stats struct {
	Insert           uint64
	Pop              uint64
	DecreasePriority uint64
	IncreasePriority uint64
	UpdatePriority   uint64

	// currentOp is heap-internal: set by startOperation, cleared by endOperation.
	// Users do not touch this directly.
	currentOp OperationType
}

// Total returns the sum of all five counter buckets.
//
// Nil-safe: returns 0 when called on a nil receiver. This makes
// `pq.Stats().Total()` safe even when no Stats was attached at construction —
// matching the C++ `pq.stats().total()` semantics where the default NoOpStats
// policy always returns 0.
func (s *Stats) Total() uint64 {
	if s == nil {
		return 0
	}
	return s.Insert + s.Pop + s.DecreasePriority + s.IncreasePriority + s.UpdatePriority
}

// Reset zeroes all counters and clears the active-operation tag. The heap
// state is unaffected by this call — the heap and the stats are independent.
//
// Nil-safe: a no-op when called on a nil receiver.
//
// Not safe to call concurrently with heap operations on another goroutine —
// the package as a whole assumes single-goroutine use.
func (s *Stats) Reset() {
	if s == nil {
		return
	}
	*s = Stats{}
}

// startOperation is called by the heap at the entry of each public mutator.
// Unexported: heap-internal hook.
func (s *Stats) startOperation(op OperationType) {
	s.currentOp = op
}

// endOperation is called by the heap at the exit of each public mutator
// (typically via defer).
func (s *Stats) endOperation() {
	s.currentOp = OpNone
}

// countComparison is called by the heap each time the comparator runs.
// Comparisons attributed to OpNone (i.e. outside a bracketed operation) are
// dropped — there shouldn't be any in normal flow.
func (s *Stats) countComparison() {
	switch s.currentOp {
	case OpInsert:
		s.Insert++
	case OpPop:
		s.Pop++
	case OpDecreasePriority:
		s.DecreasePriority++
	case OpIncreasePriority:
		s.IncreasePriority++
	case OpUpdatePriority:
		s.UpdatePriority++
	}
}
