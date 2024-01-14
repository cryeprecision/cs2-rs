use anyhow::Context;
use windows::core::*;
use windows::Win32::System::Diagnostics::Debug::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::System::SystemServices::*;

struct ModuleHeaders64 {
    #[allow(dead_code)]
    dos: &'static IMAGE_DOS_HEADER,
    nt: &'static IMAGE_NT_HEADERS64,
}

unsafe fn get_module_headers_64(module_base: *const u8) -> anyhow::Result<ModuleHeaders64> {
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

unsafe fn get_module_base(module_name: PCSTR) -> anyhow::Result<*const u8> {
    Ok(GetModuleHandleA(module_name)
        .context("couldn't get module base address")?
        .0 as *const u8)
}

pub unsafe fn get_module(module_name: PCSTR) -> anyhow::Result<&'static [u8]> {
    let module_base = get_module_base(module_name)?;
    let module_headers = get_module_headers_64(module_base)?;
    Ok(std::slice::from_raw_parts(
        module_base,
        module_headers.nt.OptionalHeader.SizeOfImage as usize,
    ))
}

pub unsafe fn get_module_code_section(module_name: PCSTR) -> anyhow::Result<&'static [u8]> {
    let module_base = get_module_base(module_name)?;
    let module_headers = get_module_headers_64(module_base)?;
    Ok(std::slice::from_raw_parts(
        module_base.add(module_headers.nt.OptionalHeader.BaseOfCode as usize),
        module_headers.nt.OptionalHeader.SizeOfCode as usize,
    ))
}
