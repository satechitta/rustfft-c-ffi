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

# for link fftw3
find_package(PkgConfig REQUIRED) 
pkg_check_modules(FFTW IMPORTED_TARGET REQUIRED fftw3)
include_directories(${FFTW_INCLUDE_DIRS})
link_directories(${FFTW_LIBRARY_DIRS})

# Add an executable
add_executable(test main.cpp)

# link the rust lib and fftw with c++ target
target_link_libraries(test PUBLIC rustfftcffi fftw3)