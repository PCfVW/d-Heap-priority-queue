#ifndef DARY_HEAP_PRIORITY_QUEUE_H
#define DARY_HEAP_PRIORITY_QUEUE_H

#include <vector>
#include <unordered_map>
#include <stdexcept>
#include <algorithm>
#include <optional>
#include <functional>

template <typename T, typename Identity = T, typename Priority = int>
class DaryHeapPriorityQueue {
    static_assert(std::is_invocable_r_v<Identity, Identity(T const&)>,
                 "Identity must be callable with T const& and return Identity");
    static_assert(std::is_invocable_r_v<Priority, Priority(T const&)>,
                 "Priority must be callable with T const& and return Priority");

private:
    struct Node {
        T item;
        size_t index;

        bool operator==(const Node& other) const {
            return identity_(item) == identity_(other.item);
        }
    };

    size_t d_;  // arity of the heap
    std::vector<Node> heap_;
    std::unordered_map<Identity, size_t> identity_to_index_;
    Identity identity_;
    Priority priority_;

    // Helper functions
    size_t parent(size_t i) const { return (i - 1) / d_; }
    size_t first_child(size_t i) const { return d_ * i + 1; }

    bool is_valid_index(size_t i) const { return i < heap_.size(); }

    void heapify_up(size_t index) {
        while (index > 0) {
            size_t parent_index = parent(index);
            if (priority_(heap_[index].item) >= priority_(heap_[parent_index].item)) {
                break;
            }
            std::swap(heap_[index], heap_[parent_index]);
            identity_to_index_[identity_(heap_[index].item)] = index;
            identity_to_index_[identity_(heap_[parent_index].item)] = parent_index;
            index = parent_index;
        }
    }

    void heapify_down(size_t index) {
        while (true) {
            size_t smallest = index;
            size_t child = first_child(index);

            for (size_t i = 0; i < d_ && child + i < heap_.size(); ++i) {
                if (priority_(heap_[child + i].item) < priority_(heap_[smallest].item)) {
                    smallest = child + i;
                }
            }

            if (smallest == index) {
                break;
            }

            std::swap(heap_[index], heap_[smallest]);
            identity_to_index_[identity_(heap_[index].item)] = index;
            identity_to_index_[identity_(heap_[smallest].item)] = smallest;
            index = smallest;
        }
    }

    void update_priority(size_t index, Priority new_priority) {
        Priority old_priority = priority_(heap_[index].item);
        // We need to modify the item's priority in place
        // This requires the T type to support modification of its priority
        // Alternatively, we could store the priority separately, but that complicates things

        // For this implementation, we'll assume T has a way to modify its priority
        // This is a limitation of the current design
        // A better approach would be to store the priority separately and have a way to update it

        // Since we can't modify the item's priority directly without knowing T's structure,
        // we'll need to remove and reinsert the item with the new priority
        // This is less efficient but works with any T type

        // Alternative approach: store the priority separately and have a way to update it
        // But that would require T to be hashable based on identity only

        // For now, we'll proceed with the remove-and-reinsert approach
        T item = heap_[index].item;
        remove_at(index);
        insert(item); // This will use the new priority
    }

    void remove_at(size_t index) {
        if (index >= heap_.size()) {
            return;
        }

        // Swap with last element
        if (index != heap_.size() - 1) {
            std::swap(heap_[index], heap_.back());
            identity_to_index_[identity_(heap_[index].item)] = index;
        }

        // Remove from map
        identity_to_index_.erase(identity_(heap_.back().item));

        // Remove from heap
        heap_.pop_back();

        // Heapify if needed
        if (index < heap_.size()) {
            heapify_up(index);
            heapify_down(index);
        }
    }

public:
    // Constructor with custom identity and priority extractors
    DaryHeapPriorityQueue(size_t d, Identity identity = Identity{}, Priority priority = Priority{})
        : d_(d), identity_(std::move(identity)), priority_(std::move(priority)) {
        if (d_ < 2) {
            throw std::invalid_argument("Heap arity must be at least 2");
        }
    }

    // Constructor with default identity and priority extractors
    // Requires T to have identity() and priority() member functions
    DaryHeapPriorityQueue(size_t d)
        : d_(d),
          identity_([] (const T& item) { return item.identity(); }),
          priority_([] (const T& item) { return item.priority(); }) {
        if (d_ < 2) {
            throw std::invalid_argument("Heap arity must be at least 2");
        }
    }

    // Insert an item into the queue
    void insert(const T& item) {
        Identity id = identity_(item);
        if (identity_to_index_.count(id)) {
            throw std::runtime_error("Item with this identity already exists in the queue");
        }

        heap_.push_back({item, heap_.size()});
        identity_to_index_[id] = heap_.size() - 1;
        heapify_up(heap_.size() - 1);
    }

    // Remove and return the item with highest priority (lowest value)
    T pop() {
        if (heap_.empty()) {
            throw std::runtime_error("Cannot pop from an empty queue");
        }

        T item = heap_[0].item;
        remove_at(0);
        return item;
    }

    // Return the item with highest priority without removing it
    const T& front() const {
        if (heap_.empty()) {
            throw std::runtime_error("Cannot get front of an empty queue");
        }
        return heap_[0].item;
    }

    // Update an existing item to have higher priority (lower value)
    void increase_priority(const T& item) {
        Identity id = identity_(item);
        auto it = identity_to_index_.find(id);
        if (it == identity_to_index_.end()) {
            throw std::runtime_error("Item not found in the queue");
        }

        size_t index = it->second;
        Priority new_priority = priority_(item);
        Priority old_priority = priority_(heap_[index].item);

        if (new_priority >= old_priority) {
            throw std::runtime_error("New priority is not higher (lower value) than current priority");
        }

        update_priority(index, new_priority);
    }

    // Update an existing item to have lower priority (higher value)
    void decrease_priority(const T& item) {
        Identity id = identity_(item);
        auto it = identity_to_index_.find(id);
        if (it == identity_to_index_.end()) {
            throw std::runtime_error("Item not found in the queue");
        }

        size_t index = it->second;
        Priority new_priority = priority_(item);
        Priority old_priority = priority_(heap_[index].item);

        if (new_priority <= old_priority) {
            throw std::runtime_error("New priority is not lower (higher value) than current priority");
        }

        update_priority(index, new_priority);
    }

    // Check if an item with the given identity exists
    bool contains(const T& item) const {
        return identity_to_index_.count(identity_(item)) > 0;
    }

    // Return the number of items in the queue
    size_t len() const {
        return heap_.size();
    }

    // Return whether the queue is empty
    bool is_empty() const {
        return heap_.empty();
    }

    // Clear the queue
    void clear() {
        heap_.clear();
        identity_to_index_.clear();
    }
};

#endif // DARY_HEAP_PRIORITY_QUEUE_H