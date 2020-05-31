use crate::{
    message::Message,
    ui::{add_vn::AddVnContent, header::Category, vn::content::VnContent},
};
use iced::Element;

pub(crate) enum Content {
    Vn(VnContent),
    AddVn(AddVnContent),
}

impl Content {
    pub(crate) fn view(&mut self) -> Element<Message> {
        match self {
            Self::Vn(vn_content) => vn_content.view(),
            Self::AddVn(add_vn_content) => add_vn_content.view(),
        }
    }
    pub(crate) fn get_category(&self) -> Category {
        match self {
            Self::Vn(_) => Category::Vns,
            Self::AddVn(_) => Category::AddVn,
        }
    }
}
