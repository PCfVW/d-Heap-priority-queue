#include <vector>
#include <unordered_map>
#include <string>
#include <stdexcept>
#include <cassert>

struct Item {
    std::string id;
    int priority;

    Item(const std::string& id, int priority) : id(id), priority(priority) {}

    bool operator==(const Item& other) const {
        return id == other.id;
    }
};

class DaryHeapPriorityQueue {
private:
    int d; // arity of the heap
    std::vector<Item> heap;
    std::unordered_map<std::string, size_t> itemToIndex; // for O(1) lookups

    // Helper functions
    size_t parent(size_t index) const {
        return (index - 1) / d;
    }

    size_t firstChild(size_t index) const {
        return d * index + 1;
    }

    void bubbleUp(size_t index) {
        while (index > 0) {
            size_t parentIndex = parent(index);
            if (heap[index].priority < heap[parentIndex].priority) {
                std::swap(heap[index], heap[parentIndex]);
                itemToIndex[heap[index].id] = index;
                itemToIndex[heap[parentIndex].id] = parentIndex;
                index = parentIndex;
            } else {
                break;
            }
        }
    }

    void bubbleDown(size_t index) {
        while (true) {
            size_t minIndex = index;
            size_t firstChildIndex = firstChild(index);

            // Check all children
            for (size_t i = 0; i < d; ++i) {
                size_t childIndex = firstChildIndex + i;
                if (childIndex < heap.size() &&
                    heap[childIndex].priority < heap[minIndex].priority) {
                    minIndex = childIndex;
                }
            }

            if (minIndex != index) {
                std::swap(heap[index], heap[minIndex]);
                itemToIndex[heap[index].id] = index;
                itemToIndex[heap[minIndex].id] = minIndex;
                index = minIndex;
            } else {
                break;
            }
        }
    }

public:
    DaryHeapPriorityQueue(int arity) : d(arity) {
        if (d < 2) {
            throw std::invalid_argument("Heap arity must be at least 2");
        }
    }

    void insert(const Item& item) {
        if (itemToIndex.find(item.id) != itemToIndex.end()) {
            // Item already exists, we could update it but the tests expect insert to add new items
            return;
        }

        heap.push_back(item);
        size_t index = heap.size() - 1;
        itemToIndex[item.id] = index;
        bubbleUp(index);
    }

    void pop() {
        if (heap.empty()) {
            throw std::runtime_error("Cannot pop from empty heap");
        }

        // Remove the root
        itemToIndex.erase(heap[0].id);

        if (heap.size() > 1) {
            // Move last element to root and bubble down
            heap[0] = heap.back();
            itemToIndex[heap[0].id] = 0;
        }

        heap.pop_back();

        if (!heap.empty()) {
            bubbleDown(0);
        }
    }

    const Item& front() const {
        if (heap.empty()) {
            throw std::runtime_error("Cannot get front of empty heap");
        }
        return heap[0];
    }

    void increase_priority(const Item& item) {
        auto it = itemToIndex.find(item.id);
        if (it == itemToIndex.end()) {
            throw std::runtime_error("Item not found in heap");
        }

        size_t index = it->second;
        if (item.priority > heap[index].priority) {
            throw std::runtime_error("Cannot increase priority to a lower value");
        }

        heap[index].priority = item.priority;
        bubbleUp(index);
    }

    void decrease_priority(const Item& item) {
        auto it = itemToIndex.find(item.id);
        if (it == itemToIndex.end()) {
            throw std::runtime_error("Item not found in heap");
        }

        size_t index = it->second;
        if (item.priority < heap[index].priority) {
            throw std::runtime_error("Cannot decrease priority to a higher value");
        }

        heap[index].priority = item.priority;
        bubbleDown(index);
    }

    bool contains(const Item& item) const {
        return itemToIndex.find(item.id) != itemToIndex.end();
    }

    size_t len() const {
        return heap.size();
    }

    bool is_empty() const {
        return heap.empty();
    }
};