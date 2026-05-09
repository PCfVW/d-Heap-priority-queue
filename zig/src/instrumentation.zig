//! Phase 2 comparison-count instrumentation for the d-ary heap.
//!
//! This module provides the contract used by every instrumented language
//! (TypeScript, C++, Go, Rust, Zig) to count *comparisons* — not operations —
//! bucketed by which heap operation triggered them. The bucket is selected
//! by the currently-active `OperationType`, set by the heap via
//! `startOperation` before its first comparison and cleared by `endOperation`
//! when the operation returns.
//!
//! Zig's mechanism is a `comptime bool` parameter on `DHeapWithStats` that
//! gates every interaction with this module: when `false`, the `stats` field
//! is `void`, the bracket calls are dead code, and the generated assembly is
//! identical to the pre-instrumentation library.

const std = @import("std");

/// Identifies which heap operation is currently in flight, so subsequent
/// comparisons can be attributed to the right bucket.
///
/// `none` is the initial / cleared state — comparisons that arrive while
/// `current_op == .none` are silently ignored, mirroring the C++/Go/Rust
/// "no active operation" guard. This makes the contract robust to internal
/// helper calls that should not be double-counted.
pub const OperationType = enum {
    none,
    insert,
    pop,
    decrease_priority,
    increase_priority,
    update_priority,
};

/// Comparison-count buckets, one per heap operation, plus the active-operation
/// marker. All counters are plain `u64`; mutator methods are `*@This()` so the
/// heap can hold the struct by value (no pointer indirection per comparison).
///
/// Cross-language equivalents:
///   - TypeScript: `ComparisonStats` from `instrumentation.ts`
///   - C++: `ComparisonStats` from `instrumentation.hpp`
///   - Go: `Stats` from `stats.go`
///   - Rust: `ComparisonStats` from `src/instrumentation.rs`
pub const ComparisonStats = struct {
    insert_count: u64 = 0,
    pop_count: u64 = 0,
    decrease_priority_count: u64 = 0,
    increase_priority_count: u64 = 0,
    update_priority_count: u64 = 0,
    current_op: OperationType = .none,

    /// Mark the start of a heap operation. Subsequent `countComparison` calls
    /// will accumulate into this operation's bucket until `endOperation`.
    pub fn startOperation(self: *@This(), op: OperationType) void {
        self.current_op = op;
    }

    /// Mark the end of a heap operation. Subsequent `countComparison` calls
    /// (e.g. from internal helpers re-entered out of band) are ignored until
    /// the next `startOperation`.
    pub fn endOperation(self: *@This()) void {
        self.current_op = .none;
    }

    /// Increment the bucket for the currently-active operation. No-op when
    /// no operation is active.
    pub fn countComparison(self: *@This()) void {
        switch (self.current_op) {
            .insert => self.insert_count += 1,
            .pop => self.pop_count += 1,
            .decrease_priority => self.decrease_priority_count += 1,
            .increase_priority => self.increase_priority_count += 1,
            .update_priority => self.update_priority_count += 1,
            .none => {},
        }
    }

    /// Sum of all five buckets. Excludes `current_op` (not a counter).
    pub fn total(self: @This()) u64 {
        return self.insert_count + self.pop_count + self.decrease_priority_count + self.increase_priority_count + self.update_priority_count;
    }

    /// Zero all five buckets and clear the active-operation marker.
    /// Heap state is unaffected.
    pub fn reset(self: *@This()) void {
        self.* = .{};
    }
};
