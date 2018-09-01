use std::ffi::CStr;
use std::ffi::CString;
use std::mem;
use std::os::raw::{c_char, c_void};

#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod block;
mod block_parser;
mod inline_parser;
mod translator;
mod tree;

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn translate(data: *mut c_char) -> *mut c_char {
    let input = unsafe { CStr::from_ptr(data).to_string_lossy().into_owned() };
    let output = translator::exec(&input);
    CString::new(output).unwrap().into_raw()
}
