use rustfft::{num_complex::Complex, Fft, FftPlanner};
use std::ptr;
use std::slice;
use std::sync::Arc;

/// Each instance has std::sync::Arc after initialization.
pub struct RustFftC {
    fft: Arc<dyn Fft<f32>>,
}
impl RustFftC {
    /// Constructs a new `RustFftC` instance.
    ///
    /// A new instance is initialized by given `fft_size` which is applicable both the power-of-two case and the other cases.  
    /// If `is_ifft` boolean flag is true, the instance is created to work an inverse FFT.
    fn new(fft_size: usize, is_ifft: bool) -> Self {
        let mut planner: FftPlanner<f32> = FftPlanner::new();
        let fft: Arc<dyn Fft<f32>> = if is_ifft == false {
            planner.plan_fft_forward(fft_size)
        } else {
            planner.plan_fft_inverse(fft_size)
        };
        RustFftC { fft: fft }
    }

    /// Returns computed FFT results.
    fn run(&mut self, buffer: &mut Vec<Complex<f32>>) {
        self.fft.process(buffer);
    }
}

/// FFI interface to initialize `RustFftC` is called by C/C++.
///
/// Returns a pointer of a `RustFftC` object which is initialized by given `fft_size`
/// If `is_ifft` boolean flag is true, the `RustFftC` is created to work an inverse FFT.
#[no_mangle]
pub extern "C" fn rustfft_new(fft_size: usize, is_ifft: bool) -> *mut RustFftC {
    Box::into_raw(Box::new(RustFftC::new(fft_size, is_ifft)))
}

/// FFI interface to delete `RustFftC` is called by C/C++.
///
/// a pointer of RustFftC object argument is deleted by calling the API.
#[no_mangle]
pub extern "C" fn rustfft_delete(ptr: *mut RustFftC) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(ptr);
    }
}

/// FFI interface to compute FFT using a pointer of `RustFftC` set is called by C/C++.
///
/// `re_list` is a real number of complex, `im_list` is a imaginary number of complex.
/// C/C++ side is not requred a dedicated type represented complex number.
/// The both lists need to have `fft_size` elements.
/// The value computed FFT replaces `re_list` and `im_list` arguments.
#[no_mangle]
pub extern "C" fn rustfft_run(
    ptr: *mut RustFftC,
    re_list: *mut f32,
    im_list: *mut f32,
    fft_size: usize,
) {
    let rustfft_c_ffi: &mut RustFftC = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let mut _re_list: Vec<f32> = unsafe {
        assert!(!re_list.is_null());

        slice::from_raw_parts(re_list, fft_size as usize).to_vec()
    };
    let mut _im_list: Vec<f32> = unsafe {
        assert!(!im_list.is_null());

        slice::from_raw_parts(im_list, fft_size as usize).to_vec()
    };
    let mut buffer: Vec<Complex<f32>> = vec![
        Complex {
            re: 0.0f32,
            im: 0.0f32
        };
        fft_size
    ];
    for (i, val) in buffer.iter_mut().enumerate() {
        val.re = _re_list[i];
        val.im = _im_list[i];
    }
    rustfft_c_ffi.run(&mut buffer);
    let mut ret_re_list: Vec<f32> = vec![0.0f32; fft_size];
    let mut ret_im_list: Vec<f32> = vec![0.0f32; fft_size];
    for (i, val) in buffer.iter().enumerate() {
        ret_re_list[i] = val.re;
        ret_im_list[i] = val.im;
    }
    let ret_re_list_ptr: *const f32 = ret_re_list[..].as_ptr();
    let ret_im_list_ptr: *const f32 = ret_im_list[..].as_ptr();
    unsafe {
        ptr::copy(ret_re_list_ptr, re_list, fft_size);
        ptr::copy(ret_im_list_ptr, im_list, fft_size);
    }
}

#[test]
fn test_rustfft_c_ffi() {
    let fft_size = 256;
    let mut buffer = vec![
        Complex {
            re: 1.0f32,
            im: 0.0f32
        };
        fft_size
    ];

    // run the c ffi rustfft code
    let rustfft_c_ffi = rustfft_new(fft_size, false);
    let mut buffer_re = vec![0.0f32; fft_size];
    let mut buffer_im = vec![0.0f32; fft_size];
    for (i, val) in buffer.iter().enumerate() {
        buffer_re[i] = val.re;
        buffer_im[i] = val.im;
    }
    let mut buffer_re_ptr = buffer_re[..].as_mut_ptr(); // need mut in rustfft_run
    let mut buffer_im_ptr = buffer_im[..].as_mut_ptr(); // need mut in rustfft_run
    rustfft_run(rustfft_c_ffi, buffer_re_ptr, buffer_im_ptr, fft_size);
    buffer_re = unsafe {
        assert!(!buffer_re_ptr.is_null());
        slice::from_raw_parts(buffer_re_ptr, fft_size as usize).to_vec()
    };
    buffer_im = unsafe {
        assert!(!buffer_im_ptr.is_null());
        slice::from_raw_parts(buffer_im_ptr, fft_size as usize).to_vec()
    };

    // run the original rustffi code
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(fft_size);
    fft.process(&mut buffer);
    let mut original_fft_buffer_re = vec![0.0f32; fft_size];
    let mut original_fft_buffer_im = vec![0.0f32; fft_size];
    for (i, val) in buffer.iter().enumerate() {
        original_fft_buffer_re[i] = val.re;
        original_fft_buffer_im[i] = val.im;
    }

    // compare between c ffi rustfft results and original one
    for (i, ans) in original_fft_buffer_re.iter().enumerate() {
        assert_eq!(&buffer_re[i], ans);
    }
    for (i, ans) in original_fft_buffer_im.iter().enumerate() {
        assert_eq!(&buffer_im[i], ans);
    }
}
