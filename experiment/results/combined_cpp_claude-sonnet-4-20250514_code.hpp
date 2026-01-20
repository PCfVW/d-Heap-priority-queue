// priority_queue.h
#ifndef PRIORITY_QUEUE_H
#define PRIORITY_QUEUE_H

#include <vector>
#include <unordered_map>
#include <cassert>
#include <functional>

template<typename T>
class DaryHeap {
private:
    std::vector<T> heap;
    std::unordered_map<std::string, size_t> position_map;
    size_t d; // arity
    
    // Helper functions for navigation
    size_t parent_index(size_t i) const {
        return (i - 1) / d;
    }
    
    size_t first_child_index(size_t i) const {
        return d * i + 1;
    }
    
    size_t last_child_index(size_t i) const {
        return d * i + d;
    }
    
    // Sift operations
    void sift_up(size_t index) {
        while (index > 0) {
            size_t parent_idx = parent_index(index);
            if (heap[index].priority >= heap[parent_idx].priority) {
                break;
            }
            
            // Swap with parent
            std::swap(heap[index], heap[parent_idx]);
            
            // Update position map
            position_map[heap[index].id] = index;
            position_map[heap[parent_idx].id] = parent_idx;
            
            index = parent_idx;
        }
    }
    
    void sift_down(size_t index) {
        while (true) {
            size_t min_child_idx = index;
            size_t first_child = first_child_index(index);
            size_t last_child = std::min(last_child_index(index), heap.size() - 1);
            
            // Find child with minimum priority
            for (size_t child_idx = first_child; child_idx <= last_child; ++child_idx) {
                if (heap[child_idx].priority < heap[min_child_idx].priority) {
                    min_child_idx = child_idx;
                }
            }
            
            // If no child has higher priority, we're done
            if (min_child_idx == index) {
                break;
            }
            
            // Swap with minimum child
            std::swap(heap[index], heap[min_child_idx]);
            
            // Update position map
            position_map[heap[index].id] = index;
            position_map[heap[min_child_idx].id] = min_child_idx;
            
            index = min_child_idx;
        }
    }

public:
    explicit DaryHeap(size_t arity) : d(arity) {
        assert(d >= 2);
    }
    
    void insert(const T& item) {
        assert(position_map.find(item.id) == position_map.end());
        
        heap.push_back(item);
        size_t new_index = heap.size() - 1;
        position_map[item.id] = new_index;
        
        sift_up(new_index);
    }
    
    void pop() {
        assert(!heap.empty());
        
        if (heap.size() == 1) {
            position_map.erase(heap[0].id);
            heap.clear();
            return;
        }
        
        // Remove root from position map
        position_map.erase(heap[0].id);
        
        // Move last element to root
        heap[0] = heap.back();
        heap.pop_back();
        
        // Update position map for moved element
        position_map[heap[0].id] = 0;
        
        // Restore heap property
        sift_down(0);
    }
    
    const T& front() const {
        assert(!heap.empty());
        return heap[0];
    }
    
    void increase_priority(const T& item) {
        auto it = position_map.find(item.id);
        assert(it != position_map.end());
        
        size_t index = it->second;
        heap[index].priority = item.priority;
        
        sift_up(index);
    }
    
    void decrease_priority(const T& item) {
        auto it = position_map.find(item.id);
        assert(it != position_map.end());
        
        size_t index = it->second;
        heap[index].priority = item.priority;
        
        sift_down(index);
    }
    
    bool contains(const T& item) const {
        return position_map.find(item.id) != position_map.end();
    }
    
    size_t len() const {
        return heap.size();
    }
    
    bool is_empty() const {
        return heap.empty();
    }
};

#endif // PRIORITY_QUEUE_H