use std::os::raw::c_char;

// DON'T CHANGE THIS
#[repr(C)]
pub enum ResultType {
    RETRY,
    HIT,
    BAD,
}

// DON'T CHANGE THIS
#[repr(C)]
pub enum ProxyType {
    HTTPS,
    SOCKS4,
    SOCKS5,
}

// Use lower_snake_case syntax for naming and add \0 add the end of the string
#[no_mangle]
pub extern "C" fn PLUGIN_NAME() -> *const c_char {
    "name\0".as_ptr() as *const c_char
}

// Use semantic versioning (SemVer) and add \0 add the end of the string
#[no_mangle]
pub extern "C" fn PLUGIN_VERSION() -> *const c_char {
    "0.0.1\0".as_ptr() as *const c_char
}

#[no_mangle]
pub extern "C" fn get_combo_result(
    user: *mut ::std::os::raw::c_char,
    pass: *mut ::std::os::raw::c_char,
    proxy: *mut ::std::os::raw::c_char,
    proxy_type: ProxyType,
) -> ResultType {
    // ADD YOUR CODE HERE

    ResultType::BAD
}