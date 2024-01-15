#![allow(dead_code)]

mod console;
mod interfaces;
mod logger;
mod module;
mod pattern;
mod ptr;
mod util;

use std::ffi::c_void;

use anyhow::Context;
use windows::core::s;
use windows::Win32::Foundation::*;
use windows::Win32::System::SystemServices::*;
use windows::Win32::System::Threading::*;

use crate::interfaces::engine_client::EngineClient;
use crate::interfaces::{register_iterator, InterfaceRegister};
use crate::module::Module;
use crate::pattern::Pattern;

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "system" fn DllMain(_module: HINSTANCE, reason: u32, _reserved: *mut c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => match on_dll_process_attach() {
            Ok(_) => TRUE,
            Err(err) => {
                log::error!("error in 'DLL_PROCESS_ATTACH': {}", err);
                FALSE
            }
        },
        DLL_PROCESS_DETACH => match on_dll_process_detach() {
            Ok(_) => TRUE,
            Err(err) => {
                log::error!("error in 'DLL_PROCESS_DETACH': {}", err);
                FALSE
            }
        },
        _ => TRUE,
    }
}

const LEVEL_INIT: &str = "48 89 5C 24 ? 56 48 83 EC ? 48 8B 0D ? ? ? ? 48 8B F2";
const LEVEL_SHUTDOWN: &str = "48 83 EC ? 48 8B 0D ? ? ? ? 48 8D 15 ? ? ? ? 45 33 C9 45 33 C0 48 \
                              8B 01 FF 50 ? 48 85 C0 74 ? 48 8B 0D ? ? ? ? 48 8B D0 4C 8B 01 48 \
                              83 C4 ? 49 FF 60 ? 48 83 C4 ? C3 CC CC CC 48 83 EC ? 4C 8B D9";

unsafe fn on_dll_process_attach() -> anyhow::Result<()> {
    logger::init_logger().unwrap();

    // make sure panics are logged to the logfile
    std::panic::set_hook(Box::new(|info| {
        log::error!("panicked: {:#?}", info);
    }));

    let process = GetCurrentProcessId();
    let thread = GetCurrentThreadId();
    log::info!("attached to process {} with thread {}", process, thread);

    // https://github.com/maecry/asphyxia-cs2/blob/cd9e151cf92a2bcad43809a12555bdda7f7d5067/cstrike/core/hooks.cpp#L98
    let level_init_pattern: Pattern = LEVEL_INIT.parse().context("parse 'LEVEL_INIT' pattern")?;

    // https://github.com/maecry/asphyxia-cs2/blob/cd9e151cf92a2bcad43809a12555bdda7f7d5067/cstrike/core/hooks.cpp#L103
    let level_shutdown_pattern: Pattern = LEVEL_SHUTDOWN
        .parse()
        .context("parse 'LEVEL_SHUTDOWN' pattern")?;

    let client_dll = Module::new(s!("client.dll")).context("find 'client.dll'")?;
    let engine_2_dll = Module::new(s!("engine2.dll")).context("find 'engine2.dll'")?;
    let schema_system_dll =
        Module::new(s!("schemasystem.dll")).context("find 'schemasystem.dll'")?;

    let level_init = level_init_pattern
        .find_in(client_dll.code_section())
        .map(|offset| client_dll.code_section().as_ptr().add(offset))
        .context("find 'level_init' pattern")?;
    log::info!("level_init: {:p}", level_init);

    let level_shutdown = level_shutdown_pattern
        .find_in(client_dll.code_section())
        .map(|offset| client_dll.code_section().as_ptr().add(offset))
        .context("find 'level_shutdown' pattern")?;
    log::info!("level_shutdown: {:p}", level_shutdown);

    let client_dll_register_list = InterfaceRegister::find_list(&client_dll)
        .context("find interface list for 'client.dll'")?;
    for register in register_iterator(client_dll_register_list) {
        log::info!("found 'client.dll' interface: {:?}", register.name());
    }

    let schema_system_dll_register_list = InterfaceRegister::find_list(&schema_system_dll)
        .context("find interface list for 'schemasystem.dll'")?;
    for register in register_iterator(schema_system_dll_register_list) {
        log::info!("found 'schemasystem.dll' interface: {:?}", register.name());
    }

    let engine_2_register_list = InterfaceRegister::find_list(&engine_2_dll)
        .context("find interface list for 'engine2.dll'")?;
    for register in register_iterator(engine_2_register_list) {
        log::info!("found 'engine2.dll' interface: {:?}", register.name());
    }

    let engine = interfaces::capture_interface(engine_2_register_list, "Source2EngineToClient001")
        .context("couldn't create interface 'Source2EngineToClient001'")?;

    log::info!("is_connected: {}", EngineClient::is_connected(engine));
    log::info!("is_in_name: {}", EngineClient::is_in_game(engine));

    Err(anyhow::anyhow!("error to cause dll unload"))
}

unsafe fn on_dll_process_detach() -> anyhow::Result<()> {
    let process = GetCurrentProcessId();
    let thread = GetCurrentThreadId();
    log::info!("detached from process {} with thread {}", process, thread);

    Ok(())
}
