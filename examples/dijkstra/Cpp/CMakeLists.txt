cmake_minimum_required(VERSION 3.20)
project(dijkstra_example CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(dijkstra main.cpp)

# Include path to the main Cpp directory for PriorityQueue.h
target_include_directories(dijkstra PRIVATE ${CMAKE_SOURCE_DIR}/../../..)
