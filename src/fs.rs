use std::fs::{self, File};
use std::io::{Error, Write};
use std::path::{Path, PathBuf};

pub fn create_dir(path: &str) -> std::io::Result<()> {
    std::fs::create_dir(Path::new(path))
}

pub fn create_nested_dirs(full_path: &str, base: &str) {
    let paths: Vec<&str> = full_path.split('/').collect();
    let mut sub_path: String = format!("./{}", base);

    for path in paths {
        sub_path.push_str(&format!("/{}", path));
        fs::create_dir(&sub_path).unwrap_or_else(|_| panic!("Cannot create folder {}", sub_path));
    }
}

pub fn create_file(path_buf: PathBuf, content: String) -> Result<(), Error> {
    let file = File::create(path_buf);
    if file.is_err() {
        return Err(file.err().unwrap());
    }
    file.unwrap().write_all(content.as_bytes())
}
