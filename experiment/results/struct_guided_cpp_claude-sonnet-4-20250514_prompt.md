Implement a d-ary heap priority queue in C++17 based on the following type definitions and signatures.

Requirements:
1. Items have two distinct properties: an identity (for equality) and a priority (for ordering)
2. Two items are equal if they have the same identity, regardless of priority
3. The queue should support O(1) lookup to check if an item exists
4. Implement a min-heap where lower priority values have higher importance

#include <vector>
#include <unordered_map>
#include <optional>
#include <functional>

/// Item represents an element in the priority queue.
/// The `id` field determines identity (equality via Hash and Equal).
/// The `priority` field determines ordering in the heap.
template<typename K>
struct Item {
    K id;
    int priority;
};

/// PriorityQueue is a d-ary min-heap with O(1) item lookup.
/// Hash: functor to hash the item's id
/// Equal: functor to compare items by id for equality
template<typename K, typename Hash = std::hash<K>, typename Equal = std::equal_to<K>>
class PriorityQueue {
private:
    size_t d_;                                              // arity
    std::vector<Item<K>> container_;                        // heap array
    std::unordered_map<K, size_t, Hash, Equal> positions_;  // id -> index

public:
    /// Creates a new priority queue with the given arity d.
    /// Throws if d < 2.
    explicit PriorityQueue(size_t d);

    /// Adds an item to the queue.
    /// Throws if an item with the same id already exists.
    void insert(const Item<K>& item);

    /// Removes and returns the item with highest priority (lowest value).
    /// Returns std::nullopt if empty.
    std::optional<Item<K>> pop();

    /// Returns the item with highest priority without removing it.
    /// Throws if empty.
    const Item<K>& front() const;

    /// Updates an item to have higher priority (lower value).
    /// The item parameter contains the id to find and the new priority.
    /// Throws if the item doesn't exist.
    void increase_priority(const Item<K>& item);

    /// Updates an item to have lower priority (higher value).
    /// The item parameter contains the id to find and the new priority.
    /// Throws if the item doesn't exist.
    void decrease_priority(const Item<K>& item);

    /// Returns true if an item with the given id exists.
    bool contains(const Item<K>& item) const;

    /// Returns the number of items in the queue.
    size_t len() const;

    /// Returns true if the queue contains no items.
    bool is_empty() const;
};

Provide the complete implementation for all methods as a header-only template.