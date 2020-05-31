use super::error::AppError;
use anyhow::{Context, Result};
use std::path::PathBuf;

const VALID_EXECUTABLES: [&str; 1] = ["exe"];

#[cfg(target_os = "linux")]
pub(crate) fn play_vn(path: &PathBuf) -> Result<()> {
    use std::process::Command;
    match Command::new(&path)
        .current_dir(
            &path
                .parent()
                .context("Could not find parent dir of executable")?,
        )
        .env("LC_ALL", "ja_JP")
        .spawn()
    {
        Ok(_) => Ok(()),
        Err(_) => Err(AppError::PlayError.into()),
    }
}

#[cfg(not(target_os = "linux"))]
pub(crate) fn play_vn(path: &PathBuf) -> Result<()> {
    use std::process::Command;
    match Command::new(&path)
        .current_dir(
            &path
                .parent()
                .context("Could not find parent dir of executable")?,
        )
        .spawn()
    {
        Ok(_) => Ok(()),
        Err(_) => Err(AppError::VnPlayError.into()),
    }
}

// TODO probably change this to check magic values instead of extensions
pub(crate) fn check_executable(path: &PathBuf) -> Result<()> {
    if let Some(ext) = path.extension() {
        if !VALID_EXECUTABLES.iter().any(|valid| valid == &ext) {
            return Err(AppError::ExecutableInvalid(format!(
                "Expected one of: {:?}",
                VALID_EXECUTABLES
            ))
            .into());
        }
    } else {
        return Err(AppError::ExecutableInvalid(format!(
            "Expected one of: {:?}",
            VALID_EXECUTABLES
        ))
        .into());
    };
    Ok(())
}
