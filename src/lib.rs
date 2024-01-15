#![allow(dead_code)]

mod console;
mod interfaces;
mod logger;
mod module;
mod pattern;
mod ptr;
mod str;
mod util;

use std::ffi::{c_char, c_void};

use anyhow::Context;
use retour::GenericDetour;
use windows::core::{s, PCSTR};
use windows::Win32::Foundation::*;
use windows::Win32::System::SystemServices::*;
use windows::Win32::System::Threading::*;

use crate::interfaces::engine_client::EngineClient;
use crate::interfaces::InterfaceRegister;
use crate::module::Module;
use crate::pattern::Pattern;
use crate::ptr::get_vfunc_ptr_as;

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

const CLIENT_DLL: PCSTR = s!("client.dll");
const SCHEMASYSTEM_DLL: PCSTR = s!("schemasystem.dll");
const ENGINE2_DLL: PCSTR = s!("engine2.dll");

unsafe fn on_dll_process_attach() -> anyhow::Result<()> {
    logger::init_logger().unwrap();

    // make sure panics are logged to the logfile
    std::panic::set_hook(Box::new(|info| {
        log::error!("panicked: {:#?}", info);
    }));

    let process = GetCurrentProcessId();
    let thread = GetCurrentThreadId();
    log::info!("attached to process {} with thread {}", process, thread);

    let client_dll = Module::new(CLIENT_DLL).context("find client.dll")?;
    let engine2_dll = Module::new(ENGINE2_DLL).context("find engine2.dll")?;
    let schema_system_dll = Module::new(SCHEMASYSTEM_DLL).context("find schemasystem.dll")?;

    // pattern scanning example
    {
        // https://github.com/maecry/asphyxia-cs2/blob/cd9e151cf92a2bcad43809a12555bdda7f7d5067/cstrike/core/hooks.cpp#L98
        let level_init = LEVEL_INIT
            .parse::<Pattern>()
            .context("parse level_init pattern")?
            .find_in(client_dll.code_section())
            .map(|offset| client_dll.code_section().as_ptr().add(offset))
            .context("find level_init pattern")?;
        log::info!("found level_init at {:p}", level_init);

        // https://github.com/maecry/asphyxia-cs2/blob/cd9e151cf92a2bcad43809a12555bdda7f7d5067/cstrike/core/hooks.cpp#L103
        let level_shutdown = LEVEL_SHUTDOWN
            .parse::<Pattern>()
            .context("parse level_shutdown pattern")?
            .find_in(client_dll.code_section())
            .map(|offset| client_dll.code_section().as_ptr().add(offset))
            .context("find level_shutdown pattern")?;
        log::info!("found level_shutdown at {:p}", level_shutdown);
    }

    // log all interfaces because why not
    {
        log::info!(
            "client.dll interfaces: {:?}",
            InterfaceRegister::all_interfaces(&client_dll)
        );
        log::info!(
            "schemasystem.dll interfaces: {:?}",
            InterfaceRegister::all_interfaces(&schema_system_dll)
        );
        log::info!(
            "engine2.dll interfaces: {:?}",
            InterfaceRegister::all_interfaces(&engine2_dll)
        );
    }

    // get the engine client pointer and use our wrapper for calling vfuncs
    let engine = EngineClient::new(
        InterfaceRegister::capture_interface(&engine2_dll, "Source2EngineToClient001")
            .context("get engine client pointer")?,
    );

    // example of using a captured interface
    {
        log::info!("engine.is_connected() = {}", engine.is_connected());
        log::info!("engine.is_in_game() = {}", engine.is_in_game());
        log::info!("engine.get_level_name() = {:?}", engine.get_level_name());
    }

    // example of hooking a function
    {
        // type of the function we want to hook
        type GetLevelNameFn = unsafe extern "win64" fn(this: *mut u8) -> *const c_char;

        // our hook that is called instead of the original function
        unsafe extern "win64" fn get_level_name_hook_fn(_: *mut u8) -> *const c_char {
            // logging works from inside hooks
            log::info!("hello from inside get_level_name hook");
            // return a way better level name
            s!("oof-software").0 as _
        }

        // this does some sanity checks
        let get_level_name_hook = GenericDetour::<GetLevelNameFn>::new(
            get_vfunc_ptr_as(engine.this_ptr, 53),
            get_level_name_hook_fn,
        )
        .context("create get_level_name hook")?;

        // enable the hook, call the function and disable the hook again
        get_level_name_hook.enable().context("enable hook")?;
        log::info!(
            "hooked engine.get_level_name() = {:?}",
            engine.get_level_name()
        );
        get_level_name_hook.disable().context("remove hook")?;
    }

    Err(anyhow::anyhow!("error to cause dll unload"))
}

unsafe fn on_dll_process_detach() -> anyhow::Result<()> {
    let process = GetCurrentProcessId();
    let thread = GetCurrentThreadId();
    log::info!("detached from process {} with thread {}", process, thread);

    Ok(())
}
