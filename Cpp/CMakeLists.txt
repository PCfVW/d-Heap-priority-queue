cmake_minimum_required(VERSION 3.25)
project(PriorityQueueTests LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Main demo executable
add_executable(main main.cpp)

# Comprehensive test suite
add_executable(test_comprehensive test_comprehensive.cpp)

# Unified API test
add_executable(test_unified_api test_unified_api.cpp)

# Decrease priority test
add_executable(test_decrease_priority test_decrease_priority.cpp)

# Phase 2 comparison-count instrumentation test
add_executable(test_instrumentation test_instrumentation.cpp)

# Enable testing
enable_testing()
add_test(NAME ComprehensiveTests COMMAND test_comprehensive)
add_test(NAME UnifiedAPITests COMMAND test_unified_api)
add_test(NAME DecreasePriorityTests COMMAND test_decrease_priority)
add_test(NAME InstrumentationTests COMMAND test_instrumentation)
