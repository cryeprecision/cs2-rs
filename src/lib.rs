#![allow(dead_code)]

mod oof;
mod sdk;

use std::ffi::{c_char, c_void};

use anyhow::Context;
use retour::GenericDetour;
use windows::core::s;
use windows::Win32::Foundation::*;
use windows::Win32::System::SystemServices::*;
use windows::Win32::System::Threading::*;

use crate::oof::ptr::get_vfunc_ptr_as;
use crate::sdk::interfaces::InterfaceRegister;

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "system" fn DllMain(_module: HINSTANCE, reason: u32, _reserved: *mut c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => match on_dll_process_attach() {
            Ok(_) => TRUE,
            Err(err) => {
                log::error!("error in 'DLL_PROCESS_ATTACH': {:?}", err);
                FALSE
            }
        },
        DLL_PROCESS_DETACH => match on_dll_process_detach() {
            Ok(_) => TRUE,
            Err(err) => {
                log::error!("error in 'DLL_PROCESS_DETACH': {:?}", err);
                FALSE
            }
        },
        _ => TRUE,
    }
}

unsafe fn on_dll_process_attach() -> anyhow::Result<()> {
    oof::logger::init_logger().unwrap();

    // make sure panics are logged to the logfile
    std::panic::set_hook(Box::new(|info| {
        log::error!("panicked: {:#?}", info);
    }));

    let process = GetCurrentProcessId();
    let thread = GetCurrentThreadId();
    log::info!("attached to process {} with thread {}", process, thread);

    let ctx = {
        let start = std::time::Instant::now();
        let ctx = sdk::global_context::GlobalContext::new().context("initialize global context")?;
        let elapsed = start.elapsed().as_secs_f64();
        log::info!("initialized global context in {:.3}ms", elapsed * 1e3);
        ctx
    };

    log::info!("global context: {:#?}", ctx);

    // log some interfaces because why not
    {
        log::info!(
            "client.dll interfaces: {:?}",
            InterfaceRegister::all_interfaces(&ctx.modules.client)
        );
        log::info!(
            "schemasystem.dll interfaces: {:?}",
            InterfaceRegister::all_interfaces(&ctx.modules.schema_system)
        );
        log::info!(
            "engine2.dll interfaces: {:?}",
            InterfaceRegister::all_interfaces(&ctx.modules.engine2)
        );
    }

    // example of using a captured interface
    {
        let engine = &ctx.interfaces.engine_client;
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
            get_vfunc_ptr_as(ctx.interfaces.engine_client.this_ptr, 53),
            get_level_name_hook_fn,
        )
        .context("create get_level_name hook")?;

        // enable the hook, call the function and disable the hook again
        get_level_name_hook.enable().context("enable hook")?;
        log::info!(
            "hooked engine.get_level_name() = {:?}",
            ctx.interfaces.engine_client.get_level_name()
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
