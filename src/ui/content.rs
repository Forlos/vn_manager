use crate::{
    message::Message,
    ui::{add_vn::AddVnContent, header::Category, vn::content::VnContent},
};
use iced::{Container, Element, Length, Row};

pub(crate) enum Content {
    Vn(VnContent),
    AddVn(AddVnContent),
    NoContent,
}

impl Content {
    pub(crate) fn view(&mut self) -> Element<Message> {
        match self {
            Self::Vn(vn_content) => vn_content.view(),
            Self::AddVn(add_vn_content) => add_vn_content.view(),
            Self::NoContent => {
                let content =
                    Row::new().height(Length::Fill).width(Length::Fill);

                Container::new(content)
                    .width(Length::Fill)
                    .height(Length::FillPortion(9))
                    .padding(0)
                    .into()
            }
        }
    }
    pub(crate) fn get_category(&self) -> Category {
        match self {
            Self::Vn(_) => Category::Vns,
            Self::AddVn(_) => Category::AddVn,
            Self::NoContent => Category::NoCategory,
        }
    }
}
