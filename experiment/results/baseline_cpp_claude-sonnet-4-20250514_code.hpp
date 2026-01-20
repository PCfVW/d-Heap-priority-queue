#pragma once

#include <vector>
#include <unordered_map>
#include <stdexcept>
#include <functional>

template<typename T, typename Hash = std::hash<T>, typename Equal = std::equal_to<T>>
class DaryHeapPriorityQueue {
public:
    struct Item {
        T identity;
        double priority;
        
        Item() = default;
        Item(const T& id, double prio) : identity(id), priority(prio) {}
        Item(T&& id, double prio) : identity(std::move(id)), priority(prio) {}
        
        // Items are equal if they have the same identity
        bool operator==(const Item& other) const {
            return Equal{}(identity, other.identity);
        }
        
        // For priority comparison (min-heap: lower priority value = higher importance)
        bool operator<(const Item& other) const {
            return priority < other.priority;
        }
        
        bool operator>(const Item& other) const {
            return priority > other.priority;
        }
    };

private:
    std::vector<Item> heap_;
    std::unordered_map<T, size_t, Hash, Equal> position_map_; // identity -> heap index
    size_t arity_;
    
    size_t parent_index(size_t index) const {
        return index == 0 ? 0 : (index - 1) / arity_;
    }
    
    size_t first_child_index(size_t index) const {
        return arity_ * index + 1;
    }
    
    void swap_items(size_t i, size_t j) {
        if (i == j) return;
        
        // Update position map
        position_map_[heap_[i].identity] = j;
        position_map_[heap_[j].identity] = i;
        
        // Swap items in heap
        std::swap(heap_[i], heap_[j]);
    }
    
    void heapify_up(size_t index) {
        while (index > 0) {
            size_t parent_idx = parent_index(index);
            if (heap_[index] < heap_[parent_idx]) {
                swap_items(index, parent_idx);
                index = parent_idx;
            } else {
                break;
            }
        }
    }
    
    void heapify_down(size_t index) {
        while (true) {
            size_t min_index = index;
            size_t first_child = first_child_index(index);
            
            // Find the child with minimum priority
            for (size_t i = 0; i < arity_ && first_child + i < heap_.size(); ++i) {
                size_t child_idx = first_child + i;
                if (heap_[child_idx] < heap_[min_index]) {
                    min_index = child_idx;
                }
            }
            
            if (min_index != index) {
                swap_items(index, min_index);
                index = min_index;
            } else {
                break;
            }
        }
    }
    
    void remove_at_index(size_t index) {
        if (index >= heap_.size()) {
            throw std::out_of_range("Index out of range");
        }
        
        // Remove from position map
        position_map_.erase(heap_[index].identity);
        
        if (index == heap_.size() - 1) {
            // Removing last element
            heap_.pop_back();
            return;
        }
        
        // Move last element to the removed position
        Item last_item = std::move(heap_.back());
        heap_.pop_back();
        
        heap_[index] = std::move(last_item);
        position_map_[heap_[index].identity] = index;
        
        // Restore heap property
        if (index > 0 && heap_[index] < heap_[parent_index(index)]) {
            heapify_up(index);
        } else {
            heapify_down(index);
        }
    }

public:
    explicit DaryHeapPriorityQueue(size_t arity = 2) : arity_(arity) {
        if (arity < 2) {
            throw std::invalid_argument("Arity must be at least 2");
        }
    }
    
    // Insert an item into the queue
    void insert(const Item& item) {
        insert(item.identity, item.priority);
    }
    
    void insert(const T& identity, double priority) {
        if (contains(identity)) {
            throw std::invalid_argument("Item with this identity already exists");
        }
        
        size_t new_index = heap_.size();
        heap_.emplace_back(identity, priority);
        position_map_[identity] = new_index;
        
        heapify_up(new_index);
    }
    
    void insert(T&& identity, double priority) {
        if (contains(identity)) {
            throw std::invalid_argument("Item with this identity already exists");
        }
        
        size_t new_index = heap_.size();
        T identity_copy = identity; // For the map key
        heap_.emplace_back(std::move(identity), priority);
        position_map_[identity_copy] = new_index;
        
        heapify_up(new_index);
    }
    
    // Remove and return the item with highest priority (lowest value)
    Item pop() {
        if (is_empty()) {
            throw std::runtime_error("Queue is empty");
        }
        
        Item result = heap_[0];
        remove_at_index(0);
        return result;
    }
    
    // Return the item with highest priority without removing it
    const Item& front() const {
        if (is_empty()) {
            throw std::runtime_error("Queue is empty");
        }
        return heap_[0];
    }
    
    // Update an existing item to have higher priority (lower value)
    void increase_priority(const T& identity, double new_priority) {
        auto it = position_map_.find(identity);
        if (it == position_map_.end()) {
            throw std::invalid_argument("Item not found");
        }
        
        size_t index = it->second;
        if (new_priority >= heap_[index].priority) {
            throw std::invalid_argument("New priority must be lower (higher importance) than current priority");
        }
        
        heap_[index].priority = new_priority;
        heapify_up(index);
    }
    
    // Update an existing item to have lower priority (higher value)
    void decrease_priority(const T& identity, double new_priority) {
        auto it = position_map_.find(identity);
        if (it == position_map_.end()) {
            throw std::invalid_argument("Item not found");
        }
        
        size_t index = it->second;
        if (new_priority <= heap_[index].priority) {
            throw std::invalid_argument("New priority must be higher (lower importance) than current priority");
        }
        
        heap_[index].priority = new_priority;
        heapify_down(index);
    }
    
    // Check if an item with the given identity exists
    bool contains(const T& identity) const {
        return position_map_.find(identity) != position_map_.end();
    }
    
    // Return the number of items in the queue
    size_t len() const {
        return heap_.size();
    }
    
    // Return whether the queue is empty
    bool is_empty() const {
        return heap_.empty();
    }
    
    // Get the current priority of an item
    double get_priority(const T& identity) const {
        auto it = position_map_.find(identity);
        if (it == position_map_.end()) {
            throw std::invalid_argument("Item not found");
        }
        return heap_[it->second].priority;
    }
    
    // Remove an item by identity
    void remove(const T& identity) {
        auto it = position_map_.find(identity);
        if (it == position_map_.end()) {
            throw std::invalid_argument("Item not found");
        }
        remove_at_index(it->second);
    }
    
    // Clear all items
    void clear() {
        heap_.clear();
        position_map_.clear();
    }
    
    // Get the arity of the heap
    size_t arity() const {
        return arity_;
    }
};