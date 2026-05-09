package dheap

import "testing"

// =====================================================================
// Tests for v2.6.0 Phase 2 comparison-count instrumentation.
//
// Verifies two properties:
//   1. The default heap (no Stats) behaves identically to the instrumented
//      heap on the observable side (popped-item order).
//   2. With Stats attached, comparisons are correctly attributed to the
//      operation that triggered them.
// =====================================================================

// newIntStatsHeap builds a min-heap of ints with comparison-count instrumentation.
func newIntStatsHeap(d int) (*PriorityQueue[int, int], *Stats) {
	stats := &Stats{}
	pq := New(Options[int, int]{
		D:            d,
		Comparator:   MinNumber,
		KeyExtractor: func(x int) int { return x },
		Stats:        stats,
	})
	return pq, stats
}

func TestStatsNilByDefault(t *testing.T) {
	pq := New(Options[int, int]{
		D:            2,
		Comparator:   MinNumber,
		KeyExtractor: func(x int) int { return x },
	})
	if pq.Stats() != nil {
		t.Fatalf("expected Stats() == nil for default heap, got %p", pq.Stats())
	}
}

func TestNilStatsTotalAndResetAreSafe(t *testing.T) {
	// pq.Stats() returns nil by default. Calling Total() or Reset() on that nil
	// pointer must not panic — matches the C++ semantics where the default
	// NoOpStats path always returns 0.
	pq := New(Options[int, int]{
		D:            2,
		Comparator:   MinNumber,
		KeyExtractor: func(x int) int { return x },
	})
	pq.Insert(1)
	pq.Insert(2)
	pq.Insert(3)

	stats := pq.Stats()
	if stats != nil {
		t.Fatalf("expected nil Stats() for default heap, got %p", stats)
	}
	if got := stats.Total(); got != 0 {
		t.Fatalf("expected nil-safe Total() == 0, got %d", got)
	}
	stats.Reset() // must not panic
}

func TestStatsInitialState(t *testing.T) {
	_, stats := newIntStatsHeap(2)
	if stats.Insert != 0 || stats.Pop != 0 || stats.DecreasePriority != 0 ||
		stats.IncreasePriority != 0 || stats.UpdatePriority != 0 {
		t.Fatalf("expected all five buckets zero on a fresh heap, got %+v", *stats)
	}
	if stats.Total() != 0 {
		t.Fatalf("expected Total() == 0, got %d", stats.Total())
	}
}

func TestInsertBucket(t *testing.T) {
	pq, stats := newIntStatsHeap(2)
	for _, v := range []int{5, 3, 8, 1, 9} {
		pq.Insert(v)
	}
	if stats.Insert == 0 {
		t.Fatal("expected Insert bucket to be > 0 after a series of Insert calls")
	}
	if stats.Pop != 0 || stats.DecreasePriority != 0 ||
		stats.IncreasePriority != 0 || stats.UpdatePriority != 0 {
		t.Fatalf("expected only Insert bucket to be non-zero, got %+v", *stats)
	}
	if stats.Total() != stats.Insert {
		t.Fatalf("expected Total() == Insert, got Total=%d Insert=%d", stats.Total(), stats.Insert)
	}
}

func TestPopBucket(t *testing.T) {
	pq, stats := newIntStatsHeap(2)
	for _, v := range []int{5, 3, 8, 1, 9} {
		pq.Insert(v)
	}
	insertBaseline := stats.Insert

	popped, ok := pq.Pop()
	if !ok || popped != 1 {
		t.Fatalf("expected to pop 1, got %d ok=%v", popped, ok)
	}

	if stats.Insert != insertBaseline {
		t.Fatalf("Insert bucket should be unchanged by Pop; baseline=%d now=%d",
			insertBaseline, stats.Insert)
	}
	if stats.Pop == 0 {
		t.Fatal("expected Pop bucket > 0 after Pop()")
	}
	if stats.DecreasePriority != 0 || stats.IncreasePriority != 0 || stats.UpdatePriority != 0 {
		t.Fatalf("expected only Insert and Pop buckets to be non-zero, got %+v", *stats)
	}
}

func TestDecreasePriorityBucket(t *testing.T) {
	pq, stats := newIntStatsHeap(2)
	for _, v := range []int{1, 2, 3, 4} {
		pq.Insert(v)
	}
	stats.Reset()

	pq.DecreasePriorityByIndex(0)

	if stats.DecreasePriority == 0 {
		t.Fatal("expected DecreasePriority bucket > 0 after DecreasePriorityByIndex(0)")
	}
	if stats.Insert != 0 || stats.Pop != 0 ||
		stats.IncreasePriority != 0 || stats.UpdatePriority != 0 {
		t.Fatalf("expected only DecreasePriority bucket to be non-zero, got %+v", *stats)
	}
}

func TestIncreasePriorityBucket(t *testing.T) {
	pq, stats := newIntStatsHeap(2)
	for _, v := range []int{1, 2, 3, 4} {
		pq.Insert(v)
	}
	stats.Reset()

	pq.IncreasePriorityByIndex(3) // leaf node — moveUp compares with parent at least once

	if stats.IncreasePriority == 0 {
		t.Fatal("expected IncreasePriority bucket > 0 after IncreasePriorityByIndex(3)")
	}
	if stats.Insert != 0 || stats.Pop != 0 ||
		stats.DecreasePriority != 0 || stats.UpdatePriority != 0 {
		t.Fatalf("expected only IncreasePriority bucket to be non-zero, got %+v", *stats)
	}
}

func TestUpdatePriorityBucket(t *testing.T) {
	pq, stats := newIntStatsHeap(2)
	for _, v := range []int{1, 2, 3, 4} {
		pq.Insert(v)
	}
	stats.Reset()

	if err := pq.UpdatePriority(1); err != nil {
		t.Fatalf("UpdatePriority returned error: %v", err)
	}

	if stats.UpdatePriority == 0 {
		t.Fatal("expected UpdatePriority bucket > 0 after UpdatePriority(1)")
	}
	if stats.Insert != 0 || stats.Pop != 0 ||
		stats.DecreasePriority != 0 || stats.IncreasePriority != 0 {
		t.Fatalf("expected only UpdatePriority bucket to be non-zero, got %+v", *stats)
	}
}

func TestReset(t *testing.T) {
	pq, stats := newIntStatsHeap(2)
	for _, v := range []int{5, 3, 8, 1, 9} {
		pq.Insert(v)
	}
	pq.Pop()

	if stats.Total() == 0 {
		t.Fatal("expected non-zero counters before Reset()")
	}

	front, _ := pq.Front()
	sizeBefore := pq.Len()

	stats.Reset()

	if stats.Total() != 0 {
		t.Fatalf("expected Total() == 0 after Reset(), got %d", stats.Total())
	}
	if stats.Insert != 0 || stats.Pop != 0 || stats.DecreasePriority != 0 ||
		stats.IncreasePriority != 0 || stats.UpdatePriority != 0 {
		t.Fatalf("expected all buckets zero after Reset(), got %+v", *stats)
	}

	// Heap state is independent of Stats — Reset() must not perturb it.
	frontAfter, _ := pq.Front()
	if frontAfter != front {
		t.Fatalf("Reset() perturbed heap state: front before=%d after=%d", front, frontAfter)
	}
	if pq.Len() != sizeBefore {
		t.Fatalf("Reset() perturbed heap length: before=%d after=%d", sizeBefore, pq.Len())
	}
}

func TestTotalEqualsSumOfBuckets(t *testing.T) {
	pq, stats := newIntStatsHeap(2)
	for _, v := range []int{5, 3, 8, 1, 9} {
		pq.Insert(v)
	}
	pq.Pop()
	pq.Pop()
	pq.DecreasePriorityByIndex(0)
	if err := pq.UpdatePriority(pq.container[0]); err != nil {
		t.Fatalf("UpdatePriority returned error: %v", err)
	}

	manualSum := stats.Insert + stats.Pop + stats.DecreasePriority +
		stats.IncreasePriority + stats.UpdatePriority
	if stats.Total() != manualSum {
		t.Fatalf("Total()=%d != manual sum=%d (Insert=%d Pop=%d Dec=%d Inc=%d Upd=%d)",
			stats.Total(), manualSum,
			stats.Insert, stats.Pop, stats.DecreasePriority, stats.IncreasePriority, stats.UpdatePriority)
	}
}

func TestNilStatsHeapBehaviorUnchanged(t *testing.T) {
	// Two heaps, identical inputs; one with stats, one without. The popped-item
	// order must match exactly — instrumentation is observation-only.
	defaultPQ := New(Options[int, int]{
		D:            4,
		Comparator:   MinNumber,
		KeyExtractor: func(x int) int { return x },
	})
	statsPQ, _ := newIntStatsHeap(4)

	input := []int{42, 17, 99, 3, 8, 25, 61, 5, 88, 1}
	for _, v := range input {
		defaultPQ.Insert(v)
		statsPQ.Insert(v)
	}

	for !defaultPQ.IsEmpty() {
		a, _ := defaultPQ.Pop()
		b, _ := statsPQ.Pop()
		if a != b {
			t.Fatalf("nil-stats heap and stats heap diverged in pop order: %d vs %d", a, b)
		}
	}
	if !statsPQ.IsEmpty() {
		t.Fatal("stats heap did not empty in lockstep with the default heap")
	}
}
