use rustfft::{num_complex::Complex, Fft, FftPlanner};
use std::ptr;
use std::slice;
use std::sync::Arc;

pub struct RustFftCFfi {
    fft: Arc<dyn Fft<f32>>,
}
impl RustFftCFfi {
    fn new(num: usize, is_ifft: bool) -> Self {
        let mut planner: FftPlanner<f32> = FftPlanner::new();
        let fft: Arc<dyn Fft<f32>> = if is_ifft == false {
            planner.plan_fft_forward(num)
        } else {
            planner.plan_fft_inverse(num)
        };
        RustFftCFfi { fft: fft }
    }

    fn run(&mut self, buffer: &mut Vec<Complex<f32>>) {
        self.fft.process(buffer);
    }
}

#[no_mangle]
pub extern "C" fn rustfft_new(len: usize, is_ifft: bool) -> *mut RustFftCFfi {
    Box::into_raw(Box::new(RustFftCFfi::new(len, is_ifft)))
}

#[no_mangle]
pub extern "C" fn rustfft_free(ptr: *mut RustFftCFfi) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn rustfft_run(
    ptr: *mut RustFftCFfi,
    re_list: *mut f32,
    im_list: *mut f32,
    len: usize,
) {
    let rustfft_c_ffi: &mut RustFftCFfi = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let mut _re_list: Vec<f32> = unsafe {
        assert!(!re_list.is_null());

        slice::from_raw_parts(re_list, len as usize).to_vec()
    };
    let mut _im_list: Vec<f32> = unsafe {
        assert!(!im_list.is_null());

        slice::from_raw_parts(im_list, len as usize).to_vec()
    };
    let mut buffer: Vec<Complex<f32>> = vec![
        Complex {
            re: 0.0f32,
            im: 0.0f32
        };
        len
    ];
    for (i, val) in buffer.iter_mut().enumerate() {
        val.re = _re_list[i];
        val.im = _im_list[i];
    }
    rustfft_c_ffi.run(&mut buffer);
    let mut ret_re_list: Vec<f32> = vec![0.0f32; len];
    let mut ret_im_list: Vec<f32> = vec![0.0f32; len];
    for (i, val) in buffer.iter().enumerate() {
        ret_re_list[i] = val.re;
        ret_im_list[i] = val.im;
    }
    let ret_re_list_ptr: *const f32 = ret_re_list[..].as_ptr();
    let ret_im_list_ptr: *const f32 = ret_im_list[..].as_ptr();
    unsafe {
        ptr::copy(ret_re_list_ptr, re_list, len);
        ptr::copy(ret_im_list_ptr, im_list, len);
    }
}

#[test]
fn test_rustfft_c_ffi() {
    let num = 256;
    let mut buffer = vec![
        Complex {
            re: 1.0f32,
            im: 0.0f32
        };
        num
    ];

    // run the c ffi rustfft code
    let rustfft_c_ffi = rustfft_new(num, false);
    let mut buffer_re = vec![0.0f32; num];
    let mut buffer_im = vec![0.0f32; num];
    for (i, val) in buffer.iter().enumerate() {
        buffer_re[i] = val.re;
        buffer_im[i] = val.im;
    }
    let mut buffer_re_ptr = buffer_re[..].as_mut_ptr(); // need mut in rustfft_run
    let mut buffer_im_ptr = buffer_im[..].as_mut_ptr(); // need mut in rustfft_run
    rustfft_run(rustfft_c_ffi, buffer_re_ptr, buffer_im_ptr, num);
    buffer_re = unsafe {
        assert!(!buffer_re_ptr.is_null());
        slice::from_raw_parts(buffer_re_ptr, num as usize).to_vec()
    };
    buffer_im = unsafe {
        assert!(!buffer_im_ptr.is_null());
        slice::from_raw_parts(buffer_im_ptr, num as usize).to_vec()
    };

    // run the original rustffi code
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(num);
    fft.process(&mut buffer);
    let mut original_fft_buffer_re = vec![0.0f32; num];
    let mut original_fft_buffer_im = vec![0.0f32; num];
    for (i, val) in buffer.iter().enumerate() {
        original_fft_buffer_re[i] = val.re;
        original_fft_buffer_im[i] = val.im;
    }

    // comparison between c ffi rustfft and original one
    for (i, ans) in original_fft_buffer_re.iter().enumerate() {
        assert_eq!(&buffer_re[i], ans);
    }
    for (i, ans) in original_fft_buffer_im.iter().enumerate() {
        assert_eq!(&buffer_im[i], ans);
    }
}
