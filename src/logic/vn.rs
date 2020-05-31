use crate::logic::{file::fetch_and_write_to_file, path::get_app_dir};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use std::{convert::TryFrom, path::PathBuf};
use vndb_rs::common::get::vn::GetVnResults;

#[serde(default)]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct Vn {
    pub id: usize,
    pub title: String,
    pub original_title: Option<String>,
    pub released: Option<String>,
    pub languages: Vec<String>,
    pub description: String,
    pub executable_path: Option<PathBuf>,
    pub image_path: Option<PathBuf>,
}

impl Vn {
    pub(crate) fn set_executable_path(&mut self, executable_path: PathBuf) {
        self.executable_path = Some(executable_path);
    }
}

impl TryFrom<GetVnResults> for Vn {
    type Error = anyhow::Error;

    fn try_from(mut results: GetVnResults) -> Result<Self> {
        let vn = results.items.pop().context("No VN found")?;
        let mut path = get_app_dir()?;
        let image_url = vn.image.context("Could not get image url")?;

        path.push(
            image_url
                .rsplitn(2, "/")
                .next()
                .context("Could not get image path")?,
        );
        fetch_and_write_to_file(&path, &image_url)?;

        Ok(Self {
            id: vn.id,
            title: vn.title.unwrap_or_default(),
            original_title: vn.original_title,
            released: vn.released,
            languages: vn.languages.unwrap_or_default(),
            description: vn.description.unwrap_or_default(),
            executable_path: None,
            image_path: Some(path),
        })
    }
}
