use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct CopyRequest {
    pub from: PathBuf,
    pub to: PathBuf,
}

impl CopyRequest {
    pub fn add_file(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Self {
        let from = src.as_ref().to_owned();
        let to = dst.as_ref().to_owned();
        let req = CopyRequest { from, to };

        req
    }
}
