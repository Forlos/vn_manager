use std::path::PathBuf;

pub(crate) fn string_to_pathbuf(s: String) -> Option<PathBuf> {
    if s.is_empty() {
        None
    } else {
        Some(s.into())
    }
}
