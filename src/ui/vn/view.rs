use crate::{logic::vn::Vn, message::Message, style};
use iced::{
    button, image, scrollable, Button, Column, Container, Element, Image,
    Length, Row, Scrollable, Text,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub(crate) struct VnView {
    vn_info: Option<VnInfo>,

    #[serde(skip)]
    error_msg: String,

    #[serde(skip)]
    scrollable_state: scrollable::State,
    #[serde(skip)]
    play_button_state: button::State,
    #[serde(skip)]
    delete_button_state: button::State,
}

#[serde(default)]
#[derive(Serialize, Deserialize, Default)]
struct VnInfo {
    vn_id: usize,
    title: String,
    original_title: Option<String>,
    released: Option<String>,
    languages: Vec<String>,
    description: String,
    image_path: Option<PathBuf>,
    executable_path: Option<PathBuf>,
}

impl VnView {
    pub(crate) fn init() -> Self {
        VnView {
            vn_info: None,

            error_msg: String::default(),

            scrollable_state: scrollable::State::new(),
            play_button_state: button::State::new(),
            delete_button_state: button::State::new(),
        }
    }
    pub(crate) fn view(&mut self) -> Element<Message> {
        let play_button =
            Button::new(&mut self.play_button_state, Text::new("Play"))
                .style(style::dark::Dark);
        let content = match &self.vn_info {
            Some(vn) => Column::new()
                .padding(5)
                .spacing(1)
                .push(Text::new(&vn.title).size(48))
                .push(if let Some(title) = &vn.original_title {
                    Text::new(title)
                } else {
                    Text::new("")
                })
                .push(if let Some(release) = &vn.released {
                    Text::new(format!("Released: {}", release))
                } else {
                    Text::new("")
                })
                .push(vn.languages.iter().fold(
                    Row::new().spacing(10).push(Text::new("Languages: ")),
                    |row, lang| row.push(Text::new(lang)),
                ))
                .push(
                    Row::new()
                        .spacing(5)
                        .push(
                            Column::new()
                                .push(Image::new(image::Handle::from_path(
                                    vn.image_path.clone().unwrap(),
                                )))
                                .push(
                                    Row::new()
                                        .spacing(10)
                                        .push(
                                            if let Some(path) =
                                                &vn.executable_path
                                            {
                                                play_button.on_press(
                                                    Message::PlayVn(
                                                        path.clone(),
                                                    ),
                                                )
                                            } else {
                                                play_button
                                            },
                                        )
                                        .push(
                                            Button::new(
                                                &mut self.delete_button_state,
                                                Text::new("Delete"),
                                            )
                                            .on_press(Message::DeleteVn(
                                                vn.vn_id,
                                            ))
                                            .style(style::dark::Dark),
                                        ),
                                ),
                        )
                        .push(Text::new(&vn.description)),
                )
                .push(
                    Container::new(Text::new(&self.error_msg))
                        .style(style::dark::Error),
                ),
            None => Column::new().push(Text::new("No Vns found.").size(60)),
        };

        Container::new(
            Scrollable::new(&mut self.scrollable_state).push(content),
        )
        .width(Length::FillPortion(17))
        .height(Length::Fill)
        .padding(0)
        .style(style::dark::Dark)
        .into()
    }
    pub(crate) fn set_error_msg(&mut self, error_msg: String) {
        self.error_msg = error_msg;
    }
}

impl From<Vn> for VnView {
    fn from(vn: Vn) -> Self {
        Self {
            vn_info: Some(VnInfo {
                vn_id: vn.id,
                title: vn.title,
                original_title: vn.original_title,
                released: vn.released,
                languages: vn.languages,
                description: vn.description,
                image_path: vn.image_path,
                executable_path: vn.executable_path,
            }),

            error_msg: String::default(),

            scrollable_state: scrollable::State::new(),
            play_button_state: button::State::new(),
            delete_button_state: button::State::new(),
        }
    }
}

impl Default for VnView {
    fn default() -> Self {
        VnView::init()
    }
}
