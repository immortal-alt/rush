use std::fs;

pub fn load_file(filename: &str) -> std::io::Result<String> {
    match fs::metadata(filename) {
        Ok(meta) if meta.is_dir() => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Path is a directory",
        )),
        _ => fs::read_to_string(filename),
    }
}
