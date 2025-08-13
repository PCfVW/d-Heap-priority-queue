/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////// main.cpp
///
/// C++ 17 Some code showing how to use the d-Heap priority queue template
///			to implement both minimum and maximum cost priority queues.
/// 
/// Copyright (c) 2023-2025 Eric Jacopin
/// 
/// Licensed under the Apache License, Version 2.0 (the "License")
/// 
/// =================================================================================================================== File Content
///
/// main.cpp
///
#include <iostream>
#include <unordered_map>
#include <vector>
#include <cassert>
#include <limits>

#include "PriorityQueue.h"

// Helper: print the priority queue followed by a newline
template <typename PQ>
inline void PrintPQ(const PQ& pq, std::ostream& os = std::cout)
{
    pq.put(os);
    os << '\n';
}

// Queue items
struct INT
{
	public:
		uint32_t number;	// This item is merely a 32-bit integer
		uint32_t cost;		// Holds the cost of this item

		INT() = default;
		INT(uint32_t i) : number(i), cost(i) {};
};
// Inserting queue items in an output stream (to print the queue content)
std::ostream& operator<<(std::ostream& os, const INT& i)
{
	os << "(" << i.number << "," << i.cost << ")";
	return os;
}

// Hashing queue items
template<typename Size> class INTHash
{
	public:
		std::size_t operator() (const INT& i) const noexcept
		{
			return std::hash<Size>()(i.number);
		}
};

// Comparing the priority (i.e. cost) of two queue items: which one has a lower cost?
struct INTLess
{
	public:
		bool operator() (const INT& lhs, const INT& rhs) const
		{
			return (lhs.cost < rhs.cost);
		}
};

// Comparing the priority (i.e. cost) of two queue items: which one has a higher cost?
struct INTGreater
{
	public:
		bool operator() (const INT& lhs, const INT& rhs) const
		{
			return (lhs.cost > rhs.cost);
		}
};

// Matching one item in the queue
struct INTEqual
{
	public:
		bool operator() (const INT& lhs, const INT& rhs) const
		{
			return (lhs.number == rhs.number);
		}
};

int main()
{
	/////////////////////////////////////////////////////////////////////////////////////////////////////
	///
	/// Declare a new queue whose priority is based on minimum cost
	///
	TOOLS::PriorityQueue<
							INT,				// item in the queue,
							INTHash<uint32_t>,	// Computing a number from an item
							INTLess,			// Is the priority of one item less than the priority of another item?
							INTEqual			// Is this item equal to another item?
						>
		MyPQ_Less(3);				// Depth of the heap == 3

	//
	// Here are a set of inputs for testing purposes
	//
	// const std::vector<int> input = { 5, 8, 9, 10, 12, 13, 14, 15, 16, 17, 18, 20, 21, 22, 27, 28, 29, 31, 32, 36, 38, 39, 41, 42, 48, 52 };
	//
	// const std::vector<int> input = { 41, 42, 48, 52 };

	const std::vector<int> input = { 20, 5, 22, 16, 18, 17, 12, 9, 42, 27, 48, 36, 32, 13, 14, 28, 52, 10, 21, 8, 39, 29, 15, 38, 31, 41 };

	// Insert items in the queue while printing the content of the queue
	for(size_t i = 0; i < input.size() ; ++i)
	{
		MyPQ_Less.insert(INT(input[i]));
		PrintPQ(MyPQ_Less);
	}

	// One more item to test the dynamic update of the priority of this item in the queue
	INT I1(19);
	MyPQ_Less.insert(I1);
	PrintPQ(MyPQ_Less);

	INT IFront = MyPQ_Less.front();
	std::cout << "front: " << IFront << '\n';

	// Increase the priority (== decreasing cost) of the item
	I1.cost = 6;
	// Testing dynamic update of the position of the item in the queue
	MyPQ_Less.increase_priority(I1);
	PrintPQ(MyPQ_Less);

	// Pop the highest priority item while printing the content of the queue
	// Verify non-decreasing order of popped costs (min-heap)
	bool first = true;
	uint32_t last_cost = 0;
	while (! MyPQ_Less.empty())
	{
		INT top = MyPQ_Less.front();
		if (!first) {
			assert(top.cost >= last_cost);
		} else {
			first = false;
		}
		last_cost = top.cost;
		MyPQ_Less.pop();
		PrintPQ(MyPQ_Less);
	}

	// Clear the priority queue and reset its depth to 6
	MyPQ_Less.clear(6);

	/////////////////////////////////////////////////////////////////////////////////////////////////////
	///
	/// Declare a new queue whose priority is based on maximum cost
	///
	TOOLS::PriorityQueue<
							INT,				// items in the queue,
							INTHash<uint32_t>,	// Computing a number from an item
							INTGreater,			// Is the priority of one item more than the priority of another item? */
							INTEqual			// Is this item equal to another item?
						>
		MyPQ_Greater(3);	// Depth of the heap == 3

	// Insert items in the queue while printing the content of the queue
	for (size_t i = 0; i < input.size(); ++i)
	{
		MyPQ_Greater.insert(INT(input[i]));
		PrintPQ(MyPQ_Greater);
	}

	// One more item to test the dynamic update of the priority of this item in the queue
	INT I2(40);
	MyPQ_Greater.insert(I2);
	PrintPQ(MyPQ_Greater);

	// Increase the priority (== increasing cost) of the item
	I2.cost = 50;
	// Testing dynamic update of the position of the item in the queue
	MyPQ_Greater.increase_priority(I2);
	PrintPQ(MyPQ_Greater);

	// Pop the highest priority item while printing the content of the queue
    // Verify non-increasing order of popped costs (max-heap)
    bool first_max = true;
    uint32_t last_cost_max = 0;
    while (!MyPQ_Greater.empty())
    {
        INT top = MyPQ_Greater.front();
        if (!first_max) {
            assert(top.cost <= last_cost_max);
        } else {
            first_max = false;
        }
        last_cost_max = top.cost;
        MyPQ_Greater.pop();
        PrintPQ(MyPQ_Greater);
    }


	/////////////////////////////////////////////////////////////////////////////////////////////////////
	///
	/// Sure. Everything went fine. ;)
	///
	return 0;
}
