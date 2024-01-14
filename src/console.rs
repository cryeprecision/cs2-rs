use anyhow::Context;
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Storage::FileSystem::*;
use windows::Win32::System::Console::*;
use windows::Win32::UI::WindowsAndMessaging::{PostMessageA, WM_CLOSE};

#[allow(dead_code)]
pub struct Console {
    handle: HANDLE,
}

impl Console {
    #[allow(dead_code)]
    pub unsafe fn attach_console() -> anyhow::Result<Console> {
        AllocConsole().context("couldn't allocate console")?;

        let handle = CreateFileA(
            s!("CONOUT$"),
            GENERIC_WRITE.0,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            HANDLE(0),
        )
        .context("couldn't create console handle")?;

        SetStdHandle(STD_OUTPUT_HANDLE, handle).context("couldn't set default stdout handle")?;
        SetConsoleTitleA(s!("very cool")).context("couldn't set console title")?;

        Ok(Console { handle })
    }

    #[allow(dead_code)]
    pub unsafe fn detach_console(self: Console) -> anyhow::Result<()> {
        CloseHandle(self.handle).context("couldn't close console handle")?;
        FreeConsole().context("couldn't free console")?;

        let console_window = GetConsoleWindow();
        if console_window != HWND(0) {
            PostMessageA(console_window, WM_CLOSE, WPARAM(0), LPARAM(0))
                .context("couldn't send message to close console window")?;
        }

        Ok(())
    }
}
