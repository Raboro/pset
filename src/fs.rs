use std::fs::File;
use std::io::{Error, Write};
use std::path::{Path, PathBuf};

pub fn create_dir(path: &str) -> std::io::Result<()> {
    std::fs::create_dir(Path::new(path))
}

pub fn create_file(path_buf: PathBuf, content: String) -> Result<(), Error> {
    let file = File::create(path_buf);
    if file.is_err() {
        return Err(file.err().unwrap());
    }
    file.unwrap().write_all(content.as_bytes())
}
