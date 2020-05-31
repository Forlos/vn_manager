pub mod view;

use crate::message::Message;
use iced::{Container, Element, Length, Row};

use view::AddVnView;

pub(crate) struct AddVnContent {
    pub(crate) view: AddVnView,
}

impl AddVnContent {
    pub(crate) fn init() -> Self {
        Self {
            view: AddVnView::init(),
        }
    }
    pub(crate) fn view(&mut self) -> Element<Message> {
        let content = Row::new()
            .spacing(10)
            .height(Length::Fill)
            .width(Length::Fill)
            .push(self.view.view());

        Container::new(content)
            .width(Length::Fill)
            .height(Length::FillPortion(9))
            .padding(0)
            .into()
    }
    pub(crate) fn set_vn_id(&mut self, vn_id: String) {
        self.view.set_vn_id(vn_id);
    }
    pub(crate) fn set_executable_path(&mut self, executable_path: String) {
        self.view.set_executable_path(executable_path);
    }
    pub(crate) fn set_error_msg(&mut self, error_msg: String) {
        self.view.set_error_msg(error_msg);
    }
    pub(crate) fn set_success_msg(&mut self, success_msg: String) {
        self.view.set_success_msg(success_msg);
    }
    pub(crate) fn clear_msg(&mut self) {
        self.view.clear_msg();
    }
}
