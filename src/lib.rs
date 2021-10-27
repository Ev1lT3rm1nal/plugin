use std::ffi::CString;
use std::os::raw::c_char;

// DON'T CHANGE THIS
#[repr(C)]
pub enum ResultType {
    RETRY,
    HIT,
    BAD,
}

//THE NAME OF YOUR PLUGIN
#[no_mangle]
pub extern "C" fn PLUGIN_NAME() -> *mut c_char {
    CString::new("Name").unwrap().into_raw()
}

//THE NAME OF YOUR PLUGIN
#[no_mangle]
pub extern "C" fn PLUGIN_VERSION() -> *const c_char {
    CString::new("0.0.1").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn get_combo_result(
    user: *mut ::std::os::raw::c_char,
    pass: *mut ::std::os::raw::c_char,
    proxy: *mut ::std::os::raw::c_char,
) -> ResultType {
    // ADD YOUR CODE HERE

    ResultType::BAD
}