pub mod engine_client;

use std::ffi::{c_char, CStr};

use anyhow::Context;
use windows::core::s;

use crate::module::Module;
use crate::ptr;

type CreateInterface = unsafe extern "win64" fn() -> *mut u8;

#[repr(C)]
pub struct InterfaceRegister {
    /// Call this function to instantiate the interface
    create: CreateInterface,
    /// Name of the interface
    name: *const c_char,
    /// Next element in the linked list
    next: *const InterfaceRegister,
}

impl InterfaceRegister {
    /// Get the interface name. Does a `strlen` call and utf-8 validation.
    pub unsafe fn name(&self) -> anyhow::Result<&'static str> {
        CStr::from_ptr(self.name)
            .to_str()
            .context("interpret interface name as utf-8")
    }

    /// Get the first node of the interface register list
    pub unsafe fn find_list(module: &Module) -> anyhow::Result<*const InterfaceRegister> {
        let create_interface = module
            .get_export(s!("CreateInterface"))
            .context("find 'CreateInterface' export")?;

        // https://github.com/maecry/asphyxia-cs2/blob/cd9e151cf92a2bcad43809a12555bdda7f7d5067/cstrike/core/interfaces.cpp#L42
        Ok(*ptr::resolve_relative_address(create_interface, 0x03, 0x07)
            .cast::<*const InterfaceRegister>())
    }
}

pub struct ReigsterIterator {
    ptr: *const InterfaceRegister,
}

pub fn register_iterator(register_list: *const InterfaceRegister) -> ReigsterIterator {
    ReigsterIterator { ptr: register_list }
}

impl Iterator for ReigsterIterator {
    type Item = &'static InterfaceRegister;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr.is_null() {
            // reached the end of the linked list
            return None;
        }
        // SAFETY: `self.ptr` is not null because of the check above
        let next = unsafe { (*self.ptr).next };
        Some(unsafe { &*std::mem::replace(&mut self.ptr, next) })
    }
}

unsafe fn find_interface_register(
    register_list: *const InterfaceRegister,
    name: &str,
) -> anyhow::Result<&'static InterfaceRegister> {
    for register in register_iterator(register_list) {
        if register.name()? == name {
            return Ok(register);
        }
    }
    Err(anyhow::anyhow!("couldn't find interface register"))
}

pub unsafe fn capture_interface(
    register_list: *const InterfaceRegister,
    name: &str,
) -> anyhow::Result<*mut u8> {
    let register = find_interface_register(register_list, name)?;
    let interface = (register.create)();

    (!interface.is_null())
        .then_some(interface)
        .context("create function returned nullptr")
}
