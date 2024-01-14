mod console;
mod logger;
mod module;
mod pattern;
mod util;

use std::ffi::c_void;

use windows::core::s;
use windows::Win32::Foundation::*;
use windows::Win32::System::SystemServices::*;
use windows::Win32::System::Threading::*;

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "system" fn DllMain(_module: HINSTANCE, reason: u32, _reserved: *mut c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => on_dll_process_attach(),
        DLL_PROCESS_DETACH => on_dll_process_detach(),
        _ => TRUE,
    }
}

unsafe fn on_dll_process_attach() -> BOOL {
    logger::init_logger().unwrap();

    let process = GetCurrentProcessId();
    let thread = GetCurrentThreadId();
    log::info!("attached to process {} with thread {}", process, thread);

    log::info!("oof-dir: {:?}", logger::get_oof_dir());
    log::info!(
        "oof-len: {:?}",
        module::get_module(s!("cs2_rs.dll")).map(|b| b.len())
    );
    log::info!(
        "oof-code-len: {:?}",
        module::get_module_code_section(s!("cs2_rs.dll")).map(|b| b.len())
    );

    TRUE
}

unsafe fn on_dll_process_detach() -> BOOL {
    let process = GetCurrentProcessId();
    let thread = GetCurrentThreadId();
    log::info!("detached from process {} with thread {}", process, thread);

    TRUE
}
