use crate::{
    contents::Contents, logic::path::create_app_dir, message::Message,
    ui::header::Header, update::handle_message,
};
use iced::{
    executor, Application, Column, Command, Container, Element, Length,
};
use std::net::TcpStream;
use vndb_rs::{sync::client::Client, API_URL};

pub(crate) struct App {
    header: Header,
    pub contents: Contents,
    pub vndb_client: Client<TcpStream>,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (App, Command<Message>) {
        let contents = Contents::init();
        let header = Header::from_content_list(&contents.content_list);
        let mut vndb_client = Client::new(
            TcpStream::connect(API_URL).expect("Could not connect to vndb API"),
        );
        vndb_client.login().expect("Could not login to vndb API");
        (
            App {
                header,
                contents,
                vndb_client,
            },
            Command::perform(async { create_app_dir() }, Message::Empty),
        )
    }

    fn title(&self) -> String {
        String::from("VN manager")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match handle_message(self, message) {
            Ok(command) => command,
            Err(err) => {
                log::error!("{:?}", err);
                Command::perform(async move { err.to_string() }, Message::Error)
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let content = Column::new()
            .push(self.header.view())
            .push(self.contents.get_current_content().view());

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::none()
    }
}
