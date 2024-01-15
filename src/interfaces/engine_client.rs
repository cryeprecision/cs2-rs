use std::ffi::*;

use crate::ptr::get_vfunc_ptr_as;
use crate::str::{c_str_to_str_slice, Error};

pub struct EngineClient {
    this_ptr: *mut u8,
}

impl EngineClient {
    pub unsafe fn new(this: *mut u8) -> EngineClient {
        EngineClient { this_ptr: this }
    }

    pub unsafe fn get_max_clients(&self) -> c_int {
        type Fn = unsafe extern "win64" fn(this: *mut u8) -> c_int;
        get_vfunc_ptr_as::<Fn>(self.this_ptr, 31)(self.this_ptr)
    }
    pub unsafe fn is_in_game(&self) -> bool {
        type Fn = unsafe extern "win64" fn(this: *mut u8) -> bool;
        get_vfunc_ptr_as::<Fn>(self.this_ptr, 32)(self.this_ptr)
    }
    pub unsafe fn is_connected(&self) -> bool {
        type Fn = unsafe extern "win64" fn(this: *mut u8) -> bool;
        get_vfunc_ptr_as::<Fn>(self.this_ptr, 33)(self.this_ptr)
    }
    pub unsafe fn get_level_name(&self) -> Result<&'static str, Error> {
        type Fn = unsafe extern "win64" fn(this: *mut u8) -> *const c_char;
        c_str_to_str_slice(get_vfunc_ptr_as::<Fn>(self.this_ptr, 53)(self.this_ptr))
    }
    pub unsafe fn get_level_name_short(&self) -> Result<&'static str, Error> {
        type Fn = unsafe extern "win64" fn(this: *mut u8) -> *const c_char;
        c_str_to_str_slice(get_vfunc_ptr_as::<Fn>(self.this_ptr, 54)(self.this_ptr))
    }
    pub unsafe fn get_product_version_string(&self) -> Result<&'static str, Error> {
        // FIXME: This crashes when called from the main menu
        type Fn = unsafe extern "win64" fn(this: *mut u8) -> *const c_char;
        c_str_to_str_slice(get_vfunc_ptr_as::<Fn>(self.this_ptr, 77)(self.this_ptr))
    }
}
