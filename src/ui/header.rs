use super::content::Content;
use crate::message::Message;
use crate::style;
use iced::{
    button, Align, Button, Container, Element, HorizontalAlignment, Length,
    Row, Text, VerticalAlignment,
};
use strum_macros::AsRefStr;

pub(crate) struct Header {
    headers: Vec<(Category, button::State)>,
}

impl Header {
    fn new(headers: Vec<(Category, button::State)>) -> Self {
        Self { headers }
    }
    pub(crate) fn from_content_list(content_list: &[Content]) -> Self {
        Self::new(
            content_list
                .iter()
                .map(|content| (content.get_category(), button::State::new()))
                .collect(),
        )
    }
    pub(crate) fn view(&mut self) -> Element<Message> {
        let content = self.headers.iter_mut().fold(
            Row::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(20)
                .spacing(20)
                .align_items(Align::Center),
            |content, category| {
                content.push(
                    Button::new(
                        &mut category.1,
                        Text::new(category.0.as_ref())
                            .size(40)
                            .vertical_alignment(VerticalAlignment::Center)
                            .horizontal_alignment(HorizontalAlignment::Center),
                    )
                    .style(style::dark::Header)
                    .on_press(Message::Header(category.0)),
                )
            },
        );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::FillPortion(1))
            .padding(0)
            .style(style::dark::Header)
            .into()
    }
}

#[derive(Debug, Copy, Clone, AsRefStr, PartialEq, Eq)]
pub(crate) enum Category {
    Vns,
    AddVn,
}
