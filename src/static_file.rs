use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/<path..>", rank = 5)]
pub fn all(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}
