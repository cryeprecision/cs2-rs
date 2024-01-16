pub mod engine_client;
pub mod mem_alloc;

use std::ffi::{c_char, CStr};

use anyhow::Context;
use windows::core::s;

use crate::oof::module::Module;
use crate::oof::ptr::resolve_relative_address;

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

pub struct RegisterIterator {
    current_node: *const InterfaceRegister,
}

impl Iterator for RegisterIterator {
    type Item = &'static InterfaceRegister;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_node.is_null() {
            // reached the end of the linked list
            return None;
        }
        // SAFETY: `self.ptr` is not null because of the check above
        let next = unsafe { (*self.current_node).next };
        Some(unsafe { &*std::mem::replace(&mut self.current_node, next) })
    }
}

impl InterfaceRegister {
    /// Get the interface name. Does a `strlen` call and utf-8 validation.
    pub unsafe fn name(&self) -> anyhow::Result<&'static str> {
        CStr::from_ptr(self.name)
            .to_str()
            .context("interpret interface name as utf-8")
    }

    /// Get the first node of the interface register list
    pub unsafe fn iter_from_module(module: &Module) -> anyhow::Result<RegisterIterator> {
        let create_interface = module
            .get_export(s!("CreateInterface"))
            .context("find 'CreateInterface' export")?;

        // https://github.com/maecry/asphyxia-cs2/blob/cd9e151cf92a2bcad43809a12555bdda7f7d5067/cstrike/core/interfaces.cpp#L42
        let first_node = resolve_relative_address(create_interface, 0x03, 0x07)
            .cast::<*const InterfaceRegister>()
            .read();

        Ok(RegisterIterator {
            current_node: first_node,
        })
    }

    /// Get all interface names (excluding names that aren't valid utf-8)
    pub unsafe fn all_interfaces(module: &Module) -> anyhow::Result<Vec<String>> {
        Ok(Self::iter_from_module(module)?
            .map(|register| register.name().ok())
            .filter_map(|register| register.map(String::from))
            .collect())
    }

    /// Find a interface register with the given name in the linked list of
    /// registers
    pub unsafe fn find_interface_register(
        module: &Module,
        name: &str,
    ) -> anyhow::Result<&'static InterfaceRegister> {
        Self::iter_from_module(module)?
            .find(|register| register.name().map(|name_| name_ == name).unwrap_or(false))
            .context("couldn't find interface register")
    }

    /// Find a interface register with the given name and call its `create`
    /// function
    pub unsafe fn capture_interface(module: &Module, name: &str) -> anyhow::Result<*mut u8> {
        let register = Self::find_interface_register(module, name)?;
        let interface = (register.create)();

        (!interface.is_null())
            .then_some(interface)
            .context("interface create function returned nullptr")
    }
}
