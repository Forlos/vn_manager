use crate::{
    logic::{error::AppError, play::check_executable, vn::Vn},
    message::Message,
    style,
};
use anyhow::Result;
use iced::{
    button, scrollable, Align, Button, Container, Element, Length, Scrollable,
    Text,
};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, path::PathBuf};
use vndb_rs::common::get::vn::GetVnResults;

#[derive(Serialize, Deserialize)]
pub(crate) struct VnList {
    pub(crate) list: Vec<VnListItem>,
    #[serde(skip)]
    scroll: scrollable::State,
}

#[derive(Serialize, Deserialize)]
pub struct VnListItem {
    pub(crate) vn: Vn,
    #[serde(skip)]
    state: button::State,
}

impl VnList {
    pub(crate) fn init() -> Self {
        VnList {
            list: vec![],
            scroll: scrollable::State::new(),
        }
    }
    pub(crate) fn view(&mut self) -> Element<Message> {
        let content = self.list.iter_mut().fold(
            Scrollable::new(&mut self.scroll)
                .width(Length::Fill)
                .align_items(Align::Start),
            |content, item| {
                content.push(
                    Button::new(
                        &mut item.state,
                        Text::new(&item.vn.title).size(16),
                    )
                    .on_press(Message::Vn(item.vn.clone()))
                    .style(style::dark::List),
                )
            },
        );

        Container::new(content)
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .padding(0)
            .style(style::dark::Dark)
            .into()
    }
    pub(super) fn add_vn(
        &mut self,
        vn_response: GetVnResults,
        executable_path: Option<PathBuf>,
    ) -> Result<()> {
        use std::fs::canonicalize;

        let mut vn = Vn::try_from(vn_response)?;
        if let Some(path) = executable_path {
            check_executable(&path)?;
            vn.set_executable_path(match canonicalize(&path) {
                Ok(p) => p,
                Err(_err) => {
                    return Err(AppError::ExecutableNotFound(path).into())
                }
            });
        }

        self.list.push(VnListItem {
            vn,
            state: button::State::new(),
        });
        self.list.sort_by(|a, b| a.vn.title.cmp(&b.vn.title));
        Ok(())
    }
    pub(crate) fn vn_exists(&self, vn_id: usize) -> bool {
        self.list.iter().any(|vn| vn.vn.id == vn_id)
    }
    pub(crate) fn delete_vn(&mut self, vn_id: usize) {
        self.list.retain(|vn| vn.vn.id != vn_id);
    }
}
