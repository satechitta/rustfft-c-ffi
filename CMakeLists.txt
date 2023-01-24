# Set the minimum version of CMake that can be used
# To find the cmake version run
# $ cmake --version
cmake_minimum_required(VERSION 3.15)

# Set the project name
project(RustfftCSample LANGUAGES CXX)

############################################################
# Create a Rust library
############################################################
# Generate the static Rust library from Cargo.toml using corrosion
# the corrosion project: https://github.com/corrosion-rs/corrosion
find_package(Corrosion REQUIRED)
corrosion_import_crate(MANIFEST_PATH  ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml)

############################################################
# Add sub directories
############################################################
add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/sample)

# If -DTEST=ON, test case with comparison fftw is build.
if(TEST)
    message(" [test] comparison fftw build")
    add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/test)
endif()
