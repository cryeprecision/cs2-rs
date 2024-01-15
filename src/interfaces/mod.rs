mod engine_client;
use std::ffi::CStr;

use anyhow::Context;
pub use engine_client::EngineClient;

type CreateInterface = unsafe extern "win64" fn() -> *mut u8;

#[repr(C)]
struct InterfaceRegister {
    /// Call this function to instantiate the interface
    create: CreateInterface,
    /// Name of the interface
    name: *const u8,
    /// Next element in the linked list
    next: *const InterfaceRegister,
}

impl InterfaceRegister {
    unsafe fn name(&self) -> anyhow::Result<&'static str> {
        CStr::from_ptr(self.name as _)
            .to_str()
            .context("interpret interface name as utf-8")
    }
}

struct ReigsterIterator {
    ptr: *const InterfaceRegister,
}
fn register_iterator(register_list: *const InterfaceRegister) -> ReigsterIterator {
    ReigsterIterator { ptr: register_list }
}

impl Iterator for ReigsterIterator {
    type Item = &'static InterfaceRegister;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr.is_null() {
            // reached the end of the linked list
            return None;
        }
        // SAFETY: `self.ptr` is not a null because of the check above
        let next = unsafe { (*self.ptr).next };
        Some(unsafe { &*std::mem::replace(&mut self.ptr, next) })
    }
}

unsafe fn capture_interface(
    register_list: *const InterfaceRegister,
    name: &str,
) -> Option<*mut u8> {
    let register = register_iterator(register_list)
        .find(|register| register.name().map(|n| n == name).unwrap_or(false))?;
    todo!("https://github.com/maecry/asphyxia-cs2/blob/cd9e151cf92a2bcad43809a12555bdda7f7d5067/cstrike/core/interfaces.cpp#L46")
}
