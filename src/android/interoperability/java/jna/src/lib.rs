
use std::ffi;

#[no_mangle]
pub extern "C" fn hello(hello_str: * const c_char) {
    let name = CStr::from_ptr(hello_str);
    return format!("Hello, {name}");
} 