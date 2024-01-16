use anyhow::Context;
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::Diagnostics::Debug::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::System::SystemServices::*;

struct ModuleHeaders64 {
    #[allow(dead_code)]
    dos: &'static IMAGE_DOS_HEADER,
    nt: &'static IMAGE_NT_HEADERS64,
}

unsafe fn get_module_headers_64(module_handle: HMODULE) -> anyhow::Result<ModuleHeaders64> {
    let module_base = module_handle.0 as *const u8;

    let dos = &*(module_base as *const IMAGE_DOS_HEADER);
    if dos.e_magic != IMAGE_DOS_SIGNATURE {
        return Err(anyhow::anyhow!("module dos header has invalid signature"));
    }

    let nt = &*(module_base.offset(dos.e_lfanew as isize) as *const IMAGE_NT_HEADERS64);
    if nt.Signature != IMAGE_NT_SIGNATURE {
        return Err(anyhow::anyhow!("module nt header has invalid signature"));
    }

    Ok(ModuleHeaders64 { dos, nt })
}

unsafe fn get_module_handle(module_name: PCSTR) -> anyhow::Result<HMODULE> {
    // TODO: Replace with https://github.com/maecry/asphyxia-cs2/blob/cd9e151cf92a2bcad43809a12555bdda7f7d5067/cstrike/utilities/memory.cpp#L107
    Ok(GetModuleHandleA(module_name)?)
}

unsafe fn get_proc_address(module_handle: HMODULE, proc_name: PCSTR) -> anyhow::Result<*const u8> {
    // TODO: Replace with https://github.com/maecry/asphyxia-cs2/blob/cd9e151cf92a2bcad43809a12555bdda7f7d5067/cstrike/utilities/memory.cpp#L158
    GetProcAddress(module_handle, proc_name)
        .map(|addr| addr as _)
        .context("couldn't find exported function")
}

pub struct Module {
    handle: HMODULE,
    headers: ModuleHeaders64,
}

impl std::fmt::Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Module")
            .field("handle", &(self.handle.0 as *mut u8))
            .finish()
    }
}

impl Module {
    pub unsafe fn new(module_name: PCSTR) -> anyhow::Result<Module> {
        let handle = get_module_handle(module_name).context("get a handle to the module")?;
        let headers = get_module_headers_64(handle).context("read the module headers")?;
        Ok(Module { handle, headers })
    }

    /// Get the handle to the module
    ///
    /// Note: Copying the handle here is sound, because it is acquired from
    /// `GetModuleHandle` which doesn't increment the reference count of the
    /// library.
    pub fn module_handle(&self) -> HMODULE {
        self.handle
    }

    /// Get a pointer to the base address of the module.
    pub fn module_base(&self) -> *const u8 {
        self.handle.0 as _
    }

    /// Get a slice over the whole module.
    pub unsafe fn module_slice(&self) -> &'static [u8] {
        std::slice::from_raw_parts(
            self.module_base(),
            self.headers.nt.OptionalHeader.SizeOfImage as usize,
        )
    }

    /// Get a slice over the code section of the module.
    pub unsafe fn code_section(&self) -> &'static [u8] {
        std::slice::from_raw_parts(
            self.module_base()
                .add(self.headers.nt.OptionalHeader.BaseOfCode as usize),
            self.headers.nt.OptionalHeader.SizeOfCode as usize,
        )
    }

    /// Get an exported function of the module.
    pub unsafe fn get_export(&self, export: PCSTR) -> anyhow::Result<*const u8> {
        get_proc_address(self.handle, export).context("get module export")
    }
}
