use std::ffi::{c_char, CStr};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("null pointer")]
    NullPtr,
    #[error("invalid utf-8")]
    Utf8,
}

pub unsafe fn c_str_to_str_slice(c_str: *const c_char) -> Result<&'static str, Error> {
    if c_str.is_null() {
        return Err(Error::NullPtr);
    }
    CStr::from_ptr(c_str).to_str().map_err(|_| Error::Utf8)
}
