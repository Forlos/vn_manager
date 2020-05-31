use crate::{
    logic::vn::Vn,
    ui::{add_vn::view::AddVnInput, header::Category},
};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Header(Category),
    Vn(Vn),
    AddVnInputChanged(String, AddVnInput),
    AddVn(String, Option<PathBuf>),
    PlayVn(PathBuf),
    DeleteVn(usize),
    Error(String),
    Empty(()),
}
