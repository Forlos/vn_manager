use crate::{
    logic::path::get_app_state_path,
    ui::{
        add_vn::AddVnContent, content::Content, header::Category,
        vn::content::VnContent,
    },
};
use anyhow::{Context, Result};

pub(crate) struct Contents {
    pub content_list: [Content; 2],
    pub current_content_index: usize,
}

impl Contents {
    pub(crate) fn init() -> Self {
        let path = get_app_state_path().expect("Could not get app path");

        let vn_content: VnContent = match std::fs::File::open(&path) {
            Ok(f) => serde_json::from_reader(f)
                .expect("Could not read app state from file"),
            Err(_) => VnContent::init(),
        };

        let content_list = [
            Content::Vn(vn_content),
            Content::AddVn(AddVnContent::init()),
        ];
        Contents {
            content_list,
            current_content_index: 0,
        }
    }
    pub(crate) fn get_current_content(&mut self) -> Result<&mut Content> {
        Ok(self
            .content_list
            .get_mut(self.current_content_index)
            .context("Could not get currently displayed content")?)
    }
    pub(crate) fn set_current_content(
        &mut self,
        category: Category,
    ) -> Result<()> {
        self.current_content_index = self
            .content_list
            .iter()
            .enumerate()
            .find(|content| content.1.get_category() == category)
            .context("Category not found")?
            .0;
        Ok(())
    }
    pub(crate) fn get_content(
        &mut self,
        category: Category,
    ) -> Result<&mut Content> {
        Ok(self
            .content_list
            .iter_mut()
            .find(|content| content.get_category() == category)
            .context("Category not found")?)
    }
}
