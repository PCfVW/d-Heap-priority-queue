#ifndef DARY_HEAP_H
#define DARY_HEAP_H

#include <vector>
#include <unordered_map>
#include <cassert>
#include <algorithm>

struct Item {
    std::string id;
    int priority;
    
    Item(const std::string& id, int priority) : id(id), priority(priority) {}
    
    bool operator==(const Item& other) const {
        return id == other.id;
    }
};

class DaryHeap {
private:
    std::vector<Item> heap;
    std::unordered_map<std::string, size_t> position_map;
    size_t d;  // arity of the heap
    
    size_t parent(size_t i) const {
        return (i - 1) / d;
    }
    
    size_t first_child(size_t i) const {
        return d * i + 1;
    }
    
    void bubble_up(size_t i) {
        while (i > 0) {
            size_t p = parent(i);
            if (heap[i].priority >= heap[p].priority) {
                break;
            }
            
            // Swap and update position map
            std::swap(heap[i], heap[p]);
            position_map[heap[i].id] = i;
            position_map[heap[p].id] = p;
            
            i = p;
        }
    }
    
    void bubble_down(size_t i) {
        while (true) {
            size_t min_child = i;
            size_t first = first_child(i);
            
            // Find the child with minimum priority
            for (size_t j = first; j < first + d && j < heap.size(); j++) {
                if (heap[j].priority < heap[min_child].priority) {
                    min_child = j;
                }
            }
            
            if (min_child == i) {
                break;  // Heap property satisfied
            }
            
            // Swap and update position map
            std::swap(heap[i], heap[min_child]);
            position_map[heap[i].id] = i;
            position_map[heap[min_child].id] = min_child;
            
            i = min_child;
        }
    }
    
public:
    explicit DaryHeap(size_t arity) : d(arity) {}
    
    void insert(const Item& item) {
        // If item already exists, this is effectively an update
        if (position_map.find(item.id) != position_map.end()) {
            size_t pos = position_map[item.id];
            int old_priority = heap[pos].priority;
            heap[pos].priority = item.priority;
            
            if (item.priority < old_priority) {
                bubble_up(pos);
            } else if (item.priority > old_priority) {
                bubble_down(pos);
            }
            return;
        }
        
        // Add new item
        heap.push_back(item);
        size_t pos = heap.size() - 1;
        position_map[item.id] = pos;
        bubble_up(pos);
    }
    
    void pop() {
        assert(!heap.empty());
        
        // Remove from position map
        position_map.erase(heap[0].id);
        
        if (heap.size() == 1) {
            heap.pop_back();
            return;
        }
        
        // Move last element to root and bubble down
        heap[0] = heap.back();
        heap.pop_back();
        position_map[heap[0].id] = 0;
        bubble_down(0);
    }
    
    const Item& front() const {
        assert(!heap.empty());
        return heap[0];
    }
    
    void increase_priority(const Item& item) {
        assert(position_map.find(item.id) != position_map.end());
        
        size_t pos = position_map[item.id];
        assert(item.priority < heap[pos].priority);  // Ensure it's actually an increase
        
        heap[pos].priority = item.priority;
        bubble_up(pos);
    }
    
    void decrease_priority(const Item& item) {
        assert(position_map.find(item.id) != position_map.end());
        
        size_t pos = position_map[item.id];
        assert(item.priority > heap[pos].priority);  // Ensure it's actually a decrease
        
        heap[pos].priority = item.priority;
        bubble_down(pos);
    }
    
    bool contains(const Item& item) const {
        return position_map.find(item.id) != position_map.end();
    }
    
    size_t len() const {
        return heap.size();
    }
    
    bool is_empty() const {
        return heap.empty();
    }
};

#endif // DARY_HEAP_H