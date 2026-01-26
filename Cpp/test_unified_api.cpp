/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////// test_unified_api.cpp
///
/// Test suite for unified API methods in C++ d-ary heap priority queue
/// 
/// Copyright (c) 2023-2026 Eric Jacopin
/// 
/// Licensed under the Apache License, Version 2.0 (the "License")
/// 
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "PriorityQueue.h"
#include <cassert>
#include <iostream>
#include <sstream>
#include <string>

using namespace TOOLS;

void test_unified_api_methods() {
    std::cout << "Testing unified API methods..." << std::endl;
    
    // Create a priority queue with d=3
    PriorityQueue<int> pq(3);
    
    // Test len() method (unified API)
    assert(pq.len() == 0);
    std::cout << "✓ len() method works correctly" << std::endl;
    
    // Test is_empty() method (unified API)
    assert(pq.is_empty() == true);
    std::cout << "✓ is_empty() method works correctly" << std::endl;
    
    // Test d() method (unified API)
    assert(pq.d() == 3);
    std::cout << "✓ d() method works correctly" << std::endl;
    
    // Insert some items
    pq.insert(10);
    pq.insert(5);
    pq.insert(15);
    
    // Test len() after insertions
    assert(pq.len() == 3);
    std::cout << "✓ len() returns correct size after insertions" << std::endl;
    
    // Test is_empty() after insertions
    assert(pq.is_empty() == false);
    std::cout << "✓ is_empty() returns false after insertions" << std::endl;
    
    // Test to_string() method (unified API)
    std::string output = pq.to_string();
    assert(output.front() == '{');
    assert(output.back() == '}');
    std::cout << "✓ to_string() method works correctly: " << output << std::endl;
}

void test_position_type_alias() {
    std::cout << "\nTesting Position type alias..." << std::endl;

    PriorityQueue<int> pq(2);
    pq.insert(10);

    // Test that Position type alias works - use the safe std::expected version
    PriorityQueue<int>::Position pos = 0;
    auto result = pq.increase_priority_by_index(pos);
    assert(result.has_value());

    assert(pq.len() == 1);
    std::cout << "✓ Position type alias works correctly" << std::endl;
}

void test_parameter_naming_consistency() {
    std::cout << "\nTesting parameter naming consistency..." << std::endl;

    PriorityQueue<int> pq(2);
    pq.insert(10);
    pq.insert(20);

    // Test updated parameter name (updated_item instead of t_with_new_higher_priority)
    // Use the safe std::expected version to avoid ambiguity with int type
    auto result = pq.increase_priority_by_index(0);  // Move item at position 0 up
    assert(result.has_value());

    assert(pq.len() == 2);
    std::cout << "✓ Parameter naming consistency works correctly" << std::endl;
}

void test_backward_compatibility() {
    std::cout << "\nTesting backward compatibility..." << std::endl;
    
    PriorityQueue<int> pq(3);
    pq.insert(10);
    pq.insert(5);
    pq.insert(15);
    
    // Test that old methods still work
    assert(pq.size() == 3);
    assert(pq.empty() == false);
    std::cout << "✓ Backward compatibility maintained for size() and empty()" << std::endl;
    
    // Test that both old and new methods return the same values
    assert(pq.size() == pq.len());
    assert(pq.empty() == pq.is_empty());
    std::cout << "✓ Old and new methods return consistent values" << std::endl;
}

void test_internal_consistency() {
    std::cout << "\nTesting internal consistency after refactoring..." << std::endl;
    
    PriorityQueue<int> pq(2);
    
    // Test basic operations still work after internal renamings
    pq.insert(20);
    pq.insert(10);
    pq.insert(30);
    pq.insert(5);
    
    assert(pq.front() == 5);  // Min heap behavior
    std::cout << "✓ Min heap property maintained after internal refactoring" << std::endl;

    // Test priority increase using position-based method (safe version)
    auto result = pq.increase_priority_by_index(0);  // Move item at position 0 up
    assert(result.has_value());
    assert(pq.front() == 5);  // Should still be 5 (minimum)
    std::cout << "✓ Priority increase works correctly after refactoring" << std::endl;
    
    // Test pop operation
    int old_front = pq.front();
    pq.pop();
    int new_front = pq.front();
    assert(new_front >= old_front);  // New front should be >= old front (min heap property)
    std::cout << "✓ Pop operation works correctly after refactoring" << std::endl;
    
    // Test clear operation
    pq.clear();
    assert(pq.is_empty());
    assert(pq.len() == 0);
    std::cout << "✓ Clear operation works correctly after refactoring" << std::endl;
}

void test_string_output_consistency() {
    std::cout << "\nTesting string output consistency..." << std::endl;
    
    PriorityQueue<int> pq(2);
    pq.insert(1);
    pq.insert(2);
    pq.insert(3);
    
    // Test that to_string() and put() produce similar output format
    std::string str_output = pq.to_string();
    
    std::ostringstream oss;
    pq.put(oss);
    std::string stream_output = oss.str();
    
    // Both should start with { and end with }
    assert(str_output.front() == '{' && str_output.back() == '}');
    assert(stream_output.front() == '{' && stream_output.back() == '}');
    
    std::cout << "✓ String output methods produce consistent format" << std::endl;
    std::cout << "  to_string(): " << str_output << std::endl;
    std::cout << "  put():       " << stream_output << std::endl;
}

int main() {
    std::cout << "=== C++ Unified API Test Suite ===" << std::endl;
    
    try {
        test_unified_api_methods();
        test_position_type_alias();
        test_parameter_naming_consistency();
        test_backward_compatibility();
        test_internal_consistency();
        test_string_output_consistency();
        
        std::cout << "\n🎉 All tests passed! Unified API implementation successful." << std::endl;
        return 0;
    } catch (const std::exception& e) {
        std::cerr << "❌ Test failed with exception: " << e.what() << std::endl;
        return 1;
    } catch (...) {
        std::cerr << "❌ Test failed with unknown exception" << std::endl;
        return 1;
    }
}
