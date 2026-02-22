use std::{
    fs,
    io::Write, //
};

pub fn write_temp_file(file_name: &str, content: &[u8]) -> std::path::PathBuf {
    let dir = std::env::temp_dir();
    let path = dir.join(file_name);

    let mut file = fs::File::create(&path).expect("Failed to create temp file");
    file.write_all(content).expect("Failed to write temp file");

    path
}
