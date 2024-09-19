mod test;

use std::os::raw::c_char;
use std::ffi::{CStr, CString};
use std::ptr;
use charabia::Tokenize;

#[repr(C)]
pub struct BreakResult {
    pub start: u64,
    pub end: u64,
}

#[no_mangle]
pub extern "C" fn tokenize(input: *const c_char, len: *mut usize) -> *mut BreakResult {
    let c_str = unsafe { CStr::from_ptr(input) };
    let r_str = match c_str.to_str() {
        Err(_) => "",
        Ok(string) => string,
    };
    let tokens = r_str.tokenize();
    let mut result:Vec<BreakResult> = Vec::new();
    for token in tokens {
        if(token.is_word()){
            result.push(BreakResult{ start: token.char_start as u64, end: token.char_end as u64})
        }
    }
    let length = result.len();
    unsafe {
        *len = length;
    }
    let ptr = result.as_mut_ptr();
    std::mem::forget(result); // Prevent Rust from deallocating the memory

    ptr
}

#[no_mangle]
pub extern "C" fn cleanup(ptr: *mut BreakResult, len: usize) {
    if !ptr.is_null() {
        unsafe {
            Vec::from_raw_parts(ptr, len, len);
        }
    }
}


#[cfg(test)]
mod tests {
    use std::ffi::CStr;
    use super::*;

    #[test]
    fn it_works() {
        let orig = "Hello world!";
        let c_str = CString::new(orig).unwrap();
        let c_world: *const c_char = c_str.as_ptr() as *const c_char;
        let length: *mut usize = &mut 0;
        let result = tokenize(c_world, length);
    }

    #[test]
    fn it_works_with_korean() {
        let orig = "결정하겠다";
        //let result = tokenize(orig.as_ref());
    }


    #[test]
    fn it_works_with_korean_part_2() {
        let orig = "아버지가방에들어가신다";
        //let result = tokenize(orig.as_ref());
    }
}