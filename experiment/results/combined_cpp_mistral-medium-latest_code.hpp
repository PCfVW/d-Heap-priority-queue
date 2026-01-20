#include <vector>
#include <unordered_map>
#include <string>
#include <stdexcept>
#include <cassert>

template <typename T>
class DaryHeapPriorityQueue {
private:
    struct Item {
        std::string id;
        T priority;

        Item(const std::string& id, T priority) : id(id), priority(priority) {}

        bool operator==(const Item& other) const {
            return id == other.id;
        }
    };

    int d; // arity of the heap
    std::vector<Item> heap;
    std::unordered_map<std::string, size_t> position_map;

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

    void sift_up(size_t index) {
        while (index > 0) {
            size_t parent_index = parent(index);
            if (heap[index].priority < heap[parent_index].priority) {
                std::swap(heap[index], heap[parent_index]);
                position_map[heap[index].id] = index;
                position_map[heap[parent_index].id] = parent_index;
                index = parent_index;
            } else {
                break;
            }
        }
    }

    void sift_down(size_t index) {
        while (true) {
            size_t first = first_child(index);
            size_t last = last_child(index);
            size_t min_index = index;

            // Find the child with minimum priority
            for (size_t i = first; i <= last && i < heap.size(); ++i) {
                if (heap[i].priority < heap[min_index].priority) {
                    min_index = i;
                }
            }

            if (min_index != index) {
                std::swap(heap[index], heap[min_index]);
                position_map[heap[index].id] = index;
                position_map[heap[min_index].id] = min_index;
                index = min_index;
            } else {
                break;
            }
        }
    }

public:
    DaryHeapPriorityQueue(int arity) : d(arity) {
        assert(d >= 2 && "Ary must be at least 2");
    }

    void insert(const Item& item) {
        assert(!contains(item) && "Item with same identity already exists");

        heap.push_back(item);
        position_map[item.id] = heap.size() - 1;
        sift_up(heap.size() - 1);
    }

    void pop() {
        assert(!is_empty() && "Cannot pop from empty heap");

        // Swap root with last element
        std::swap(heap[0], heap.back());
        position_map[heap[0].id] = 0;

        // Remove last element
        position_map.erase(heap.back().id);
        heap.pop_back();

        // Sift down the new root
        if (!is_empty()) {
            sift_down(0);
        }
    }

    const Item& front() const {
        assert(!is_empty() && "Cannot get front of empty heap");
        return heap[0];
    }

    void increase_priority(const Item& item) {
        assert(contains(item) && "Item not found in heap");

        size_t index = position_map.at(item.id);
        assert(item.priority < heap[index].priority && "New priority must be higher (lower value)");

        heap[index].priority = item.priority;
        sift_up(index);
    }

    void decrease_priority(const Item& item) {
        assert(contains(item) && "Item not found in heap");

        size_t index = position_map.at(item.id);
        assert(item.priority > heap[index].priority && "New priority must be lower (higher value)");

        heap[index].priority = item.priority;
        sift_down(index);
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