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
/// - Inclusion of files			(Line 58)
///	- Namespace declaration			(Line 70)
///	- Class declaration				(Line 79)
///
///	- End of file					(line 293)
///
/// =================================================================================================================== File history
///
/// [Author, Created, Last Modification] = [Eric JACOPIN, 2023/07/29, 2025/06/18]
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
///				- O(1) for finding the item with highest priority
///				- O(d x ln(n) / ln(d)) for deleting the item with highest priority
///				- O(ln(d) / ln(d)) for inserting an item
///				- O(ln(d) / ln(d)) for updating the queue when the priority of an item increases
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
				typename THash = std::hash<size_t>,				/* O(1) access to positions */
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
		}

		// ----- Cherry on the cake constructor setting the depth of the heap while inserting the very first item in the queue
		PriorityQueue(const ItemPositionInPriorityQueue d, T& t) : depth(d)
		{
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
			return static_cast<ItemPositionInPriorityQueue>(std::floor(static_cast<float>(i - 1) / static_cast<float>(depth)));
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
			// Keep swapping down the item initially at position i until its minimum child has lower priority
			while (i <= parent(static_cast<ItemPositionInPriorityQueue>(Container.size() - 1)))
			{
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