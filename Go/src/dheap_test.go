package dheap

import (
	"fmt"
	"math/rand"
	"testing"
)

// Test item type for complex tests
type Item struct {
	ID   string
	Cost int
}

func (i Item) String() string {
	return fmt.Sprintf("Item(%s, %d)", i.ID, i.Cost)
}

// Helper to create a min-heap of ints
func newIntMinHeap(d int) *PriorityQueue[int, int] {
	return New(Options[int, int]{
		D:            d,
		Comparator:   MinNumber,
		KeyExtractor: func(x int) int { return x },
	})
}

// Helper to create a max-heap of ints
func newIntMaxHeap(d int) *PriorityQueue[int, int] {
	return New(Options[int, int]{
		D:            d,
		Comparator:   MaxNumber,
		KeyExtractor: func(x int) int { return x },
	})
}

// Helper to create a min-heap of Items by cost
func newItemMinHeap(d int) *PriorityQueue[Item, string] {
	return New(Options[Item, string]{
		D:            d,
		Comparator:   MinBy(func(i Item) int { return i.Cost }),
		KeyExtractor: func(i Item) string { return i.ID },
	})
}

// ===========================================================================
// Basic Operations
// ===========================================================================

func TestNew(t *testing.T) {
	pq := newIntMinHeap(4)
	if pq == nil {
		t.Fatal("New returned nil")
	}
	if pq.Len() != 0 {
		t.Errorf("New heap should have len 0, got %d", pq.Len())
	}
	if pq.D() != 4 {
		t.Errorf("Expected d=4, got %d", pq.D())
	}
}

func TestNewDefaultArity(t *testing.T) {
	pq := New(Options[int, int]{
		D:            0, // Should default to 2
		Comparator:   MinNumber,
		KeyExtractor: func(x int) int { return x },
	})
	if pq.D() != 2 {
		t.Errorf("Expected default d=2, got %d", pq.D())
	}
}

func TestNewPanicsOnInvalidArity(t *testing.T) {
	defer func() {
		if r := recover(); r == nil {
			t.Error("Expected panic for d=-1")
		}
	}()
	New(Options[int, int]{
		D:            -1,
		Comparator:   MinNumber,
		KeyExtractor: func(x int) int { return x },
	})
}

func TestWithFirst(t *testing.T) {
	pq := WithFirst(Options[int, int]{
		D:            4,
		Comparator:   MinNumber,
		KeyExtractor: func(x int) int { return x },
	}, 42)

	if pq.Len() != 1 {
		t.Errorf("Expected len=1, got %d", pq.Len())
	}
	front, err := pq.Front()
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}
	if front != 42 {
		t.Errorf("Expected front=42, got %d", front)
	}
}

func TestLen(t *testing.T) {
	pq := newIntMinHeap(2)
	if pq.Len() != 0 {
		t.Errorf("Expected len=0, got %d", pq.Len())
	}
	pq.Insert(5)
	if pq.Len() != 1 {
		t.Errorf("Expected len=1, got %d", pq.Len())
	}
	pq.Insert(3)
	if pq.Len() != 2 {
		t.Errorf("Expected len=2, got %d", pq.Len())
	}
}

func TestIsEmpty(t *testing.T) {
	pq := newIntMinHeap(2)
	if !pq.IsEmpty() {
		t.Error("Expected IsEmpty() = true")
	}
	pq.Insert(5)
	if pq.IsEmpty() {
		t.Error("Expected IsEmpty() = false")
	}
	pq.Pop()
	if !pq.IsEmpty() {
		t.Error("Expected IsEmpty() = true after pop")
	}
}

func TestIsEmptyAlias(t *testing.T) {
	pq := newIntMinHeap(2)
	if pq.IsEmpty() != pq.Is_empty() {
		t.Error("Is_empty alias should match IsEmpty")
	}
}

func TestD(t *testing.T) {
	for _, d := range []int{1, 2, 3, 4, 8, 16} {
		pq := newIntMinHeap(d)
		if pq.D() != d {
			t.Errorf("Expected D()=%d, got %d", d, pq.D())
		}
	}
}

// ===========================================================================
// Insert and Pop
// ===========================================================================

func TestInsert(t *testing.T) {
	pq := newIntMinHeap(4)
	pq.Insert(5)
	pq.Insert(3)
	pq.Insert(7)

	if pq.Len() != 3 {
		t.Errorf("Expected len=3, got %d", pq.Len())
	}

	front, _ := pq.Front()
	if front != 3 {
		t.Errorf("Expected front=3, got %d", front)
	}
}

func TestInsertMany(t *testing.T) {
	pq := newIntMinHeap(4)
	pq.InsertMany([]int{5, 3, 7, 1, 9, 2})

	if pq.Len() != 6 {
		t.Errorf("Expected len=6, got %d", pq.Len())
	}

	front, _ := pq.Front()
	if front != 1 {
		t.Errorf("Expected front=1, got %d", front)
	}
}

func TestInsertManyEmpty(t *testing.T) {
	pq := newIntMinHeap(4)
	pq.InsertMany([]int{})
	if pq.Len() != 0 {
		t.Errorf("Expected len=0, got %d", pq.Len())
	}
}

func TestPop(t *testing.T) {
	pq := newIntMinHeap(4)
	pq.Insert(5)
	pq.Insert(3)
	pq.Insert(7)

	item, ok := pq.Pop()
	if !ok {
		t.Error("Pop should return ok=true")
	}
	if item != 3 {
		t.Errorf("Expected 3, got %d", item)
	}

	item, ok = pq.Pop()
	if !ok || item != 5 {
		t.Errorf("Expected 5, got %d (ok=%v)", item, ok)
	}

	item, ok = pq.Pop()
	if !ok || item != 7 {
		t.Errorf("Expected 7, got %d (ok=%v)", item, ok)
	}
}

func TestPopMany(t *testing.T) {
	pq := newIntMinHeap(4)
	pq.InsertMany([]int{5, 3, 7, 1, 9, 2})

	items := pq.PopMany(3)
	if len(items) != 3 {
		t.Errorf("Expected 3 items, got %d", len(items))
	}
	expected := []int{1, 2, 3}
	for i, v := range expected {
		if items[i] != v {
			t.Errorf("Expected items[%d]=%d, got %d", i, v, items[i])
		}
	}
	if pq.Len() != 3 {
		t.Errorf("Expected len=3 remaining, got %d", pq.Len())
	}
}

func TestPopEmpty(t *testing.T) {
	pq := newIntMinHeap(4)
	_, ok := pq.Pop()
	if ok {
		t.Error("Pop on empty heap should return ok=false")
	}
}

// ===========================================================================
// Front/Peek
// ===========================================================================

func TestFront(t *testing.T) {
	pq := newIntMinHeap(4)
	pq.Insert(5)
	pq.Insert(3)

	front, err := pq.Front()
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}
	if front != 3 {
		t.Errorf("Expected 3, got %d", front)
	}

	// Front should not remove the item
	if pq.Len() != 2 {
		t.Errorf("Front should not modify len, got %d", pq.Len())
	}
}

func TestFrontEmpty(t *testing.T) {
	pq := newIntMinHeap(4)
	_, err := pq.Front()
	if err != ErrEmptyQueue {
		t.Errorf("Expected ErrEmptyQueue, got %v", err)
	}
}

func TestPeek(t *testing.T) {
	pq := newIntMinHeap(4)
	pq.Insert(5)
	pq.Insert(3)

	item, ok := pq.Peek()
	if !ok {
		t.Error("Peek should return ok=true")
	}
	if item != 3 {
		t.Errorf("Expected 3, got %d", item)
	}
}

func TestPeekEmpty(t *testing.T) {
	pq := newIntMinHeap(4)
	_, ok := pq.Peek()
	if ok {
		t.Error("Peek on empty heap should return ok=false")
	}
}

// ===========================================================================
// Contains
// ===========================================================================

func TestContains(t *testing.T) {
	pq := newIntMinHeap(4)
	pq.Insert(5)
	pq.Insert(3)

	if !pq.Contains(5) {
		t.Error("Expected Contains(5) = true")
	}
	if !pq.Contains(3) {
		t.Error("Expected Contains(3) = true")
	}
	if pq.Contains(10) {
		t.Error("Expected Contains(10) = false")
	}
}

func TestContainsKey(t *testing.T) {
	pq := newItemMinHeap(4)
	pq.Insert(Item{ID: "a", Cost: 5})
	pq.Insert(Item{ID: "b", Cost: 3})

	if !pq.ContainsKey("a") {
		t.Error("Expected ContainsKey(a) = true")
	}
	if !pq.ContainsKey("b") {
		t.Error("Expected ContainsKey(b) = true")
	}
	if pq.ContainsKey("c") {
		t.Error("Expected ContainsKey(c) = false")
	}
}

func TestGetPosition(t *testing.T) {
	pq := newIntMinHeap(4)
	pq.Insert(5)
	pq.Insert(3)
	pq.Insert(7)

	pos, ok := pq.GetPosition(3)
	if !ok {
		t.Error("GetPosition(3) should return ok=true")
	}
	if pos != 0 {
		t.Errorf("Expected position 0 for min element, got %d", pos)
	}

	_, ok = pq.GetPosition(100)
	if ok {
		t.Error("GetPosition(100) should return ok=false")
	}
}

// ===========================================================================
// Priority Updates
// ===========================================================================

func TestIncreasePriority(t *testing.T) {
	pq := newItemMinHeap(4)
	pq.Insert(Item{ID: "a", Cost: 10})
	pq.Insert(Item{ID: "b", Cost: 5})

	// 'b' is at front with cost 5
	front, _ := pq.Front()
	if front.ID != "b" {
		t.Errorf("Expected b at front, got %s", front.ID)
	}

	// Increase priority of 'a' by decreasing its cost (min-heap)
	err := pq.IncreasePriority(Item{ID: "a", Cost: 1})
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}

	// Now 'a' should be at front
	front, _ = pq.Front()
	if front.ID != "a" {
		t.Errorf("Expected a at front after priority increase, got %s", front.ID)
	}
	if front.Cost != 1 {
		t.Errorf("Expected cost=1, got %d", front.Cost)
	}
}

func TestDecreasePriority(t *testing.T) {
	pq := newItemMinHeap(4)
	pq.Insert(Item{ID: "a", Cost: 1})
	pq.Insert(Item{ID: "b", Cost: 5})
	pq.Insert(Item{ID: "c", Cost: 10})

	// 'a' is at front with cost 1
	front, _ := pq.Front()
	if front.ID != "a" {
		t.Errorf("Expected a at front, got %s", front.ID)
	}

	// Decrease priority of 'a' by increasing its cost (min-heap)
	err := pq.DecreasePriority(Item{ID: "a", Cost: 100})
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}

	// Now 'b' should be at front
	front, _ = pq.Front()
	if front.ID != "b" {
		t.Errorf("Expected b at front after priority decrease, got %s", front.ID)
	}
}

func TestIncreasePriorityNotFound(t *testing.T) {
	pq := newItemMinHeap(4)
	pq.Insert(Item{ID: "a", Cost: 10})

	err := pq.IncreasePriority(Item{ID: "nonexistent", Cost: 1})
	if err != ErrItemNotFound {
		t.Errorf("Expected ErrItemNotFound, got %v", err)
	}
}

func TestDecreasePriorityNotFound(t *testing.T) {
	pq := newItemMinHeap(4)
	pq.Insert(Item{ID: "a", Cost: 10})

	err := pq.DecreasePriority(Item{ID: "nonexistent", Cost: 100})
	if err != ErrItemNotFound {
		t.Errorf("Expected ErrItemNotFound, got %v", err)
	}
}

func TestIncreasePriorityAliases(t *testing.T) {
	pq := newItemMinHeap(4)
	pq.Insert(Item{ID: "a", Cost: 10})

	err := pq.Increase_priority(Item{ID: "a", Cost: 1})
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}
}

func TestDecreasePriorityAliases(t *testing.T) {
	pq := newItemMinHeap(4)
	pq.Insert(Item{ID: "a", Cost: 10})

	err := pq.Decrease_priority(Item{ID: "a", Cost: 100})
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}
}

// ===========================================================================
// Min/Max Heap
// ===========================================================================

func TestMinHeap(t *testing.T) {
	pq := newIntMinHeap(4)
	values := []int{5, 3, 7, 1, 9, 2, 8, 4, 6}
	for _, v := range values {
		pq.Insert(v)
	}

	// Pop all values, should come out in ascending order
	expected := []int{1, 2, 3, 4, 5, 6, 7, 8, 9}
	for i, exp := range expected {
		val, ok := pq.Pop()
		if !ok {
			t.Fatalf("Pop %d failed", i)
		}
		if val != exp {
			t.Errorf("Expected %d, got %d", exp, val)
		}
	}
}

func TestMaxHeap(t *testing.T) {
	pq := newIntMaxHeap(4)
	values := []int{5, 3, 7, 1, 9, 2, 8, 4, 6}
	for _, v := range values {
		pq.Insert(v)
	}

	// Pop all values, should come out in descending order
	expected := []int{9, 8, 7, 6, 5, 4, 3, 2, 1}
	for i, exp := range expected {
		val, ok := pq.Pop()
		if !ok {
			t.Fatalf("Pop %d failed", i)
		}
		if val != exp {
			t.Errorf("Expected %d, got %d", exp, val)
		}
	}
}

// ===========================================================================
// Different Arities
// ===========================================================================

func TestArity2(t *testing.T) {
	pq := newIntMinHeap(2)
	testArityHelper(t, pq)
}

func TestArity4(t *testing.T) {
	pq := newIntMinHeap(4)
	testArityHelper(t, pq)
}

func TestArity8(t *testing.T) {
	pq := newIntMinHeap(8)
	testArityHelper(t, pq)
}

func testArityHelper(t *testing.T, pq *PriorityQueue[int, int]) {
	values := []int{5, 3, 7, 1, 9, 2, 8, 4, 6}
	for _, v := range values {
		pq.Insert(v)
	}

	expected := []int{1, 2, 3, 4, 5, 6, 7, 8, 9}
	for _, exp := range expected {
		val, ok := pq.Pop()
		if !ok || val != exp {
			t.Errorf("Expected %d, got %d (ok=%v)", exp, val, ok)
		}
	}
}

// ===========================================================================
// Clear
// ===========================================================================

func TestClear(t *testing.T) {
	pq := newIntMinHeap(4)
	pq.Insert(5)
	pq.Insert(3)

	pq.Clear()
	if !pq.IsEmpty() {
		t.Error("Expected empty after Clear()")
	}
	if pq.D() != 4 {
		t.Errorf("Expected D()=4 after Clear(), got %d", pq.D())
	}
}

func TestClearWithNewArity(t *testing.T) {
	pq := newIntMinHeap(4)
	pq.Insert(5)
	pq.Insert(3)

	pq.Clear(8)
	if !pq.IsEmpty() {
		t.Error("Expected empty after Clear(8)")
	}
	if pq.D() != 8 {
		t.Errorf("Expected D()=8 after Clear(8), got %d", pq.D())
	}
}

// ===========================================================================
// String
// ===========================================================================

func TestString(t *testing.T) {
	pq := newIntMinHeap(4)
	pq.Insert(5)
	pq.Insert(3)
	pq.Insert(7)

	s := pq.String()
	if s != "{3, 5, 7}" && s != "{3, 7, 5}" {
		// Order of children may vary, but root should be 3
		if s[0:2] != "{3" {
			t.Errorf("Expected string to start with {3, got %s", s)
		}
	}
}

func TestStringEmpty(t *testing.T) {
	pq := newIntMinHeap(4)
	s := pq.String()
	if s != "{}" {
		t.Errorf("Expected {}, got %s", s)
	}
}

func TestToStringAlias(t *testing.T) {
	pq := newIntMinHeap(4)
	pq.Insert(5)
	if pq.String() != pq.To_string() {
		t.Error("To_string alias should match String")
	}
}

func TestToArray(t *testing.T) {
	pq := newIntMinHeap(4)
	pq.Insert(5)
	pq.Insert(3)
	pq.Insert(7)

	arr := pq.ToArray()
	if len(arr) != 3 {
		t.Errorf("Expected len=3, got %d", len(arr))
	}
	// First element should be the min
	if arr[0] != 3 {
		t.Errorf("Expected arr[0]=3, got %d", arr[0])
	}
}

// ===========================================================================
// Heap Property Verification
// ===========================================================================

func TestHeapPropertyMaintained(t *testing.T) {
	pq := newIntMinHeap(4)

	// Insert random values
	for i := 0; i < 100; i++ {
		pq.Insert(rand.Intn(1000))
	}

	// Verify heap property
	arr := pq.ToArray()
	d := pq.D()
	for i := 0; i < len(arr); i++ {
		for j := 1; j <= d; j++ {
			childIdx := i*d + j
			if childIdx < len(arr) {
				if arr[i] > arr[childIdx] {
					t.Errorf("Heap property violated: parent[%d]=%d > child[%d]=%d",
						i, arr[i], childIdx, arr[childIdx])
				}
			}
		}
	}
}

// ===========================================================================
// Performance
// ===========================================================================

func TestLargeHeap(t *testing.T) {
	pq := newIntMinHeap(4)
	n := 10000

	// Insert n random values
	for i := 0; i < n; i++ {
		pq.Insert(rand.Intn(n * 10))
	}

	if pq.Len() != n {
		t.Errorf("Expected len=%d, got %d", n, pq.Len())
	}

	// Pop all and verify sorted order
	prev := -1
	for !pq.IsEmpty() {
		val, ok := pq.Pop()
		if !ok {
			t.Fatal("Pop failed")
		}
		if val < prev {
			t.Errorf("Order violated: %d after %d", val, prev)
		}
		prev = val
	}
}

// ===========================================================================
// Comparators
// ===========================================================================

func TestMinBy(t *testing.T) {
	type Item struct {
		Name  string
		Value int
	}

	pq := New(Options[Item, string]{
		D:            4,
		Comparator:   MinBy(func(i Item) int { return i.Value }),
		KeyExtractor: func(i Item) string { return i.Name },
	})

	pq.Insert(Item{Name: "a", Value: 5})
	pq.Insert(Item{Name: "b", Value: 3})
	pq.Insert(Item{Name: "c", Value: 7})

	front, _ := pq.Front()
	if front.Name != "b" {
		t.Errorf("Expected b (lowest value), got %s", front.Name)
	}
}

func TestMaxBy(t *testing.T) {
	type Item struct {
		Name  string
		Value int
	}

	pq := New(Options[Item, string]{
		D:            4,
		Comparator:   MaxBy(func(i Item) int { return i.Value }),
		KeyExtractor: func(i Item) string { return i.Name },
	})

	pq.Insert(Item{Name: "a", Value: 5})
	pq.Insert(Item{Name: "b", Value: 3})
	pq.Insert(Item{Name: "c", Value: 7})

	front, _ := pq.Front()
	if front.Name != "c" {
		t.Errorf("Expected c (highest value), got %s", front.Name)
	}
}

func TestReverse(t *testing.T) {
	pq := New(Options[int, int]{
		D:            4,
		Comparator:   Reverse(MinNumber), // Should behave like max-heap
		KeyExtractor: func(x int) int { return x },
	})

	pq.Insert(5)
	pq.Insert(3)
	pq.Insert(7)

	front, _ := pq.Front()
	if front != 7 {
		t.Errorf("Expected 7 (reversed min = max), got %d", front)
	}
}

func TestChain(t *testing.T) {
	type Task struct {
		ID       string
		Priority int
		Order    int
	}

	pq := New(Options[Task, string]{
		D: 4,
		Comparator: Chain(
			MinBy(func(t Task) int { return t.Priority }),
			MinBy(func(t Task) int { return t.Order }),
		),
		KeyExtractor: func(t Task) string { return t.ID },
	})

	pq.Insert(Task{ID: "a", Priority: 1, Order: 2})
	pq.Insert(Task{ID: "b", Priority: 1, Order: 1}) // Same priority, lower order
	pq.Insert(Task{ID: "c", Priority: 2, Order: 0})

	front, _ := pq.Front()
	if front.ID != "b" {
		t.Errorf("Expected b (priority 1, order 1), got %s", front.ID)
	}
}

// ===========================================================================
// Edge Cases
// ===========================================================================

func TestSingleElement(t *testing.T) {
	pq := newIntMinHeap(4)
	pq.Insert(42)

	front, _ := pq.Front()
	if front != 42 {
		t.Errorf("Expected 42, got %d", front)
	}

	val, ok := pq.Pop()
	if !ok || val != 42 {
		t.Errorf("Expected 42, got %d (ok=%v)", val, ok)
	}

	if !pq.IsEmpty() {
		t.Error("Expected empty after popping single element")
	}
}

func TestDuplicateValues(t *testing.T) {
	// Using items with same value but different keys
	pq := newItemMinHeap(4)
	pq.Insert(Item{ID: "a", Cost: 5})
	pq.Insert(Item{ID: "b", Cost: 5})
	pq.Insert(Item{ID: "c", Cost: 5})

	if pq.Len() != 3 {
		t.Errorf("Expected len=3, got %d", pq.Len())
	}

	// All have same priority, order doesn't matter
	for i := 0; i < 3; i++ {
		item, ok := pq.Pop()
		if !ok {
			t.Fatalf("Pop %d failed", i)
		}
		if item.Cost != 5 {
			t.Errorf("Expected cost=5, got %d", item.Cost)
		}
	}
}

func TestIncreasePriorityByIndex(t *testing.T) {
	pq := newIntMinHeap(4)
	pq.Insert(5)
	pq.Insert(3)
	pq.Insert(7)

	// Get position of 7
	pos, _ := pq.GetPosition(7)
	// Manually modifying container would be needed to test this properly
	// For now, just verify it doesn't panic
	pq.IncreasePriorityByIndex(pos)
}
