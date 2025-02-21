cmake_minimum_required(VERSION 3.20)

project(leetcode)

set(CMAKE_CXX_STANDARD 20)

file(GLOB SOURCES "src/solutions/*.cpp")
file(GLOB TEST_SOURCES "tests/*.cpp")

add_executable(tests ${SOURCES} ${TEST_SOURCES})

find_package(GTest REQUIRED)
include_directories(${GTEST_INCLUDE_DIRS})

target_link_libraries(tests ${GTEST_LIBRARIES} pthread)

include(GoogleTest)
gtest_discover_tests(tests)
