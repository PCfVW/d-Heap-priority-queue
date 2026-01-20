#include <vector>
#include <unordered_map>
#include <optional>
#include <functional>
#include <stdexcept>

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

    /// Returns the index of the parent of node at index i
    size_t parent(size_t i) const {
        return (i - 1) / d_;
    }

    /// Returns the index of the first child of node at index i
    size_t first_child(size_t i) const {
        return d_ * i + 1;
    }

    /// Moves an item up the heap until heap property is satisfied
    void heapify_up(size_t index) {
        while (index > 0) {
            size_t parent_idx = parent(index);
            if (container_[index].priority >= container_[parent_idx].priority) {
                break;
            }
            
            // Update positions map before swapping
            positions_[container_[index].id] = parent_idx;
            positions_[container_[parent_idx].id] = index;
            
            std::swap(container_[index], container_[parent_idx]);
            index = parent_idx;
        }
    }

    /// Moves an item down the heap until heap property is satisfied
    void heapify_down(size_t index) {
        while (true) {
            size_t min_idx = index;
            size_t first_child_idx = first_child(index);
            
            // Find the child with minimum priority
            for (size_t i = 0; i < d_ && first_child_idx + i < container_.size(); ++i) {
                size_t child_idx = first_child_idx + i;
                if (container_[child_idx].priority < container_[min_idx].priority) {
                    min_idx = child_idx;
                }
            }
            
            if (min_idx == index) {
                break;
            }
            
            // Update positions map before swapping
            positions_[container_[index].id] = min_idx;
            positions_[container_[min_idx].id] = index;
            
            std::swap(container_[index], container_[min_idx]);
            index = min_idx;
        }
    }

public:
    /// Creates a new priority queue with the given arity d.
    /// Throws if d < 2.
    explicit PriorityQueue(size_t d) : d_(d) {
        if (d < 2) {
            throw std::invalid_argument("Arity must be at least 2");
        }
    }

    /// Adds an item to the queue.
    /// Throws if an item with the same id already exists.
    void insert(const Item<K>& item) {
        if (positions_.find(item.id) != positions_.end()) {
            throw std::invalid_argument("Item with this id already exists");
        }
        
        size_t new_index = container_.size();
        container_.push_back(item);
        positions_[item.id] = new_index;
        
        heapify_up(new_index);
    }

    /// Removes and returns the item with highest priority (lowest value).
    /// Returns std::nullopt if empty.
    std::optional<Item<K>> pop() {
        if (container_.empty()) {
            return std::nullopt;
        }
        
        Item<K> result = container_[0];
        positions_.erase(result.id);
        
        if (container_.size() == 1) {
            container_.clear();
            return result;
        }
        
        // Move last element to root and update positions
        container_[0] = container_.back();
        positions_[container_[0].id] = 0;
        container_.pop_back();
        
        heapify_down(0);
        
        return result;
    }

    /// Returns the item with highest priority without removing it.
    /// Throws if empty.
    const Item<K>& front() const {
        if (container_.empty()) {
            throw std::runtime_error("Queue is empty");
        }
        return container_[0];
    }

    /// Updates an item to have higher priority (lower value).
    /// The item parameter contains the id to find and the new priority.
    /// Throws if the item doesn't exist.
    void increase_priority(const Item<K>& item) {
        auto it = positions_.find(item.id);
        if (it == positions_.end()) {
            throw std::invalid_argument("Item does not exist");
        }
        
        size_t index = it->second;
        container_[index].priority = item.priority;
        heapify_up(index);
    }

    /// Updates an item to have lower priority (higher value).
    /// The item parameter contains the id to find and the new priority.
    /// Throws if the item doesn't exist.
    void decrease_priority(const Item<K>& item) {
        auto it = positions_.find(item.id);
        if (it == positions_.end()) {
            throw std::invalid_argument("Item does not exist");
        }
        
        size_t index = it->second;
        container_[index].priority = item.priority;
        heapify_down(index);
    }

    /// Returns true if an item with the given id exists.
    bool contains(const Item<K>& item) const {
        return positions_.find(item.id) != positions_.end();
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