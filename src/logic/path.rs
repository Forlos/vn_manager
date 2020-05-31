use anyhow::{Context, Result};
use std::path::PathBuf;

#[inline]
pub(crate) fn get_app_dir() -> Result<PathBuf> {
    let mut path = dirs::home_dir().context("Could not get HOME directory")?;
    path.push("vn_manager");
    Ok(path)
}

pub(crate) fn get_app_state_path() -> Result<PathBuf> {
    let mut path = get_app_dir()?;
    path.push("state.json");
    Ok(path)
}

pub(crate) fn create_app_dir() {
    let mut path = dirs::home_dir().expect("Could not get HOME directory");
    path.push("vn_manager");
    std::fs::create_dir_all(&path).expect("Could not create app dir");
}
