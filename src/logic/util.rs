use std::path::PathBuf;

pub(crate) fn string_to_pathbuf(s: String) -> Option<PathBuf> {
    match s.is_empty() {
        true => None,
        false => Some(s.into()),
    }
}
