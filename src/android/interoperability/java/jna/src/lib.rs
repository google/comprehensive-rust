
use std::ffi;
use std::ffi::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn hello(input_num: u32, java_string: * mut c_char){
    let name = if input_num % 2 == 0 {"World"} else {"Students"};
    let composed_str = CString::new("Hello ".to_owned() + name).unwrap();
    let composed_str_p = composed_str.as_ptr() as * const c_char;
    unsafe {
        ptr::copy(composed_str_p, java_string, 20);
    }
}
