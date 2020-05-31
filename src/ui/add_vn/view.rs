use crate::{logic::util::string_to_pathbuf, message::Message, style};
use iced::{
    button, text_input, Button, Column, Container, Element, Length, Space,
    Text, TextInput,
};

pub(crate) struct AddVnView {
    inputs: Inputs,
    error_msg: String,
    success_msg: String,
    submit_state: button::State,
}

struct Inputs {
    vn_id: String,
    executable_path: String,

    vn_id_state: text_input::State,
    executable_path_state: text_input::State,
}

impl AddVnView {
    pub(crate) fn init() -> Self {
        Self {
            inputs: Inputs {
                vn_id: String::default(),
                executable_path: String::default(),
                vn_id_state: text_input::State::new(),
                executable_path_state: text_input::State::new(),
            },
            error_msg: String::default(),
            success_msg: String::default(),
            submit_state: button::State::new(),
        }
    }
    pub(crate) fn view(&mut self) -> Element<Message> {
        let content = Column::new()
            .padding(5)
            .spacing(5)
            .push(Text::new("Add vn").size(32))
            .push(
                TextInput::new(
                    &mut self.inputs.vn_id_state,
                    "VN id...",
                    &self.inputs.vn_id,
                    |input| {
                        Message::AddVnInputChanged(input, AddVnInput::VnName)
                    },
                )
                .padding(5)
                .style(style::dark::Dark)
                .width(Length::Units(300)),
            )
            .push(
                TextInput::new(
                    &mut self.inputs.executable_path_state,
                    "Executable path...",
                    &self.inputs.executable_path,
                    |input| {
                        Message::AddVnInputChanged(
                            input,
                            AddVnInput::ExecutablePath,
                        )
                    },
                )
                .padding(5)
                .style(style::dark::Dark)
                .width(Length::Units(300)),
            )
            .push(
                Button::new(&mut self.submit_state, Text::new("Add"))
                    .style(style::dark::Dark)
                    .on_press(Message::AddVn(
                        self.inputs.vn_id.clone(),
                        string_to_pathbuf(self.inputs.executable_path.clone()),
                    )),
            )
            .push(if !self.error_msg.is_empty() {
                Container::new(Text::new(&self.error_msg))
                    .style(style::dark::Error)
            } else if !self.success_msg.is_empty() {
                Container::new(Text::new(&self.success_msg))
                    .style(style::dark::Success)
            } else {
                Container::new(Space::new(Length::Units(0), Length::Units(0)))
            });

        Container::new(content)
            .width(Length::FillPortion(9))
            .height(Length::Fill)
            .padding(5)
            .style(style::dark::Dark)
            .into()
    }
    pub(crate) fn set_vn_id(&mut self, vn_id: String) {
        self.inputs.vn_id = vn_id;
    }
    pub(crate) fn set_executable_path(&mut self, executable_path: String) {
        self.inputs.executable_path = executable_path;
    }
    pub(crate) fn set_error_msg(&mut self, error_msg: String) {
        self.error_msg = error_msg;
    }
    pub(crate) fn set_success_msg(&mut self, success_msg: String) {
        self.success_msg = success_msg;
    }
    pub(crate) fn clear_msg(&mut self) {
        self.error_msg = "".into();
        self.success_msg = "".into();
    }
}

#[derive(Debug, Clone)]
pub(crate) enum AddVnInput {
    VnName,
    ExecutablePath,
}
