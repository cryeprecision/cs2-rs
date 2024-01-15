use std::ffi::*;

use crate::ptr::get_vfunc_ptr_as;

pub struct EngineClient;

impl EngineClient {
    pub unsafe fn get_max_clients(this: *mut u8) -> c_int {
        type GetMaxClients = unsafe extern "win64" fn(this: *mut u8) -> c_int;
        get_vfunc_ptr_as::<GetMaxClients>(this, 31)(this)
    }
    pub unsafe fn is_in_game(this: *mut u8) -> bool {
        type IsInGame = unsafe extern "win64" fn(this: *mut u8) -> bool;
        get_vfunc_ptr_as::<IsInGame>(this, 32)(this)
    }
    pub unsafe fn is_connected(this: *mut u8) -> bool {
        type IsConnected = unsafe extern "win64" fn(this: *mut u8) -> bool;
        get_vfunc_ptr_as::<IsConnected>(this, 33)(this)
    }
}
