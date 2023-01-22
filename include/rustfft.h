#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct RustFftCFfi;

extern "C" {

RustFftCFfi *rustfft_new(uintptr_t len, bool is_ifft);

void rustfft_free(RustFftCFfi *ptr);

void rustfft_run(RustFftCFfi *ptr, float *re_list, float *im_list, uintptr_t len);

} // extern "C"
