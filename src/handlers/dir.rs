use std::path::PathBuf;

pub fn dir_str(current_dir: PathBuf) -> String {
    current_dir.to_str().unwrap().to_string()
}