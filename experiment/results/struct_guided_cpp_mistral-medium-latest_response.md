Here's the complete implementation of the d-ary heap priority queue in C++17:

```cpp
#include <vector>
#include <unordered_map>
#include <optional>
#include <functional>
#include <stdexcept>
#include <algorithm>

/// Item represents an element in the priority queue.
/// The `id` field determines identity (equality via Hash and Equal).
/// The `priority` field determines ordering in the heap.
template<typename K>
struct Item {
    K id;
    int priority;

    bool operator==(const Item<K>& other) const {
        return id == other.id;
    }
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

    // Helper functions
    size_t parent(size_t i) const {
        return (i - 1) / d_;
    }

    size_t first_child(size_t i) const {
        return d_ * i + 1;
    }

    void heapify_up(size_t i) {
        while (i > 0) {
            size_t p = parent(i);
            if (container_[i].priority >= container_[p].priority) {
                break;
            }
            std::swap(container_[i], container_[p]);
            positions_[container_[i].id] = i;
            positions_[container_[p].id] = p;
            i = p;
        }
    }

    void heapify_down(size_t i) {
        while (true) {
            size_t smallest = i;
            size_t child = first_child(i);
            size_t end = first_child(i) + d_;

            for (; child < end && child < container_.size(); ++child) {
                if (container_[child].priority < container_[smallest].priority) {
                    smallest = child;
                }
            }

            if (smallest == i) {
                break;
            }

            std::swap(container_[i], container_[smallest]);
            positions_[container_[i].id] = i;
            positions_[container_[smallest].id] = smallest;
            i = smallest;
        }
    }

public:
    /// Creates a new priority queue with the given arity d.
    /// Throws if d < 2.
    explicit PriorityQueue(size_t d) : d_(d) {
        if (d < 2) {
            throw std::invalid_argument("Ary must be at least 2");
        }
    }

    /// Adds an item to the queue.
    /// Throws if an item with the same id already exists.
    void insert(const Item<K>& item) {
        if (positions_.count(item.id) > 0) {
            throw std::invalid_argument("Item with this id already exists");
        }

        container_.push_back(item);
        size_t index = container_.size() - 1;
        positions_[item.id] = index;
        heapify_up(index);
    }

    /// Removes and returns the item with highest priority (lowest value).
    /// Returns std::nullopt if empty.
    std::optional<Item<K>> pop() {
        if (container_.empty()) {
            return std::nullopt;
        }

        Item<K> result = container_[0];
        positions_.erase(result.id);

        if (container_.size() > 1) {
            container_[0] = container_.back();
            positions_[container_[0].id] = 0;
        }
        container_.pop_back();

        if (!container_.empty()) {
            heapify_down(0);
        }

        return result;
    }

    /// Returns the item with highest priority without removing it.
    /// Throws if empty.
    const Item<K>& front() const {
        if (container_.empty()) {
            throw std::out_of_range("Queue is empty");
        }
        return container_[0];
    }

    /// Updates an item to have higher priority (lower value).
    /// The item parameter contains the id to find and the new priority.
    /// Throws if the item doesn't exist.
    void increase_priority(const Item<K>& item) {
        auto it = positions_.find(item.id);
        if (it == positions_.end()) {
            throw std::invalid_argument("Item not found");
        }

        size_t index = it->second;
        if (item.priority > container_[index].priority) {
            throw std::invalid_argument("New priority is not higher (lower value)");
        }

        container_[index].priority = item.priority;
        heapify_up(index);
    }

    /// Updates an item to have lower priority (higher value).
    /// The item parameter contains the id to find and the new priority.
    /// Throws if the item doesn't exist.
    void decrease_priority(const Item<K>& item) {
        auto it = positions_.find(item.id);
        if (it == positions_.end()) {
            throw std::invalid_argument("Item not found");
        }

        size_t index = it->second;
        if (item.priority < container_[index].priority) {
            throw std::invalid_argument("New priority is not lower (higher value)");
        }

        container_[index].priority = item.priority;
        heapify_down(index);
    }

    /// Returns true if an item with the given id exists.
    bool contains(const Item<K>& item) const {
        return positions_.count(item.id) > 0;
    }

    /// Returns the number of items in the queue.
    size_t len() const {
        return container_.size();
    }

    /// Returns true if the queue contains no items.
    bool is_empty() const {
        return container_.empty();
    }
};
```

This implementation provides:

1. A d-ary min-heap structure where each node has up to d children
2. O(1) lookup for item existence using an unordered_map
3. Proper heap operations:
   - insert: O(log_d n) time
   - pop: O(d log_d n) time (due to finding the smallest child)
   - increase_priority: O(log_d n) time
   - decrease_priority: O(d log_d n) time
4. All required methods with proper error handling
5. Template support for different key types with custom hash and equality functions

The implementation maintains the heap property after each operation and keeps the positions map updated to ensure O(1) lookups. The heapify_up and heapify_down methods handle the d-ary nature of the heap by considering all d children when moving elements down the heap.