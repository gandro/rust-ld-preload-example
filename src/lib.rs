#[macro_use]
extern crate lazy_static;
extern crate libc;

use std::ffi::{CStr, CString};
use std::mem;

// this is only needed if the original function is supposed to be called
lazy_static! {
    static ref ORIGINAL_PUTS: extern fn(*const libc::c_char) = unsafe {
        let fn_name = CStr::from_bytes_with_nul(b"puts\0").unwrap();
        let fn_ptr = libc::dlsym(libc::RTLD_NEXT, fn_name.as_ptr());

        mem::transmute(fn_ptr)
    };
}

#[no_mangle]
pub unsafe extern fn puts(s: *const libc::c_char) {
    let rust_string = CStr::from_ptr(s)
        .to_str()
        .expect("invalid utf8")
        .replace("C", "Rust");
    let c_string = CString::new(rust_string).unwrap();
    ORIGINAL_PUTS(c_string.as_ptr())
}
