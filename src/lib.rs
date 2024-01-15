#![allow(dead_code)]

mod console;
mod interfaces;
mod logger;
mod module;
mod pattern;
mod ptr;
mod util;

use std::ffi::c_void;

use windows::core::s;
use windows::Win32::Foundation::*;
use windows::Win32::System::SystemServices::*;
use windows::Win32::System::Threading::*;

use crate::module::get_module_code_section;
use crate::pattern::Pattern;

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "system" fn DllMain(_module: HINSTANCE, reason: u32, _reserved: *mut c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => on_dll_process_attach(),
        DLL_PROCESS_DETACH => on_dll_process_detach(),
        _ => TRUE,
    }
}

const LEVEL_INIT: &str = "48 89 5C 24 ? 56 48 83 EC ? 48 8B 0D ? ? ? ? 48 8B F2";
const LEVEL_SHUTDOWN: &str = "48 83 EC ? 48 8B 0D ? ? ? ? 48 8D 15 ? ? ? ? 45 33 C9 45 33 C0 48 \
                              8B 01 FF 50 ? 48 85 C0 74 ? 48 8B 0D ? ? ? ? 48 8B D0 4C 8B 01 48 \
                              83 C4 ? 49 FF 60 ? 48 83 C4 ? C3 CC CC CC 48 83 EC ? 4C 8B D9";

unsafe fn on_dll_process_attach() -> BOOL {
    logger::init_logger().unwrap();

    let process = GetCurrentProcessId();
    let thread = GetCurrentThreadId();
    log::info!("attached to process {} with thread {}", process, thread);

    // https://github.com/maecry/asphyxia-cs2/blob/cd9e151cf92a2bcad43809a12555bdda7f7d5067/cstrike/core/hooks.cpp#L98
    let level_init_pattern: Pattern = match LEVEL_INIT.parse() {
        Err(err) => {
            log::error!("parse level_init_pattern ({})", err);
            return FALSE;
        }
        Ok(pattern) => pattern,
    };

    // https://github.com/maecry/asphyxia-cs2/blob/cd9e151cf92a2bcad43809a12555bdda7f7d5067/cstrike/core/hooks.cpp#L103
    let level_shutdown_pattern: Pattern = match LEVEL_SHUTDOWN.parse() {
        Err(err) => {
            log::error!("parse level_shutdown_pattern ({})", err);
            return FALSE;
        }
        Ok(pattern) => pattern,
    };

    let client_dll = match get_module_code_section(s!("client.dll")) {
        Err(err) => {
            log::error!("couldn't get client.dll code section ({})", err);
            return FALSE;
        }
        Ok(module) => module,
    };

    let level_init = level_init_pattern
        .find(client_dll)
        .map(|offset| client_dll.as_ptr().add(offset))
        .unwrap_or(std::ptr::null());

    let level_shutdown = level_shutdown_pattern
        .find(client_dll)
        .map(|offset| client_dll.as_ptr().add(offset))
        .unwrap_or(std::ptr::null());

    log::info!("level_init: {:p}", level_init);
    log::info!("level_shutdown: {:p}", level_shutdown);

    // returning FALSE here causes a dll unload
    FALSE
}

unsafe fn on_dll_process_detach() -> BOOL {
    let process = GetCurrentProcessId();
    let thread = GetCurrentThreadId();
    log::info!("detached from process {} with thread {}", process, thread);

    TRUE
}
