/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////// PriorityQueue.h
///
/// C++ 17 implementation of a d-ary heap priority queue
/// 
/// Copyright (c) 2023-2025 Eric Jacopin
/// 
/// Licensed under the Apache License, Version 2.0 (the "License")
/// 
/// =================================================================================================================== File Content
///
/// This file is divided as follows:
///	- File History					(Line 19)
/// - Inclusion of files			(Line 107)
///	- Namespace declaration			(Line 120)
///	- Class declaration				(Line 129)
///
///	- End of file					(line 399)
///
/// =================================================================================================================== File history
///
/// [Author, Created, Last Modification] = [Eric JACOPIN, 2023/07/29, 2025/12/18]
/// Version: 2.0.0
///
/// [v2.0.0] Major Release - Zig 0.15.2 Support & Generic Implementation ---------------------------------------------- 2025/12/18
///     - Zig implementation updated for Zig 0.15.2 compatibility with API changes
///     - Zig now fully generic: supports user-defined item types with HashContext(T) and Comparator(T)
///     - Added contains(const T& item) method for O(1) membership testing in C++
///     - Comprehensive test coverage: 20+ tests in Zig matching Rust implementation
///     - Zig can now be used as a dependency in other projects via build.zig.zon
///     - Enhanced error handling: removed unreachable from Zig error paths
///     - Added peek() alias and initCapacity() methods in Zig for API completeness
///     - All three implementations maintain synchronized version numbers and API parity
///
/// [v1.1.0] Enhanced Release - Three-Language Implementation ---------------------------------------------------------- 2025/09/26
///     - Complete Zig implementation added with full API parity
///     - All three languages (C++, Rust, Zig) now provide identical functionality
///     - Enhanced documentation with cross-language comparison and usage guides
///     - Synchronized version numbering across all implementations
///
/// [v1.0.0] Stable Release - Complete d-ary Priority Queue ----------------------------------------------------------- 2025/09/26
///     - Feature-complete implementation with decrease_priority() method
///     - Comprehensive test suite with 6 test categories covering all edge cases
///     - Production-ready with robust error handling and O(1) item lookup
///     - Cross-language API parity with Rust implementation
///     - Professional documentation with usage examples and design explanations
///
/// [DEV 5] Unified API with Rust ------------------------------------------------------------------------------------- 2025/09/25
///     - Unified API methods added so that both C++ and Rust implementations have the same method names:
///			- size() -> len()
///			- empty() -> is_empty()
///			- getd() -> d()
///			- put_string() -> to_string()
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
#include <string>			// Visibility for std::string and std::to_string
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
		using Position = size_t;  // Unified type alias for cross-language consistency

		// ----- Default constructor deleted; appropriate constructor must set the depth of the heap; see next two constructors
		PriorityQueue() = delete;

		// ----- All seasons constructor with mandatory setting of the depth of the heap
		PriorityQueue(const ItemPositionInPriorityQueue d) : depth(d)
		{
			// Nothing initialized but the depth of the heap
			if (TOOLS::INCLUDE_ASSERT) assert(d > 0);			// (d > 0) is a precondition for the d-ary heap to be valid
		}

		// ----- Cherry on the cake constructor setting the depth of the heap while inserting the very first item in the queue
		PriorityQueue(const ItemPositionInPriorityQueue d, T& t) : depth(d)
		{
			if (TOOLS::INCLUDE_ASSERT) assert(d > 0);			// (d > 0) is a precondition for the d-ary heap to be valid

			// Make t the very first item in the queue
			container.emplace_back(t);
			positions[container[0]] = 0;
		}

		/* inline */ ItemPositionInPriorityQueue getd() const { return depth; }
		/* inline */ ItemPositionInPriorityQueue size() const { return container.size(); }
		/* inline */ bool empty() const { return container.empty(); }

		/// Unified API methods for cross-language consistency
		/* inline */ size_t len() const { return container.size(); }
		/* inline */ bool is_empty() const { return container.empty(); }
		/* inline */ size_t d() const { return depth; }

		// ----- Check if an item with the given identity exists in the queue. O(1) time complexity.
		bool contains(const T& item) const { return positions.find(item) != positions.end(); }

		// ----- Return the highest priority item of the queue; calling this function on an empty container causes undefined behavior.
		typename std::vector<T>::const_reference front() const
		{
			return container.front();
		}

		// ----- Clear the internal data structures of the queue, optionally resetting the depth of the heap
		void clear(const std::optional<ItemPositionInPriorityQueue>& d = std::nullopt)
		{
			container.clear();
			positions.clear();
			if (d) depth = *d;
			if (TOOLS::INCLUDE_ASSERT) assert(depth > 0);		// (d > 0) is a precondition for the d-ary heap to be valid
		}

		// ----- Insert item t in the queue according to its priority
		void insert(const T& t)
		{
			// First insert item t at the very end of the queue container
			container.push_back(t);
			ItemPositionInPriorityQueue last_position_in_the_queue = static_cast<ItemPositionInPriorityQueue>(container.size() - 1);
			positions[container.back()] = last_position_in_the_queue;

			// Make item t crawl up to a position corresponding to its priority
			move_up(last_position_in_the_queue);
		}

		// ----- Move item at position i up in the queue according to its priority
		void increase_priority(ItemPositionInPriorityQueue i)
		{
			move_up(i);
		}

		// ----- Move item t up in the queue according to its priority
		void increase_priority(T& updated_item)
		{
			// Get the position of the item in the queue container
			QueueIterator it = positions.find(updated_item);		// Find the position of item t in the queue
			if (TOOLS::INCLUDE_ASSERT) assert(positions.end() != it);			// t is expected to be in the queue and this should not be included
			ItemPositionInPriorityQueue position_of_t_in_the_queue = it->second;			// Remember current-priority t's position in the queue

			// Erase the item with previous priority and immediately insert it (with new higher priority) as no item in the queue can be modified directly
			positions.erase(it->first);
			positions.insert({ updated_item, position_of_t_in_the_queue });

			// Update the priority of item t in the queue
			container[position_of_t_in_the_queue] = updated_item;

			// t in the queue now has a higher priority than before: move t up the queue
			move_up(position_of_t_in_the_queue);
		}

		// ----- Decrease the priority value of item t (increase its priority rank in min-heap)
		void decrease_priority(T& updated_item)
		{
			// Get the position of the item in the queue container
			QueueIterator it = positions.find(updated_item);		// Find the position of item t in the queue
			if (TOOLS::INCLUDE_ASSERT) assert(positions.end() != it);			// t is expected to be in the queue and this should not be included
			ItemPositionInPriorityQueue position_of_t_in_the_queue = it->second;			// Remember current-priority t's position in the queue

			// Erase the item with previous priority and immediately insert it (with new priority) as no item in the queue can be modified directly
			positions.erase(it->first);
			positions.insert({ updated_item, position_of_t_in_the_queue });

			// Update the priority of item t in the queue
			container[position_of_t_in_the_queue] = updated_item;

			// After priority update, the item may need to move up or down to maintain heap property
			// We need to check both directions since we don't know if priority actually decreased
			move_up(position_of_t_in_the_queue);
			move_down(position_of_t_in_the_queue);
		}

		// ----- Remove the highest priority item from the queue
		void pop()
		{
			// First exchange the very first (highest priority) item and the very last item in the queue container
			swap(0, static_cast<ItemPositionInPriorityQueue>(container.size() - 1));

			// Forget the position of the very first item
			positions.erase(container.back());

			// Eventually remove the (highest priority) item from the queue container
			container.pop_back();

			// IF the queue contained only one item THEN we're done!
			if (!container.empty())
				// Else adjust the position (in the queue container) of what was the very last item in the container and now is the very first item
				move_down(0);
		}

		// ----- Output the queue in a stream
		void put(std::ostream& os) const
		{
			os << "{";
			if (!container.empty())
			{
				for (auto it = container.begin(); it != container.end() - 1; ++it)
				{
					os << (*it) << ", ";
				}
				if (0 <= container.size() - 1)
					os << container[container.size() - 1];
			}
			os << "}";
		}

		/// Unified string output method for cross-language consistency
		std::string to_string() const
		{
			std::string result = "{";
			if (!container.empty())
			{
				for (auto it = container.begin(); it != container.end() - 1; ++it)
				{
					result += std::to_string(*it) + ", ";
				}
				if (0 <= container.size() - 1)
					result += std::to_string(container[container.size() - 1]);
			}
			result += "}";
			return result;
		}

	// ---------------------------
	// ----- PRIVATE CONTENT -----
	// ---------------------------
	private:
		std::vector<T> container;																					// d-ary heap warehouse where items are stored according to their priorities; highest priority item is at the very first position
		std::unordered_map<T, ItemPositionInPriorityQueue, THash, TEqual> positions;								// Positions of each item in the queue
		typedef typename std::unordered_map<T, ItemPositionInPriorityQueue, THash, TEqual>::iterator QueueIterator;	// Easy typing
		TComparisonPredicate comparator;																			// Comparing the priority of two items in the queue
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
		/* inline */ ItemPositionInPriorityQueue best_child_position(const ItemPositionInPriorityQueue i) const
		{
			// Where should we look in the container?
			ItemPositionInPriorityQueue left = min(static_cast<ItemPositionInPriorityQueue>(container.size() - 1), i * depth + 1);
			ItemPositionInPriorityQueue right = min(static_cast<ItemPositionInPriorityQueue>(container.size() - 1), (i + 1) * depth);

			// Scan all items in the [left, right] interval of the container to find the position of the minimum child
			ItemPositionInPriorityQueue position_of_minimum_child = left;
			for (ItemPositionInPriorityQueue p = left + 1; p <= right; ++p)
				if (comparator(container[p], container[position_of_minimum_child]))
					position_of_minimum_child = p;

			return position_of_minimum_child;
		}

		// ----- Swap two items in the queue
		void swap(const ItemPositionInPriorityQueue i, const ItemPositionInPriorityQueue j)
		{
			// Classical swapping of item i and item j
			T temp = container[i];
			container[i] = container[j];
			container[j] = temp;

			// Don't forget to update the dictionary to allow later access of the items just swapped
			positions[container[j]] = j;
			positions[container[i]] = i;
		}

		// ----- Move the item currently at position i in the queue to a higher priority position
		void move_up(ItemPositionInPriorityQueue i)
		{
			// Keep swapping up the item initially at position i until its parent has higher priority
			while (0 < i)
			{
				ItemPositionInPriorityQueue higher_priority_position = parent(i);
				if (comparator(container[i], container[higher_priority_position]))
				{
					swap(i, higher_priority_position);
					i = higher_priority_position;
				}
				else
					break;
			}
		}

		// ----- Move the item currently at position i in the queue to a lower priority position
		void move_down(ItemPositionInPriorityQueue i)
		{
			while (true)
			{
				ItemPositionInPriorityQueue first_child = static_cast<ItemPositionInPriorityQueue>(i * depth + 1);
				if (first_child >= container.size())
					break; // i has no children

				ItemPositionInPriorityQueue position_of_minimum_child = best_child_position(i);
				if (comparator(container[position_of_minimum_child], container[i]))
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