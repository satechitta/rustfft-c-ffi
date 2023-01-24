#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

/// Each instance has std::sync::Arc after initialization.
struct RustFftC;

extern "C" {

/// FFI interface to initialize `RustFftC` is called by C/C++.
///
/// Returns a pointer of a `RustFftC` object which is initialized by given `fft_size`
/// If `is_ifft` boolean flag is true, the `RustFftC` is created to work an inverse FFT.
RustFftC *rustfft_new(uintptr_t fft_size, bool is_ifft);

/// FFI interface to delete `RustFftC` is called by C/C++.
///
/// a pointer of RustFftC object argument is deleted by calling the API.
void rustfft_delete(RustFftC *ptr);

/// FFI interface to compute FFT using a pointer of `RustFftC` set is called by C/C++.
///
/// `re_list` is a real number of complex, `im_list` is a imaginary number of complex.
/// C/C++ side is not requred a dedicated type represented complex number.
/// The both lists need to have `fft_size` elements.
/// The value computed FFT replaces `re_list` and `im_list` arguments.
void rustfft_run(RustFftC *ptr, float *re_list, float *im_list, uintptr_t fft_size);

} // extern "C"
