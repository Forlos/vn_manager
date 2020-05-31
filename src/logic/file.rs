use anyhow::Result;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[inline]
pub(crate) fn write_to_file(path: &PathBuf, buf: &[u8]) -> Result<()> {
    let mut file = File::create(&path)?;
    Ok(file.write_all(&buf)?)
}

pub(crate) fn fetch_and_write_to_file(path: &PathBuf, url: &str) -> Result<()> {
    let bytes = reqwest::blocking::get(url)?.bytes()?;
    Ok(write_to_file(path, &bytes)?)
}

pub(crate) fn save_state_to_file<T>(path: &PathBuf, state: &T) -> Result<()>
where
    T: Serialize,
{
    write_to_file(path, serde_json::to_string_pretty(state)?.as_bytes())
}
