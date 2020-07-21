//! Implementation of resizing image with opencv FFI
use std::slice;
use std::ffi::c_void;

/// Extends Vec<u8> given as *mut c_void from C-like array, given as *mut u8 pointer and usize
/// used as callback from C/C++ code
extern "C" fn vec_extend_from_c_array(dst: *mut c_void, src_data: *mut u8, src_size: usize) {
    // Update the value in RustObject with the value received from the callback
    unsafe {
        let input = slice::from_raw_parts(src_data, src_size);
        (&mut *(dst as *mut Vec<u8>)).extend_from_slice(input);
    }
}


#[link(name = "opencv_resize")]
extern {
    /// Registration of output data and callback which store data
    fn register_output(output: *mut c_void,
                       store_function: extern fn(*mut c_void, *mut u8, usize)) -> i32;
    /// Resize image given as in_data with in_size to num_rows x num_cols jpg image
    /// Store result to registered output by store_function
    // It cannot store data to given in signature output,
    // because size of result array is not known before inference
    // That's why callback is used.
    fn resize(in_data: *const u8, in_size: i32, num_rows: i32, num_cols: i32) -> i32;
}

/// Safe function of getting data of resized image.
/// Returns binary data of 100x100 jpg image in ['Option']
///
/// # Errors
/// If data cannot be represented like image by OpenCV
///
pub fn resize_image(data: &Vec<u8>, cols: usize, rows: usize) -> Option<Vec<u8>> {
    let mut result: Vec<u8> = vec![];
    unsafe {
        assert_eq!(0, register_output(&mut result as *mut _ as *mut c_void, vec_extend_from_c_array));
        if resize(data.as_ptr(), data.len() as _, rows as _, cols as _) != 0 {
            return None;
        }
    }
    Some(result)
}
