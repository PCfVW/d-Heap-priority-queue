/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////// PriorityQueue.h
///
/// C++ 23 implementation of a d-ary heap priority queue
///
/// Copyright (c) 2023-2026 Eric Jacopin
///
/// Licensed under the Apache License, Version 2.0 (the "License")
///
/// =================================================================================================================== File Content
///
/// This file is divided as follows:
///	- File History					(Line 21)
/// - Inclusion of files			(Line 163)
///	- Namespace declaration			(Line 179)
///	- Error enum declaration		(Line 188)
///	- Class declaration				(Line 225)
///
///	- End of file					(Line 797)
///
/// =================================================================================================================== File history
///
/// [Author, Created, Last Modification] = [Eric JACOPIN, 2023/07/29, 2026/01/24]
/// Version: 2.5.0
///
/// [v2.5.0] API Completeness & C++23 Modernization ------------------------------------------------------------------- 2026/01/24
///     - C++23 error handling: std::expected for safe, expressive error propagation
///     - New Error enum: InvalidArity, ItemNotFound, IndexOutOfBounds, EmptyQueue
///     - New peek() method: safe std::optional<T> alternative to front() with UB on empty
///     - New get_position() method: O(1) position lookup returning std::optional<Position>
///     - New decrease_priority_by_index() method: index-based priority decrease
///     - New update_priority() / update_priority_by_index(): bidirectional priority updates
///     - New bulk operations: insert_many() with Floyd's O(n) heapify, pop_many()
///     - New to_array() method: cross-language parity for heap contents export
///     - Fixed const correctness: increase_priority(const T&) and decrease_priority(const T&)
///     - Comprehensive test suite: edge cases, multiple arities, stress tests
///
/// [v2.4.0] Interactive Demo & Zig Bulk Operations ------------------------------------------------------------------- 2026/01/07
///     - Interactive React Flow demo: live visualization of d-ary heaps with Dijkstra's algorithm
///     - TypeScript instrumentation: opt-in comparison counting for performance analysis
///     - Zig bulk operations: insertMany(), popMany(), toArray() with Floyd's O(n) heapify
///     - Zig error handling: proper error propagation in swapItems() and pop()
///     - Go refinements: Position type used internally, Increase_priority_by_index() alias
///
/// [v2.3.0] Go Implementation Release -------------------------------------------------------------------------------- 2025/12/27
///     - Added complete Go implementation with full API parity (fifth language)
///     - Go generics support: PriorityQueue[T any, K comparable] with Comparator and KeyExtractor
///     - Go Dijkstra example demonstrating d-heap usage in examples/dijkstra/Go/
///     - Comparator utilities: MinBy(), MaxBy() factory functions and pre-built comparators
///     - Bulk operations: InsertMany(), PopMany() with Floyd's O(n) heapify algorithm
///     - Cross-language aliases: Snake_case method aliases (Is_empty(), Increase_priority(), etc.)
///     - Go workspace configuration with go.work for multi-module development
///     - 47 test cases covering all functionality in Go implementation
///     - All five implementations now maintain synchronized version numbers and API parity
///     - Updated Rust package name to `d-ary-heap` for clarity and consistency
///     - Standardized Rust library name to `d_ary_heap` throughout the codebase
///
/// [v2.2.0] Examples Infrastructure + TypeScript Dijkstra Release ---------------------------------------------------- 2025/12/26
///     - Added examples/dijkstra/ infrastructure with Network Flows textbook example (Figure 4.7, page 110)
///     - Implemented complete TypeScript Dijkstra example with path reconstruction and performance comparisons
///     - Added shared test graph in JSON format for cross-language compatibility
///     - Enhanced documentation with algorithm explanation and complexity analysis
///     - Demonstrated d-ary heap performance advantages (d=2 vs d=4 vs d=8) in real algorithm
///
/// [v2.1.2] Documentation & Compatibility Release -------------------------------------------------------------------- 2025/12/25
///     - Fixed misleading unified API documentation: now accurately documents per-language method names
///     - Added comprehensive error handling documentation with best practices for each language
///     - Added return type variations guide with safety recommendations across all implementations
///     - Enhanced cross-language compatibility: Zig now provides to_string() alias for consistency
///     - Resolved all critical issues identified in comprehensive API audit
///     - Added TypeScript implementation to npm registry as d-ary-heap@2.1.2
///     - Updated installation guide with TypeScript instructions and cross-language compatibility notes
///     - Maintained full backward compatibility across all implementations
///
/// [v2.1.1] TypeScript Tooling Release ------------------------------------------------------------------------------- 2025/12/18
///     - Added ESLint configuration and linting support for TypeScript implementation
///     - Fixed module type configuration for better Node.js compatibility
///     - Complete NPM publishing setup with automated build pipeline
///
/// [v2.0.0] Major Release - Zig 0.15.2 Support & Generic Implementation ---------------------------------------------- 2025/12/18
///     - Zig implementation updated for Zig 0.15.2 compatibility with API changes
///     - Zig now fully generic: supports user-defined item types with HashContext(T) and Comparator(T)
///     - Added contains(const T& item) method for O(1) membership testing in C++
///     - Comprehensive test coverage: 20+ tests in Zig matching Rust implementation
///     - Zig can now be used as a dependency in other projects via build.zig.zon
///     - Enhanced error handling: removed unreachable from Zig error paths
///     - Added peek() alias and initCapacity() methods in Zig for API completeness
///     - All four implementations maintain synchronized version numbers and API parity
///
/// [v1.1.0] Enhanced Release - Three-Language Implementation --------------------------------------------------------- 2025/09/26
///     - Complete Zig implementation added with full API parity
///     - All three languages (C++, Rust, Zig) now provide identical functionality
///     - Enhanced documentation with cross-language comparison and usage guides
///     - Synchronized version numbering across all three implementations
///
/// [v1.0.0] Stable Release - Complete d-ary Priority Queue ----------------------------------------------------------- 2025/09/26
///     - Feature-complete implementation with decrease_priority() method
///     - Comprehensive test suite with 6 test categories covering all edge cases
///     - Production-ready with robust error handling and O(1) item lookup
///     - Cross-language API parity with Rust implementation
///     - Professional documentation with usage examples and design explanations
///
/// [DEV 5] Cross-Language API Consistency ---------------------------------------------------------------------------- 2025/09/25
///     - Cross-language API methods added for consistency across C++, Rust, Zig, and TypeScript:
///			- size() -> len() (unified across all languages)
///			- empty() -> is_empty() (C++/Rust snake_case, Zig/TypeScript camelCase as isEmpty())
///			- getd() -> d() (unified across all languages)
///			- put_string() -> to_string() (C++/Rust snake_case, Zig/TypeScript camelCase as toString())
///     - Note: Method names follow language conventions (snake_case vs camelCase) with compatibility aliases
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
///		- This C++ 23 file
///				- d-Heap is both made of an array of the items and a dictionary of their positions in the queue
///				- d is set once for all when declaring the queue
///				- std::less will implement min-cost priority queue (default)
///				- std::greater will implement max-cost priority queue
///				- The position of an item can be dynamically updated in the queue according to its priority 
///				- Only the highest priority item can be removed from the queue
///				- The content of the array supporting the priority queue can be exported to a stream
///
///		- Reference: Section A.3, d-Heaps, pp. 773-778 of Ravindra Ahuja, Thomas Magnanti & James Orlin, Network Flows,
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
#include <concepts>         // Visibility for std::convertible_to, std::ranges::range (C++20)
#include <expected>         // Visibility for std::expected (C++23)
#include <iostream>         // Visibility for std::ostream
#include <optional>         // Visibility for std::optional
#include <sstream>          // Visibility for std::ostringstream (generic to_string)
#include <string>           // Visibility for std::string and std::to_string
#include <unordered_map>    // Visibility for std::unordered_map implementing the dictionary of item positions
#include <vector>           // Visibility for std::vector implementing the queue itself
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
	/////////////////////////////////////////////////////////////////////////////////////////////////////////////////// Error enum declaration
	///
	/// Error types for d-ary heap operations.
	///
	/// Cross-language equivalents:
	///   - Rust: enum Error { InvalidArity, ItemNotFound, IndexOutOfBounds, EmptyQueue }
	///   - Go: ErrEmptyQueue, ErrItemNotFound
	///   - Zig: error.DepthMustBePositive, error.ItemNotFound, error.IndexOutOfBounds
	///   - TypeScript: throws Error with messages
	///
	enum class Error
	{
		InvalidArity,		// Arity (d) must be >= 1
		ItemNotFound,		// Item not found in the priority queue
		IndexOutOfBounds,	// Index is out of bounds
		EmptyQueue			// Operation requires a non-empty queue
	};

	/// String representation of Error enum for diagnostics
	inline std::string to_string(Error e)
	{
		switch (e)
		{
			case Error::InvalidArity:    return "Heap arity (d) must be >= 1";
			case Error::ItemNotFound:    return "Item not found";
			case Error::IndexOutOfBounds: return "Index out of bounds";
			case Error::EmptyQueue:      return "Operation called on empty priority queue";
			default:                      return "Unknown error";
		}
	}

	/// Stream output for Error enum
	inline std::ostream& operator<<(std::ostream& os, Error e)
	{
		return os << to_string(e);
	}
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

		/// Factory function for safe construction with std::expected error handling.
		/// Returns Error::InvalidArity if d == 0.
		///
		/// Cross-language equivalents:
		///   - Rust: new(d, comparator) -> Result<Self, Error>
		///   - Go: New(d, comparator) -> (*T, error)
		///   - Zig: init(d, ...) returns !DHeap or error
		[[nodiscard]] static std::expected<PriorityQueue, Error> create(ItemPositionInPriorityQueue d)
		{
			if (d == 0) return std::unexpected(Error::InvalidArity);
			return PriorityQueue(d);
		}

		/// Factory function for safe construction with first item and std::expected error handling.
		[[nodiscard]] static std::expected<PriorityQueue, Error> create_with_first(ItemPositionInPriorityQueue d, T first_item)
		{
			if (d == 0) return std::unexpected(Error::InvalidArity);
			return PriorityQueue(d, first_item);
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

		/// Safe alternative to front() that returns std::optional<T> instead of causing UB on empty queue.
		/// Time complexity: O(1)
		///
		/// Cross-language equivalents:
		///   - Rust: peek() -> Option<&T>
		///   - Zig: front() -> ?T
		///   - TypeScript: peek() -> T | undefined
		///   - Go: Peek() -> (T, bool)
		std::optional<T> peek() const
		{
			if (container.empty()) return std::nullopt;
			return container.front();
		}

		/// Get the position (index) of an item in the heap. O(1) time complexity.
		/// Returns std::nullopt if the item is not found.
		///
		/// Cross-language equivalents:
		///   - Rust: get_position(&item) -> Option<Position>
		///   - Zig: getPosition(item) -> ?usize
		///   - TypeScript: getPosition(item) -> Position | undefined
		///   - Go: GetPosition(item) -> (int, bool)
		std::optional<Position> get_position(const T& item) const
		{
			auto it = positions.find(item);
			if (it == positions.end()) return std::nullopt;
			return static_cast<Position>(it->second);
		}

		// ----- Clear the internal data structures of the queue, optionally resetting the depth of the heap
		void clear(const std::optional<ItemPositionInPriorityQueue>& d = std::nullopt)
		{
			container.clear();
			positions.clear();
			if (d) depth = *d;
			if (TOOLS::INCLUDE_ASSERT) assert(depth > 0);		// (d > 0) is a precondition for the d-ary heap to be valid
		}

		/// Clear with safe std::expected error handling for arity validation.
		/// Returns Error::InvalidArity if new_d is 0.
		///
		/// Cross-language equivalents:
		///   - Rust: clear(Option<usize>) -> Result<(), Error>
		///   - Zig: clear(?usize) returns !void
		///   - TypeScript: clear(newD?) throws Error
		///   - Go: Clear(d) returns error
		[[nodiscard]] std::expected<void, Error> try_clear(const std::optional<ItemPositionInPriorityQueue>& new_d = std::nullopt)
		{
			if (new_d && *new_d == 0) return std::unexpected(Error::InvalidArity);
			container.clear();
			positions.clear();
			if (new_d) depth = *new_d;
			return {};
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

		/// Insert multiple items into the heap using Floyd's heapify algorithm.
		/// This is more efficient than inserting items one at a time: O(n) vs O(n log n).
		/// Time complexity: O(n) where n is the number of items being inserted
		///
		/// Cross-language equivalents:
		///   - Rust: insert_many(items)
		///   - Zig: insertMany(items)
		///   - TypeScript: insertMany(items)
		///   - Go: InsertMany(items)
		template <typename InputIt>
		void insert_many(InputIt first, InputIt last)
		{
			if (first == last) return;

			size_t start_idx = container.size();

			// Add all items to container and positions map
			for (auto it = first; it != last; ++it)
			{
				container.push_back(*it);
				positions[*it] = container.size() - 1;
			}

			// Use Floyd's heapify: O(n) instead of O(n log n)
			if (container.size() > 1)
			{
				// Last non-leaf node: parent of last element
				ItemPositionInPriorityQueue last_non_leaf = static_cast<ItemPositionInPriorityQueue>((container.size() - 2) / depth);
				for (ItemPositionInPriorityQueue i = last_non_leaf + 1; i-- > 0; )
				{
					move_down(i);
				}
			}
		}

		/// Insert multiple items from an initializer list. Time complexity: O(n)
		void insert_many(std::initializer_list<T> items)
		{
			insert_many(items.begin(), items.end());
		}

		/// Insert multiple items from a vector. Time complexity: O(n)
		void insert_many(const std::vector<T>& items)
		{
			insert_many(items.begin(), items.end());
		}

		// ===== INCREASE PRIORITY OPERATIONS =====

		/// Increase priority of item at specified index (moves up if needed). Time complexity: O(log_d n)
		/// This version uses assert for backward compatibility.
		void increase_priority(ItemPositionInPriorityQueue i)
		{
			if (TOOLS::INCLUDE_ASSERT) assert(i < container.size());
			move_up(i);
		}

		/// Increase priority of item at specified index with std::expected error handling. Time complexity: O(log_d n)
		///
		/// Cross-language equivalents:
		///   - Rust: increase_priority_by_index(index) -> Result<(), Error>
		///   - Zig: increasePriorityByIndex(index) returns !void
		///   - TypeScript: increasePriorityByIndex(index) throws Error
		///   - Go: IncreasePriorityByIndex(index) returns error
		[[nodiscard]] std::expected<void, Error> increase_priority_by_index(Position i)
		{
			if (i >= container.size()) return std::unexpected(Error::IndexOutOfBounds);
			move_up(static_cast<ItemPositionInPriorityQueue>(i));
			return {};
		}

		/// Increase priority of existing item (moves toward root if needed). Time complexity: O(log_d n)
		/// This version uses assert for backward compatibility.
		void increase_priority(const T& updated_item)
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

		/// Increase priority of existing item with std::expected error handling. Time complexity: O(log_d n)
		///
		/// Cross-language equivalents:
		///   - Rust: increase_priority(&item) -> Result<(), Error>
		///   - Zig: increasePriority(item) returns !void
		///   - TypeScript: increasePriority(item) throws Error
		///   - Go: IncreasePriority(item) returns error
		[[nodiscard]] std::expected<void, Error> try_increase_priority(const T& updated_item)
		{
			auto it = positions.find(updated_item);
			if (it == positions.end()) return std::unexpected(Error::ItemNotFound);

			ItemPositionInPriorityQueue pos = it->second;
			positions.erase(it->first);
			positions.insert({ updated_item, pos });
			container[pos] = updated_item;
			move_up(pos);
			return {};
		}

		// ===== DECREASE PRIORITY OPERATIONS =====

		/// Decrease priority of item at specified index (moves down if needed). Time complexity: O(d · log_d n)
		///
		/// Cross-language equivalents:
		///   - Rust: decrease_priority_by_index(index) -> Result<(), Error>
		///   - Zig: decreasePriorityByIndex(index) returns !void
		///   - TypeScript: decreasePriorityByIndex(index) throws Error
		///   - Go: DecreasePriorityByIndex(index) returns error
		[[nodiscard]] std::expected<void, Error> decrease_priority_by_index(Position i)
		{
			if (i >= container.size()) return std::unexpected(Error::IndexOutOfBounds);
			move_down(static_cast<ItemPositionInPriorityQueue>(i));
			return {};
		}

		/// Decrease priority of existing item (moves toward leaves if needed). Time complexity: O(d · log_d n)
		/// This version uses assert for backward compatibility. Only moves downward.
		void decrease_priority(const T& updated_item)
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

			// Item became less important: move down only
			move_down(position_of_t_in_the_queue);
		}

		/// Decrease priority of existing item with std::expected error handling. Time complexity: O(d · log_d n)
		///
		/// Cross-language equivalents:
		///   - Rust: decrease_priority(&item) -> Result<(), Error>
		///   - Zig: decreasePriority(item) returns !void
		///   - TypeScript: decreasePriority(item) throws Error
		///   - Go: DecreasePriority(item) returns error
		[[nodiscard]] std::expected<void, Error> try_decrease_priority(const T& updated_item)
		{
			auto it = positions.find(updated_item);
			if (it == positions.end()) return std::unexpected(Error::ItemNotFound);

			ItemPositionInPriorityQueue pos = it->second;
			positions.erase(it->first);
			positions.insert({ updated_item, pos });
			container[pos] = updated_item;
			move_down(pos);
			return {};
		}

		// ===== UPDATE PRIORITY OPERATIONS (bidirectional) =====

		/// Update priority of item at specified index (moves in correct direction). Time complexity: O((d+1) · log_d n)
		/// Use this when you don't know whether the item's priority increased or decreased.
		///
		/// Cross-language equivalents:
		///   - Rust: update_priority_by_index(index) -> Result<(), Error>
		///   - TypeScript: Not available (use updatePriority with item)
		[[nodiscard]] std::expected<void, Error> update_priority_by_index(Position i)
		{
			if (i >= container.size()) return std::unexpected(Error::IndexOutOfBounds);
			ItemPositionInPriorityQueue pos = static_cast<ItemPositionInPriorityQueue>(i);
			move_up(pos);
			move_down(pos);
			return {};
		}

		/// Update priority of existing item, moving it in the correct direction. Time complexity: O((d+1) · log_d n)
		/// Use this when you don't know whether the item's priority increased or decreased.
		void update_priority(const T& updated_item)
		{
			QueueIterator it = positions.find(updated_item);
			if (TOOLS::INCLUDE_ASSERT) assert(positions.end() != it);

			ItemPositionInPriorityQueue pos = it->second;
			positions.erase(it->first);
			positions.insert({ updated_item, pos });
			container[pos] = updated_item;
			move_up(pos);
			move_down(pos);
		}

		/// Update priority of existing item with std::expected error handling. Time complexity: O((d+1) · log_d n)
		///
		/// Cross-language equivalents:
		///   - Rust: update_priority(&item) -> Result<(), Error>
		///   - Zig: updatePriority(item) returns !void
		///   - TypeScript: updatePriority(item) throws Error
		///   - Go: UpdatePriority(item) returns error
		[[nodiscard]] std::expected<void, Error> try_update_priority(const T& updated_item)
		{
			auto it = positions.find(updated_item);
			if (it == positions.end()) return std::unexpected(Error::ItemNotFound);

			ItemPositionInPriorityQueue pos = it->second;
			positions.erase(it->first);
			positions.insert({ updated_item, pos });
			container[pos] = updated_item;
			move_up(pos);
			move_down(pos);
			return {};
		}

		// ===== POP OPERATIONS =====

		/// Remove the highest priority item from the queue. (Legacy: no return value)
		void pop()
		{
			if (container.empty()) return;

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

		/// Remove and return the highest priority item. Time complexity: O(d · log_d n)
		/// Returns std::nullopt if the queue is empty.
		///
		/// Cross-language equivalents:
		///   - Rust: pop() -> Option<T>
		///   - Zig: pop() -> ?T
		///   - TypeScript: pop() -> T | undefined
		///   - Go: Pop() -> (T, bool)
		std::optional<T> pop_front()
		{
			if (container.empty()) return std::nullopt;

			T result = container.front();
			swap(0, static_cast<ItemPositionInPriorityQueue>(container.size() - 1));
			positions.erase(container.back());
			container.pop_back();
			if (!container.empty()) move_down(0);
			return result;
		}

		/// Remove and return multiple highest-priority items. Time complexity: O(count · d · log_d n)
		/// Returns up to `count` items in priority order (highest priority first).
		///
		/// Cross-language equivalents:
		///   - Rust: pop_many(count) -> Vec<T>
		///   - Zig: popMany(count) -> []T
		///   - TypeScript: popMany(count) -> T[]
		///   - Go: PopMany(count) -> []T
		std::vector<T> pop_many(size_t count)
		{
			std::vector<T> result;
			size_t actual_count = std::min(count, container.size());
			result.reserve(actual_count);
			for (size_t i = 0; i < actual_count; ++i)
			{
				if (auto item = pop_front())
					result.push_back(std::move(*item));
			}
			return result;
		}

		// ----- Output the queue in a stream
		void put(std::ostream& os) const
		{
			os << "{";
			if (!container.empty())
			{
				for (size_t i = 0; i < container.size() - 1; ++i)
				{
					os << container[i] << ", ";
				}
				os << container.back();
			}
			os << "}";
		}

		/// Returns a copy of the heap contents as a vector. Time complexity: O(n)
		/// The root element (highest priority) is at index 0. The internal heap
		/// structure is preserved—this is NOT a sorted array.
		///
		/// Cross-language equivalents:
		///   - Rust: to_array() -> Vec<T>
		///   - Zig: toArray() -> []T
		///   - TypeScript: toArray() -> T[]
		///   - Go: ToArray() -> []T
		std::vector<T> to_array() const
		{
			return container;
		}

		/// Unified string output method for cross-language consistency.
		/// Works with types that support operator<< or std::to_string.
		///
		/// Cross-language equivalents:
		///   - Rust: to_string() / Display trait
		///   - Zig: toString() / to_string()
		///   - TypeScript: toString() / to_string()
		std::string to_string() const
		{
			std::ostringstream oss;
			oss << "{";
			for (size_t i = 0; i < container.size(); ++i)
			{
				if (i > 0) oss << ", ";
				oss << container[i];
			}
			oss << "}";
			return oss.str();
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
			if (TOOLS::INCLUDE_ASSERT) assert(depth > 0 && i > 0);
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