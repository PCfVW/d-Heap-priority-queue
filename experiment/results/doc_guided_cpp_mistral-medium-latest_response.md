# D-ary Heap Priority Queue Implementation in C++

Here's a complete implementation of a d-ary heap priority queue with all the required functionality:

```cpp
#include <vector>
#include <unordered_map>
#include <stdexcept>
#include <functional>
#include <algorithm>

template <typename T, typename Identity = T, typename Priority = T>
class DaryHeap {
private:
    struct HeapItem {
        Identity identity;
        Priority priority;

        bool operator==(const HeapItem& other) const {
            return identity == other.identity;
        }
    };

    int d; // arity of the heap
    std::vector<HeapItem> heap;
    std::unordered_map<Identity, size_t> position_map;

    // Helper functions
    size_t parent(size_t index) const {
        return (index - 1) / d;
    }

    size_t first_child(size_t index) const {
        return d * index + 1;
    }

    size_t last_child(size_t index) const {
        return d * index + d;
    }

    bool has_child(size_t index) const {
        return first_child(index) < heap.size();
    }

    void swap(size_t i, size_t j) {
        std::swap(heap[i], heap[j]);
        position_map[heap[i].identity] = i;
        position_map[heap[j].identity] = j;
    }

    void sift_up(size_t index) {
        while (index > 0) {
            size_t parent_idx = parent(index);
            if (heap[index].priority < heap[parent_idx].priority) {
                swap(index, parent_idx);
                index = parent_idx;
            } else {
                break;
            }
        }
    }

    void sift_down(size_t index) {
        while (has_child(index)) {
            size_t min_child_idx = first_child(index);
            size_t last_child_idx = std::min(last_child(index), heap.size() - 1);

            // Find the child with minimum priority
            for (size_t i = first_child(index) + 1; i <= last_child_idx; ++i) {
                if (heap[i].priority < heap[min_child_idx].priority) {
                    min_child_idx = i;
                }
            }

            if (heap[index].priority > heap[min_child_idx].priority) {
                swap(index, min_child_idx);
                index = min_child_idx;
            } else {
                break;
            }
        }
    }

public:
    // Constructor
    explicit DaryHeap(int arity = 2) : d(arity) {
        if (d < 2) {
            throw std::invalid_argument("Ary must be at least 2");
        }
    }

    // Insert an item into the heap
    void insert(const Identity& identity, const Priority& priority) {
        if (position_map.find(identity) != position_map.end()) {
            throw std::invalid_argument("Item with this identity already exists");
        }

        heap.push_back({identity, priority});
        position_map[identity] = heap.size() - 1;
        sift_up(heap.size() - 1);
    }

    // Remove and return the item with highest priority
    HeapItem pop() {
        if (heap.empty()) {
            throw std::runtime_error("Heap is empty");
        }

        HeapItem result = heap[0];
        position_map.erase(result.identity);

        if (heap.size() > 1) {
            heap[0] = heap.back();
            position_map[heap[0].identity] = 0;
        }
        heap.pop_back();

        if (!heap.empty()) {
            sift_down(0);
        }

        return result;
    }

    // Return the item with highest priority without removing it
    const HeapItem& front() const {
        if (heap.empty()) {
            throw std::runtime_error("Heap is empty");
        }
        return heap[0];
    }

    // Increase priority (make more important) of an existing item
    void increase_priority(const Identity& identity, const Priority& new_priority) {
        auto it = position_map.find(identity);
        if (it == position_map.end()) {
            throw std::invalid_argument("Item not found in heap");
        }

        size_t index = it->second;
        if (new_priority > heap[index].priority) {
            throw std::invalid_argument("New priority must be lower (more important) than current");
        }

        heap[index].priority = new_priority;
        sift_up(index);
    }

    // Decrease priority (make less important) of an existing item
    void decrease_priority(const Identity& identity, const Priority& new_priority) {
        auto it = position_map.find(identity);
        if (it == position_map.end()) {
            throw std::invalid_argument("Item not found in heap");
        }

        size_t index = it->second;
        if (new_priority < heap[index].priority) {
            throw std::invalid_argument("New priority must be higher (less important) than current");
        }

        heap[index].priority = new_priority;
        sift_down(index);
    }

    // Check if an item exists in the heap
    bool contains(const Identity& identity) const {
        return position_map.find(identity) != position_map.end();
    }

    // Return the number of items in the heap
    size_t size() const {
        return heap.size();
    }

    // Check if the heap is empty
    bool empty() const {
        return heap.empty();
    }
};
```

## Usage Example

Here's how you might use this implementation:

```cpp
#include <iostream>

int main() {
    // Create a 3-ary heap
    DaryHeap<std::string, std::string, int> heap(3);

    // Insert some items
    heap.insert("task1", 5);
    heap.insert("task2", 3);
    heap.insert("task3", 7);
    heap.insert("task4", 1);

    // Check if items exist
    std::cout << "Contains task2: " << heap.contains("task2") << std::endl;
    std::cout << "Contains task5: " << heap.contains("task5") << std::endl;

    // Get the front item
    std::cout << "Front item: " << heap.front().identity
              << " with priority " << heap.front().priority << std::endl;

    // Pop items
    while (!heap.empty()) {
        auto item = heap.pop();
        std::cout << "Popped: " << item.identity
                  << " with priority " << item.priority << std::endl;
    }

    return 0;
}
```

## Key Features

1. **Generic Implementation**: Works with any types for identity and priority
2. **Efficient Operations**:
   - O(1) for contains(), size(), empty(), and front()
   - O(log_d n) for insert() and increase_priority()
   - O(d * log_d n) for pop() and decrease_priority()
3. **Position Map**: Maintains O(1) lookup for item positions
4. **Error Handling**: Proper exception throwing for invalid operations
5. **Complete API**: Implements all required methods from the specification

The implementation maintains the heap property at all times and keeps the position map synchronized with the heap array.