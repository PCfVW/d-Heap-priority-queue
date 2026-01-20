"""D-ary heap priority queue implementation with doctests."""

from dataclasses import dataclass
from typing import Optional, Callable, List, Dict

@dataclass
class Item:
    """An item with identity (number) and priority (cost).

    >>> item = Item(50, 100)
    >>> item.number
    50
    >>> item.cost
    100
    """
    number: int
    cost: int

    def __eq__(self, other):
        """Items are equal if they have the same number (identity).

        >>> Item(10, 50) == Item(10, 100)
        True
        >>> Item(10, 50) == Item(20, 50)
        False
        """
        if not isinstance(other, Item):
            return False
        return self.number == other.number

    def __hash__(self):
        return hash(self.number)

class DHeap:
    """A d-ary min-heap priority queue.

    >>> pq = DHeap(4)  # 4-ary heap
    >>> pq.is_empty()
    True
    >>> len(pq)
    0
    """

    def __init__(self, d: int = 4):
        """Initialize a d-ary heap.

        >>> pq = DHeap(2)  # binary heap
        >>> pq = DHeap(4)  # 4-ary heap
        """
        if d < 2:
            raise ValueError("Heap arity must be at least 2")
        self.d = d
        self.heap = []
        self.position_map = {}  # Maps item identity to index in heap

    def _parent(self, index: int) -> int:
        """Return the parent index of the given index."""
        return (index - 1) // self.d

    def _children(self, index: int) -> List[int]:
        """Return the indices of all children of the given index."""
        start = self.d * index + 1
        end = start + self.d
        return list(range(start, min(end, len(self.heap))))

    def _swap(self, i: int, j: int) -> None:
        """Swap items at positions i and j in the heap."""
        self.heap[i], self.heap[j] = self.heap[j], self.heap[i]
        self.position_map[self.heap[i].number] = i
        self.position_map[self.heap[j].number] = j

    def _bubble_up(self, index: int) -> None:
        """Bubble up the item at the given index to maintain heap property."""
        while index > 0:
            parent = self._parent(index)
            if self.heap[index].cost < self.heap[parent].cost:
                self._swap(index, parent)
                index = parent
            else:
                break

    def _bubble_down(self, index: int) -> None:
        """Bubble down the item at the given index to maintain heap property."""
        while True:
            children = self._children(index)
            if not children:
                break

            # Find the child with minimum cost
            min_child = min(children, key=lambda i: self.heap[i].cost)
            if self.heap[index].cost > self.heap[min_child].cost:
                self._swap(index, min_child)
                index = min_child
            else:
                break

    def insert(self, item: Item) -> None:
        """Insert an item into the heap.

        >>> pq = DHeap(4)
        >>> pq.insert(Item(50, 50))
        >>> pq.contains(Item(50, 0))  # Same identity, different cost
        True
        >>> len(pq)
        1
        """
        if item.number in self.position_map:
            raise ValueError(f"Item with identity {item.number} already exists")

        self.heap.append(item)
        self.position_map[item.number] = len(self.heap) - 1
        self._bubble_up(len(self.heap) - 1)

    def pop(self) -> Optional[Item]:
        """Remove and return the minimum item.

        >>> pq = DHeap(4)
        >>> pq.insert(Item(30, 30))
        >>> pq.insert(Item(10, 10))
        >>> pq.insert(Item(20, 20))
        >>> item = pq.pop()
        >>> item.cost
        10
        >>> len(pq)
        2
        """
        if not self.heap:
            return None

        min_item = self.heap[0]
        last_item = self.heap.pop()
        del self.position_map[min_item.number]

        if self.heap:
            self.heap[0] = last_item
            self.position_map[last_item.number] = 0
            self._bubble_down(0)

        return min_item

    def front(self) -> Optional[Item]:
        """Return the minimum item without removing it.

        >>> pq = DHeap(4)
        >>> pq.insert(Item(30, 30))
        >>> pq.insert(Item(10, 10))
        >>> pq.front().cost
        10
        >>> len(pq)  # Size unchanged
        2
        """
        if not self.heap:
            return None
        return self.heap[0]

    def increase_priority(self, item: Item) -> None:
        """Increase priority (decrease cost) of an existing item.

        >>> pq = DHeap(4)
        >>> pq.insert(Item(50, 50))
        >>> pq.insert(Item(30, 30))
        >>> pq.front().cost
        30
        >>> pq.increase_priority(Item(50, 10))  # Lower cost = higher priority
        >>> pq.front().cost
        10
        """
        if item.number not in self.position_map:
            raise ValueError(f"Item with identity {item.number} not found")

        index = self.position_map[item.number]
        if item.cost < self.heap[index].cost:
            self.heap[index].cost = item.cost
            self._bubble_up(index)
        else:
            raise ValueError("New cost must be lower than current cost")

    def decrease_priority(self, item: Item) -> None:
        """Decrease priority (increase cost) of an existing item.

        >>> pq = DHeap(4)
        >>> pq.insert(Item(10, 10))
        >>> pq.insert(Item(30, 30))
        >>> pq.front().number
        10
        >>> pq.decrease_priority(Item(10, 50))  # Higher cost = lower priority
        >>> pq.front().number
        30
        """
        if item.number not in self.position_map:
            raise ValueError(f"Item with identity {item.number} not found")

        index = self.position_map[item.number]
        if item.cost > self.heap[index].cost:
            self.heap[index].cost = item.cost
            self._bubble_down(index)
        else:
            raise ValueError("New cost must be higher than current cost")

    def contains(self, item: Item) -> bool:
        """Check if an item with the same identity exists.

        >>> pq = DHeap(4)
        >>> pq.insert(Item(50, 50))
        >>> pq.contains(Item(50, 999))  # Same number, different cost
        True
        >>> pq.contains(Item(999, 50))  # Different number
        False
        """
        return item.number in self.position_map

    def __len__(self) -> int:
        """Return the number of items.

        >>> pq = DHeap(4)
        >>> len(pq)
        0
        >>> pq.insert(Item(10, 10))
        >>> len(pq)
        1
        """
        return len(self.heap)

    def is_empty(self) -> bool:
        """Return True if the heap is empty.

        >>> pq = DHeap(4)
        >>> pq.is_empty()
        True
        >>> pq.insert(Item(10, 10))
        >>> pq.is_empty()
        False
        """
        return len(self.heap) == 0

if __name__ == "__main__":
    import doctest
    doctest.testmod()
