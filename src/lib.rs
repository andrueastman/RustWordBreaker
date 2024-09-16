mod test;

use std::os::raw::c_char;
use std::ffi::{CStr, CString};
use std::ptr;
use charabia::Tokenize;

#[no_mangle]

pub extern "C" fn tokenize(input: *const c_char) -> *mut *mut c_char {
    let c_str = unsafe { CStr::from_ptr(input) };
    let r_str = c_str.to_str().unwrap();
    let tokens = r_str.tokenize();
    let mut result: Vec<*mut c_char> = tokens.map(|t| CString::new(t.lemma().to_string()).unwrap().into_raw()).collect();
    result.push(ptr::null_mut());
    let ptr = result.as_mut_ptr();
    std::mem::forget(result); // Prevent Rust from deallocating the memory
    ptr
}


