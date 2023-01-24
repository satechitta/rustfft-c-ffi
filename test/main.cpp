#include "rustfft.h"

#include <complex.h>
#include <fftw3.h>
#include <time.h>
#include <math.h>

float execute_rustfft(int fft_size, bool is_inv, float *re_list, float *im_list, bool computation_time_without_initialization)
{
    clock_t start_rustfft = clock();
    RustFftC *rustfft = rustfft_new(fft_size, is_inv);
    if (computation_time_without_initialization)
    {
        start_rustfft = clock();
    }
    rustfft_run(rustfft, re_list, im_list, fft_size);
    clock_t end_rustfft = clock();
    float time_rustfft = static_cast<float>(end_rustfft - start_rustfft) / CLOCKS_PER_SEC * 1.0; // sec

    rustfft_delete(rustfft);
    return time_rustfft;
}

float execute_fftw(int fft_size, bool is_inv, float *re_list, float *im_list, bool computation_time_without_initialization)
{
    fftw_complex *a, *b;
    a = (fftw_complex *)fftw_malloc(sizeof(fftw_complex) * fft_size);
    b = (fftw_complex *)fftw_malloc(sizeof(fftw_complex) * fft_size);
    for (int i = 0; i < fft_size; ++i)
    {
        a[i][0] = re_list[i];
        a[i][1] = im_list[i];
    }

    clock_t start_fftw = clock();
    fftw_plan plan;
    if (is_inv == false)
    {
        plan = fftw_plan_dft_1d(fft_size, a, b, FFTW_FORWARD, FFTW_ESTIMATE);
    }
    else
    {
        plan = fftw_plan_dft_1d(fft_size, a, b, FFTW_BACKWARD, FFTW_ESTIMATE);
    }

    if (computation_time_without_initialization)
    {
        start_fftw = clock();
    }
    fftw_execute(plan);
    clock_t end_fftw = clock();
    float time_fftw = static_cast<float>(end_fftw - start_fftw) / CLOCKS_PER_SEC * 1.0; // sec
    for (int i = 0; i < fft_size; ++i)
    {
        re_list[i] = b[i][0];
        im_list[i] = b[i][1];
    }
    fftw_destroy_plan(plan);
    fftw_free(a);
    fftw_free(b);

    return time_fftw;
}

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
    const int fft_size_list[] = {255, 256, 1023, 1024, 4095, 4096, 16383, 16384, 65535, 65536, 262143, 262144};
    const int fft_size_list_size = 12;

    for (int fft_size_list_i = 0; fft_size_list_i < fft_size_list_size; ++fft_size_list_i)
    {
        const int fft_size = fft_size_list[fft_size_list_i];
        float rustfft_re_list[fft_size];
        float rustfft_im_list[fft_size];
        float fftw_re_list[fft_size];
        float fftw_im_list[fft_size];
        for (int i = 0; i < fft_size; ++i)
        {
            rustfft_re_list[i] = fftw_re_list[i] = sin(i);
            rustfft_im_list[i] = fftw_im_list[i] = 0;
        }
        float computation_time_rustfft = execute_rustfft(fft_size, false, rustfft_re_list, rustfft_im_list, true);
        float computation_time_fftw = execute_fftw(fft_size, false, fftw_re_list, fftw_im_list, true);
        float mae_rustfft_fftw = compute_mae_complex_distance(fft_size, rustfft_re_list, rustfft_im_list, fftw_re_list, fftw_im_list);
        printf("fft size %d, rustfft %f, fftw %f, MAE %f\n", fft_size, computation_time_rustfft, computation_time_fftw, mae_rustfft_fftw);
    }
    return 0;
}