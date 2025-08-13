/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////// PriorityQueue.h
///
/// C++ 17 implementation of a d-Heap priority queue
/// 
/// Copyright (c) 2023-2025 Eric Jacopin
/// 
/// Licensed under the Apache License, Version 2.0 (the "License")
/// 
/// =================================================================================================================== File Content
///
/// This file is divided as follows:
///	- File History					(Line 19)
/// - Inclusion of files			(Line 81)
///	- Namespace declaration			(Line 89)
///	- Class declaration				(Line 98)
///
///	- End of file					(line 319)
///
/// =================================================================================================================== File history
///
/// [Author, Created, Last Modification] = [Eric JACOPIN, 2023/07/29, 2025/08/13]
///
/// [DEV 4] Comments -------------------------------------------------------------------------------------------------- 2025/08/13
///		- Fixed line numbers in comments
///     - Added checking negative depth of the heap in both constructors and clear/0
///		- Change: float arithmetic to integer arithmetic in parent/1
///		- Added checking negative depth of the heap and negative item positions in parent/1
///		- MoveDownTheQueue() loop no longer uses parent(n-1)
///			- Stops when the current node has no children by checking
///				first_child = i * depth + 1 against Container.size().
///			- Avoids calling parent() with i == 0 in loop conditions
///			- Directly reflects "has children" logic,
///			- Avoids "tricky" off-by-one math
///		- Change:
///			From: typename THash = std::hash<size_t>
///			To : typename THash = std::hash<T>
///		  This aligns the default hash with the actual key type used by T2PositionInContainer(which is T),
///		  preventing surprises or compilation issues for custom types.
///		- Added function identifiers before each time complexity in [DEV 1] section
/// 
///	[DEV 3] Comments -------------------------------------------------------------------------------------------------- 2025/06/18
///		- Comments improved
///		- Web references:
///			1. https://mitmgmtfaculty.mit.edu/jorlin/network-flows/
///			2. https://en.wikipedia.org/wiki/D-heap
/// 
///	[DEV 2] Comments -------------------------------------------------------------------------------------------------- 2024/11/01
///		- Comments and structure improved
///		- No C++ 20 feature used; however, Visual C++ project successfully compiled with C++ 20 standard
/// 
///	[DEV 1] d-Heap Priority Queues ------------------------------------------------------------------------------------ 2023/10/19
///		- d-Heap is a data structure which supports both minimum and maximum cost priority queues
///		- d-Heap is a tree where each parent-node has at most d children
///		- Root node is the highest priority node (either min or max cost)
///		- Children nodes have lower priority than their parent node
///		- Children nodes are unordered
///		- Time complexities of basic operations over n items in a d-Heap are:
///			front/0				- O(1) for finding the item with highest priority
///			pop/0				- O(d x ln(n) / ln(d)) for deleting the item with highest priority
///			insert/1			- O(ln(n) / ln(d)) for inserting an item
///			increase_priority/1	- O(ln(n) / ln(d)) for updating the queue when the priority of an item increases
///
///		- This C++ 17 file
///				- d-Heap is both made of an array of the items and a dictionary of their positions in the queue
///				- d is set once for all when declaring the queue
///				- std::less will implement min-cost priority queue (default)
///				- std::greater will implement max-cost priority queue
///				- The position of an item can be dynamically updated in the queue according to its priority 
///				- Only the highest priority item can be removed from the queue
///				- The content of the array supporting the priority queue can be exported to a stream
///
///		- Reference: Section A.3, d-Heaps, pp. 773-778 of Ravindra Aruja, Thomas Magnanti & James Orlin, Network Flows,
///						Prentice Hall (1993), 846 pages.
///
///
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////// File inclusion
///
#pragma once
///
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///
#include <cassert>          // Visibility for assert
#include <iostream>			// Visibility for std::ostream
#include <optional>			// Visibility for std::optional
#include <unordered_map>	// Visibility for std::unordered_map implementing the dictionary of item positions
#include <vector>			// Visibility for std::vector implementing the queue itself
///
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////// Namespace declaration
///
namespace TOOLS				// Happy Tooling Happy Gaming
{
	///
	///////////////////////////////////////////////////////////////////////////////////////////////////////////////////
	///
	constexpr bool INCLUDE_ASSERT = true;				// WHEN false assert/1 are excluded at compile-time
	///
	/////////////////////////////////////////////////////////////////////////////////////////////////////////////////// Class declaration
	///
	template <	class T,										/* items in the queue */
				typename THash = std::hash<T>,					/* O(1) access to positions */
				typename TComparisonPredicate = std::less<T>,	/* std::less by default implements min cost priority queues: is the priority of one item less than the priority of another item? */
				typename TEqual = std::equal_to<T>				/* O(1) access to items */
			 > class PriorityQueue
	{
	// --------------------------
	// ----- PUBLIC CONTENT -----
	// --------------------------
	public:
		typedef typename std::vector<T>::size_type ItemPositionInPriorityQueue;

		// ----- Default constructor deleted; appropriate constructor must set the depth of the heap; see next two constructors
		PriorityQueue() = delete;

		// ----- All seasons constructor with mandatory setting of the depth of the heap
		PriorityQueue(const ItemPositionInPriorityQueue d) : depth(d)
		{
			// Nothing initialized but the depth of the heap
			if (TOOLS::INCLUDE_ASSERT) assert(d > 0);			// (d > 0) is a precondition for the d-Heap to be valid
		}

		// ----- Cherry on the cake constructor setting the depth of the heap while inserting the very first item in the queue
		PriorityQueue(const ItemPositionInPriorityQueue d, T& t) : depth(d)
		{
			if (TOOLS::INCLUDE_ASSERT) assert(d > 0);			// (d > 0) is a precondition for the d-Heap to be valid

			// Make t the very first item in the queue
			Container.emplace_back(t);
			T2PositionInContainer[Container[0]] = 0;
		}

		/* inline */ ItemPositionInPriorityQueue getd() const { return depth; }
		/* inline */ ItemPositionInPriorityQueue size() const { return Container.size(); }
		/* inline */ bool empty() const { return Container.empty(); }

		// ----- Return the highest priority item of the queue; calling this function on an empty Container causes undefined behavior.
		typename std::vector<T>::const_reference front() const
		{
			return Container.front();
		}

		// ----- Clear the internal data structures of the queue, optionally resetting the depth of the heap
		void clear(const std::optional<ItemPositionInPriorityQueue>& d = std::nullopt)
		{
			Container.clear();
			T2PositionInContainer.clear();
			if (d) depth = *d;
			if (TOOLS::INCLUDE_ASSERT) assert(depth > 0);		// (d > 0) is a precondition for the d-Heap to be valid
		}

		// ----- Insert item t in the queue according to its priority
		void insert(const T& t)
		{
			// First insert item t at the very end of the queue container
			Container.push_back(t);
			ItemPositionInPriorityQueue last_position_in_the_queue = static_cast<ItemPositionInPriorityQueue>(Container.size() - 1);
			T2PositionInContainer[Container.back()] = last_position_in_the_queue;

			// Make item t crawl up to a position corresponding to its priority
			MoveUpTheQueue(last_position_in_the_queue);
		}

		// ----- Move item at position i up in the queue according to its priority
		void increase_priority(ItemPositionInPriorityQueue i)
		{
			MoveUpTheQueue(i);
		}

		// ----- Move item t up in the queue according to its priority
		void increase_priority(T& t_with_new_higher_priority)
		{
			// Get the position of the item in the queue container
			QueueIterator it = T2PositionInContainer.find(t_with_new_higher_priority);		// Find the position of item t in the queue
			if (TOOLS::INCLUDE_ASSERT) assert(T2PositionInContainer.end() != it);			// t is expected to be in the queue and this should not be included
			ItemPositionInPriorityQueue position_of_t_in_the_queue = it->second;			// Remember current-priority t's position in the queue

			// Erase the item with previous priority and immediately insert it (with new higher priority) as no item in the queue can be modified directly
			T2PositionInContainer.erase(it->first);
			T2PositionInContainer.insert({ t_with_new_higher_priority, position_of_t_in_the_queue });
			
			// Update the priority of item t in the queue
			Container[position_of_t_in_the_queue] = t_with_new_higher_priority;

			// t in the queue now has a higher priority than before: move t up the queue
			MoveUpTheQueue(position_of_t_in_the_queue);
		}

		// ----- Remove the highest priority item from the queue
		void pop()
		{
			// First exchange the very first (highest priority) item and the very last item in the queue container
			swap(0, static_cast<ItemPositionInPriorityQueue>(Container.size() - 1));

			// Forget the position of the very first item
			T2PositionInContainer.erase(Container.back());

			// Eventually remove the (highest priority) item from the queue container
			Container.pop_back();

			// IF the queue contained only one item THEN we're done!
			if (!Container.empty())
				// Else adjust the position (in the queue container) of what was the very last item in the container and now is the very first item
				MoveDownTheQueue(0);
		}

		// ----- Output the queue in a stream
		void put(std::ostream& os) const
		{
			os << "{";
			if (!Container.empty())
			{
				for (auto it = Container.begin(); it != Container.end() - 1; ++it)
				{
					os << (*it) << ", ";
				}
				if (0 <= Container.size() - 1)
					os << Container[Container.size() - 1];
			}
			os << "}";
		}

	// ---------------------------
	// ----- PRIVATE CONTENT -----
	// ---------------------------
	private:
		std::vector<T> Container;																					// d-Heap warehouse where item are stored according to their prorities; highest priority item is at the very first position
		std::unordered_map<T, ItemPositionInPriorityQueue, THash, TEqual> T2PositionInContainer;					// Positions of each item in the queue
		typedef typename std::unordered_map<T, ItemPositionInPriorityQueue, THash, TEqual>::iterator QueueIterator;	// Easy typing
		TComparisonPredicate HigherPriority;																		// Comparing the priority of two items in the queue
		ItemPositionInPriorityQueue depth;																			// What is the maximum number of children per node? Set when a queue is declared; once set, cannot be modified.

		// ----- Return the smallest of 2 item positions
		/* inline */ ItemPositionInPriorityQueue min(ItemPositionInPriorityQueue ip1, ItemPositionInPriorityQueue ip2) const
		{
			return (ip1 < ip2) ? ip1 : ip2;
		}

		// ----- Return the position of the parent of the item at the position i in the queue
		/* inline */ ItemPositionInPriorityQueue parent(ItemPositionInPriorityQueue i) const
		{
			// Precondition: i > 0 and depth > 0
			if (TOOLS::INCLUDE_ASSERT) assert(depth > 0 && i >= 0);
			return static_cast<ItemPositionInPriorityQueue>((i - 1) / depth);
		}

		// ----- Return the position of the child of the item at the position i in the queue which has the highest priority
		/* inline */ ItemPositionInPriorityQueue GetMinimumChildPositionInTheQueue(const ItemPositionInPriorityQueue i) const
		{
			// Where should we look in the container?
			ItemPositionInPriorityQueue left = min(static_cast<ItemPositionInPriorityQueue>(Container.size() - 1), i * depth + 1);
			ItemPositionInPriorityQueue right = min(static_cast<ItemPositionInPriorityQueue>(Container.size() - 1), (i + 1) * depth);

			// Scan all items in the [left, right] interval of the container to find the position of the minimum child
			ItemPositionInPriorityQueue position_of_minimum_child = left;
			for (ItemPositionInPriorityQueue p = left + 1; p <= right; ++p)
				if (HigherPriority(Container[p], Container[position_of_minimum_child]))
					position_of_minimum_child = p;

			return position_of_minimum_child;
		}

		// ----- Swap two items in the queue
		void swap(const ItemPositionInPriorityQueue i, const ItemPositionInPriorityQueue j)
		{
			// Classical swapping of item i and item j
			T temp = Container[i];
			Container[i] = Container[j];
			Container[j] = temp;

			// Don't forget to update the dictionary to allow later access of the items just swapped
			T2PositionInContainer[Container[j]] = j;
			T2PositionInContainer[Container[i]] = i;
		}

		// ----- Move the item currently at position i in the queue to a higher priority position
		void MoveUpTheQueue(ItemPositionInPriorityQueue i)
		{
			// Keep swapping up the item initially at position i until its parent has higher priority
			while (0 < i)
			{
				ItemPositionInPriorityQueue higher_priority_position = parent(i);
				if (HigherPriority(Container[i], Container[higher_priority_position]))
				{
					swap(i, higher_priority_position);
					i = higher_priority_position;
				}
				else
					break;
			}
		}

		// ----- Move the item currently at position i in the queue to a lower priority position
		void MoveDownTheQueue(ItemPositionInPriorityQueue i)
		{
			while (true)
			{
				ItemPositionInPriorityQueue first_child = static_cast<ItemPositionInPriorityQueue>(i * depth + 1);
				if (first_child >= Container.size())
					break; // i has no children

				ItemPositionInPriorityQueue position_of_minimum_child = GetMinimumChildPositionInTheQueue(i);
				if (HigherPriority(Container[position_of_minimum_child], Container[i]))
				{
					swap(i, position_of_minimum_child);
					i = position_of_minimum_child;
				}
				else
					break;
			}
		}
	};
	///
	///////////////////////////////////////////////////////////////////////////////////////////////////////////////////
	///
}	/// namespace TOOLS ends here
	///
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///
///      END OF FILE
///
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////// PriorityQueue.h