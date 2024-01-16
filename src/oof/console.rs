use anyhow::Context;
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Storage::FileSystem::*;
use windows::Win32::System::Console::*;
use windows::Win32::UI::WindowsAndMessaging::{PostMessageA, WM_CLOSE};

pub struct Console {
    handle: HANDLE,
}

impl Console {
    pub unsafe fn attach_console() -> anyhow::Result<Console> {
        AllocConsole().context("allocate console")?;

        let handle = CreateFileA(
            s!("CONOUT$"),
            GENERIC_WRITE.0,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            HANDLE(0),
        )
        .context("create console handle")?;

        SetStdHandle(STD_OUTPUT_HANDLE, handle).context("set default stdout handle")?;
        SetConsoleTitleA(s!("very cool")).context("set console title")?;

        Ok(Console { handle })
    }

    pub unsafe fn detach_console(self: Console) -> anyhow::Result<()> {
        CloseHandle(self.handle).context("close console handle")?;
        FreeConsole().context("free console")?;

        let console_window = GetConsoleWindow();
        if console_window != HWND(0) {
            PostMessageA(console_window, WM_CLOSE, WPARAM(0), LPARAM(0))
                .context("send message to close console window")?;
        }

        Ok(())
    }
}
