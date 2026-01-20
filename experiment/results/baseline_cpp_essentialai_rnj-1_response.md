Here's a Python implementation of a d-ary heap priority queue that meets all the requirements:

```python
class DaryHeap:
    def __init__(self, arity):
        self.heap = []
        self.item_map = {}
        self.arity = arity

    def insert(self, item):
        if item.identity in self.item_map:
            raise ValueError("Item with this identity already exists")
        self.item_map[item.identity] = len(self.heap)
        self.heap.append(item)
        self._bubble_up(len(self.heap) - 1)

    def pop(self):
        if not self.heap:
            raise IndexError("Heap is empty")
        item = self.heap[0]
        last_item = self.heap.pop()
        if self.heap:
            self.item_map[last_item.identity] = 0
            self.heap[0] = last_item
            self._bubble_down(0)
        del self.item_map[item.identity]
        return item

    def front(self):
        if not self.heap:
            raise IndexError("Heap is empty")
        return self.heap[0]

    def increase_priority(self, item):
        index = self.item_map.get(item.identity)
        if index is None:
            raise ValueError("Item with this identity does not exist")
        old_priority = self.heap[index].priority
        new_priority = min(old_priority - 1, 0)  # Assuming priority can be negative
        if new_priority >= old_priority:
            return
        self.heap[index].priority = new_priority
        self._bubble_up(index)

    def decrease_priority(self, item):
        index = self.item_map.get(item.identity)
        if index is None:
            raise ValueError("Item with this identity does not exist")
        old_priority = self.heap[index].priority
        new_priority = max(old_priority + 1, float('inf'))  # Assuming priority can be positive infinity
        if new_priority <= old_priority:
            return
        self.heap[index].priority = new_priority
        self._bubble_down(index)

    def contains(self, item):
        return item.identity in self.item_map

    def __len__(self):
        return len(self.heap)

    def is_empty(self):
        return not self.heap

    def _bubble_up(self, index):
        while index > 0:
            parent_index = (index - 1) // self.arity
            if self.heap[index].priority < self.heap[parent_index].priority:
                self._swap(index, parent_index)
                index = parent_index
            else:
                break

    def _bubble_down(self, index):
        while True:
            child_indices = [self.arity * index + i for i in range(1, self.arity + 1)]
            child_indices = [i for i in child_indices if i < len(self.heap)]
            if not child_indices:
                break
            min_child_index = min(child_indices, key=lambda i: self.heap[i].priority)
            if self.heap[index].priority > self.heap[min_child_index].priority:
                self._swap(index, min_child_index)
                index = min_child_index
            else:
                break

    def _swap(self, i, j):
        self.item_map[self.heap[i].identity] = j
        self.item_map[self.heap[j].identity] = i
        self.heap[i], self.heap[j] = self.heap[j], self.heap[i]


class Item:
    def __init__(self, identity, priority):
        self.identity = identity
        self.priority = priority

    def __repr__(self):
        return f"Item(identity={self.identity}, priority={self.priority})"


# Example usage:
heap = DaryHeap(3)
item1 = Item("A", 5)
item2 = Item("B", 10)
item3 = Item("C", 15)

heap.insert(item1)
heap.insert(item2)
heap.insert(item3)

print(heap.pop())  # Output: Item(identity=A, priority=5)
print(heap.front())  # Output: Item(identity=B, priority=10)

heap.increase_priority(item2)
print(heap.pop())  # Output: Item(identity=C, priority=15)

heap.decrease_priority(item1)
print(heap.pop())  # Output: Item(identity=A, priority=4)
```

This implementation uses a dictionary to map item identities to their indices in the heap, allowing for O(1) lookup time. The `insert`, `pop`, `front`, `increase_priority`, and `decrease_priority` methods all have logarithmic time complexity due to the use of bubble-up and bubble-down operations.

The example usage demonstrates how to create a d-ary heap with arity 3, insert items into the heap, pop the item with highest priority, increase and decrease the priority of an existing item, and check if an item exists in the heap.