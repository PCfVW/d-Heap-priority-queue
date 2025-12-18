//! Item type definition for priority queue elements.
//!
//! This module defines the Item type used in the d-heap priority queue implementation.
//! Items have both an identity (number) and a priority (cost).

const std = @import("std");

/// Item type representing an element in the priority queue.
///
/// Each item has two fields:
/// - `number`: Unique identifier used for hashing and equality (identity)
/// - `cost`: Priority value used by comparators (priority)
///
/// The identity (number) must remain stable throughout the item's lifetime,
/// while the priority (cost) may be updated via increasePriority/decreasePriority.
///
/// Example:
/// ```zig
/// const item = Item{ .number = 42, .cost = 100 };
/// ```
pub const Item = struct {
    /// Unique identifier for this item (used for identity/hashing)
    number: u32,

    /// Priority value (interpretation depends on comparator: min-heap vs max-heap)
    cost: u32,

    /// Create a new Item with the given number and cost.
    ///
    /// Parameters:
    /// - `number`: Unique identifier for the item
    /// - `cost`: Initial priority value
    ///
    /// Returns: A new Item instance
    pub fn init(number: u32, cost: u32) Item {
        return Item{ .number = number, .cost = cost };
    }

    /// Format the item for printing.
    ///
    /// Implements the std.fmt interface to enable printing with `{f}`.
    /// Output format: `(number, cost)`
    ///
    /// Example output: `(42, 100)`
    pub fn format(self: Item, writer: anytype) !void {
        try writer.print("({}, {})", .{ self.number, self.cost });
    }

    /// Compute hash value for this item based on its identity (number).
    ///
    /// Uses Wyhash algorithm for fast, high-quality hashing.
    /// Only the `number` field is hashed, as it represents the item's identity.
    ///
    /// Returns: 64-bit hash value
    pub fn hash(self: Item) u64 {
        var hasher = std.hash.Wyhash.init(0);
        std.hash.autoHash(&hasher, self.number);
        return hasher.final();
    }

    /// Check equality between two items based on identity (number).
    ///
    /// Two items are considered equal if they have the same `number`,
    /// regardless of their `cost` values.
    ///
    /// Parameters:
    /// - `a`: First item to compare
    /// - `b`: Second item to compare
    ///
    /// Returns: true if items have the same identity, false otherwise
    pub fn eq(a: Item, b: Item) bool {
        return a.number == b.number;
    }
};
