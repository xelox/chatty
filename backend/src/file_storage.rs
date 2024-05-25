use std::{fs::{self, OpenOptions}, io::{self, Write}, path::Path};
use axum::body::Bytes;

static BASE_PATH_STR: &str = "/home/alex/dev/chatty/media";

pub fn save(path: &Path, bytes: &Bytes) -> io::Result<()> {
    let full_path = Path::new(BASE_PATH_STR).join(path);
    if let Some(parent_dir) = full_path.parent() {
        fs::create_dir_all(parent_dir)?;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&full_path)?;

    file.write_all(&bytes)?;

    Ok(())
}
