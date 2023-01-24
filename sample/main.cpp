#include "rustfft.h"

#include <math.h>

float compute_mae_complex_distance(int fft_size, float *re_list_a, float *im_list_a, float *re_list_b, float *im_list_b)
{
    float mae = 0.;
    for (int i = 0; i < fft_size; ++i)
    {
        mae += sqrtf(powf((re_list_a[i] - re_list_b[i]), 2) + powf((im_list_a[i] - im_list_b[i]), 2));
    }
    return mae / fft_size;
}

int main()
{
    const int fft_size = 256;
   
    float re_list[fft_size];
    float im_list[fft_size];
    float original_re_list[fft_size];
    float original_im_list[fft_size];
    for (int i = 0; i < fft_size; ++i)
    {
        re_list[i] = original_re_list[i] = sin(i);
        im_list[i] = original_im_list[i] = 0;
    }
    RustFftC *rustfft = rustfft_new(fft_size, false);
    rustfft_run(rustfft, re_list, im_list, fft_size);
    rustfft_delete(rustfft);
    RustFftC *irustfft = rustfft_new(fft_size, true);
    rustfft_run(irustfft, re_list, im_list, fft_size);
    rustfft_delete(irustfft);
    // normalize the results by scaling each element by 1/fft_size.
    for (int i = 0; i < fft_size; ++i)
    {
        re_list[i] /= fft_size;
        im_list[i] /= fft_size;
    }
    float mae_rustfft_fftw = compute_mae_complex_distance(fft_size, re_list, im_list, original_re_list, original_im_list);
    printf("fft_size %d, MAE |original complex list - complex list processed fft -> ifft| %f\n", fft_size, mae_rustfft_fftw);
    
    return 0;
}