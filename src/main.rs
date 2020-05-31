mod app;
mod contents;
mod logic;
mod message;
mod style;
mod ui;
mod update;

use crate::app::App;
use iced::{Application, Settings};

pub fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter_module("iced_wgpu", log::LevelFilter::Warn)
        .filter_module("winit", log::LevelFilter::Warn)
        .filter_module("iced_winit", log::LevelFilter::Warn)
        .filter_module("wgpu_native", log::LevelFilter::Warn)
        .filter_module("vndb_rs", log::LevelFilter::Debug)
        .filter(None, log::LevelFilter::Info)
        .init();
    App::run(Settings {
        // TODO this is workaround until iced supports fallback fonts
        // See: https://github.com/hecrj/iced/issues/33
        default_font: Some(include_bytes!(
            "../fonts/RictyDiminished-with-FiraCode-Regular.ttf"
        )),
        antialiasing: true,
        ..Default::default()
    })
}
