#include "rustfft.h"

int main(){
    const int num = 256;
    RustFftCFfi* rustfft = rustfft_new(num, false);
    float re_list[num];
    float im_list[num];
    for( int i=0; i<num; ++i)
    {
        re_list[i] = 1;
        im_list[i] = 0;
    }
    rustfft_run(rustfft, re_list, im_list, num);
    return 0;
}