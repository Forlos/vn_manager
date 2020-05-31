use crate::{
    logic::{file::save_state_to_file, path::get_app_state_path, vn::Vn},
    message::Message,
    ui::vn::{list::VnList, view::VnView},
};
use anyhow::{Context, Result};

use iced::{Container, Element, Length, Row};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use vndb_rs::common::get::vn::GetVnResults;

#[derive(Serialize, Deserialize)]
pub(crate) struct VnContent {
    list: VnList,
    view: VnView,
}

impl VnContent {
    pub(crate) fn init() -> Self {
        Self {
            list: VnList::init(),
            view: VnView::init(),
        }
    }
    pub(crate) fn view(&mut self) -> Element<Message> {
        let content = Row::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .push(self.list.view())
            .push(self.view.view());

        Container::new(content)
            .width(Length::Fill)
            .height(Length::FillPortion(9))
            .padding(0)
            .into()
    }
    pub(crate) fn update_view(&mut self, vn: Vn) {
        self.view = VnView::from(vn);
    }
    pub(crate) fn add_vn(
        &mut self,
        vn_response: GetVnResults,
        executable_path: Option<PathBuf>,
    ) -> Result<()> {
        self.list.add_vn(vn_response, executable_path)?;
        save_state_to_file(&get_app_state_path()?, &self)
    }
    pub(crate) fn delete_vn(&mut self, vn_id: usize) -> Result<()> {
        self.list.delete_vn(vn_id);
        self.update_view(self.list.list.first().context("asdf")?.vn.clone());
        save_state_to_file(&get_app_state_path()?, &self)
    }
    pub(crate) fn vn_exists(&self, vn_id: usize) -> bool {
        self.list.vn_exists(vn_id)
    }
    pub(crate) fn set_error_msg(&mut self, error_msg: String) {
        self.view.set_error_msg(error_msg);
    }
}
