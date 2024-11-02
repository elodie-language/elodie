use std::path::PathBuf;

pub struct Source {
    content: String,
    path: Option<PathBuf>,
}