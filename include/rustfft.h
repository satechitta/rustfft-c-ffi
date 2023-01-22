#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct RustFftC;

extern "C" {

RustFftC *rustfft_new(uintptr_t len, bool is_ifft);

void rustfft_free(RustFftC *ptr);

void rustfft_run(RustFftC *ptr, float *re_list, float *im_list, uintptr_t len);

} // extern "C"
