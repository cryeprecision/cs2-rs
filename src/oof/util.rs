use anyhow::Context;
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Globalization::*;

pub unsafe fn wide_string_to_utf8(wide_string: &[u16]) -> anyhow::Result<String> {
    let bytes_needed = WideCharToMultiByte(
        CP_UTF8,
        WC_ERR_INVALID_CHARS,
        wide_string,
        None,
        PCSTR(std::ptr::null()),
        None,
    );

    if bytes_needed == 0 {
        return Err(anyhow::anyhow!(
            "couldn't get length of string as utf-8 ({:?})",
            GetLastError()
        ));
    }

    let mut buffer = vec![0u8; bytes_needed as usize];
    let result = WideCharToMultiByte(
        CP_UTF8,
        WC_ERR_INVALID_CHARS,
        wide_string,
        Some(&mut buffer),
        PCSTR(std::ptr::null()),
        None,
    );

    if result == 0 {
        return Err(anyhow::anyhow!(
            "couldn't convert wide string to utf-8 ({:?})",
            GetLastError()
        ));
    }

    let str = std::ffi::CStr::from_ptr(buffer.as_ptr() as *const i8)
        .to_str()
        .context("converted string is not valid utf-8")?;

    Ok(str.to_string())
}
