# RustFFT C FFI

This project is a C wrapper for the [`rustfft`](https://github.com/ejmahler/RustFFT) library, which provides efficient Fast Fourier Transform (FFT) operations.

## Motivation

[FFTW](https://www.fftw.org/) is a widely used library for performing FFT operations in C++, but its GPL license can pose challenges for use in commercial embedded systems. This project provides a C wrapper for the `rustfft` library, which is licensed under a more permissive MIT and Apache-2.0 license, making it easier to incorporate into a wider range of projects. This wrapper allows developers to use `rustfft` in C++ projects as an alternative to FFTW.


## Structure

The project is structured as follows:

- `src/`: Contains the Rust source code for the `rustfft` wrapper.
- `include/`: Contains the C header file for the `rustfft` wrapper.
- `sample/`: Contains a sample C++ application that uses the `rustfft` wrapper.
- `test/`: Contains tests for the `rustfft` wrapper, including comparisons with FFTW.

## Building

This project uses both Cargo (for the Rust code) and CMake (for the C++ code). To build the project, you can use the following commands:

```sh
cmake -B build -S . -DCMAKE_BUILD_TYPE=Release 
cmake --build build --config Release
```

This will produce a static library `librustfftcffi.a` in the `build/` directory.

To build the project, including the `test/`, you can use the following commands:

```sh
cmake -B build -S . -DCMAKE_BUILD_TYPE=Release -DTEST=ON 
cmake --build build --config Release
```

## Usage

To use the `rustfft` wrapper in your C++ code, include the `rustfft.h` header file and link against `librustfftcffi.a`.
For a detailed example of how to use the rustfft wrapper, refer to the sample application in sample/main.cpp.

To use the `rustfft` wrapper in your C++ code, follow these steps:

1. Include the `rustfft.h` header file in your source code:

```cpp
#include "rustfft.h"
```

2. Create an instance of `RustFftC` with the desired FFT size and direction (forward or inverse):

```cpp
const int fft_size = 256;
RustFftC *rustfft = rustfft_new(fft_size, false); // for forward FFT
RustFftC *irustfft = rustfft_new(fft_size, true); // for inverse FFT
```

3. Prepare your input data in arrays of real and imaginary parts:

```cpp
float re_list[fft_size];
float im_list[fft_size];
for (int i = 0; i < fft_size; ++i)
{
    re_list[i] = sin(i);
    im_list[i] = 0;
}
```

4. Run the FFT operation:

```cpp
rustfft_run(rustfft, re_list, im_list, fft_size);
```

5. Don't forget to delete the `RustFftC` instance when you're done:

```cpp
rustfft_delete(rustfft);
```

For a detailed example of how to use the rustfft wrapper, refer to the sample application in `sample/main.cpp`.

## Testing

To run the test, navigate to the `test/` directory.

This test include comparisons with FFTW to ensure the accuracy and performance of the rustfft wrapper.

## Dependencies

This project uses the following tools:

- [Corrosion](https://github.com/corrosion-rs/corrosion): This project that provides Rust and Cargo support for the CMake build system. This project uses Corrosion to integrate the Rust and CMake parts of the build process.

- [cbindgen](https://github.com/mozilla/cbindgen): A tool for generating C bindings for Rust code. This project uses cbindgen to generate the C header file for the `rustfft` wrapper from the Rust source code.
