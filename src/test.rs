use crate::tokenize;
use std::os::raw::c_char;

#[cfg(test)]
mod tests {
    use std::ffi::CStr;
    use super::*;

    #[test]
    fn it_works() {
        let orig = "Hello world!";
        let result = tokenize(orig.into_raw());
    }

    #[test]
    fn it_works_with_korean() {
        let orig = "결정하겠다";
        let result = tokenize(orig.into_raw());
    }


    #[test]
    fn it_works_with_korean_part_2() {
        let orig = "아버지가방에들어가신다";
        let result = tokenize(orig.into_raw());
    }
}