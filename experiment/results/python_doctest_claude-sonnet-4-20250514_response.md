Here's a complete implementation of the d-ary heap priority queue:

```python
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
        self.d = d
        self.heap: List[Item] = []
        self.position: Dict[int, int] = {}  # Maps item number to heap index

    def _parent(self, i: int) -> int:
        """Get parent index of node at index i."""
        return (i - 1) // self.d

    def _children(self, i: int) -> List[int]:
        """Get children indices of node at index i."""
        start = i * self.d + 1
        end = min(start + self.d, len(self.heap))
        return list(range(start, end))

    def _swap(self, i: int, j: int) -> None:
        """Swap items at indices i and j and update position tracking."""
        self.heap[i], self.heap[j] = self.heap[j], self.heap[i]
        self.position[self.heap[i].number] = i
        self.position[self.heap[j].number] = j

    def _heapify_up(self, i: int) -> None:
        """Move item up the heap to maintain min-heap property."""
        while i > 0:
            parent = self._parent(i)
            if self.heap[i].cost >= self.heap[parent].cost:
                break
            self._swap(i, parent)
            i = parent

    def _heapify_down(self, i: int) -> None:
        """Move item down the heap to maintain min-heap property."""
        while True:
            children = self._children(i)
            if not children:
                break
            
            # Find child with minimum cost
            min_child = children[0]
            for child in children[1:]:
                if self.heap[child].cost < self.heap[min_child].cost:
                    min_child = child
            
            if self.heap[i].cost <= self.heap[min_child].cost:
                break
            
            self._swap(i, min_child)
            i = min_child

    def insert(self, item: Item) -> None:
        """Insert an item into the heap.

        >>> pq = DHeap(4)
        >>> pq.insert(Item(50, 50))
        >>> pq.contains(Item(50, 0))  # Same identity, different cost
        True
        >>> len(pq)
        1
        """
        if item.number in self.position:
            # Update existing item's cost
            idx = self.position[item.number]
            old_cost = self.heap[idx].cost
            self.heap[idx].cost = item.cost
            
            if item.cost < old_cost:
                self._heapify_up(idx)
            else:
                self._heapify_down(idx)
        else:
            # Add new item
            self.heap.append(item)
            self.position[item.number] = len(self.heap) - 1
            self._heapify_up(len(self.heap) - 1)

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
        del self.position[min_item.number]
        
        if len(self.heap) == 1:
            self.heap.pop()
            return min_item
        
        # Move last item to root and heapify down
        last_item = self.heap.pop()
        self.heap[0] = last_item
        self.position[last_item.number] = 0
        self._heapify_down(0)
        
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
        return self.heap[0] if self.heap else None

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
        if item.number not in self.position:
            raise ValueError(f"Item with number {item.number} not found")
        
        idx = self.position[item.number]
        if item.cost >= self.heap[idx].cost:
            raise ValueError("New cost must be lower than current cost")
        
        self.heap[idx].cost = item.cost
        self._heapify_up(idx)

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
        if item.number not in self.position:
            raise ValueError(f"Item with number {item.number} not found")
        
        idx = self.position[item.number]
        if item.cost <= self.heap[idx].cost:
            raise ValueError("New cost must be higher than current cost")
        
        self.heap[idx].cost = item.cost
        self._heapify_down(idx)

    def contains(self, item: Item) -> bool:
        """Check if an item with the same identity exists.

        >>> pq = DHeap(4)
        >>> pq.insert(Item(50, 50))
        >>> pq.contains(Item(50, 999))  # Same number, different cost
        True
        >>> pq.contains(Item(999, 50))  # Different number
        False
        """
        return item.number in self.position

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
```

This implementation provides:

1. **Configurable arity**: The heap can be configured with any d-value at construction time
2. **Item identity vs priority**: Items are considered equal based on their `number` (identity), but ordered by their `cost` (priority)
3. **O(1) lookup**: The `position` dictionary provides constant-time lookup for existence checks and position tracking
4. **Min-heap property**: Lower cost values have higher priority and bubble up to the root
5. **All required operations**: 
   - `insert`: O(log_d n) - adds item or updates existing one
   - `pop`: O(d log_d n) - removes minimum item
   - `front`: O(1) - returns minimum item
   - `increase_priority`/`decrease_priority`: O(log_d n) - updates item priority
   - `contains`: O(1) - checks existence
   - `__len__` and `is_empty`: O(1) - size operations

The implementation maintains the heap invariant through `_heapify_up` and `_heapify_down` operations, and keeps the position dictionary synchronized with any swaps made during heapification.