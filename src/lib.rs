mod console;

use std::ffi::c_void;

use console::Console;
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::FreeLibraryAndExitThread;
use windows::Win32::System::SystemServices::*;
use windows::Win32::System::Threading::{GetCurrentProcessId, GetCurrentThreadId};
use windows::Win32::UI::WindowsAndMessaging::*;

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "system" fn DllMain(module: HINSTANCE, reason: u32, _: *mut c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            on_process_attach(module);
        }
        DLL_PROCESS_DETACH => {
            on_process_detach();
        }
        DLL_THREAD_ATTACH => (),
        DLL_THREAD_DETACH => (),
        _ => (),
    };
    TRUE
}

unsafe fn on_process_attach(module: HINSTANCE) {
    let process = GetCurrentProcessId();
    let thread = GetCurrentThreadId();
    let text = format!("Hi from process {} (thread {})\0", process, thread);

    let _ = MessageBoxA(
        HWND(0),
        PCSTR(text.as_ptr()),
        s!("oof-software"),
        MB_YESNOCANCEL,
    );

    let console = Console::attach_console().unwrap();
    println!("hello! ({}, {})", process, thread);
    std::thread::sleep(std::time::Duration::from_secs(3));
    console.detach_console().unwrap();

    FreeLibraryAndExitThread(module, 0);
}

unsafe fn on_process_detach() {
    let process = GetCurrentProcessId();
    let thread = GetCurrentThreadId();
    let text = format!("Bye from process {} (thread {})\0", process, thread);

    let _ = MessageBoxA(
        HWND(0),
        PCSTR(text.as_ptr()),
        s!("oof-software"),
        MB_YESNOCANCEL,
    );
}
