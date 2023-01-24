# Set the minimum version of CMake that can be used
# To find the cmake version run
# $ cmake --version
cmake_minimum_required(VERSION 3.15)

# Set the project name
project(${PROJECT_NAME} LANGUAGES CXX)
add_compile_options(-O2 -g)

############################################################
# Create an executable
############################################################
# for rustfft.h
set(RUST_ROOT_DIR ${CMAKE_CURRENT_SOURCE_DIR}/..)
include_directories(${RUST_ROOT_DIR}/include)

# Add an executable
add_executable(sample main.cpp)

# link the rust lib with c++ target
target_link_libraries(sample PUBLIC rustfftcffi)