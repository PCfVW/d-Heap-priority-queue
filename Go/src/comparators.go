package dheap

import "cmp"

// ===========================================================================
// Comparator Factory Functions
// ===========================================================================

// MinBy creates a min-heap comparator using a key extractor.
// Lower key values have higher priority (appear closer to root).
//
// Example:
//
//	type Task struct {
//		ID       string
//		Priority int
//	}
//	comparator := dheap.MinBy(func(t Task) int { return t.Priority })
//
// Cross-language equivalents:
//   - Rust: MinBy<F>
//   - TypeScript: minBy<T, K>(keyFn)
func MinBy[T any, K cmp.Ordered](keyFn func(T) K) Comparator[T] {
	return func(a, b T) bool { return keyFn(a) < keyFn(b) }
}

// MaxBy creates a max-heap comparator using a key extractor.
// Higher key values have higher priority (appear closer to root).
//
// Example:
//
//	type Task struct {
//		ID       string
//		Priority int
//	}
//	comparator := dheap.MaxBy(func(t Task) int { return t.Priority })
//
// Cross-language equivalents:
//   - Rust: MaxBy<F>
//   - TypeScript: maxBy<T, K>(keyFn)
func MaxBy[T any, K cmp.Ordered](keyFn func(T) K) Comparator[T] {
	return func(a, b T) bool { return keyFn(a) > keyFn(b) }
}

// ===========================================================================
// Pre-built Comparators for Primitive Types
// ===========================================================================

// MinNumber is a min-heap comparator for int values.
// Lower numbers have higher priority.
//
// Cross-language equivalents:
//   - TypeScript: minNumber
var MinNumber Comparator[int] = func(a, b int) bool { return a < b }

// MaxNumber is a max-heap comparator for int values.
// Higher numbers have higher priority.
//
// Cross-language equivalents:
//   - TypeScript: maxNumber
var MaxNumber Comparator[int] = func(a, b int) bool { return a > b }

// MinFloat is a min-heap comparator for float64 values.
// Lower numbers have higher priority.
//
// Cross-language equivalents:
//   - TypeScript: minNumber (for floats)
var MinFloat Comparator[float64] = func(a, b float64) bool { return a < b }

// MaxFloat is a max-heap comparator for float64 values.
// Higher numbers have higher priority.
//
// Cross-language equivalents:
//   - TypeScript: maxNumber (for floats)
var MaxFloat Comparator[float64] = func(a, b float64) bool { return a > b }

// MinString is a min-heap comparator for string values.
// Lexicographically smaller strings have higher priority.
//
// Cross-language equivalents:
//   - TypeScript: minString
var MinString Comparator[string] = func(a, b string) bool { return a < b }

// MaxString is a max-heap comparator for string values.
// Lexicographically larger strings have higher priority.
//
// Cross-language equivalents:
//   - TypeScript: maxString
var MaxString Comparator[string] = func(a, b string) bool { return a > b }

// MinInt8 is a min-heap comparator for int8 values.
var MinInt8 Comparator[int8] = func(a, b int8) bool { return a < b }

// MaxInt8 is a max-heap comparator for int8 values.
var MaxInt8 Comparator[int8] = func(a, b int8) bool { return a > b }

// MinInt16 is a min-heap comparator for int16 values.
var MinInt16 Comparator[int16] = func(a, b int16) bool { return a < b }

// MaxInt16 is a max-heap comparator for int16 values.
var MaxInt16 Comparator[int16] = func(a, b int16) bool { return a > b }

// MinInt32 is a min-heap comparator for int32 values.
var MinInt32 Comparator[int32] = func(a, b int32) bool { return a < b }

// MaxInt32 is a max-heap comparator for int32 values.
var MaxInt32 Comparator[int32] = func(a, b int32) bool { return a > b }

// MinInt64 is a min-heap comparator for int64 values.
var MinInt64 Comparator[int64] = func(a, b int64) bool { return a < b }

// MaxInt64 is a max-heap comparator for int64 values.
var MaxInt64 Comparator[int64] = func(a, b int64) bool { return a > b }

// MinUint is a min-heap comparator for uint values.
var MinUint Comparator[uint] = func(a, b uint) bool { return a < b }

// MaxUint is a max-heap comparator for uint values.
var MaxUint Comparator[uint] = func(a, b uint) bool { return a > b }

// MinFloat32 is a min-heap comparator for float32 values.
var MinFloat32 Comparator[float32] = func(a, b float32) bool { return a < b }

// MaxFloat32 is a max-heap comparator for float32 values.
var MaxFloat32 Comparator[float32] = func(a, b float32) bool { return a > b }

// ===========================================================================
// Comparator Combinators
// ===========================================================================

// Reverse creates a comparator that reverses another comparator.
// Useful for converting min-heap to max-heap or vice versa.
//
// Example:
//
//	maxByCost := dheap.Reverse(dheap.MinBy(func(t Task) int { return t.Cost }))
//
// Cross-language equivalents:
//   - TypeScript: reverse(cmp)
func Reverse[T any](cmp Comparator[T]) Comparator[T] {
	return func(a, b T) bool { return cmp(b, a) }
}

// Chain creates a comparator that compares by multiple keys in order.
// Falls back to subsequent comparators when items are equal.
//
// Example:
//
//	// Sort by priority first, then by timestamp
//	cmp := dheap.Chain(
//		dheap.MinBy(func(t Task) int { return t.Priority }),
//		dheap.MinBy(func(t Task) int64 { return t.Timestamp }),
//	)
//
// Cross-language equivalents:
//   - TypeScript: chain(...comparators)
func Chain[T any](comparators ...Comparator[T]) Comparator[T] {
	return func(a, b T) bool {
		for _, cmp := range comparators {
			if cmp(a, b) {
				return true
			}
			if cmp(b, a) {
				return false
			}
		}
		return false
	}
}
