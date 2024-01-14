use std::path::PathBuf;

use anyhow::Context;
use windows::Win32::Foundation::*;
use windows::Win32::System::Com::*;
use windows::Win32::UI::Shell::*;

use crate::util;

pub unsafe fn get_oof_dir() -> anyhow::Result<PathBuf> {
    let path = {
        let path_wide = SHGetKnownFolderPath(&FOLDERID_Documents, KF_FLAG_DEFAULT, HANDLE(0))
            .context("couldn't get path to documents folder")?;
        let path = util::wide_string_to_utf8(path_wide.as_wide())?;
        CoTaskMemFree(Some(path_wide.0 as _));

        let mut path = PathBuf::from(path);
        path.push("oof-software");
        path
    };

    match std::fs::metadata(&path) {
        Err(_) => {
            std::fs::create_dir(&path).context("couldn't create oof-software directory")?;
        }
        Ok(metadata) if !metadata.is_dir() => {
            return Err(anyhow::anyhow!(
                "path to oof-software directory already exists but is not a directory"
            ));
        }
        Ok(_) => (/* no-op */),
    };

    Ok(path)
}

pub fn init_logger() -> anyhow::Result<()> {
    let mut config = simplelog::ConfigBuilder::default();

    config
        .set_target_level(simplelog::LevelFilter::Off)
        .set_location_level(simplelog::LevelFilter::Off)
        .set_time_level(simplelog::LevelFilter::Info)
        .set_time_format_rfc3339();

    config.set_time_offset_to_local().unwrap();

    let mut log_file_path = unsafe { get_oof_dir() }?;
    log_file_path.push("log.txt");

    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)
        .context("couldn't open logfile for writing")?;

    simplelog::WriteLogger::init(simplelog::LevelFilter::Info, config.build(), file)
        .context("couldn't init term logger")
}