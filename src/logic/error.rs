use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum AppError {
    #[error("Vn with id: {0} already exists")]
    VnExists(usize),
    #[error("Vn with id: {0} not found")]
    VnNotFound(usize),
    #[error("{0} is not valid vn id")]
    VnInvalidId(String),
    #[error(
        "Vn executable not found: {0}. Please provide full path to executable."
    )]
    VnExecutableNotFound(PathBuf),
    #[error("Could not start Vn")]
    VnPlayError,
    #[error("File is not valid executable")]
    VnExecutableInvalid,
}

pub(crate) trait ErrorView {
    type View;
    fn set_error_msg(view: &mut Self::View, error_msg: String);
}
